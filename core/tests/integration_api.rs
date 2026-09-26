//! Tests d'intégration de la surface FFI (`api.rs`) avec un sink factice.
//!
//! `StreamSink::deserialize("0")` construit un canal vers un port Dart
//! inexistant : les envois échouent silencieusement (ignorés par `let _ =`),
//! ce qui permet d'exercer scan + protection sans VM Dart.
//! Aucun contenu dangereux (compatible antivirus hôte) ; dossiers temporaires
//! uniquement (jamais de scan des vrais dossiers utilisateur).

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use cleanx_core::api;
use cleanx_core::frb_generated::StreamSink;
use flutter_rust_bridge::for_generated::SseCodec;

type SinkTest = StreamSink<cleanx_core::api::EvenementMoteur, SseCodec>;

/// Sink vers nulle part (port Dart 0 = inexistant, envois ignorés).
fn sink_muet() -> SinkTest {
    SinkTest::deserialize("0".to_string())
}

/// Base temporaire unique par test (pas de course sur l'état global :
/// UN SEUL test touche `ETAT` à la fois — voir `pipeline_bout_en_bout`).
fn base_temp(prefixe: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let base = dir.path().join(prefixe);
    std::fs::create_dir_all(&base).expect("mkdir base");
    (dir, base)
}

#[test]
fn pipeline_bout_en_bout() {
    let (_dir, base) = base_temp("e2e");
    let base_txt = base.to_string_lossy().into_owned();

    // 1. Init + statut.
    let statut = api::initialiser(base_txt.clone()).expect("initialiser");
    assert!(statut.signatures >= 1);
    assert_eq!(api::statut().expect("statut").signatures, statut.signatures);

    // 2. Fonctions pures.
    let fixtures = base.join("fixtures");
    std::fs::create_dir_all(&fixtures).expect("mkdir fixtures");
    let sain = fixtures.join("bonjour.txt");
    std::fs::write(&sain, b"Document de test, inoffensif.").expect("write sain");
    let sha = api::calculer_sha256(sain.to_string_lossy().into_owned()).expect("sha");
    assert_eq!(sha.len(), 64);
    let h = api::analyser_heuristique_api(sain.to_string_lossy().into_owned()).expect("heur");
    assert_eq!(h.score, 0);
    assert!(matches!(
        h.verdict,
        cleanx_core::heuristics::VerdictHeuristique::Sain
    ));
    let suspect = fixtures.join("note.txt.exe");
    std::fs::write(&suspect, b"Contenu inoffensif.").expect("write suspect");
    let hs = api::analyser_heuristique_api(suspect.to_string_lossy().into_owned()).expect("heur2");
    assert!(hs.score >= 40); // double extension .txt.exe détectée

    // 3. Scan personnalisé (2 fichiers sains/suspects, 0 menace).
    api::scan_personnalise(vec![fixtures.to_string_lossy().into_owned()], sink_muet())
        .expect("scan");
    assert!(api::statut().expect("statut2").fichiers_analyses >= 2);

    // 4. Racine invalide + scan déjà en cours gérés proprement.
    assert!(api::scan_personnalise(vec!["/chemin/inexistant/xyz".into()], sink_muet()).is_err());
    assert!(!api::arreter_scan().expect("stop idle")); // aucun scan : false
    api::suspendre_scan(true).expect("pause");
    api::suspendre_scan(false).expect("reprise");

    // 5. Quarantaine directe + logs + restauration + suppression.
    let id = api::mettre_en_quarantaine(
        suspect.to_string_lossy().into_owned(),
        "Test intégration".into(),
    )
    .expect("quarantaine");
    assert!(!suspect.exists());
    assert_eq!(api::lister_quarantaine().expect("liste").len(), 1);
    let restaure = fixtures.join("restaure.exe");
    api::restaurer_quarantaine(id, restaure.to_string_lossy().into_owned()).expect("restaure");
    assert!(restaure.is_file());
    let id2 = api::mettre_en_quarantaine(
        restaure.to_string_lossy().into_owned(),
        "Test intégration (2)".into(),
    )
    .expect("quarantaine2");
    api::supprimer_quarantaine(id2).expect("supprime");
    assert!(api::lister_quarantaine().expect("liste2").is_empty());
    let logs = api::lister_logs(50).expect("logs");
    assert!(!logs.is_empty());
    assert!(api::exporter_logs_csv()
        .expect("csv")
        .starts_with("id,date,niveau,message"));

    // 6. Planifications CRUD.
    let pid = api::ajouter_planification(
        "Hebdo-Test".into(),
        vec![fixtures.to_string_lossy().into_owned()],
        604800,
    )
    .expect("plan");
    assert_eq!(api::lister_planifications().expect("plans").len(), 1);
    api::supprimer_planification(pid).expect("plan del");
    assert!(api::lister_planifications().expect("plans2").is_empty());

    // 7. Dossiers surveillés.
    let dossier = base.join("surveille");
    std::fs::create_dir_all(&dossier).expect("mkdir surveille");
    let liste = api::ajouter_dossier(dossier.to_string_lossy().into_owned()).expect("add");
    assert!(liste.contains(&dossier.to_string_lossy().into_owned()));
    assert!(api::ajouter_dossier("/chemin/inexistant/xyz".into()).is_err());
    let liste = api::retirer_dossier(dossier.to_string_lossy().into_owned()).expect("del");
    assert!(!liste.contains(&dossier.to_string_lossy().into_owned()));
    assert!(!api::dossiers_surveilles().expect("dossiers").is_empty()); // défauts init

    // 8. Divers : dépôt injoignable, intégrité, clés, paquets.
    assert!(futures_executor_block_on(api::mettre_a_jour_signatures(
        "http://127.0.0.1:9/injoignable".into(),
        "00".repeat(32)
    ))
    .is_err());
    assert_eq!(api::integrite_binaire().expect("empreinte").len(), 64);
    assert!(api::verifier_integrite_api(None).expect("integrite"));
    let cle =
        api::deriver_cle_api("phrase".into(), "c2VsLXRlc3QtMTYtb2N0ZXRz".into()).expect("argon2");
    assert_eq!(cle.len(), 64);
    assert!(
        !api::verifier_paquet_api(vec![1, 2, 3], "00".repeat(64), "00".repeat(32)).expect("paquet")
    );

    // 9. Protection temps réel : active, détecte, se coupe.
    let dossier_rt = base.join("rt");
    std::fs::create_dir_all(&dossier_rt).expect("mkdir rt");
    api::ajouter_dossier(dossier_rt.to_string_lossy().into_owned()).expect("add rt");
    let arret = AtomicBool::new(false);
    let handle = std::thread::spawn(|| api::activer_protection(sink_muet()));
    std::thread::sleep(Duration::from_secs(1));
    // Fichier bénin déposé pendant la surveillance (grâce d'écriture 1 s + analyse).
    std::fs::write(dossier_rt.join("arrive.txt"), b"Bonjour temps reel.").expect("write rt");
    std::thread::sleep(Duration::from_secs(4));
    assert!(api::desactiver_protection().expect("stop protection"));
    handle.join().expect("thread protection").expect("activer");
    assert!(!api::desactiver_protection().expect("stop idle2")); // déjà coupée : false
    let _ = arret.load(Ordering::Relaxed);

    // 10. Libération des ressources (verrous SQLite relâchés).
    api::liberer_ressources().expect("liberer");

    // 11. Mode jeu : bascule + état, sans effet sur un moteur au repos.
    assert!(!api::mode_jeu_actif().expect("jeu0"));
    assert!(!api::mode_jeu(true).expect("jeu on")); // précédent : inactif
    assert!(api::mode_jeu_actif().expect("jeu1"));
    assert!(api::mode_jeu(false).expect("jeu off")); // précédent : actif
    assert!(!api::mode_jeu_actif().expect("jeu2"));

    // 12. P14 : en Prudent (défaut), une menace est SIGNALÉE sans action :
    // fichier intact + quarantaine vide. Preuve du consentement par défaut.
    // Suivi D26 : le marqueur est un hash CONFIRMÉ injecté en base
    // (confiance 100) — le Prudent signale (jamais d'auto), l'Automatique
    // opt-in l'isole (100 ≥ 95). Contenu Defender-safe, comme avant.
    {
        use sha2::{Digest, Sha256};
        let sha = hex::encode(Sha256::digest(b"powershell -enc aGVsbG8="));
        let conn = cleanx_core::db::ouvrir(&base.join("cleanx.db")).expect("open db");
        conn.execute(
            "INSERT INTO signatures(hash, nom) VALUES (?1, ?2)",
            (sha.as_str(), "Test-Prudent-Auto-Confirme"),
        )
        .expect("insert menace");
    }
    api::definir_mode(cleanx_core::mode::ModeDecision::Prudent).expect("mode prudent");
    let menace_dir = base.join("prudent");
    std::fs::create_dir_all(&menace_dir).expect("mkdir prudent");
    let menace_f = menace_dir.join("dropper.txt.exe");
    std::fs::write(&menace_f, b"powershell -enc aGVsbG8=").expect("write menace");
    api::scan_personnalise(vec![menace_dir.to_string_lossy().into_owned()], sink_muet())
        .expect("scan prudent");
    assert!(
        menace_f.is_file(),
        "mode Prudent : le fichier ne doit être ni déplacé ni supprimé"
    );
    assert!(
        api::lister_quarantaine()
            .expect("quarantaine vide")
            .is_empty(),
        "mode Prudent : aucune quarantaine automatique"
    );
    // ... alors qu'en Automatique, la même menace est isolée (opt-in).
    api::definir_mode(cleanx_core::mode::ModeDecision::Automatique).expect("mode auto");
    api::scan_personnalise(vec![menace_dir.to_string_lossy().into_owned()], sink_muet())
        .expect("scan auto");
    assert!(
        !menace_f.exists(),
        "mode Automatique : la menace doit être mise en quarantaine"
    );
    assert_eq!(api::lister_quarantaine().expect("quarantaine").len(), 1);
    // Restauration du défaut usine pour les autres tests du binaire.
    api::definir_mode(cleanx_core::mode::ModeDecision::Prudent).expect("mode reset");
}

/// Exécuteur minimal pour la future async sans dépendance de test.
fn futures_executor_block_on<F: std::future::Future>(f: F) -> F::Output {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime test");
    rt.block_on(f)
}
