//! Non-régression B14 — zéro faux positif, zéro action sans consentement.
//!
//! Scénarios (mode Agressif = le plus permissif, pour prouver que même lui
//! n'agit jamais sans base confirmée ni sur chemin protégé) :
//! 1. Fichiers sains (vide, « test », document) → 0 quarantaine, intacts.
//! 2. Marqueur CONFIRMÉ (hash injecté en base, confiance 100) placé dans un
//!    chemin protégé (`CLEANX_PROTECTED_EXTRA`) et dans `node_modules` →
//!    0 quarantaine, fichiers intacts (AGENTS.md §4bis : jamais d'action
//!    auto sur chemin système/confiance/dev, même confiance 100).
//! 3. Même marqueur hors zone protégée → isolé (le moteur sait encore agir).
//!
//! Contenus Defender-safe (aucun malware réel, EICAR non écrit sur disque).

use cleanx_core::api;
use cleanx_core::frb_generated::StreamSink;
use flutter_rust_bridge::for_generated::SseCodec;

type SinkTest = StreamSink<cleanx_core::api::EvenementMoteur, SseCodec>;

/// Marqueur confirmé : hash injecté en base (simule un malware avéré).
const MARQUEUR: &[u8] = b"marqueur-b14-confirme-7f3a-faux-positif-jamais-isole-auto";

fn sink_muet() -> SinkTest {
    SinkTest::deserialize("0".to_string())
}

/// Verrou global du binaire (état moteur + env partagés entre tests).
static VERROU_FP: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn zero_faux_positif_et_protege_jamais_auto() {
    // Les tests de ce binaire partagent l'état global (ETAT, MODE_DECISION,
    // variables d'environnement) : sérialisation obligatoire.
    let _garde = VERROU_FP.lock().unwrap();
    let dir = tempfile::tempdir().expect("tempdir");
    let base = dir.path().join("fp");
    std::fs::create_dir_all(&base).expect("mkdir");
    api::initialiser(base.to_string_lossy().into_owned()).expect("init");

    // Injection du marqueur comme malware confirmé (confiance 100).
    let db_path = base.join("cleanx.db");
    let sha = {
        use sha2::{Digest, Sha256};
        hex::encode(Sha256::digest(MARQUEUR))
    };
    {
        let conn = cleanx_core::db::ouvrir(&db_path).expect("open db");
        conn.execute(
            "INSERT INTO signatures(hash, nom) VALUES (?1, ?2)",
            (sha.as_str(), "B14-Marqueur-Confirme"),
        )
        .expect("insert marqueur");
    }

    // Arborescence : zone protégée simulée + projet dev + dossier normal.
    let protege = dir.path().join("fake-sys").join("System32");
    let dev = dir.path().join("projet").join("node_modules").join("lib");
    let normal = dir.path().join("docs");
    for d in [&protege, &dev, &normal] {
        std::fs::create_dir_all(d).expect("mkdir arbre");
    }
    // Chemins protégés : via la surcouche dev/CI (Correctif 2 : EXIGE
    // CLEANX_DEV_MODE=1, sinon la variable est ignorée).
    std::env::set_var("CLEANX_DEV_MODE", "1");
    std::env::set_var(
        "CLEANX_PROTECTED_EXTRA",
        dir.path().join("fake-sys").to_string_lossy().into_owned(),
    );

    // Fichiers sains (anciens seeds dangereux : vide + « test »).
    std::fs::write(normal.join("vide.bin"), b"").expect("w vide");
    std::fs::write(normal.join("test.txt"), b"test").expect("w test");
    std::fs::write(normal.join("doc.txt"), b"Bonjour, document inoffensif.").expect("w doc");
    // Marqueur confirmé en zones protégées.
    let m_protege = protege.join("driver.sys");
    let m_dev = dev.join("payload.js");
    std::fs::write(&m_protege, MARQUEUR).expect("w protege");
    std::fs::write(&m_dev, MARQUEUR).expect("w dev");

    // Mode le plus permissif : s'il n'isole pas ici, aucun mode ne le fera.
    api::definir_mode(cleanx_core::mode::ModeDecision::Agressif).expect("mode agressif");
    api::scan_personnalise(
        vec![
            dir.path().join("fake-sys").to_string_lossy().into_owned(),
            dir.path().join("projet").to_string_lossy().into_owned(),
            normal.to_string_lossy().into_owned(),
        ],
        sink_muet(),
    )
    .expect("scan");

    let quar = api::lister_quarantaine().expect("quarantaine");
    assert!(
        quar.is_empty(),
        "B14 : 0 quarantaine attendue (sains + protégés), trouvé : {quar:?}"
    );
    for f in [
        normal.join("vide.bin"),
        normal.join("test.txt"),
        normal.join("doc.txt"),
        m_protege.clone(),
        m_dev.clone(),
    ] {
        assert!(f.is_file(), "B14 : fichier intact exigé : {}", f.display());
    }

    // Contre-preuve : le même marqueur hors zone protégée EST isolé en
    // Automatique (source confirmée, confiance 100 — le moteur sait agir
    // sur menace avérée dès opt-in, sans exiger de score heuristique).
    let m_normal = normal.join("malware.bin");
    std::fs::write(&m_normal, MARQUEUR).expect("w normal");
    api::definir_mode(cleanx_core::mode::ModeDecision::Automatique).expect("mode auto");
    api::scan_personnalise(vec![normal.to_string_lossy().into_owned()], sink_muet())
        .expect("scan2");
    assert_eq!(
        api::lister_quarantaine().expect("quarantaine2").len(),
        1,
        "B14 : le marqueur confirmé hors zone protégée doit être isolé en Automatique"
    );

    // Nettoyage global (Défaut usine + env) pour les autres tests du binaire.
    std::env::remove_var("CLEANX_PROTECTED_EXTRA");
    std::env::remove_var("CLEANX_DEV_MODE");
    api::definir_mode(cleanx_core::mode::ModeDecision::Prudent).expect("mode reset");
    api::liberer_ressources().expect("liberer");
}

/// Contenu « signé » simulé (inoffensif, Defender-safe) : en l'absence
/// d'Authenticode (D26 reporté), la mitigation traite tout fichier de
/// confiance < 95 comme potentiellement légitime → jamais d'auto.
const MARQUEUR_SIGNE: &[u8] = b"contenu signe simule B14 (Microsoft/Adobe/Google, inoffensif)";

/// Mitigation D26 : hors chemin protégé, confiance < 95 → notification
/// UNIQUEMENT, même en Automatique ; Agressif exige ≥ 80 ET hors zone
/// utilisateur. Cas exigés : D:\Temp, C:\Users\Public, /tmp (simulés par
/// des dossiers neutres — les littéraux Windows sont prouvés au niveau
/// gating dans `mode::tests::mitigation_d26_litteraux_surveilles`).
#[test]
fn mitigation_d26_zero_auto_hors_confirme() {
    let _garde = VERROU_FP.lock().unwrap();
    let dir = tempfile::tempdir().expect("tempdir");
    let base = dir.path().join("d26");
    std::fs::create_dir_all(&base).expect("mkdir");
    api::initialiser(base.to_string_lossy().into_owned()).expect("init");

    // Motif générique (confiance 70) couvrant MARQUEUR_SIGNE, construit
    // par programme (pas d'hexadécimal recopié à la main).
    let motif_hex: String = MARQUEUR_SIGNE.iter().map(|b| format!("{b:02X}")).collect();
    cleanx_core::generiques::enregistrer_source(&format!("Test-Signe-Simule:0:*:{motif_hex}"))
        .expect("motif signe");
    // Marqueur confirmé (confiance 100) pour la règle « zone utilisateur ».
    let sha_confirme = {
        use sha2::{Digest, Sha256};
        hex::encode(Sha256::digest(MARQUEUR))
    };
    {
        let conn = cleanx_core::db::ouvrir(&base.join("cleanx.db")).expect("open db");
        conn.execute(
            "INSERT INTO signatures(hash, nom) VALUES (?1, ?2)",
            (sha_confirme.as_str(), "D26-Marqueur-Confirme"),
        )
        .expect("insert confirme");
    }

    // Stand-ins neutres (ni protégés, ni zones utilisateur) + Documents réel.
    let dtemp = dir.path().join("sim-dtemp"); // stand-in D:\Temp
    let public = dir.path().join("sim-public"); // stand-in C:\Users\Public
    let tmp = dir.path().join("sim-tmp"); // stand-in /tmp
    let docs = dir.path().join("Documents"); // segment utilisateur réel
    for d in [&dtemp, &public, &tmp, &docs] {
        std::fs::create_dir_all(d).expect("mkdir d26");
        assert!(
            !cleanx_core::mode::est_chemin_protege(&d.to_string_lossy()),
            "stand-in protégé par accident : {}",
            d.display()
        );
    }
    assert!(cleanx_core::mode::est_zone_utilisateur(
        &docs.to_string_lossy()
    ));
    let f_dtemp = dtemp.join("ms_signed.exe");
    let f_public = public.join("adobe_setup.exe");
    let f_tmp = tmp.join("google_tool");
    let f_docs = docs.join("outil.exe");
    for f in [&f_dtemp, &f_public, &f_tmp, &f_docs] {
        std::fs::write(f, MARQUEUR_SIGNE).expect("w signe");
    }
    // Marqueur CONFIRMÉ en zone utilisateur (règle Agressif).
    let f_docs_confirme = docs.join("malware.exe");
    std::fs::write(&f_docs_confirme, MARQUEUR).expect("w confirme");

    let racines = vec![
        dtemp.to_string_lossy().into_owned(),
        public.to_string_lossy().into_owned(),
        tmp.to_string_lossy().into_owned(),
        docs.to_string_lossy().into_owned(),
    ];

    // Automatique : confiance 70 < 95 → 0 quarantaine, tout intact
    // (y compris le marqueur confirmé… non : le confirmé en Documents EST
    // isolé en Automatique — voir contre-preuve ci-dessous. On scanne donc
    // d'abord SANS lui : renommage temporaire.
    std::fs::rename(&f_docs_confirme, dir.path().join("malware.hold")).expect("hold");
    api::definir_mode(cleanx_core::mode::ModeDecision::Automatique).expect("mode auto");
    api::scan_personnalise(racines.clone(), sink_muet()).expect("scan auto");
    assert!(
        api::lister_quarantaine()
            .expect("quarantaine auto")
            .is_empty(),
        "D26 : 0 quarantaine auto attendue à confiance 70"
    );
    for f in [&f_dtemp, &f_public, &f_tmp, &f_docs] {
        assert!(f.is_file(), "D26 : intact exigé : {}", f.display());
    }

    // Agressif : même à confiance 70 → 0 ; et le confirmé en zone
    // utilisateur → 0 aussi (règle zone). Tout reste intact.
    std::fs::rename(dir.path().join("malware.hold"), &f_docs_confirme).expect("restore");
    api::definir_mode(cleanx_core::mode::ModeDecision::Agressif).expect("mode agressif");
    api::scan_personnalise(racines.clone(), sink_muet()).expect("scan agressif");
    assert!(
        api::lister_quarantaine()
            .expect("quarantaine agressif")
            .is_empty(),
        "D26 : 0 quarantaine en Agressif (confiance < 80 + zone utilisateur)"
    );
    assert!(
        f_docs_confirme.is_file(),
        "D26 : le confirmé en zone utilisateur ne doit pas être isolé en Agressif"
    );

    // Contre-preuve : le confirmé en Documents EST isolé en Automatique
    // (la règle « zone utilisateur » est spécifique à l'Agressif).
    api::definir_mode(cleanx_core::mode::ModeDecision::Automatique).expect("mode auto2");
    api::scan_personnalise(racines, sink_muet()).expect("scan auto2");
    assert_eq!(
        api::lister_quarantaine().expect("quarantaine auto2").len(),
        1,
        "D26 : le confirmé hors règle de zone doit être isolé en Automatique"
    );

    api::definir_mode(cleanx_core::mode::ModeDecision::Prudent).expect("mode reset");
    api::liberer_ressources().expect("liberer");
}
