//! Fuzzing du cœur heuristique pur (`analyser_contenu`) — Cycle 8.
//!
//! Lancement (nightly requis) :
//! `cargo +nightly fuzz run heuristique -- -max_total_time=1800`
//! Invariants vérifiés : score borné 0–100, verdict cohérent avec les seuils,
//! aucun panic quelle que soit l'entrée (nom + octets arbitraires).
#![no_main]

use cleanx_core::heuristics::{Seuils, VerdictHeuristique, analyser_contenu};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Nom de fichier arbitraire (64 premiers octets), contenu = le reste.
    let coupure = data.len().min(64);
    let (nom_octets, contenu) = data.split_at(coupure);
    let nom = String::from_utf8_lossy(nom_octets).to_lowercase();
    let seuils = Seuils::default();

    let a = analyser_contenu(&nom, contenu, contenu.len() as u64, seuils);

    assert!(a.score <= 100, "score hors bornes : {}", a.score);
    let attendu = if a.score >= seuils.menace {
        VerdictHeuristique::Menace
    } else if a.score >= seuils.suspect {
        VerdictHeuristique::Suspect
    } else {
        VerdictHeuristique::Sain
    };
    assert_eq!(a.verdict, attendu, "verdict incohérent pour score {}", a.score);
});
