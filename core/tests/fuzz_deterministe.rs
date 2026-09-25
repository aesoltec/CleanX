//! Fuzzing structurel déterministe du cœur heuristique pur.
//!
//! Contexte : `cargo-fuzz`/libfuzzer ne lie pas sur Windows pour cette crate
//! (`crate-type` cdylib + `panic="abort"` → LNK2001 ; cible conservée dans
//! `core/fuzz/` pour la CI Linux). En attendant, ce test martèle
//! `analyser_contenu` avec 100 000+ entrées adverses générées (LCG seedé :
//! reproductible, sans dépendance) et vérifie les invariants.
//! Lancer : `cargo test --release --test fuzz_deterministe` (rapide même en debug).

use cleanx_core::heuristics::{analyser_contenu, Seuils, VerdictHeuristique};

/// Générateur congruentiel linéaire (déterministe, seed fixe).
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        // Constantes de Knuth (MMIX).
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() as usize) % n.max(1)
    }
}

/// Fragments adverses : motifs, syntaxes limites, unicode, traversal.
const FRAGMENTS: &[&str] = &[
    "powershell -enc ",
    "CreateRemoteThread",
    "mimikatz",
    "eval(",
    "exec(",
    "chr(65)",
    "frombase64string",
    "vssadmin delete shadows",
    "keylog",
    ".",
    "..",
    "...",
    ".exe",
    ".pdf.exe",
    "\u{0}",
    "\u{202e}",
    "\u{feff}",
    "é",
    "日本語",
    "a",
    "A",
    "-",
    "_",
    " ",
    "\t",
    "\n",
    "\r",
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    "powershell ",
    "-e",
    "enc",
    "cmd /c",
    "| sh",
    "base64",
];

/// Nom de fichier adverse (0–300 caractères).
fn nom_adverse(rng: &mut Lcg) -> String {
    let n = rng.below(12);
    let mut s = String::new();
    for _ in 0..n {
        s.push_str(FRAGMENTS[rng.below(FRAGMENTS.len())]);
    }
    s.to_lowercase()
}

/// Contenu adverse : octets aléatoires + fragments + blocs haute entropie.
fn contenu_adverse(rng: &mut Lcg) -> Vec<u8> {
    match rng.below(5) {
        // 0 : octets purement aléatoires (entropie max).
        0 => (0..rng.below(8192)).map(|_| rng.below(256) as u8).collect(),
        // 1 : fragments concaténés (patterns + syntaxes).
        1 => {
            let mut s = String::new();
            for _ in 0..rng.below(40) {
                s.push_str(FRAGMENTS[rng.below(FRAGMENTS.len())]);
            }
            s.into_bytes()
        }
        // 2 : répétition massive d'un motif (quasi-backtracking).
        2 => {
            let frag = FRAGMENTS[rng.below(FRAGMENTS.len())];
            frag.repeat(rng.below(5000)).into_bytes()
        }
        // 3 : texte ASCII calme + un motif noyé.
        _ => {
            let mut s = "lorem ipsum dolor sit amet ".repeat(rng.below(200));
            if rng.below(2) == 0 {
                s.push_str(FRAGMENTS[rng.below(FRAGMENTS.len())]);
            }
            s.into_bytes()
        }
    }
}

#[test]
fn fuzz_100k_entrees_sans_panic_ni_incoherence() {
    // Volume configurable (CI debug : CLEANX_FUZZ_N=20000 ≈ 100 s ;
    // run complet local/CI release : 100 000 par défaut).
    let n: usize = std::env::var("CLEANX_FUZZ_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000);
    let mut rng = Lcg(0xC0FFEE);
    let seuils = Seuils::default();
    let debut = std::time::Instant::now();
    for _ in 0..n {
        let nom = nom_adverse(&mut rng);
        let contenu = contenu_adverse(&mut rng);
        let taille = contenu.len() as u64;
        let a = analyser_contenu(&nom, &contenu, taille, seuils);
        assert!(a.score <= 100, "score hors bornes pour {nom:?}");
        let attendu = if a.score >= seuils.menace {
            VerdictHeuristique::Menace
        } else if a.score >= seuils.suspect {
            VerdictHeuristique::Suspect
        } else {
            VerdictHeuristique::Sain
        };
        assert_eq!(a.verdict, attendu, "verdict incohérent pour {nom:?}");
    }
    println!(
        "FUZZ : {n} entrées en {:.1}s sans crash",
        debut.elapsed().as_secs_f64()
    );
}

#[test]
fn fuzz_cas_limites() {
    let seuils = Seuils::default();
    // Vide, géant (3 Mo aléatoires), nom vide, que des points.
    assert_eq!(
        analyser_contenu("", &[], 0, seuils).verdict,
        VerdictHeuristique::Sain
    );
    let gros = vec![0xABu8; 3 * 1024 * 1024];
    let a = analyser_contenu("x.bin", &gros, gros.len() as u64, seuils);
    assert!(a.score <= 100);
    let points = analyser_contenu("...", b"...", 3, seuils);
    assert_eq!(points.verdict, VerdictHeuristique::Sain);
    let nul = analyser_contenu("a\x00.exe", b"\x00\x01\x02", 3, seuils);
    assert!(nul.score <= 100);
}
