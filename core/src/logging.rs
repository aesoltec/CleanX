//! Journalisation : `tracing` JSON + rotation quotidienne + table SQLite.
//!
//! - Fichier : `<base>/logs/cleanx.log` (rotation journalière, JSON, niveau configurable).
//! - Table `logs` : événements destinés à l'UI (filtrables, exportables CSV côté Dart).

use std::path::Path;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::CleanXError;

/// Niveaux reconnus par l'UI (stockés tels quels en base).
pub const NIVEAUX_VALIDES: &[&str] = &["info", "alerte", "quarantaine"];

/// Entrée de journal (exposée à Dart).
#[derive(Debug, Clone)]
pub struct EntreeLog {
    pub id: i64,
    pub date: String,
    pub niveau: String,
    pub message: String,
}

/// Horodatage local `AAAA-MM-JJ HH:MM:SS` (sans dépendance `chrono`).
pub fn horodatage() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Conversion calendaire simple (UTC) : algorithme civil de Howard Hinnant.
    let jours = (secs / 86400) as i64;
    let reste = (secs % 86400) as i64;
    let z = jours + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    y += i64::from(m <= 2);
    format!(
        "{y:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        m,
        d,
        reste / 3600,
        (reste % 3600) / 60,
        reste % 60
    )
}

/// Initialise le subscriber global (JSON, rotation quotidienne). Idempotent :
/// un second appel réussit silencieusement (déjà initialisé).
pub fn initialiser(dossier_logs: &Path, niveau: &str) -> Result<(), CleanXError> {
    std::fs::create_dir_all(dossier_logs).map_err(|e| crate::error::erreur_io(dossier_logs, e))?;
    let appender = tracing_appender::rolling::daily(dossier_logs, "cleanx.log");
    let filtre = EnvFilter::try_new(niveau).unwrap_or_else(|_| EnvFilter::new("info"));
    let couche_fichier = fmt::layer().json().with_writer(appender);
    // `try_init` : Ok si premier, Err si déjà initialisé → on considère Ok.
    let _ = tracing_subscriber::registry()
        .with(filtre)
        .with(couche_fichier)
        .try_init();
    Ok(())
}

/// Écrit un événement en base ET dans le log structuré.
pub fn journaliser(db: &Path, niveau: &str, message: &str) -> Result<(), CleanXError> {
    let niveau = if NIVEAUX_VALIDES.contains(&niveau) {
        niveau
    } else {
        "info"
    };
    crate::db::avec_connexion(db, |conn| {
        conn.execute(
            "INSERT INTO logs(date, niveau, message) VALUES (?1, ?2, ?3)",
            (horodatage(), niveau, message),
        )?;
        Ok(())
    })?;
    match niveau {
        "alerte" | "quarantaine" => tracing::warn!(message),
        _ => tracing::info!(message),
    }
    Ok(())
}

/// Derniers événements, ordre antéchronologique (limités à `limite`).
pub fn lister(db: &Path, limite: u32) -> Result<Vec<EntreeLog>, CleanXError> {
    crate::db::avec_connexion(db, |conn| {
        let mut stmt =
            conn.prepare("SELECT id, date, niveau, message FROM logs ORDER BY id DESC LIMIT ?1")?;
        let lignes = stmt.query_map([limite as i64], |r| {
            Ok(EntreeLog {
                id: r.get(0)?,
                date: r.get(1)?,
                niveau: r.get(2)?,
                message: r.get(3)?,
            })
        })?;
        lignes
            .collect::<Result<Vec<_>, _>>()
            .map_err(CleanXError::from)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horodatage_format() {
        let h = horodatage();
        assert_eq!(h.len(), 19);
        assert_eq!(&h[4..5], "-");
    }

    #[test]
    fn journaliser_persiste() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("j.db");
        journaliser(&base, "alerte", "test").unwrap();
        journaliser(&base, "niveau-inconnu", "repli info").unwrap();
        let conn = crate::db::ouvrir(&base).unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM logs", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 2);
    }
}
