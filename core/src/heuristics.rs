//! Analyse heuristique : score de suspicion 0–100 sans exécuter le fichier.
//!
//! Signaux : extensions à risque, double extension trompeuse (`doc.pdf.exe`),
//! taille anormale, motifs suspects (regex), entropie élevée (packé/chiffré).
//! Seuils : ≥ 70 = menace, ≥ 40 = suspect. Seuils configurables via [`Seuils`].

use std::collections::HashSet;
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

/// Extensions exécutables / scriptées à risque.
const EXTENSIONS_RISQUE: &[&str] = &[
    ".exe", ".scr", ".com", ".bat", ".cmd", ".ps1", ".vbs", ".vbe", ".js", ".jse", ".wsf", ".wsh",
    ".jar", ".msi", ".dll", ".lnk", ".hta", ".cpl", ".apk", ".dylib", ".so",
];

/// Extensions "leurre" typiques des doubles extensions.
const EXTENSIONS_LEURRE: &[&str] = &[
    ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".jpg", ".png", ".mp4", ".zip", ".txt", ".csv", ".md",
];

/// Octets lus au maximum pour la recherche de motifs (2 Mo suffisent).
const MAX_OCTETS_LUS: u64 = 2 * 1024 * 1024;

/// Verdict heuristique (exposé à Dart comme enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictHeuristique {
    Sain,
    Suspect,
    Menace,
}

/// Seuils de classification (modifiables depuis les paramètres).
#[derive(Debug, Clone, Copy)]
pub struct Seuils {
    /// Score minimal pour `Menace` (défaut 70).
    pub menace: u8,
    /// Score minimal pour `Suspect` (défaut 40).
    pub suspect: u8,
}

impl Default for Seuils {
    fn default() -> Self {
        Self {
            menace: 70,
            suspect: 40,
        }
    }
}

/// Résultat de l'analyse heuristique (exposé à Dart).
#[derive(Debug, Clone)]
pub struct AnalyseHeuristique {
    /// Score 0–100.
    pub score: u8,
    /// Signaux détectés (libellés français).
    pub signaux: Vec<String>,
    /// Classification selon les seuils.
    pub verdict: VerdictHeuristique,
}

/// Motifs suspects : (regex, points, libellé).
fn motifs() -> &'static [(Regex, u8, &'static str)] {
    static CACHE: OnceLock<Vec<(Regex, u8, &'static str)>> = OnceLock::new();
    const DEFS: &[(&str, u8, &str)] = &[
        (r"powershell\s+-[eE]nc", 30, "PowerShell encodé (-enc)"),
        (
            r"frombase64string|tobase64string",
            15,
            "Encodage Base64 via API",
        ),
        (
            r"createremotethread|virtualallocex|writeprocessmemory",
            30,
            "Injection de processus",
        ),
        (r"mimikatz|sekurlsa|lsass", 35, "Vol d'identifiants"),
        (
            r"currentversion\\run|hkcu.*\\run",
            15,
            "Persistance au démarrage",
        ),
        (
            r"bitsadmin|certutil\s+-urlcache|invoke-mimikatz|invoke-shellcode",
            25,
            "Téléchargement furtif",
        ),
        (
            r"subprocess|os\.system|shell\s*=\s*true",
            20,
            "Exécution de sous-processus",
        ),
        (
            r"\beval\s*\(|\bexec\s*\(|obfuscat|chr\s*\(\s*\d+",
            10,
            "Obfuscation possible",
        ),
        (
            r"curl.+\|\s*(ba)?sh|wget.+\|\s*(ba)?sh",
            20,
            "Pipe shell distant",
        ),
        (
            r"ransom|decrypt.*bitcoin|vssadmin\s+delete\s+shadows",
            35,
            "Comportement rançongiciel",
        ),
        (
            r"keylog|setwindowshookex|getasynckeystate",
            25,
            "Keylogger potentiel",
        ),
    ];
    CACHE.get_or_init(|| {
        // `filter_map` volontaire : les motifs sont des constantes relues en
        // revue ; un motif invalide désactive UN signal au lieu de paniquer
        // (fail-open localisé, jamais de crash sur entrée utilisateur).
        DEFS.iter()
            .filter_map(|(m, p, l)| Regex::new(&format!("(?i){m}")).ok().map(|re| (re, *p, *l)))
            .collect()
    })
}

/// Entropie de Shannon (bits/octet). > 7.5 ≈ packé/chiffré/aléatoire.
pub fn entropie_shannon(donnees: &[u8]) -> f64 {
    if donnees.is_empty() {
        return 0.0;
    }
    let mut freq = [0u64; 256];
    for &b in donnees {
        freq[b as usize] += 1;
    }
    let n = donnees.len() as f64;
    freq.iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.log2()
        })
        .sum()
}

/// Analyse un fichier. Ne panique jamais : toute erreur E/S devient un signal.
pub fn analyser(chemin: &Path, seuils: Seuils) -> AnalyseHeuristique {
    let nom = chemin
        .file_name()
        .map(|s| s.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    match lire_echantillon(chemin) {
        Ok((contenu, taille)) => analyser_contenu(&nom, &contenu, taille, seuils),
        Err(e) => AnalyseHeuristique {
            score: 0,
            signaux: vec![format!("Contenu illisible (verrouillé ?) : {e}")],
            verdict: VerdictHeuristique::Sain,
        },
    }
}

/// Cœur pur de l'heuristique (aucune E/S) : fuzzable (`fuzz/fuzz_targets/`)
/// et testable unitairement avec des octets arbitraires.
pub fn analyser_contenu(
    nom_minuscule: &str,
    contenu: &[u8],
    taille: u64,
    seuils: Seuils,
) -> AnalyseHeuristique {
    let mut signaux = Vec::new();
    let mut score: u32 = 0;

    // Extension au sens `Path::extension` (après le dernier point, sauf
    // fichier dotfile comme ".htaccess" qui n'a pas d'extension).
    let ext = nom_minuscule
        .rfind('.')
        .filter(|&i| i > 0)
        .map(|i| nom_minuscule[i..].to_string())
        .unwrap_or_default();

    let risque: HashSet<&str> = HashSet::from_iter(EXTENSIONS_RISQUE.iter().copied());
    let leurre: HashSet<&str> = HashSet::from_iter(EXTENSIONS_LEURRE.iter().copied());

    if risque.contains(ext.as_str()) {
        score += 20;
        signaux.push(format!("Extension exécutable à risque : {ext}"));
    }
    // Double extension : avant-dernière partie = leurre, finale = exécutable.
    let parties: Vec<&str> = nom_minuscule.split('.').collect();
    if parties.len() >= 3 {
        let avant = format!(".{}", parties[parties.len() - 2]);
        if leurre.contains(avant.as_str()) && !leurre.contains(ext.as_str()) {
            score += 25;
            signaux.push(format!("Double extension trompeuse : {nom_minuscule}"));
        }
    }

    if risque.contains(ext.as_str()) && taille < 10 * 1024 {
        score += 10;
        signaux.push(format!("Exécutable anormalement petit ({taille} octets)"));
    }
    let texte = String::from_utf8_lossy(contenu);
    for (re, points, libelle) in motifs() {
        if re.is_match(&texte) {
            score += *points as u32;
            signaux.push(format!("Motif suspect : {libelle}"));
        }
    }
    let ent = entropie_shannon(contenu);
    if ent > 7.5 && taille > 1024 {
        score += 15;
        signaux.push(format!("Entropie élevée ({ent:.2}) : packé/chiffré ?"));
    }

    let score = score.min(100) as u8;
    let verdict = if score >= seuils.menace {
        VerdictHeuristique::Menace
    } else if score >= seuils.suspect {
        VerdictHeuristique::Suspect
    } else {
        VerdictHeuristique::Sain
    };
    AnalyseHeuristique {
        score,
        signaux,
        verdict,
    }
}

/// Lit jusqu'à `MAX_OCTETS_LUS` octets + taille totale. Tolère les verrous partiels.
fn lire_echantillon(chemin: &Path) -> std::io::Result<(Vec<u8>, u64)> {
    use std::io::Read;
    let meta = std::fs::metadata(chemin)?;
    let mut f = std::fs::File::open(chemin)?;
    let mut buf = Vec::new();
    f.by_ref().take(MAX_OCTETS_LUS).read_to_end(&mut buf)?;
    Ok((buf, meta.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn double_extension_trompeuse() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("facture.pdf.exe");
        std::fs::write(&p, b"contenu anodin").unwrap();
        let a = analyser(&p, Seuils::default());
        assert!(a.signaux.iter().any(|s| s.contains("Double extension")));
        assert!(a.score >= 40);
    }

    #[test]
    fn powershell_encode_detecte() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("script.ps1");
        std::fs::write(&p, b"powershell -enc aGVsbG8= + CreateRemoteThread").unwrap();
        let a = analyser(&p, Seuils::default());
        assert_eq!(a.verdict, VerdictHeuristique::Menace);
    }

    #[test]
    fn fichier_sain() {
        let mut f = tempfile::NamedTempFile::with_suffix(".txt").unwrap();
        f.write_all(b"Bonjour, ceci est un document tout a fait normal.")
            .unwrap();
        let a = analyser(f.path(), Seuils::default());
        assert_eq!(a.verdict, VerdictHeuristique::Sain);
    }

    #[test]
    fn entropie_aleatoire_elevee() {
        let alea: Vec<u8> = (0u8..=255).cycle().take(4096).collect();
        assert!(entropie_shannon(&alea) > 7.5);
        assert_eq!(entropie_shannon(b"aaaaaaaaaaaaaaaa"), 0.0);
    }

    #[test]
    fn inexistant_sans_panic() {
        let a = analyser(Path::new("/n/existe/pas.bin"), Seuils::default());
        assert_eq!(a.verdict, VerdictHeuristique::Sain);
        assert!(!a.signaux.is_empty());
    }

    #[test]
    fn contenu_pur_equivalent_fichier() {
        // Non-régression du refactor : même verdict via octets directs.
        let contenu = b"powershell -enc aGVsbG8= + CreateRemoteThread";
        let via_contenu = analyser_contenu(
            "outil.exe",
            contenu,
            contenu.len() as u64,
            Seuils::default(),
        );
        assert_eq!(via_contenu.verdict, VerdictHeuristique::Menace);
        // Sans extension ni point : pas de signal d'extension.
        let sans_ext = analyser_contenu("README", b"texte", 5, Seuils::default());
        assert!(sans_ext.signaux.iter().all(|s| !s.contains("Extension")));
        // Dotfile : pas d'extension non plus.
        let dot = analyser_contenu(".htaccess", b"texte", 5, Seuils::default());
        assert!(dot.signaux.iter().all(|s| !s.contains("Extension")));
    }
}
