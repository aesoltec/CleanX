//! Scan de signatures : SHA-256 asynchrone + base SQLite + mises à jour signées.
//!
//! - Lecture par blocs de 64 Ko via `tokio::fs` (jamais de fichier entier en RAM).
//! - Erreurs E/S normalisées ([`erreur_io`](crate::error::erreur_io)).
//! - Mise à jour : paquet vérifié Ed25519 (mock de transport, crypto réelle).

use std::path::Path;

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;

use crate::CleanXError;

/// Taille des blocs de lecture (64 Ko : bon compromis disque/mémoire).
const TAILLE_BLOC: usize = 64 * 1024;

/// Source d'une détection (B14) : détermine la CONFIANCE, donc qui peut agir.
/// - `SignatureConnue` : hash exact d'un malware confirmé → 100.
/// - `Generique` : motif d'octets (famille) → 70.
/// - `Heuristique` : comportement suspect seul → 30–50 (moitié du score).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceMenace {
    SignatureConnue,
    Generique,
    Heuristique,
}

/// Confiance 0–100 associée à une source (calibrage documenté, 4bis).
pub fn confiance(source: SourceMenace, score_heuristique: u8) -> u8 {
    match source {
        SourceMenace::SignatureConnue => 100,
        SourceMenace::Generique => 70,
        // Heuristique seule : 35–50 (score 70–100 → moitié, plancher 30).
        SourceMenace::Heuristique => (score_heuristique / 2).max(30),
    }
}

/// Verdict de l'analyse par signatures (jamais d'exception : voir `erreur`).
#[derive(Debug, Clone)]
pub struct VerdictSignatures {
    /// Fichier analysé (tel que fourni).
    pub chemin: String,
    /// SHA-256 hexadécimal, ou `None` si le fichier est illisible.
    pub sha256: Option<String>,
    /// Nom de la menace si le hash est connu.
    pub menace: Option<String>,
    /// Raison de l'échec de lecture (verrou, permission…), le cas échéant.
    pub erreur: Option<String>,
}

/// Calcule le SHA-256 d'un fichier sans bloquer exagérément (async, 64 Ko/bloc).
pub async fn sha256_fichier(chemin: &Path) -> Result<String, CleanXError> {
    let mut fichier = tokio::fs::File::open(chemin)
        .await
        .map_err(|e| crate::error::erreur_io(chemin, e))?;
    let mut ctx = Sha256::new();
    let mut tampon = vec![0u8; TAILLE_BLOC];
    loop {
        let n = fichier
            .read(&mut tampon)
            .await
            .map_err(|e| crate::error::erreur_io(chemin, e))?;
        if n == 0 {
            break;
        }
        ctx.update(&tampon[..n]);
    }
    Ok(hex::encode(ctx.finalize()))
}

/// Scanne un fichier par signatures. `db` = chemin de la base SQLite.
///
/// Ne lève que les erreurs de base ; les erreurs de lecture du fichier sont
/// encapsulées dans [`VerdictSignatures::erreur`] (le scan continue).
pub async fn verifier_signature(
    db: &Path,
    chemin: &Path,
) -> Result<VerdictSignatures, CleanXError> {
    let chemin_txt = chemin.to_string_lossy().into_owned();
    let sha = match sha256_fichier(chemin).await {
        Ok(s) => s,
        Err(e) => {
            return Ok(VerdictSignatures {
                chemin: chemin_txt,
                sha256: None,
                menace: None,
                erreur: Some(e.to_string()),
            })
        }
    };
    let hash = sha.clone();
    // `spawn_blocking` exige `'static` : on clone le chemin de base.
    let db = db.to_path_buf();
    let menace = tokio::task::spawn_blocking(move || -> Result<Option<String>, CleanXError> {
        crate::db::avec_connexion(&db, |conn| crate::db::chercher_menace(conn, &hash))
    })
    .await
    .map_err(|e| CleanXError::Interne {
        detail: format!("worker base : {e}"),
    })??;
    Ok(VerdictSignatures {
        chemin: chemin_txt,
        sha256: Some(sha),
        menace,
        erreur: None,
    })
}

/// Vérifie un paquet de signatures avec Ed25519 (transport mock, crypto réelle).
///
/// Retourne `true` si la signature est valide pour `paquet` avec `cle_publique_hex`.
pub fn verifier_paquet(
    paquet: &[u8],
    signature_hex: &str,
    cle_publique_hex: &str,
) -> Result<bool, CleanXError> {
    let sig_bytes = hex::decode(signature_hex).map_err(|_| CleanXError::Interne {
        detail: "signature hex invalide".into(),
    })?;
    let cle_bytes = hex::decode(cle_publique_hex).map_err(|_| CleanXError::Interne {
        detail: "clé publique hex invalide".into(),
    })?;
    let sig_arr: [u8; 64] = sig_bytes.try_into().map_err(|_| CleanXError::Interne {
        detail: "signature : 64 octets attendus".into(),
    })?;
    let cle_arr: [u8; 32] = cle_bytes.try_into().map_err(|_| CleanXError::Interne {
        detail: "clé publique : 32 octets attendus".into(),
    })?;
    let signature = Signature::from_bytes(&sig_arr);
    let cle = VerifyingKey::from_bytes(&cle_arr).map_err(|e| CleanXError::Interne {
        detail: format!("clé Ed25519 : {e}"),
    })?;
    Ok(cle.verify(paquet, &signature).is_ok())
}

/// Entrée de signature distante (format du dépôt HTTPS).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntreeDepot {
    /// SHA-256 hexadécimal (64 caractères, validé à l'import).
    pub hash: String,
    /// Nom de menace (1–256 caractères).
    pub nom: String,
}

/// Paquet distant : signatures + signature Ed25519 des octets canoniques
/// (`serde_json::to_vec(&signatures)`, ordre des champs stable).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaquetDepot {
    /// Version monotone du paquet (mise à jour différentielle).
    pub version: u64,
    pub signatures: Vec<EntreeDepot>,
    /// Signature hexadécimale (64 octets).
    pub signature: String,
}

/// Télécharge un paquet HTTPS, vérifie Ed25519, importe en transaction.
///
/// Mise à jour DIFFÉRENTIELLE : la version locale (table `config`,
/// clé `version_signatures`) est envoyée en `?depuis=N` ; le dépôt ne
/// retourne que les entrées plus récentes. La version stockée avance
/// uniquement après import vérifié et commité.
/// Atomicité : `ROLLBACK` automatique si la signature est invalide ou si une
/// entrée est malformée (la transaction est annulée à la destruction).
/// Retourne le nombre de signatures AJOUTÉES (ignorées si déjà connues).
pub async fn mettre_a_jour_depuis_depot(
    db: &Path,
    url: &str,
    cle_publique_hex: &str,
) -> Result<u32, CleanXError> {
    let version_locale = lire_version(db)?;
    let separateur = if url.contains('?') { '&' } else { '?' };
    let url_complete = format!("{url}{separateur}depuis={version_locale}");
    let corps = reqwest::get(url_complete)
        .await
        .map_err(|e| CleanXError::Interne {
            detail: format!("dépôt injoignable : {e}"),
        })?
        .error_for_status()
        .map_err(|e| CleanXError::Interne {
            detail: format!("dépôt HTTP : {e}"),
        })?
        .bytes()
        .await
        .map_err(|e| CleanXError::Interne {
            detail: format!("lecture dépôt : {e}"),
        })?;
    let paquet: PaquetDepot = serde_json::from_slice(&corps).map_err(|e| CleanXError::Interne {
        detail: format!("paquet malformé : {e}"),
    })?;

    // Rejet anti-rollback : un paquet plus vieux que le local est refusé.
    if paquet.version < version_locale {
        return Err(CleanXError::Interne {
            detail: format!("paquet obsolète (v{} < v{version_locale})", paquet.version),
        });
    }

    // Octets canoniques re-sérialisés (même struct, même ordre) puis vérifiés.
    let canoniques = serde_json::to_vec(&paquet.signatures)?;
    if !verifier_paquet(&canoniques, &paquet.signature, cle_publique_hex)? {
        return Err(CleanXError::Interne {
            detail: "signature Ed25519 invalide : paquet rejeté".into(),
        });
    }

    // Transaction exige `&mut Connection` : connexion dédiée (mise à jour
    // rare, pas la voie chaude du scan qui utilise le pool).
    let mut conn = crate::db::ouvrir(db)?;
    let tx = conn.transaction()?;
    let mut ajoutees = 0u32;
    for entree in &paquet.signatures {
        valider_entree(entree)?;
        let n = tx.execute(
            "INSERT OR IGNORE INTO signatures(hash, nom) VALUES (?1, ?2)",
            (entree.hash.to_lowercase(), entree.nom.clone()),
        )?;
        ajoutees += n as u32;
    }
    tx.execute(
        "INSERT INTO config(cle, valeur) VALUES ('version_signatures', ?1)
         ON CONFLICT(cle) DO UPDATE SET valeur = excluded.valeur",
        [paquet.version.to_string()],
    )?;
    tx.commit()?;
    Ok(ajoutees)
}

/// Version locale des signatures (0 si jamais synchronisé).
fn lire_version(db: &Path) -> Result<u64, CleanXError> {
    let txt: Option<String> = crate::db::avec_connexion(db, |conn| {
        Ok(conn
            .query_row(
                "SELECT valeur FROM config WHERE cle = 'version_signatures'",
                [],
                |r| r.get(0),
            )
            .ok())
    })?;
    Ok(txt.and_then(|t| t.parse().ok()).unwrap_or(0))
}

/// Valide une entrée (hash 64 hex, nom 1–256) avant import.
fn valider_entree(entree: &EntreeDepot) -> Result<(), CleanXError> {
    let invalide = entree.hash.len() != 64
        || !entree.hash.bytes().all(|b| b.is_ascii_hexdigit())
        || entree.nom.is_empty()
        || entree.nom.len() > 256;
    if invalide {
        return Err(CleanXError::Interne {
            detail: format!("entrée dépôt invalide : {:?}", entree.nom),
        });
    }
    Ok(())
}

/// Signe un paquet (côté serveur / tests uniquement).
pub fn signer_paquet(paquet: &[u8], cle_privee: &[u8; 32]) -> String {
    let signing = SigningKey::from_bytes(cle_privee);
    hex::encode(signing.sign(paquet).to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn base_test() -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("t.db");
        crate::db::ouvrir(&base).unwrap();
        (dir, base)
    }

    #[tokio::test]
    async fn hash_connu_detecte_par_signature() {
        // Note : le fichier EICAR *sur disque* est intercepté par Defender
        // (os error 225) — voir BUGS.md. On teste donc le MÉCANISME avec un
        // hash confirmé injecté dans la base de test (B14 : plus aucun hash
        // fictif global) ; EICAR reste couvert au niveau base par
        // `db::tests::base_se_cree_et_contient_eicar`.
        let (_dir, base) = base_test().await;
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("demo.bin");
        let contenu = b"contenu-confirme-7f3a-test-mecanisme";
        std::fs::write(&p, contenu).unwrap();
        let sha = super::sha256_fichier(&p).await.unwrap();
        let conn = crate::db::ouvrir(&base).unwrap();
        conn.execute(
            "INSERT INTO signatures(hash, nom) VALUES (?1, ?2)",
            (sha.as_str(), "Test-Mecanisme-Confirme"),
        )
        .unwrap();
        drop(conn);
        let v = verifier_signature(&base, &p).await.unwrap();
        assert!(v.erreur.is_none(), "erreur inattendue : {:?}", v.erreur);
        assert!(v.menace.unwrap().contains("Test-Mecanisme-Confirme"));
    }

    #[tokio::test]
    async fn fichier_sain_non_signale() {
        let (_dir, base) = base_test().await;
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("document.txt");
        std::fs::write(&p, b"bonjour, document inoffensif").unwrap();
        let v = verifier_signature(&base, &p).await.unwrap();
        assert!(v.erreur.is_none(), "erreur inattendue : {:?}", v.erreur);
        assert!(v.menace.is_none());
    }

    #[tokio::test]
    async fn fichier_introuvable_encapsule() {
        let (_dir, base) = base_test().await;
        let v = verifier_signature(&base, Path::new("/chemin/inexistant/x.bin"))
            .await
            .unwrap();
        assert!(v.sha256.is_none() && v.erreur.is_some());
    }

    #[test]
    fn ed25519_sign_verify_roundtrip() {
        // Clé fixe (test uniquement : aucune valeur secrète en production).
        let sk = SigningKey::from_bytes(&[42u8; 32]);
        let paquet = b"signatures v42";
        let sig = signer_paquet(paquet, &sk.to_bytes());
        let ok =
            verifier_paquet(paquet, &sig, &hex::encode(sk.verifying_key().to_bytes())).unwrap();
        assert!(ok);
        assert!(
            !verifier_paquet(b"altere", &sig, &hex::encode(sk.verifying_key().to_bytes())).unwrap()
        );
    }

    /// Serveur HTTP minimaliste servant un corps par chemin (/v1, /v2, /v3).
    /// `requetes` = nombre de connexions à servir (retries inclus).
    async fn servir_fixtures(
        routes: std::collections::HashMap<String, Vec<u8>>,
        requetes: usize,
    ) -> String {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            for _ in 0..requetes {
                let (mut s, _) = listener.accept().await.unwrap();
                let mut buf = [0u8; 4096];
                let n = s.read(&mut buf).await.unwrap_or(0);
                let requete = String::from_utf8_lossy(&buf[..n]).into_owned();
                let chemin = requete
                    .lines()
                    .next()
                    .and_then(|l| l.split_whitespace().nth(1))
                    .unwrap_or("/")
                    .split('?')
                    .next()
                    .unwrap_or("/")
                    .to_string();
                let corps = routes.get(&chemin).cloned().unwrap_or_default();
                let reponse = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    corps.len()
                );
                s.write_all(reponse.as_bytes()).await.unwrap();
                s.write_all(&corps).await.unwrap();
            }
        });
        format!("http://127.0.0.1:{port}")
    }

    fn paquet_signe(version: u64, entrees: Vec<EntreeDepot>, cle: &[u8; 32]) -> Vec<u8> {
        let canoniques = serde_json::to_vec(&entrees).unwrap();
        let sig = signer_paquet(&canoniques, cle);
        serde_json::to_vec(&PaquetDepot {
            version,
            signatures: entrees,
            signature: sig,
        })
        .unwrap()
    }

    #[tokio::test]
    async fn depot_signe_importe_et_rollback() {
        let (_dir, base) = base_test().await;
        let sk = SigningKey::from_bytes(&[42u8; 32]);
        let vk = hex::encode(sk.verifying_key().to_bytes());
        let bon_hash = "ab".repeat(32);

        // Cas 1 : paquet valide → +1 signature requêtable.
        let valide = paquet_signe(
            1,
            vec![EntreeDepot {
                hash: bon_hash.clone(),
                nom: "Depot-Test".into(),
            }],
            &sk.to_bytes(),
        );
        // Cas 2 : corps altéré, signature d'origine → rejet AVANT transaction.
        let mut altere = valide.clone();
        altere[20] ^= 0xFF;
        // Cas 3 : signature valide mais entrée malformée en 2e position →
        // rejet PENDANT la transaction (rollback : la 1re ne doit pas rester).
        let mixte = paquet_signe(
            1,
            vec![
                EntreeDepot {
                    hash: "cd".repeat(32),
                    nom: "Ne-Doit-Pas-Rester".into(),
                },
                EntreeDepot {
                    hash: "PAS-UN-HASH".into(),
                    nom: "Malforme".into(),
                },
            ],
            &sk.to_bytes(),
        );
        let hote = servir_fixtures(
            [
                ("/v1".to_string(), valide),
                ("/v2".to_string(), altere),
                ("/v3".to_string(), mixte),
            ]
            .into_iter()
            .collect(),
            3,
        )
        .await;

        let n = mettre_a_jour_depuis_depot(&base, &format!("{hote}/v1"), &vk)
            .await
            .unwrap();
        assert_eq!(n, 1);
        let conn = crate::db::ouvrir(&base).unwrap();
        assert!(crate::db::chercher_menace(&conn, &bon_hash)
            .unwrap()
            .is_some());
        drop(conn);

        let avant: u32 = {
            let conn = crate::db::ouvrir(&base).unwrap();
            crate::db::compter_signatures(&conn).unwrap()
        };
        assert!(
            mettre_a_jour_depuis_depot(&base, &format!("{hote}/v2"), &vk)
                .await
                .is_err()
        );
        assert!(
            mettre_a_jour_depuis_depot(&base, &format!("{hote}/v3"), &vk)
                .await
                .is_err()
        );
        let apres: u32 = {
            let conn = crate::db::ouvrir(&base).unwrap();
            crate::db::compter_signatures(&conn).unwrap()
        };
        assert_eq!(avant, apres, "rollback transactionnel exigé");
    }

    #[tokio::test]
    async fn depot_differentiel_versionne() {
        // v1 : 2 entrées → +2, version stockée = 1.
        // v2 : 1 nouvelle entrée → +1, version = 2.
        // Rejeu v1 (obsolète) → rejet anti-rollback.
        let (_dir, base) = base_test().await;
        let sk = SigningKey::from_bytes(&[7u8; 32]);
        let vk = hex::encode(sk.verifying_key().to_bytes());
        let v1 = paquet_signe(
            1,
            vec![
                EntreeDepot {
                    hash: "aa".repeat(32),
                    nom: "Delta-A".into(),
                },
                EntreeDepot {
                    hash: "bb".repeat(32),
                    nom: "Delta-B".into(),
                },
            ],
            &sk.to_bytes(),
        );
        let v2 = paquet_signe(
            2,
            vec![EntreeDepot {
                hash: "cc".repeat(32),
                nom: "Delta-C".into(),
            }],
            &sk.to_bytes(),
        );
        // v0 : antérieur à tout → rejet anti-rollback même signé valide.
        let v0 = paquet_signe(
            0,
            vec![EntreeDepot {
                hash: "dd".repeat(32),
                nom: "Trop-Vieux".into(),
            }],
            &sk.to_bytes(),
        );
        let hote = servir_fixtures(
            [
                ("/v1".to_string(), v1),
                ("/v2".to_string(), v2),
                ("/v0".to_string(), v0),
            ]
            .into_iter()
            .collect(),
            5,
        )
        .await;

        assert_eq!(
            mettre_a_jour_depuis_depot(&base, &format!("{hote}/v1"), &vk)
                .await
                .unwrap(),
            2
        );
        // Re-demande v1 : rejouée à l'identique → acceptée mais +0
        // (idempotence : les retries réseau sont sans danger).
        assert_eq!(
            mettre_a_jour_depuis_depot(&base, &format!("{hote}/v1"), &vk)
                .await
                .unwrap(),
            0
        );
        assert_eq!(
            mettre_a_jour_depuis_depot(&base, &format!("{hote}/v2"), &vk)
                .await
                .unwrap(),
            1
        );
        // Rejeu v2 : déjà connu → +0 (idempotent), version inchangée.
        assert_eq!(
            mettre_a_jour_depuis_depot(&base, &format!("{hote}/v2"), &vk)
                .await
                .unwrap(),
            0
        );
        // Paquet antérieur (v0 < v2) : rejet anti-rollback, rien d'importé.
        assert!(
            mettre_a_jour_depuis_depot(&base, &format!("{hote}/v0"), &vk)
                .await
                .is_err()
        );
        let conn = crate::db::ouvrir(&base).unwrap();
        for h in ["aa".repeat(32), "bb".repeat(32), "cc".repeat(32)] {
            assert!(crate::db::chercher_menace(&conn, &h).unwrap().is_some());
        }
    }
}
