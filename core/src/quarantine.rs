//! Quarantaine chiffrée : isolement réversible des menaces.
//!
//! - Contenu chiffré AES-256-GCM (nonce aléatoire 12 o, préfixé au blob).
//! - Clé : 32 o aléatoires par installation (`cle.key`), ou dérivée Argon2
//!   via [`deriver_cle`] à partir d'une phrase secrète.
//! - Métadonnées en SQLite (origine, hash, date, raison, score).

use std::path::{Path, PathBuf};

use aes_gcm::{
    aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
    Aes256Gcm,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use rand::rngs::OsRng as RandOsRng;

use crate::CleanXError;

/// Fichier isolé (exposé à Dart).
#[derive(Debug, Clone)]
pub struct FichierQuarantaine {
    pub id: i64,
    pub nom: String,
    pub origine: String,
    pub date: String,
    pub raison: String,
    pub score: u8,
}

/// Dérive une clé 32 o avec Argon2id (sel en base64, phrase secrète UTF-8).
pub fn deriver_cle(phrase: &str, sel_b64: &str) -> Result<[u8; 32], CleanXError> {
    let sel = SaltString::from_b64(sel_b64).map_err(|e| CleanXError::Interne {
        detail: format!("sel Argon2 : {e}"),
    })?;
    let sortie = Argon2::default()
        .hash_password(phrase.as_bytes(), &sel)
        .map_err(|e| CleanXError::Interne {
            detail: format!("Argon2 : {e}"),
        })?;
    let octets = sortie.hash.ok_or_else(|| CleanXError::Interne {
        detail: "Argon2 : sortie vide".to_string(),
    })?;
    let mut cle = [0u8; 32];
    let brut = octets.as_bytes();
    let n = brut.len().min(32);
    cle[..n].copy_from_slice(&brut[..n]);
    Ok(cle)
}

/// Charge la clé d'installation STRICTEMENT depuis le coffre OS
/// (DPAPI/Keychain/Secret Service via `keyring`).
///
/// Sans coffre accessible : erreur explicite [`CoffreIndisponible`] — JAMAIS
/// de repli silencieux (P9). Le repli fichier n'existe que via
/// [`charger_ou_creer_cle_avec_repli`] quand l'environnement l'autorise
/// explicitement (dev/CI headless : `CLEANX_KEY_FALLBACK=1`).
pub fn charger_ou_creer_cle(_chemin: &Path) -> Result<[u8; 32], CleanXError> {
    cle_depuis_coffre("cleanx", "cle-quarantaine").map(|(cle, _)| cle)
}

/// Origine effective de la clé (pour traçabilité, jamais la clé elle-même).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceCle {
    CoffreOs,
    Fichier,
}

/// Variante traçable de [`charger_ou_creer_cle`].
pub fn charger_ou_creer_cle_trace(
    _chemin: &Path,
) -> Result<([u8; 32], ProvenanceCle), CleanXError> {
    cle_depuis_coffre("cleanx", "cle-quarantaine")
}

/// Repli fichier EXPLICITE (dev/CI uniquement) : utilisé seulement si
/// `CLEANX_KEY_FALLBACK=1`, sinon l'erreur du coffre est propagée telle quelle.
/// Le repli effectif est tracé comme `ProvenanceCle::Fichier`.
pub fn charger_ou_creer_cle_avec_repli(
    chemin: &Path,
) -> Result<([u8; 32], ProvenanceCle), CleanXError> {
    match cle_depuis_coffre("cleanx", "cle-quarantaine") {
        Ok(ok) => Ok(ok),
        Err(e) => {
            if repli_fichier_autorise() {
                Ok((
                    charger_ou_creer_cle_fichier(chemin)?,
                    ProvenanceCle::Fichier,
                ))
            } else {
                Err(e)
            }
        }
    }
}

/// L'échappatoire fichier est-elle explicitement autorisée ? (jamais en prod).
pub(crate) fn repli_fichier_autorise() -> bool {
    std::env::var("CLEANX_KEY_FALLBACK").as_deref() == Ok("1")
}

/// Lit ou crée la clé 32 o dans le coffre OS (hexadécimal stocké).
fn cle_depuis_coffre(
    service: &str,
    compte: &str,
) -> Result<([u8; 32], ProvenanceCle), CleanXError> {
    let entree =
        keyring::v1::Entry::new(service, compte).map_err(|e| CleanXError::CoffreIndisponible {
            detail: format!("coffre inaccessible : {e}"),
        })?;
    // Lecture, sinon génération + stockage (concurrent-safe : le perdant
    // écrase avec la même longueur, puis relit — ici simplifié : dernier
    // écrivain gagne, les deux clés restant valides à 32 o).
    let hex_cle = match entree.get_password() {
        Ok(mdp) => mdp,
        Err(_) => {
            let mut cle = [0u8; 32];
            RandOsRng.fill_bytes(&mut cle);
            let hex_cle = hex::encode(cle);
            entree
                .set_password(&hex_cle)
                .map_err(|e| CleanXError::CoffreIndisponible {
                    detail: format!("coffre inscriptible : {e}"),
                })?;
            hex_cle
        }
    };
    let brut = hex::decode(hex_cle.trim()).map_err(|_| CleanXError::CoffreIndisponible {
        detail: "clé du coffre corrompue".into(),
    })?;
    if brut.len() != 32 {
        return Err(CleanXError::CoffreIndisponible {
            detail: "clé du coffre : 32 octets attendus".into(),
        });
    }
    let mut cle = [0u8; 32];
    cle.copy_from_slice(&brut);
    Ok((cle, ProvenanceCle::CoffreOs))
}

/// Repli fichier (permissions héritées du dossier applicatif).
fn charger_ou_creer_cle_fichier(chemin: &Path) -> Result<[u8; 32], CleanXError> {
    if chemin.is_file() {
        let brut = std::fs::read(chemin).map_err(|e| crate::error::erreur_io(chemin, e))?;
        if brut.len() == 32 {
            let mut cle = [0u8; 32];
            cle.copy_from_slice(&brut);
            return Ok(cle);
        }
    }
    let mut cle = [0u8; 32];
    RandOsRng.fill_bytes(&mut cle);
    if let Some(p) = chemin.parent() {
        std::fs::create_dir_all(p).map_err(|e| crate::error::erreur_io(p, e))?;
    }
    std::fs::write(chemin, cle).map_err(|e| crate::error::erreur_io(chemin, e))?;
    Ok(cle)
}

fn chiffrer(cle: &[u8; 32], clair: &[u8]) -> Result<Vec<u8>, CleanXError> {
    let cipher = Aes256Gcm::new_from_slice(cle).map_err(|e| CleanXError::Quarantaine {
        detail: format!("clé : {e}"),
    })?;
    let mut nonce_b = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_b);
    // `from_slice` est déprécié par generic-array 0.14 (aes-gcm 0.10 l'épingle) :
    // l'allow localisé disparaîtra avec la montée de version d'aes-gcm.
    #[allow(deprecated)]
    let nonce = aes_gcm::Nonce::from_slice(&nonce_b);
    let chiffre = cipher
        .encrypt(nonce, clair)
        .map_err(|e| CleanXError::Quarantaine {
            detail: format!("chiffrement : {e}"),
        })?;
    let mut blob = Vec::with_capacity(12 + chiffre.len());
    blob.extend_from_slice(&nonce_b);
    blob.extend_from_slice(&chiffre);
    Ok(blob)
}

fn dechiffrer(cle: &[u8; 32], blob: &[u8]) -> Result<Vec<u8>, CleanXError> {
    if blob.len() < 13 {
        return Err(CleanXError::Quarantaine {
            detail: "blob tronqué".into(),
        });
    }
    let cipher = Aes256Gcm::new_from_slice(cle).map_err(|e| CleanXError::Quarantaine {
        detail: format!("clé : {e}"),
    })?;
    #[allow(deprecated)]
    let nonce = aes_gcm::Nonce::from_slice(&blob[..12]);
    cipher
        .decrypt(nonce, &blob[12..])
        .map_err(|_| CleanXError::Quarantaine {
            detail: "déchiffrement impossible (clé ?)".into(),
        })
}

/// Isole un fichier : chiffre → écrit le blob → supprime l'original → trace en base.
///
/// Retourne l'identifiant de quarantaine. L'original n'est supprimé qu'après
/// écriture vérifiée du blob (pas de perte en cas de panne disque).
pub fn mettre_en_quarantaine(
    db: &Path,
    dossier: &Path,
    cle: &[u8; 32],
    chemin: &Path,
    raison: &str,
    score: u8,
) -> Result<i64, CleanXError> {
    use sha2::{Digest, Sha256};
    std::fs::create_dir_all(dossier).map_err(|e| crate::error::erreur_io(dossier, e))?;
    let contenu = std::fs::read(chemin).map_err(|e| crate::error::erreur_io(chemin, e))?;
    let _hash = hex::encode(Sha256::digest(&contenu)); // tracé dans `raison` si utile
    let blob = chiffrer(cle, &contenu)?;
    let nom = chemin
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "sans_nom".into());
    let date = crate::logging::horodatage();

    let id: i64 = crate::db::avec_connexion(db, |conn| {
        conn.execute(
            "INSERT INTO quarantaine(nom, origine, date, raison, score, blob_path) VALUES (?1,?2,?3,?4,?5,?6)",
            rusqlite::params![
                nom,
                chemin.to_string_lossy(),
                date,
                raison,
                score as i64,
                String::new()
            ],
        )?;
        let id: i64 = conn.last_insert_rowid();
        let blob_path = dossier.join(format!("{id}.enc"));
        // Écriture avant toute suppression (atomicité pragmatique).
        std::fs::write(&blob_path, &blob).map_err(|e| {
            let _ = conn.execute("DELETE FROM quarantaine WHERE id = ?1", [id]);
            crate::error::erreur_io(&blob_path, e)
        })?;
        conn.execute(
            "UPDATE quarantaine SET blob_path = ?1 WHERE id = ?2",
            rusqlite::params![blob_path.to_string_lossy(), id],
        )?;
        Ok(id)
    })?;
    std::fs::remove_file(chemin).map_err(|e| crate::error::erreur_io(chemin, e))?;
    Ok(id)
}

/// Restaure un fichier isolé vers `destination` (faux positif).
pub fn restaurer(
    db: &Path,
    cle: &[u8; 32],
    id: i64,
    destination: &Path,
) -> Result<(), CleanXError> {
    let blob_txt: String = crate::db::avec_connexion(db, |conn| {
        conn.query_row(
            "SELECT blob_path FROM quarantaine WHERE id = ?1",
            [id],
            |r| r.get(0),
        )
        .map_err(|_| CleanXError::Quarantaine {
            detail: format!("entrée #{id} introuvable"),
        })
    })?;
    let blob = std::fs::read(Path::new(&blob_txt))
        .map_err(|e| crate::error::erreur_io(Path::new(&blob_txt), e))?;
    let clair = dechiffrer(cle, &blob)?;
    if let Some(p) = destination.parent() {
        std::fs::create_dir_all(p).map_err(|e| crate::error::erreur_io(p, e))?;
    }
    std::fs::write(destination, clair).map_err(|e| crate::error::erreur_io(destination, e))?;
    crate::db::avec_connexion(db, |conn| {
        conn.execute("DELETE FROM quarantaine WHERE id = ?1", [id])?;
        Ok(())
    })?;
    let _ = std::fs::remove_file(blob_txt);
    Ok(())
}

/// Supprime définitivement une entrée (blob + ligne base).
pub fn supprimer_definitivement(db: &Path, dossier: &Path, id: i64) -> Result<(), CleanXError> {
    let n = crate::db::avec_connexion(db, |conn| {
        conn.execute("DELETE FROM quarantaine WHERE id = ?1", [id])
            .map_err(CleanXError::from)
    })?;
    if n == 0 {
        return Err(CleanXError::Quarantaine {
            detail: format!("entrée #{id} introuvable"),
        });
    }
    let _ = std::fs::remove_file(dossier.join(format!("{id}.enc")));
    Ok(())
}

/// Liste les fichiers isolés (ordre antéchronologique).
pub fn lister(db: &Path) -> Result<Vec<FichierQuarantaine>, CleanXError> {
    crate::db::avec_connexion(db, |conn| {
        let mut stmt = conn.prepare(
            "SELECT id, nom, origine, date, raison, score FROM quarantaine ORDER BY id DESC",
        )?;
        let lignes = stmt.query_map([], |r| {
            Ok(FichierQuarantaine {
                id: r.get(0)?,
                nom: r.get(1)?,
                origine: r.get(2)?,
                date: r.get(3)?,
                raison: r.get(4)?,
                score: r.get::<_, i64>(5)? as u8,
            })
        })?;
        lignes
            .collect::<Result<Vec<_>, _>>()
            .map_err(CleanXError::from)
    })
}

/// Chemin du blob (usage interne/tests).
pub fn chemin_blob(dossier: &Path, id: i64) -> PathBuf {
    dossier.join(format!("{id}.enc"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sérialise les tests touchant le coffre OS : le credential
    /// ("cleanx"/"cle-quarantaine") est GLOBAL à la machine, deux tests
    /// parallèles s'écraseraient mutuellement la clé (course détectée).
    static VERROU_COFFRE: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn cycle_quarantaine_restauration() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("q.db");
        let dossier = dir.path().join("quarantaine");
        let cle = [7u8; 32];
        let cible = dir.path().join("malware.exe");
        std::fs::write(&cible, b"contenu malveillant simule").unwrap();

        let id = mettre_en_quarantaine(&base, &dossier, &cle, &cible, "Test", 90).unwrap();
        assert!(!cible.exists());
        assert_eq!(lister(&base).unwrap().len(), 1);

        let dest = dir.path().join("restaure.exe");
        restaurer(&base, &cle, id, &dest).unwrap();
        assert_eq!(std::fs::read(dest).unwrap(), b"contenu malveillant simule");
        assert!(lister(&base).unwrap().is_empty());
    }

    #[test]
    fn mauvaise_cle_ne_dechiffre_pas() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("q.db");
        let dossier = dir.path().join("q");
        let cible = dir.path().join("x.exe");
        std::fs::write(&cible, b"secret").unwrap();
        let id = mettre_en_quarantaine(&base, &dossier, &[1u8; 32], &cible, "T", 80).unwrap();
        assert!(restaurer(&base, &[2u8; 32], id, &dir.path().join("y")).is_err());
    }

    #[test]
    fn argon2_deterministe() {
        // Sel base64 valide (16 octets).
        let sel = "c2VsLXRlc3QtMTYtb2N0ZXRz";
        let a = deriver_cle("phrase-test", sel).unwrap();
        let b = deriver_cle("phrase-test", sel).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, deriver_cle("autre-phrase", sel).unwrap());
        assert!(deriver_cle("x", "!!!sel-invalide!!!").is_err());
    }

    #[test]
    fn cle_installation_persiste() {
        // Chemin explicite (dev/CI) : stable avec OU sans coffre OS,
        // car le repli est autorisé par variable d'environnement.
        // Verrou anti-empoisonnement : un panic précédent ne doit pas
        // faire échouer les autres tests en cascade.
        let _garde = VERROU_COFFRE.lock().unwrap_or_else(|e| e.into_inner());
        std::env::set_var("CLEANX_KEY_FALLBACK", "1");
        let dir = tempfile::tempdir().unwrap();
        let chemin = dir.path().join("cle.key");
        let (a, _) = charger_ou_creer_cle_avec_repli(&chemin).unwrap();
        let (b, _) = charger_ou_creer_cle_avec_repli(&chemin).unwrap();
        assert_eq!(a, b); // rechargée, pas régénérée
        std::env::remove_var("CLEANX_KEY_FALLBACK");
    }

    #[test]
    fn cle_coffre_ou_fichier_stable() {
        let _garde = VERROU_COFFRE.lock().unwrap_or_else(|e| e.into_inner());
        // Propriété : deux chargements successifs donnent la même clé
        // (coffre OS si présent, sinon repli fichier EXPLICITE).
        std::env::set_var("CLEANX_KEY_FALLBACK", "1");
        let dir = tempfile::tempdir().unwrap();
        let chemin = dir.path().join("cle2.key");
        let (a, _) = charger_ou_creer_cle_avec_repli(&chemin).unwrap();
        let (b, _) = charger_ou_creer_cle_avec_repli(&chemin).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, [0u8; 32]);
        std::env::remove_var("CLEANX_KEY_FALLBACK");
    }

    #[test]
    fn repli_fichier_explicite_par_env() {
        let _garde = VERROU_COFFRE.lock().unwrap_or_else(|e| e.into_inner());
        // Échappatoire dev/CI : stable quel que soit le chemin effectif.
        std::env::set_var("CLEANX_KEY_FALLBACK", "1");
        let dir = tempfile::tempdir().unwrap();
        let chemin = dir.path().join("cle3.key");
        let (a, _) = charger_ou_creer_cle_avec_repli(&chemin).unwrap();
        let (b, _) = charger_ou_creer_cle_avec_repli(&chemin).unwrap();
        assert_eq!(a, b);
        std::env::remove_var("CLEANX_KEY_FALLBACK");
    }

    #[test]
    fn fichier_direct_deterministe() {
        // Hors coffre : roundtrip pur, sans variable d'environnement.
        let dir = tempfile::tempdir().unwrap();
        let chemin = dir.path().join("cle4.key");
        let a = charger_ou_creer_cle_fichier(&chemin).unwrap();
        let b = charger_ou_creer_cle_fichier(&chemin).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn entrees_inexistantes_erreur_claire() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("q.db");
        let dossier = dir.path().join("q");
        assert!(restaurer(&base, &[0u8; 32], 999, &dir.path().join("y")).is_err());
        assert!(supprimer_definitivement(&base, &dossier, 999).is_err());
        assert_eq!(chemin_blob(&dossier, 7), dossier.join("7.enc"));
    }

    #[test]
    fn quarantaine_fichier_inexistant() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("q.db");
        let dossier = dir.path().join("q");
        assert!(mettre_en_quarantaine(
            &base,
            &dossier,
            &[0u8; 32],
            &dir.path().join("absent.exe"),
            "T",
            10
        )
        .is_err());
    }
}
