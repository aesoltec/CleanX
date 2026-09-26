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
/// `source` = `None` si pas de menace. Règles (B14/ADR-013) :
/// - chemin protégé → JAMAIS (tous modes, même confiance 100 : un binaire
///   signé ou système exige un consentement humain explicite) ;
/// - Prudent → seulement confiance ≥ 95 (hash confirmé) ;
/// - Automatique → sources confirmées (Signature, Générique) ;
/// - Agressif → toute menace de score ≥ 50 ;
/// - Silencieux → jamais.
pub fn doit_isoler_auto(
    mode: ModeDecision,
    source: Option<crate::signatures::SourceMenace>,
    score: u8,
    protege: bool,
) -> bool {
    if protege {
        return false;
    }
    match (mode, source) {
        (_, None) => false,
        (ModeDecision::Prudent, Some(s)) => crate::signatures::confiance(s, score) >= 95,
        (ModeDecision::Silencieux, _) => false,
        (ModeDecision::Automatique, Some(s)) => matches!(
            s,
            crate::signatures::SourceMenace::SignatureConnue
                | crate::signatures::SourceMenace::Generique
        ),
        (ModeDecision::Agressif, Some(_)) => score >= 50,
    }
}

/// Chemins système critiques : AUCUNE action sans double confirmation (P18).
/// Comparaison insensible à la casse (Windows) + normalisation des séparateurs.
pub fn est_chemin_critique(chemin: &str) -> bool {
    est_chemin_protege(chemin)
}

/// Chemins protégés contre TOUTE action automatique (B14/4bis) : racines
/// système + emplacements de confiance (Program Files…) + dossiers de
/// développement (jamais de binaire signé/projet isolé sans humain).
/// `CLEANX_PROTECTED_EXTRA` (séparateur `;`) ajoute des racines en dev/CI
/// pour tester le pipeline sans toucher au vrai système.
pub fn est_chemin_protege(chemin: &str) -> bool {
    // Lecture directe (sans cache) : coût négligeable devant une E/S fichier,
    // et déterminisme total pour les tests (variable modifiable à tout moment).
    let normalise = chemin.replace('\\', "/").to_lowercase();
    const RACINES: &[&str] = &[
        "c:/windows/",
        "/system/",
        "/system32/",
        "/usr/lib/",
        "/usr/bin/",
        "/etc/",
        "/boot/",
        "c:/program files/",
        "c:/program files (x86)/",
    ];
    const SEGMENTS_DEV: &[&str] = &[
        "node_modules",
        "target",
        "build",
        "dist",
        "out",
        ".git",
        "bin",
        "obj",
        "vendor",
        "__pycache__",
    ];
    if RACINES
        .iter()
        .any(|r| normalise == r.trim_end_matches('/') || normalise.starts_with(r))
    {
        return true;
    }
    if normalise
        .split('/')
        .any(|segment| SEGMENTS_DEV.contains(&segment))
    {
        return true;
    }
    let extras: Vec<String> = std::env::var("CLEANX_PROTECTED_EXTRA")
        .unwrap_or_default()
        .split(';')
        .map(|s| s.replace('\\', "/").to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    extras.iter().any(|r| {
        let racine = r.trim_end_matches('/');
        normalise == racine || normalise.starts_with(&(racine.to_string() + "/"))
    })
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
        use crate::signatures::SourceMenace;
        // Prudent : seulement confiance ≥ 95 (hash confirmé), jamais sinon.
        assert!(doit_isoler_auto(
            ModeDecision::Prudent,
            Some(SourceMenace::SignatureConnue),
            100,
            false
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Prudent,
            Some(SourceMenace::Generique),
            70,
            false
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Prudent,
            Some(SourceMenace::Heuristique),
            100,
            false
        ));
        // Silencieux : jamais, même confiance 100.
        assert!(!doit_isoler_auto(
            ModeDecision::Silencieux,
            Some(SourceMenace::SignatureConnue),
            100,
            false
        ));
        // Automatique : sources confirmées seulement.
        assert!(doit_isoler_auto(
            ModeDecision::Automatique,
            Some(SourceMenace::SignatureConnue),
            100,
            false
        ));
        assert!(doit_isoler_auto(
            ModeDecision::Automatique,
            Some(SourceMenace::Generique),
            70,
            false
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Automatique,
            Some(SourceMenace::Heuristique),
            90,
            false
        ));
        assert!(!doit_isoler_auto(ModeDecision::Automatique, None, 0, false));
        // Agressif : score ≥ 50, jamais de suppression (quarantaine).
        assert!(doit_isoler_auto(
            ModeDecision::Agressif,
            Some(SourceMenace::Heuristique),
            55,
            false
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Agressif,
            Some(SourceMenace::Heuristique),
            49,
            false
        ));
        // Protégé : JAMAIS, tous modes, même confiance 100 (B14/4bis).
        for mode in [
            ModeDecision::Prudent,
            ModeDecision::Automatique,
            ModeDecision::Agressif,
            ModeDecision::Silencieux,
        ] {
            assert!(
                !doit_isoler_auto(mode, Some(SourceMenace::SignatureConnue), 100, true),
                "chemin protégé isolé en {mode:?} !"
            );
        }
    }

    #[test]
    fn confiance_calibree() {
        use crate::signatures::{confiance, SourceMenace};
        assert_eq!(confiance(SourceMenace::SignatureConnue, 0), 100);
        assert_eq!(confiance(SourceMenace::Generique, 0), 70);
        assert_eq!(confiance(SourceMenace::Heuristique, 70), 35);
        assert_eq!(confiance(SourceMenace::Heuristique, 100), 50);
        assert!(confiance(SourceMenace::Heuristique, 100) <= 50);
    }

    #[test]
    fn whitelist_chemins_proteges() {
        // Système (insensible à la casse).
        assert!(est_chemin_protege(r"C:\Windows\System32\evil.exe"));
        assert!(est_chemin_protege("/usr/lib/x.so"));
        assert!(est_chemin_protege("/etc/cron.d/x"));
        assert!(est_chemin_protege("C:/Program Files/App/app.exe"));
        // Dossiers de développement (segments).
        assert!(est_chemin_protege("/home/ali/proj/node_modules/evil.js"));
        assert!(est_chemin_protege(r"C:\dev\proj\target\debug\evil.exe"));
        assert!(est_chemin_protege("/home/ali/build/setup.exe"));
        // Fichiers utilisateur normaux : NON protégés.
        assert!(!est_chemin_protege("/home/ali/Downloads/evil.exe"));
        assert!(!est_chemin_protege(r"C:\Users\ali\doc.pdf.exe"));
        assert!(!est_chemin_protege("/tmp/x"));
        // Surcouche dev/CI (documentée, sans cache) : direction biaisée
        // vers la prudence (faux négatifs > faux positifs).
        std::env::set_var("CLEANX_PROTECTED_EXTRA", "/tmp/fake-sys");
        assert!(est_chemin_protege("/tmp/fake-sys/evil.exe"));
        std::env::remove_var("CLEANX_PROTECTED_EXTRA");
        assert!(!est_chemin_protege("/tmp/fake-sys/evil.exe"));
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
