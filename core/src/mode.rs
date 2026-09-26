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

/// Seuil de CONFIANCE minimal pour une action automatique (B14 + suivi D26).
/// `None` = jamais d'action auto dans ce mode, quelle que soit la confiance.
/// - Prudent : jamais (consentement absolu — AGENTS §4bis : « demande avant
///   chaque action » ; même EICAR n'est que signalé, quarantaine en 1 clic) ;
/// - Automatique : 95 (hash confirmé uniquement — mitigation D26 : en
///   attendant Authenticode, aucun fichier potentiellement légitime
///   — confiance < 95 — n'est isolé automatiquement) ;
/// - Agressif : 80, ET hors chemin protégé ET hors zone utilisateur ;
/// - Silencieux : jamais.
pub fn seuil_confiance_auto(mode: ModeDecision) -> Option<u8> {
    match mode {
        ModeDecision::Prudent => None,
        ModeDecision::Automatique => Some(95),
        ModeDecision::Agressif => Some(80),
        ModeDecision::Silencieux => None,
    }
}

/// La menace doit-elle être isolée automatiquement ?
/// `source` = `None` si pas de menace ; `chemin` = fichier tel que fourni
/// (les règles de lieu sont évaluées ICI, au même endroit — aucun risque
/// d'oubli côté appelant). Règles (B14 + suivi D26) :
/// - chemin protégé → JAMAIS (tous modes, même confiance 100 : un binaire
///   signé ou système exige un consentement humain explicite) ;
/// - Prudent / Silencieux → jamais (notification / journal seuls) ;
/// - Automatique → confiance ≥ 95 (hash confirmé uniquement) ;
/// - Agressif → confiance ≥ 80 ET hors zone utilisateur (Documents, Bureau,
///   Photos, /home… : jamais d'auto sur données personnelles).
pub fn doit_isoler_auto(
    mode: ModeDecision,
    source: Option<crate::signatures::SourceMenace>,
    score: u8,
    chemin: &str,
) -> bool {
    if est_chemin_protege(chemin) {
        return false;
    }
    let confiance = source
        .map(|s| crate::signatures::confiance(s, score))
        .unwrap_or(0);
    match mode {
        ModeDecision::Agressif => confiance >= 80 && !est_zone_utilisateur(chemin),
        autre => seuil_confiance_auto(autre).is_some_and(|seuil| confiance >= seuil),
    }
}

/// Chemins système critiques : AUCUNE action sans double confirmation (P18).
/// Comparaison insensible à la casse (Windows) + normalisation des séparateurs.
pub fn est_chemin_critique(chemin: &str) -> bool {
    est_chemin_protege(chemin)
}

/// Segments désignant des données personnelles (jamais d'action auto en
/// Agressif — suivi D26). Élargi aux variantes courantes FR/EN.
const SEGMENTS_UTILISATEUR: &[&str] = &[
    "documents",
    "desktop",
    "bureau",
    "photos",
    "images",
    "downloads",
    "download",
    "téléchargements",
    "telechargements",
    "musique",
    "music",
    "videos",
    "vidéos",
];

/// Zone utilisateur : arborescences personnelles (`/home/…`, `/users/…`,
/// `C:\Users\…`) ou tout segment de [`SEGMENTS_UTILISATEUR`].
/// Normalisation identique à [`est_chemin_protege`] (casse + séparateurs).
pub fn est_zone_utilisateur(chemin: &str) -> bool {
    let normalise = chemin.replace('\\', "/").to_lowercase();
    const RACINES_UTILISATEUR: &[&str] = &["c:/users/", "/home/", "/users/"];
    if RACINES_UTILISATEUR.iter().any(|r| normalise.starts_with(r)) {
        return true;
    }
    normalise
        .split('/')
        .any(|segment| SEGMENTS_UTILISATEUR.contains(&segment))
}

/// Chemins protégés contre TOUTE action automatique (B14/4bis) : racines
/// système + emplacements de confiance (Program Files…) + dossiers de
/// développement (jamais de binaire signé/projet isolé sans humain).
/// `CLEANX_PROTECTED_EXTRA` (séparateur `;`) ajoute des racines en dev/CI
/// pour tester le pipeline sans toucher au vrai système — voir
/// [`lister_extras`] (sécurisé : ignoré sans `CLEANX_DEV_MODE=1`, jamais en
/// production, entrées validées, ajouts journalisés).
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
    lister_extras()
        .iter()
        .any(|r| normalise == *r || normalise.starts_with(&format!("{r}/")))
}

/// Racines supplémentaires protégées — dev/CI uniquement, fail-closed
/// (Correctif 2 : `CLEANX_PROTECTED_EXTRA` est un vecteur d'attaque potentiel
/// — un malware qui définirait la variable ne doit RIEN obtenir).
/// Conditions cumulatives :
/// 1. `CLEANX_DEV_MODE=1` AUSSI défini, sinon liste vide (un processus
///    tiers qui définit `EXTRA` seul est ignoré) ;
/// 2. build debug uniquement : en release (`debug_assertions` absent), la
///    variable est IGNORÉE totalement (la production ne la lit même pas) ;
/// 3. chaque entrée est VALIDÉE ([`extra_valide`]) : refus des racines trop
///    larges (`/`, `C:\`, `C:\Users`, `/home`…), des zones utilisateur et
///    des segments `.`/`..` (traversée). Voir [`lister_extras_avec_refus`].
pub fn lister_extras() -> Vec<String> {
    lister_extras_avec_refus().0
}

/// `(acceptés, refusés)` — les refus alimentent le journal d'audit
/// (journalisés au démarrage de chaque scan / protection, voir `api.rs`).
pub fn lister_extras_avec_refus() -> (Vec<String>, Vec<String>) {
    #[cfg(not(debug_assertions))]
    {
        return (Vec::new(), Vec::new()); // production : variable ignorée
    }
    #[cfg(debug_assertions)]
    {
        if std::env::var("CLEANX_DEV_MODE").as_deref() != Ok("1") {
            return (Vec::new(), Vec::new());
        }
        let mut acceptes = Vec::new();
        let mut refuses = Vec::new();
        if let Ok(brut) = std::env::var("CLEANX_PROTECTED_EXTRA") {
            for entree in brut.split(';') {
                let normalisee = entree
                    .replace('\\', "/")
                    .to_lowercase()
                    .trim_end_matches('/')
                    .to_string();
                if extra_valide(&normalisee) && !acceptes.contains(&normalisee) {
                    acceptes.push(normalisee);
                } else if !normalisee.is_empty() {
                    refuses.push(normalisee);
                }
            }
        }
        (acceptes, refuses)
    }
}

/// Une racine EXTRA (déjà normalisée : minuscules, `/`, sans `/` final).
/// Refuse (fail-closed) : vide, racine filesystem/lecteur, `C:\Users*`,
/// `/home*`, `/users*` (trop larges), tout segment utilisateur (Documents,
/// Bureau…), `.`/`..` (traversée de chemin).
fn extra_valide(racine: &str) -> bool {
    if racine.is_empty() {
        return false;
    }
    // Racine de lecteur (`c:`) ou chemin relatif nu : trop large / ambigu.
    if racine == "/" || (racine.len() == 2 && racine.ends_with(':')) {
        return false;
    }
    const TROP_LARGES: &[&str] = &["c:/users", "/home", "/users"];
    if TROP_LARGES
        .iter()
        .any(|r| racine == *r || racine.starts_with(&format!("{r}/")))
    {
        return false;
    }
    if racine
        .split('/')
        .any(|s| s == "." || s == ".." || SEGMENTS_UTILISATEUR.contains(&s))
    {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Les tests qui touchent l'environnement partagent le processus :
    /// sérialisation obligatoire (sinon courses sur les variables).
    static VERROU_ENV: Mutex<()> = Mutex::new(());

    #[test]
    fn default_mode_is_prudent() {
        // Exigé par MISSION.md P14 : le défaut de construction est Prudent.
        // (Le global d'exécution est initialisé à Prudent dans api.rs ;
        //  chaque test qui change le mode le restaure — voir integration.)
        assert_eq!(ModeDecision::default(), ModeDecision::Prudent);
        // Seuils de confiance (suivi D26) : Prudent/Silencieux = jamais.
        assert_eq!(seuil_confiance_auto(ModeDecision::Prudent), None);
        assert_eq!(seuil_confiance_auto(ModeDecision::Automatique), Some(95));
        assert_eq!(seuil_confiance_auto(ModeDecision::Agressif), Some(80));
        assert_eq!(seuil_confiance_auto(ModeDecision::Silencieux), None);
    }

    #[test]
    fn gating_par_mode() {
        use crate::signatures::SourceMenace;
        // Chemin neutre : ni protégé, ni zone utilisateur.
        const NEUTRE: &str = "/tmp/neutre/app.exe";
        assert!(!est_chemin_protege(NEUTRE));
        assert!(!est_zone_utilisateur(NEUTRE));
        // Prudent : JAMAIS d'auto (consentement absolu, même confiance 100).
        assert!(!doit_isoler_auto(
            ModeDecision::Prudent,
            Some(SourceMenace::SignatureConnue),
            0,
            NEUTRE
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Prudent,
            Some(SourceMenace::Generique),
            70,
            NEUTRE
        ));
        // Silencieux : jamais, même confiance 100.
        assert!(!doit_isoler_auto(
            ModeDecision::Silencieux,
            Some(SourceMenace::SignatureConnue),
            0,
            NEUTRE
        ));
        // Automatique : confiance ≥ 95 uniquement (hash confirmé).
        assert!(doit_isoler_auto(
            ModeDecision::Automatique,
            Some(SourceMenace::SignatureConnue),
            0,
            NEUTRE
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Automatique,
            Some(SourceMenace::Generique),
            70,
            NEUTRE
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Automatique,
            Some(SourceMenace::Heuristique),
            100,
            NEUTRE
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Automatique,
            None,
            0,
            NEUTRE
        ));
        // Agressif : confiance ≥ 80 + hors zone utilisateur.
        assert!(doit_isoler_auto(
            ModeDecision::Agressif,
            Some(SourceMenace::SignatureConnue),
            0,
            NEUTRE
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Agressif,
            Some(SourceMenace::Generique),
            70,
            NEUTRE
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Agressif,
            Some(SourceMenace::Heuristique),
            100,
            NEUTRE
        ));
        // Agressif en zone utilisateur : JAMAIS, même confiance 100.
        assert!(!doit_isoler_auto(
            ModeDecision::Agressif,
            Some(SourceMenace::SignatureConnue),
            0,
            "/home/ali/Documents/malware.exe"
        ));
        assert!(!doit_isoler_auto(
            ModeDecision::Agressif,
            Some(SourceMenace::SignatureConnue),
            0,
            r"C:\Users\ali\Desktop\malware.exe"
        ));
        // Protégé : JAMAIS, tous modes, même confiance 100 (B14/4bis).
        const PROTEGE: &str = r"C:\Windows\System32\evil.exe";
        assert!(est_chemin_protege(PROTEGE));
        for mode in [
            ModeDecision::Prudent,
            ModeDecision::Automatique,
            ModeDecision::Agressif,
            ModeDecision::Silencieux,
        ] {
            assert!(
                !doit_isoler_auto(mode, Some(SourceMenace::SignatureConnue), 0, PROTEGE),
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
        let _garde = VERROU_ENV.lock().unwrap();
        // Système (insensible à la casse).
        assert!(est_chemin_protege(r"C:\Windows\System32\evil.exe"));
        assert!(est_chemin_protege("/usr/lib/x.so"));
        assert!(est_chemin_protege("/etc/cron.d/x"));
        assert!(est_chemin_protege("C:/Program Files/App/app.exe"));
        // Dossiers de développement (segments).
        assert!(est_chemin_protege("/home/ali/proj/node_modules/evil.js"));
        assert!(est_chemin_protege(r"C:\dev\proj\target\debug\evil.exe"));
        assert!(est_chemin_protege("/home/ali/build/setup.exe"));
        // Fichiers utilisateur normaux : NON protégés (mais zones utilisateur).
        assert!(!est_chemin_protege("/home/ali/Downloads/evil.exe"));
        assert!(!est_chemin_protege(r"C:\Users\ali\doc.pdf.exe"));
        assert!(!est_chemin_protege("/tmp/x"));
        // Surcouche dev/CI : EXIGE désormais CLEANX_DEV_MODE=1 (Correctif 2).
        std::env::set_var("CLEANX_PROTECTED_EXTRA", "/tmp/fake-sys");
        std::env::remove_var("CLEANX_DEV_MODE");
        assert!(!est_chemin_protege("/tmp/fake-sys/evil.exe")); // sans DEV_MODE : ignoré
        std::env::set_var("CLEANX_DEV_MODE", "1");
        assert!(est_chemin_protege("/tmp/fake-sys/evil.exe")); // avec DEV_MODE : actif
        std::env::remove_var("CLEANX_PROTECTED_EXTRA");
        std::env::remove_var("CLEANX_DEV_MODE");
        assert!(!est_chemin_protege("/tmp/fake-sys/evil.exe"));
    }

    #[test]
    fn zones_utilisateur() {
        // Racines personnelles.
        assert!(est_zone_utilisateur("/home/ali/malware.exe"));
        assert!(est_zone_utilisateur(r"C:\Users\ali\malware.exe"));
        assert!(est_zone_utilisateur(r"C:\Users\Public\setup.exe"));
        assert!(est_zone_utilisateur("/users/ali/x"));
        // Segments FR/EN, insensibles à la casse.
        assert!(est_zone_utilisateur("/data/Documents/rapport.exe"));
        assert!(est_zone_utilisateur(r"D:\Bureau\outil.exe"));
        assert!(est_zone_utilisateur("/mnt/Photos/vacances.exe"));
        assert!(est_zone_utilisateur("/tmp/Downloads/x.exe"));
        // Neutres : PAS des zones utilisateur.
        assert!(!est_zone_utilisateur("/tmp/neutre/app.exe"));
        assert!(!est_zone_utilisateur(r"D:\Temp\app.exe"));
        assert!(!est_zone_utilisateur("/var/lib/app/x"));
        assert!(!est_zone_utilisateur(r"C:\dev\proj\app.exe"));
    }

    #[test]
    fn mitigation_d26_litteraux_surveilles() {
        // Cas exigés par le suivi : fichiers « signés » (confiance < 95,
        // Authenticode encore absent) hors chemins système → jamais d'auto,
        // même en Automatique/Agressif. Preuve au niveau gating (le niveau
        // pipeline est couvert par `tests/faux_positifs.rs`).
        use crate::signatures::SourceMenace;
        for chemin in [
            r"D:\Temp\ms_signed.exe",           // signé Microsoft simulé
            r"C:\Users\Public\adobe_setup.exe", // signé Adobe simulé (+ zone utilisateur)
            "/tmp/google_tool",                 // signé Google simulé
        ] {
            assert!(
                !est_chemin_protege(chemin),
                "{chemin} ne doit pas être protégé (sinon le test ne prouve rien)"
            );
            for mode in [ModeDecision::Automatique, ModeDecision::Agressif] {
                assert!(
                    !doit_isoler_auto(mode, Some(SourceMenace::Generique), 70, chemin),
                    "{chemin} isolé en {mode:?} à confiance 70 !"
                );
                assert!(
                    !doit_isoler_auto(mode, Some(SourceMenace::Heuristique), 100, chemin),
                    "{chemin} isolé en {mode:?} sur heuristique seule !"
                );
            }
        }
    }

    #[test]
    fn extras_securises() {
        // Correctif 2 : sans DEV_MODE la variable est ignorée (processus
        // tiers) ; en release elle n'est même pas lue (branche `cfg!`) ;
        // les entrées trop larges / utilisateur / traversée sont refusées.
        let _garde = VERROU_ENV.lock().unwrap();
        let sauve_extra = std::env::var("CLEANX_PROTECTED_EXTRA").ok();
        let sauve_dev = std::env::var("CLEANX_DEV_MODE").ok();
        std::env::remove_var("CLEANX_PROTECTED_EXTRA");
        std::env::remove_var("CLEANX_DEV_MODE");

        // 1. Variable seule (attaquant) → ignorée dans tous les profils.
        std::env::set_var("CLEANX_PROTECTED_EXTRA", "/tmp/piege");
        assert!(lister_extras().is_empty());
        assert!(!est_chemin_protege("/tmp/piege/x"));

        if cfg!(debug_assertions) {
            // 2. Avec DEV_MODE : acceptation + refus documentés.
            std::env::set_var("CLEANX_DEV_MODE", "1");
            std::env::set_var(
                "CLEANX_PROTECTED_EXTRA",
                "/tmp/fake-sys;D:/temoin;/;/tmp/a/../b;C:/;C:/Users;/home;/home/ali;C:/Users/Public/x;E:/Users/moi/Documents/f",
            );
            let (acceptes, refuses) = lister_extras_avec_refus();
            assert_eq!(
                acceptes,
                vec!["/tmp/fake-sys".to_string(), "d:/temoin".to_string()]
            );
            let mut refuses_tries = refuses.clone();
            refuses_tries.sort();
            // ("/" se normalise en "" : ignoré silencieusement, pas journalisé.)
            assert_eq!(
                refuses_tries,
                vec![
                    "/home",
                    "/home/ali",
                    "/tmp/a/../b",
                    "c:",
                    "c:/users",
                    "c:/users/public/x",
                    "e:/users/moi/documents/f",
                ]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
            );
            assert!(est_chemin_protege("/tmp/fake-sys/evil.exe"));
            assert!(!est_chemin_protege("/tmp/a/../b/evil.exe"));
        } else {
            // 3. Release : même avec DEV_MODE, variable totalement ignorée.
            std::env::set_var("CLEANX_DEV_MODE", "1");
            std::env::set_var("CLEANX_PROTECTED_EXTRA", "/tmp/piege2");
            assert!(lister_extras().is_empty());
        }

        // Restauration de l'environnement hérité.
        match sauve_extra {
            Some(v) => std::env::set_var("CLEANX_PROTECTED_EXTRA", v),
            None => std::env::remove_var("CLEANX_PROTECTED_EXTRA"),
        }
        match sauve_dev {
            Some(v) => std::env::set_var("CLEANX_DEV_MODE", v),
            None => std::env::remove_var("CLEANX_DEV_MODE"),
        }
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
