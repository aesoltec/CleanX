//! Persistance SQLite locale (rusqlite, moteur `bundled` = statique).
//!
//! Tables : `signatures`, `quarantaine`, `logs`, `planifications`, `config`.
//! rusqlite est synchrone : les chemins async l'encapsulent via `spawn_blocking`.
//! Les chemins chauds (scan) empruntent au [`POOL`] de connexions au lieu
//! d'ouvrir (2 ms/ouverture mesurés sur Windows) — voir `bench_scan`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use rusqlite::Connection;

use crate::CleanXError;

/// Chaîne de test EICAR (inoffensive, standard de l'industrie antivirus).
pub const EICAR: &[u8] = b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*";

// NOTE (B14) : AUCUN hash fictif seedé. Historiquement, `sha256("")` et
// `sha256("test")` étaient présents et quarantinaient des fichiers sains
// (fichier vide, fichier contenant "test"). Ne seeder QUE des malwares
// confirmés. Ici : EICAR uniquement (inoffensif, standard).

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS signatures(hash TEXT PRIMARY KEY, nom TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS quarantaine(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  nom TEXT NOT NULL, origine TEXT NOT NULL, date TEXT NOT NULL,
  raison TEXT NOT NULL, score INTEGER NOT NULL, blob_path TEXT NOT NULL,
  taille_octets INTEGER NOT NULL DEFAULT 0,
  hash_sha256 TEXT NOT NULL DEFAULT '',
  regle TEXT NOT NULL DEFAULT '',
  mode_actif TEXT NOT NULL DEFAULT 'Prudent',
  decision TEXT NOT NULL DEFAULT 'Quarantaine',
  statut TEXT NOT NULL DEFAULT 'actif',
  date_restauration TEXT, date_suppression TEXT,
  expire_le TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS logs(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  date TEXT NOT NULL, niveau TEXT NOT NULL, message TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS planifications(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  nom TEXT NOT NULL, racines TEXT NOT NULL,
  intervalle_secs INTEGER NOT NULL, prochaine_exec INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS config(cle TEXT PRIMARY KEY, valeur TEXT NOT NULL);
";

/// Ouvre (et migre si besoin) la base. Crée le fichier et le schéma.
pub fn ouvrir(chemin: &Path) -> Result<Connection, CleanXError> {
    let conn = ouvrir_sans_seed(chemin)?;
    semer_si_vide(&conn)?;
    Ok(conn)
}

/// Ouvre + migre sans toucher aux signatures (voies internes/pool).
fn ouvrir_sans_seed(chemin: &Path) -> Result<Connection, CleanXError> {
    if let Some(parent) = chemin.parent() {
        std::fs::create_dir_all(parent).map_err(|e| crate::error::erreur_io(parent, e))?;
    }
    let conn = Connection::open(chemin).map_err(|e| CleanXError::BaseCorrompue {
        detail: e.to_string(),
    })?;
    // Concurrence multi-connexions : attentes au lieu d'erreurs SQLITE_BUSY.
    conn.busy_timeout(std::time::Duration::from_secs(5))?;
    conn.execute_batch(SCHEMA)?;
    migrer_quarantaine(&conn)?;
    Ok(conn)
}

/// Migration idempotente de `quarantaine` (SPEC_EXPORT/P19) : ajoute les
/// colonnes manquantes aux bases créées avant la v2. `ALTER TABLE` échoue
/// si la colonne existe → on vérifie via PRAGMA avant chaque ajout.
fn migrer_quarantaine(conn: &Connection) -> Result<(), CleanXError> {
    let mut stmt = conn.prepare("PRAGMA table_info(quarantaine)")?;
    let existantes: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(1))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(CleanXError::from)?;
    const AJOUTS: &[(&str, &str)] = &[
        (
            "taille_octets",
            "ALTER TABLE quarantaine ADD COLUMN taille_octets INTEGER NOT NULL DEFAULT 0",
        ),
        (
            "hash_sha256",
            "ALTER TABLE quarantaine ADD COLUMN hash_sha256 TEXT NOT NULL DEFAULT ''",
        ),
        (
            "regle",
            "ALTER TABLE quarantaine ADD COLUMN regle TEXT NOT NULL DEFAULT ''",
        ),
        (
            "mode_actif",
            "ALTER TABLE quarantaine ADD COLUMN mode_actif TEXT NOT NULL DEFAULT 'Prudent'",
        ),
        (
            "decision",
            "ALTER TABLE quarantaine ADD COLUMN decision TEXT NOT NULL DEFAULT 'Quarantaine'",
        ),
        (
            "statut",
            "ALTER TABLE quarantaine ADD COLUMN statut TEXT NOT NULL DEFAULT 'actif'",
        ),
        (
            "date_restauration",
            "ALTER TABLE quarantaine ADD COLUMN date_restauration TEXT",
        ),
        (
            "date_suppression",
            "ALTER TABLE quarantaine ADD COLUMN date_suppression TEXT",
        ),
        (
            "expire_le",
            "ALTER TABLE quarantaine ADD COLUMN expire_le TEXT NOT NULL DEFAULT ''",
        ),
    ];
    for (colonne, ddl) in AJOUTS {
        if !existantes.iter().any(|c| c == colonne) {
            conn.execute_batch(ddl)?;
        }
    }
    Ok(())
}

/// Insère les signatures de test si la table est vide (idempotent, 1 requête).
pub fn semer_si_vide(conn: &Connection) -> Result<(), CleanXError> {
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM signatures", [], |r| r.get(0))?;
    if n == 0 {
        semer_signatures(conn)?;
    }
    Ok(())
}

/// Insère la signature EICAR si absente (idempotent via INSERT OR IGNORE).
fn semer_signatures(conn: &Connection) -> Result<(), CleanXError> {
    use sha2::{Digest, Sha256};
    let eicar = hex::encode(Sha256::digest(EICAR));
    conn.execute(
        "INSERT OR IGNORE INTO signatures(hash, nom) VALUES (?1, ?2)",
        (eicar.as_str(), "EICAR-Test-File (test inoffensif)"),
    )?;
    Ok(())
}

/// Nom de menace associé à un hash, ou `None` si inconnu.
pub fn chercher_menace(conn: &Connection, hash: &str) -> Result<Option<String>, CleanXError> {
    let mut stmt = conn.prepare("SELECT nom FROM signatures WHERE hash = ?1")?;
    let mut rows = stmt.query([hash.to_lowercase()])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

/// Nombre de signatures (affiché sur le dashboard).
pub fn compter_signatures(conn: &Connection) -> Result<u32, CleanXError> {
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM signatures", [], |r| r.get(0))?;
    Ok(n.max(0) as u32)
}

/// Taille max du pool par base (== workers de scan).
const TAILLE_POOL: usize = 8;

static POOL: OnceLock<Mutex<HashMap<PathBuf, Vec<Connection>>>> = OnceLock::new();

fn pool() -> &'static Mutex<HashMap<PathBuf, Vec<Connection>>> {
    POOL.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Vide le pool (ferme les fichiers SQLite). À appeler avant suppression du
/// dossier de base, ou à l'arrêt de l'application. Les appels suivants
/// rouvrent à la demande (transparent).
pub fn vider_pool() {
    if let Ok(mut pool) = pool().lock() {
        pool.clear();
    }
}

/// Exécute `op` avec une connexion du pool (ou une nouvelle), rendue
/// automatiquement même en cas d'erreur (`?`).
///
/// `Connection` est `Send` (pas `Sync`) : chaque connexion n'est utilisée que
/// par un seul thread à la fois, le `Mutex` ne protège que le stock.
pub fn avec_connexion<T>(
    chemin: &Path,
    op: impl FnOnce(&Connection) -> Result<T, CleanXError>,
) -> Result<T, CleanXError> {
    let cle = chemin.to_path_buf();
    let conn = pool()
        .lock()
        .map_err(|_| CleanXError::Interne {
            detail: "mutex pool base empoisonné".into(),
        })?
        .remove(&cle)
        .and_then(|mut v| v.pop());
    let conn = match conn {
        Some(c) => c,
        None => {
            let c = ouvrir_sans_seed(chemin)?;
            semer_si_vide(&c)?;
            c
        }
    };
    let resultat = op(&conn);
    // Retour au pool (ou fermeture si plein) quel que soit le résultat.
    if let Ok(mut garde) = pool().lock() {
        let entrees = garde.entry(cle).or_default();
        if entrees.len() < TAILLE_POOL {
            entrees.push(conn);
        }
        // Pool plein : `conn` se ferme en fin de portée (pas de fuite).
    }
    resultat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_se_cree_et_contient_eicar() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("test.db");
        let conn = ouvrir(&base).unwrap();
        // Base minimale : EICAR uniquement (B14 : aucun hash fictif).
        assert_eq!(compter_signatures(&conn).unwrap(), 1);
        use sha2::{Digest, Sha256};
        let eicar = hex::encode(Sha256::digest(EICAR));
        let nom = chercher_menace(&conn, &eicar).unwrap();
        assert!(nom.unwrap().contains("EICAR"));
        assert!(chercher_menace(&conn, &"0".repeat(64)).unwrap().is_none());
        // B14 : ni fichier vide ni "test" ne matchent plus.
        let vide = hex::encode(Sha256::digest(b""));
        assert!(chercher_menace(&conn, &vide).unwrap().is_none());
    }
}
