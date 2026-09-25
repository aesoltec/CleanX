//! Scans planifiés : persistance SQLite + calcul de prochaine exécution.
//!
//! Volontairement sans exécuteur de fond : le déclenchement est piloté par
//! l'UI (ou le planificateur OS), qui appelle `scan_personnalise` avec les
//! racines de la planification due. Évite un thread permanent pour une
//! fonctionnalité à faible fréquence.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::CleanXError;

/// Une planification de scan (exposée à Dart).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planification {
    pub id: i64,
    /// Nom affiché ("Analyse hebdo").
    pub nom: String,
    /// Dossiers/racines à scanner.
    pub racines: Vec<String>,
    /// Périodicité en secondes (ex. 604800 = hebdomadaire).
    pub intervalle_secs: u64,
    /// Prochaine exécution (epoch, secondes).
    pub prochaine_exec: u64,
}

/// Epoch actuel (secondes).
pub fn maintenant_epoch() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Crée une planification (première exécution = maintenant + intervalle).
pub fn ajouter(
    db: &Path,
    nom: &str,
    racines: &[String],
    intervalle_secs: u64,
) -> Result<i64, CleanXError> {
    if nom.trim().is_empty() {
        return Err(CleanXError::Interne {
            detail: "nom de planification vide".into(),
        });
    }
    if intervalle_secs == 0 {
        return Err(CleanXError::Interne {
            detail: "intervalle nul".into(),
        });
    }
    crate::db::avec_connexion(db, |conn| {
        conn.execute(
            "INSERT INTO planifications(nom, racines, intervalle_secs, prochaine_exec) VALUES (?1,?2,?3,?4)",
            rusqlite::params![nom, serde_json::to_string(racines)?, intervalle_secs as i64, (maintenant_epoch() + intervalle_secs) as i64],
        )?;
        Ok(conn.last_insert_rowid())
    })
}

/// Liste toutes les planifications.
pub fn lister(db: &Path) -> Result<Vec<Planification>, CleanXError> {
    crate::db::avec_connexion(db, |conn| {
        let mut stmt = conn.prepare(
            "SELECT id, nom, racines, intervalle_secs, prochaine_exec FROM planifications ORDER BY prochaine_exec",
        )?;
        let lignes = stmt.query_map([], |r| {
            let racines_txt: String = r.get(2)?;
            Ok(Planification {
                id: r.get(0)?,
                nom: r.get(1)?,
                racines: serde_json::from_str(&racines_txt).unwrap_or_default(),
                intervalle_secs: r.get::<_, i64>(3)? as u64,
                prochaine_exec: r.get::<_, i64>(4)? as u64,
            })
        })?;
        lignes
            .collect::<Result<Vec<_>, _>>()
            .map_err(CleanXError::from)
    })
}

/// Supprime une planification. Erreur si inexistante.
pub fn supprimer(db: &Path, id: i64) -> Result<(), CleanXError> {
    crate::db::avec_connexion(db, |conn| {
        let n = conn.execute("DELETE FROM planifications WHERE id = ?1", [id])?;
        if n == 0 {
            return Err(CleanXError::Interne {
                detail: format!("planification #{id} introuvable"),
            });
        }
        Ok(())
    })
}

/// Planifications dues (prochaine_exec <= maintenant), triées par urgence.
pub fn dues(db: &Path) -> Result<Vec<Planification>, CleanXError> {
    Ok(lister(db)?
        .into_iter()
        .filter(|p| p.prochaine_exec <= maintenant_epoch())
        .collect())
}

/// Recalcule la prochaine exécution après un passage (maintenant + intervalle).
pub fn marquer_executee(db: &Path, id: i64) -> Result<(), CleanXError> {
    crate::db::avec_connexion(db, |conn| {
        let intervalle: i64 = conn
            .query_row(
                "SELECT intervalle_secs FROM planifications WHERE id = ?1",
                [id],
                |r| r.get(0),
            )
            .map_err(|_| CleanXError::Interne {
                detail: format!("planification #{id} introuvable"),
            })?;
        conn.execute(
            "UPDATE planifications SET prochaine_exec = ?1 WHERE id = ?2",
            rusqlite::params![(maintenant_epoch() as i64) + intervalle, id],
        )?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crud_planification() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("s.db");
        let id = ajouter(&base, "Hebdo", &["/tmp".to_string()], 604800).unwrap();
        let toutes = lister(&base).unwrap();
        assert_eq!(toutes.len(), 1);
        assert_eq!(toutes[0].racines, vec!["/tmp".to_string()]);
        assert!(toutes[0].prochaine_exec > maintenant_epoch());
        assert!(dues(&base).unwrap().is_empty());
        marquer_executee(&base, id).unwrap();
        supprimer(&base, id).unwrap();
        assert!(lister(&base).unwrap().is_empty());
        assert!(ajouter(&base, "", &[], 10).is_err());
    }

    #[test]
    fn dues_detecte_planification_echue() {
        use rusqlite::params;
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().join("s.db");
        // Insertion directe d'une échéance passée (prochaine_exec = 1).
        crate::db::avec_connexion(&base, |conn| {
            conn.execute(
                "INSERT INTO planifications(nom, racines, intervalle_secs, prochaine_exec) VALUES (?1,?2,?3,?4)",
                params!["Vieille", "[]", 60, 1],
            )?;
            Ok(())
        })
        .unwrap();
        let dues = dues(&base).unwrap();
        assert_eq!(dues.len(), 1);
        assert_eq!(dues[0].nom, "Vieille");
    }
}
