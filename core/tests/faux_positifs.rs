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

#[test]
fn zero_faux_positif_et_protege_jamais_auto() {
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
    // Chemins protégés : via la surcouche dev/CI (documentée, sans cache).
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
    api::definir_mode(cleanx_core::mode::ModeDecision::Prudent).expect("mode reset");
    api::liberer_ressources().expect("liberer");
}
