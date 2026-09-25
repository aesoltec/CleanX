//! Auto-protection (v1, pragmatique et documentée).
//!
//! Fournit : empreinte SHA-256 du binaire en cours (intégrité vérifiable par
//! l'UI/le déploiement MDM) et heuristique de débogage (variables
//! d'environnement typiques). Limites assumées : sans pilote noyau, un
//! attaquant privilégié peut toujours neutraliser un antivirus userland —
//! ces contrôles détectent les altérations accidentelles et le debug simple.

use std::path::Path;

use crate::CleanXError;

/// Variables d'environnement typiques d'un débogueur/instrumentation.
const VARS_DEBUG: &[&str] = &[
    "_INTELLIJ_FORCE_COLOR", // cascade JetBrains
    "DYLD_INSERT_LIBRARIES",
    "LD_PRELOAD",
    "RUST_BACKTRACE",
];

/// SHA-256 hexadécimal du binaire en cours d'exécution.
///
/// Sert de référence d'intégrité : comparez avec l'empreinte publiée au build
/// (CI : `sha256sum cleanx*`). Erreur si le binaire est illisible.
pub fn empreinte_binaire() -> Result<String, CleanXError> {
    let exe = std::env::current_exe().map_err(|e| CleanXError::Interne {
        detail: format!("binaire introuvable : {e}"),
    })?;
    let contenu = std::fs::read(&exe).map_err(|e| crate::error::erreur_io(&exe, e))?;
    use sha2::{Digest, Sha256};
    Ok(hex::encode(Sha256::digest(contenu)))
}

/// Vérifie l'intégrité : `true` si `attendu` est `None` (pas de référence) ou
/// correspond à l'empreinte courante.
pub fn verifier_integrite(attendu: Option<&str>) -> Result<bool, CleanXError> {
    match attendu {
        None => Ok(true),
        Some(ref_hash) => Ok(empreinte_binaire()?.eq_ignore_ascii_case(ref_hash)),
    }
}

/// Heuristique de débogage : `true` si l'environnement sent l'instrumentation.
///
/// Non bloquant par défaut : l'UI peut durcir sa posture (ex. refuser
/// d'afficher la clé, exiger une confirmation) quand c'est `true`.
pub fn signes_de_debug() -> bool {
    VARS_DEBUG.iter().any(|v| std::env::var_os(v).is_some())
}

/// Chemin du binaire (diagnostic, page "À propos").
pub fn chemin_binaire() -> String {
    std::env::current_exe()
        .map(|p: std::path::PathBuf| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| String::from("<inconnu>"))
}

/// Répertoire sanity-check : existence + inscriptible (permissions effectives).
///
/// Teste avec un fichier sonde à nom unique (PID + nanos), supprimé aussitôt.
/// Ne panique jamais : toute erreur vaut `false` (privilèges insuffisants…).
pub fn dossier_accessible(dossier: &Path) -> bool {
    use std::time::{SystemTime, UNIX_EPOCH};
    if !dossier.is_dir() {
        return false;
    }
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let sonde = dossier.join(format!(".cleanx-sonde-{}-{nanos}", std::process::id()));
    match std::fs::write(&sonde, b"sonde") {
        Ok(()) => {
            let _ = std::fs::remove_file(&sonde);
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empreinte_non_vide_et_stable() {
        let a = empreinte_binaire().unwrap();
        assert_eq!(a.len(), 64);
        assert_eq!(a, empreinte_binaire().unwrap());
        assert!(verifier_integrite(None).unwrap());
        assert!(verifier_integrite(Some(&a)).unwrap());
        assert!(!verifier_integrite(Some(&"0".repeat(64))).unwrap());
    }

    #[test]
    fn dossier_accessible_sonde_droits() {
        let dir = tempfile::tempdir().unwrap();
        assert!(dossier_accessible(dir.path()));
        assert!(!dossier_accessible(&dir.path().join("inexistant")));
        // Un fichier n'est pas un dossier inscriptible.
        let f = dir.path().join("f.txt");
        std::fs::write(&f, b"x").unwrap();
        assert!(!dossier_accessible(&f));
    }

    #[test]
    fn diagnostics_sans_panic() {
        assert!(!chemin_binaire().is_empty());
        let _ = signes_de_debug(); // booléen environnemental, jamais de panic
    }
}
