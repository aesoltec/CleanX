//! Signatures génériques style ClamAV `.ndb` simplifié (B14).
//!
//! Un hash SHA-256 ne détecte que l'exact : les variantes passent au travers,
//! et des hash de fichiers bénins créent des faux positifs. Les signatures
//! génériques détectent des MOTIFS d'octets (avec jokers `??`), donc des
//! familles entières.
//!
//! Format : `NomMenace:TypeCible:Offset:HexSignature`
//! - `NomMenace` : 1–128 caractères (ex. `EICAR-Test-File`).
//! - `TypeCible` : `0` = tout fichier (seule valeur supportée en v1).
//! - `Offset` : `*` (n'importe où) ou position décimale exacte.
//! - `HexSignature` : hexadécimal pair, `??` = octet joker, 4–256 octets
//!   décodés (ni trop court = bruit, ni trop long = coût).
//!
//! Registre global lazy (EICAR pré-chargé) + `enregistrer_source` pour tests
//! et futures mises à jour. Entrées ≤ 4 Mo scannées (au-delà : préfixe).

use std::sync::{Mutex, OnceLock};

use crate::CleanXError;

/// Taille max scannée par motif (au-delà : préfixe uniquement, documenté).
pub const MAX_OCTETS_MOTIF: usize = 4 * 1024 * 1024;
/// Longueurs de motif acceptées (décodées).
const MIN_MOTIF: usize = 4;
const MAX_MOTIF: usize = 256;

/// Un octet de motif : valeur exacte ou joker (`??`).
/// Public car exposé dans [`SignatureGenerique`] (retournée par
/// [`parser_ligne`]) — jamais construit directement hors crate en pratique.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OctetMotif {
    Exact(u8),
    Joker,
}

/// Signature générique compilée.
#[derive(Debug, Clone)]
pub struct SignatureGenerique {
    /// Nom affiché (ex. `EICAR-Test-File`).
    pub nom: String,
    /// `None` = n'importe où, `Some(n)` = position exacte.
    pub offset: Option<usize>,
    /// Motif décodé.
    pub motif: Vec<OctetMotif>,
}

/// Détection par motif (confiance calibrée : 70, voir `confiance`).
#[derive(Debug, Clone)]
pub struct DetectionGenerique {
    pub nom: String,
}

/// Erreur de parsing affichable (jamais de panic sur entrée distrusted).
fn erreur_parse(detail: &str) -> CleanXError {
    CleanXError::Interne {
        detail: format!("signature générique invalide : {detail}"),
    }
}

/// Parse une ligne `Nom:Type:Offset:Hex`. Pure, testée, sans panic.
pub fn parser_ligne(ligne: &str) -> Result<SignatureGenerique, CleanXError> {
    let ligne = ligne.trim();
    let mut parties = ligne.splitn(4, ':');
    let nom = parties.next().unwrap_or("").trim();
    let type_cible = parties.next().unwrap_or("").trim();
    let offset_txt = parties.next().unwrap_or("").trim();
    let hex_txt = parties.next().unwrap_or("").trim();

    if nom.is_empty() || nom.len() > 128 {
        return Err(erreur_parse("nom vide ou > 128 caractères"));
    }
    if type_cible != "0" {
        return Err(erreur_parse("seul TypeCible=0 (tout fichier) est supporté"));
    }
    let offset = if offset_txt == "*" {
        None
    } else {
        Some(
            offset_txt
                .parse::<usize>()
                .map_err(|_| erreur_parse("offset ni '*' ni entier"))?,
        )
    };
    let condense: String = hex_txt.chars().filter(|c| !c.is_whitespace()).collect();
    if condense.len() % 2 != 0 {
        return Err(erreur_parse("hexadécimal impair"));
    }
    let mut motif = Vec::with_capacity(condense.len() / 2);
    let octets: Vec<char> = condense.chars().collect();
    for paire in octets.chunks(2) {
        if paire == ['?', '?'] {
            motif.push(OctetMotif::Joker);
        } else if paire.contains(&'?') {
            return Err(erreur_parse("demi-octet joker interdit (utilisez '??')"));
        } else {
            let txt: String = paire.iter().collect();
            let v = u8::from_str_radix(&txt, 16).map_err(|_| erreur_parse("hex invalide"))?;
            motif.push(OctetMotif::Exact(v));
        }
    }
    if motif.len() < MIN_MOTIF {
        return Err(erreur_parse("motif < 4 octets (bruit de faux positifs)"));
    }
    if motif.len() > MAX_MOTIF {
        return Err(erreur_parse("motif > 256 octets (coût de scan)"));
    }
    Ok(SignatureGenerique {
        nom: nom.to_string(),
        offset,
        motif,
    })
}

/// Vrai si le motif matche à `position` (jokers acceptés, bornes vérifiées).
fn matche_a(donnees: &[u8], motif: &[OctetMotif], position: usize) -> bool {
    if position.saturating_add(motif.len()) > donnees.len() {
        return false;
    }
    motif.iter().enumerate().all(|(i, o)| match o {
        OctetMotif::Joker => true,
        OctetMotif::Exact(v) => donnees[position + i] == *v,
    })
}

/// Recherche : position exacte ou fenêtre glissante (premier match gagne).
/// Motif vide → toujours `false` (garde anti-faux-positif : `all()` sur un
/// itérable vide vaut `true`, ce qui matcherait TOUS les fichiers).
pub fn rechercher(donnees: &[u8], sig: &SignatureGenerique) -> bool {
    if sig.motif.is_empty() {
        return false;
    }
    let zone = &donnees[..donnees.len().min(MAX_OCTETS_MOTIF)];
    match sig.offset {
        Some(pos) => matche_a(zone, &sig.motif, pos),
        None => {
            if sig.motif.is_empty() || zone.len() < sig.motif.len() {
                return false;
            }
            (0..=(zone.len() - sig.motif.len())).any(|p| matche_a(zone, &sig.motif, p))
        }
    }
}

static REGISTRE: OnceLock<Mutex<Vec<SignatureGenerique>>> = OnceLock::new();

fn registre() -> &'static Mutex<Vec<SignatureGenerique>> {
    REGISTRE.get_or_init(|| Mutex::new(Vec::new()))
}

/// Signature EICAR compilée depuis les octets de référence (`db::EICAR`).
fn signature_eicar() -> SignatureGenerique {
    let hex_motif: String = crate::db::EICAR
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect();
    parser_ligne(&format!("EICAR-Test-File:0:*:{hex_motif}"))
        // Infaillible par construction (constante relue en revue) : en cas
        // d'échec impossible, motif vide qui ne matchera jamais (pas de panic).
        .unwrap_or(SignatureGenerique {
            nom: "EICAR-Test-File".to_string(),
            offset: None,
            motif: Vec::new(),
        })
}

/// Amorce EICAR dans le registre (idempotent, sans doublon).
fn amorcer_eicar() {
    let mut reg = match registre().lock() {
        Ok(g) => g,
        Err(_) => return, // verrou empoisonné : on ignore (pas de crash)
    };
    if !reg.iter().any(|s| s.nom == "EICAR-Test-File") {
        reg.push(signature_eicar());
    }
}

/// Enregistre une source `.ndb` (une ligne). Retourne le nombre total.
/// Idempotent pour EICAR (pas de doublon). Thread-safe (tests parallèles).
pub fn enregistrer_source(ligne: &str) -> Result<usize, CleanXError> {
    let sig = parser_ligne(ligne)?;
    let mut reg = registre().lock().map_err(|_| CleanXError::Interne {
        detail: "mutex registre générique empoisonné".into(),
    })?;
    if !reg.iter().any(|s| s.nom == sig.nom) {
        reg.push(sig);
    }
    Ok(reg.len())
}

/// Première détection sur les données, ou `None`. Auto-initialise EICAR.
pub fn analyser_bytes(donnees: &[u8]) -> Option<DetectionGenerique> {
    amorcer_eicar();
    let reg = registre().lock().ok()?;
    reg.iter()
        .find(|s| rechercher(donnees, s))
        .map(|s| DetectionGenerique { nom: s.nom.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ligne_valide_et_jokers() {
        let s = parser_ligne("Test.Dark:0:*:4D5A??0000").unwrap();
        assert_eq!(s.nom, "Test.Dark");
        assert_eq!(s.offset, None);
        assert_eq!(s.motif.len(), 5);
        assert!(rechercher(b"\x00MZ\x90\x00\x00", &s));
        assert!(!rechercher(b"\x00MZ\x90\x00\x01", &s)); // dernier octet imposé
        assert!(!rechercher(b"\x00NZ\x90\x00\x00", &s)); // 2e octet imposé
    }

    #[test]
    fn parse_offset_exact() {
        let s = parser_ligne("T:0:4:41424344").unwrap();
        assert!(rechercher(b"xxxxABCD", &s));
        assert!(!rechercher(b"ABCDxxxx", &s));
    }

    #[test]
    fn parse_rejets() {
        assert!(parser_ligne("").is_err()); // vide
        assert!(parser_ligne("N:1:*:41424344").is_err()); // type inconnu
        assert!(parser_ligne("N:0:*:ABC").is_err()); // impair
        assert!(parser_ligne("N:0:*:4142").is_err()); // < 4 octets
        assert!(parser_ligne("N:0:*:4?42").is_err()); // demi-joker
        assert!(parser_ligne("N:0:*:ZZZZZZZZ").is_err()); // hex invalide
        assert!(parser_ligne(&format!("N:0:*{}", "41".repeat(300))).is_err()); // > 256
    }

    #[test]
    fn eicar_detecte_par_motif() {
        assert_eq!(
            analyser_bytes(crate::db::EICAR).map(|d| d.nom),
            Some("EICAR-Test-File".to_string())
        );
        assert!(analyser_bytes(b"contenu parfaitement sain").is_none());
    }
}
