//! Modes de décision CleanX — consentement utilisateur (AGENTS.md §4bis.2).
//!
//! Le mode **Prudent** est le défaut absolu : détection + notification,
//! JAMAIS d'action automatique. Les modes automatiques sont opt-in explicites
//! (sélecteur Paramètres) et toujours réversibles (quarantaine 30 j, P19).

/// Mode de décision face à une menace (exposé à Dart via FFI).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModeDecision {
    /// Détection + notification uniquement. Aucune action auto. **Défaut.**
    #[default]
    Prudent,
    /// Quarantaine automatique si menace avérée (signature connue ou score ≥ 70).
    Automatique,
    /// Quarantaine automatique dès score ≥ 50 (suspect inclus). Opt-in confirmé.
    /// Ne supprime JAMAIS : toujours via quarantaine réversible.
    Agressif,
    /// Journalise seulement, n'agit pas (ni quarantaine, ni suppression).
    Silencieux,
}

/// Score minimal déclenchant une quarantaine automatique, le cas échéant.
/// `None` = aucune action automatique dans ce mode.
pub fn seuil_quarantaine_auto(mode: ModeDecision) -> Option<u8> {
    match mode {
        ModeDecision::Prudent => None,
        ModeDecision::Automatique => None, // menace avérée uniquement (pas de seuil)
        ModeDecision::Agressif => Some(50),
        ModeDecision::Silencieux => None,
    }
}

/// La menace doit-elle être isolée automatiquement dans ce mode ?
/// `menace_averee` = signature connue OU score ≥ 70 (seuil calibré, P18/4bis).
pub fn doit_isoler_auto(mode: ModeDecision, menace_averee: bool, score: u8) -> bool {
    match mode {
        ModeDecision::Prudent => false,
        ModeDecision::Silencieux => false,
        ModeDecision::Automatique => menace_averee,
        ModeDecision::Agressif => menace_averee || score >= 50,
    }
}

/// Chemins système critiques : AUCUNE action sans double confirmation (P18).
/// Comparaison insensible à la casse (Windows) + normalisation des séparateurs.
pub fn est_chemin_critique(chemin: &str) -> bool {
    let normalise = chemin.replace('\\', "/").to_lowercase();
    const RACINES: &[&str] = &[
        "c:/windows/",
        "/system/",
        "/system32/",
        "/usr/lib/",
        "/usr/bin/",
        "/etc/",
        "/boot/",
    ];
    RACINES
        .iter()
        .any(|r| normalise == r.trim_end_matches('/') || normalise.starts_with(r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_mode_is_prudent() {
        // Exigé par MISSION.md P14 : le défaut de construction est Prudent.
        // (Le global d'exécution est initialisé à Prudent dans api.rs ;
        //  chaque test qui change le mode le restaure — voir integration.)
        assert_eq!(ModeDecision::default(), ModeDecision::Prudent);
        assert_eq!(seuil_quarantaine_auto(ModeDecision::Prudent), None);
        assert_eq!(seuil_quarantaine_auto(ModeDecision::Agressif), Some(50));
    }

    #[test]
    fn gating_par_mode() {
        // Prudent/Silencieux : jamais d'action, même menace avérée.
        assert!(!doit_isoler_auto(ModeDecision::Prudent, true, 100));
        assert!(!doit_isoler_auto(ModeDecision::Silencieux, true, 100));
        // Automatique : seulement menace avérée (pas le simple suspect).
        assert!(doit_isoler_auto(ModeDecision::Automatique, true, 90));
        assert!(!doit_isoler_auto(ModeDecision::Automatique, false, 65));
        // Agressif : suspect ≥ 50 aussi, jamais de suppression (quarantaine).
        assert!(doit_isoler_auto(ModeDecision::Agressif, false, 55));
        assert!(!doit_isoler_auto(ModeDecision::Agressif, false, 49));
    }

    #[test]
    fn chemins_critiques() {
        assert!(est_chemin_critique(r"C:\Windows\System32\evil.exe"));
        assert!(est_chemin_critique("c:/windows/temp/x.dll"));
        assert!(est_chemin_critique("/System/Library/evil.dylib"));
        assert!(est_chemin_critique("/usr/lib/evil.so"));
        assert!(est_chemin_critique("/etc/cron.d/evil"));
        assert!(!est_chemin_critique("/home/ali/Downloads/evil.exe"));
        assert!(!est_chemin_critique("C:/Users/ali/doc.pdf.exe"));
        assert!(!est_chemin_critique("/tmp/x"));
    }
}
