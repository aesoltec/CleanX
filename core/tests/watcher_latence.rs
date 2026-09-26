//! Latence bout-en-bout de la protection temps réel (R5).
//!
//! Dépose un fichier menace dans un dossier surveillé et mesure le délai
//! jusqu'à sa mise en quarantaine automatique (cible < 1 s).
//! Contenu déclencheur : motifs heuristiques bénins en eux-mêmes
//! (mêmes chaînes que le test unitaire `heuristics`, tolérées par Defender).

use std::time::{Duration, Instant};

use cleanx_core::api;
use cleanx_core::frb_generated::StreamSink;
use flutter_rust_bridge::for_generated::SseCodec;

type SinkTest = StreamSink<cleanx_core::api::EvenementMoteur, SseCodec>;

/// Score attendu : .exe (20) + powershell -enc (30) + CreateRemoteThread (30) = 80.
const CONTENU_MENACE: &[u8] = b"outil.exe test\npowershell -enc aGVsbG8=\nCreateRemoteThread demo";

#[test]
fn latence_detection_menace_moins_1s() {
    let dir = tempfile::tempdir().expect("tempdir");
    let base = dir.path().join("latence");
    std::fs::create_dir_all(&base).expect("mkdir");
    api::initialiser(base.to_string_lossy().into_owned()).expect("init");
    // Contrat historique : quarantaine auto = opt-in explicite (P14).
    // Ce test mesure la voie automatique : on l'active puis on restaure Prudent.
    api::definir_mode(cleanx_core::mode::ModeDecision::Automatique).expect("mode auto");

    let rt = base.join("rt");
    std::fs::create_dir_all(&rt).expect("mkdir rt");
    api::ajouter_dossier(rt.to_string_lossy().into_owned()).expect("add");

    let guetteur =
        std::thread::spawn(|| api::activer_protection(SinkTest::deserialize("0".to_string())));
    std::thread::sleep(Duration::from_millis(500)); // montée du watcher

    let depot = Instant::now();
    std::fs::write(rt.join("evil.exe"), CONTENU_MENACE).expect("write menace");

    // Scrutation de la quarantaine (effet observable, pas le sink muet).
    let trouve = loop {
        std::thread::sleep(Duration::from_millis(50));
        let elapsed = depot.elapsed();
        let quar = api::lister_quarantaine().expect("quarantaine");
        if !quar.is_empty() {
            break Some(elapsed);
        }
        if elapsed > Duration::from_secs(5) {
            break None;
        }
    };
    api::desactiver_protection().expect("stop");
    guetteur.join().expect("thread").expect("activer");

    let latence = trouve.expect("menace non mise en quarantaine en 5 s");
    println!("LATENCE temps réel : {:.2}s", latence.as_secs_f64());
    assert!(latence < Duration::from_secs(1), "trop lent : {latence:?}");

    api::liberer_ressources().expect("liberer");
    api::definir_mode(cleanx_core::mode::ModeDecision::Prudent).expect("mode reset");
}
