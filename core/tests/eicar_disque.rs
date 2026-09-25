//! Test EICAR sur disque réel — CI Linux uniquement.
//!
//! Écrit les octets EICAR dans un fichier, lance un vrai scan, vérifie
//! détection + quarantaine + restauration, chronomètre (< 2 s visés).
//! NE S'EXÉCUTE QUE SI `CLEANX_EICAR_DISK=1` : sur un poste avec antivirus
//! hôte (ex. Defender, os error 225), l'AV verrouille le fichier avant nous
//! (cf. BUGS.md B02) — le skip est donc explicite et documenté, pas un oubli.

use std::time::Instant;

use cleanx_core::api;
use cleanx_core::frb_generated::StreamSink;
use flutter_rust_bridge::for_generated::SseCodec;

type SinkTest = StreamSink<cleanx_core::api::EvenementMoteur, SseCodec>;

#[test]
fn eicar_fichier_detecte_et_mis_en_quarantaine() {
    if std::env::var("CLEANX_EICAR_DISK").as_deref() != Ok("1") {
        println!("SKIP : CLEANX_EICAR_DISK != 1 (antivirus hôte probablement actif)");
        return;
    }
    let dir = tempfile::tempdir().expect("tempdir");
    let base = dir.path().join("eicar");
    std::fs::create_dir_all(&base).expect("mkdir");
    api::initialiser(base.to_string_lossy().into_owned()).expect("init");

    // Dépôt du fichier EICAR sur disque réel.
    let eicar = base.join("eicar.com");
    std::fs::write(&eicar, cleanx_core::db::EICAR).expect("write eicar");

    // Scan réel du dossier (quarantaine auto activée comme en production).
    let debut = Instant::now();
    api::scan_personnalise(
        vec![base.to_string_lossy().into_owned()],
        SinkTest::deserialize("0".to_string()),
    )
    .expect("scan");
    let duree = debut.elapsed();
    println!("EICAR détecté en {:.2}s", duree.as_secs_f64());

    // Menace isolée, original déplacé.
    assert!(!eicar.exists(), "EICAR doit être mis en quarantaine");
    let quar = api::lister_quarantaine().expect("quarantaine");
    assert_eq!(quar.len(), 1);
    assert!(
        quar[0].raison.contains("EICAR"),
        "raison = {}",
        quar[0].raison
    );

    // Restauration bit-à-bit.
    let restaure = base.join("eicar-restaure.com");
    api::restaurer_quarantaine(quar[0].id, restaure.to_string_lossy().into_owned())
        .expect("restaure");
    assert_eq!(
        std::fs::read(&restaure).expect("read"),
        cleanx_core::db::EICAR
    );

    api::liberer_ressources().expect("liberer");
    // Budget généreux anti-flake CI ; la mesure imprimée fait foi (< 2 s visés).
    assert!(
        duree < std::time::Duration::from_secs(10),
        "trop lent : {duree:?}"
    );
}
