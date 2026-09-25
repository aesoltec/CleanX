//! Erreurs typées du moteur, sérialisables vers Dart.
//!
//! Contrainte FFI : chaque champ est un type simple (String/entiers), car
//! flutter_rust_bridge doit pouvoir convertir chaque variante vers Dart.
//! Aucun type std opaque (`io::Error`, `rusqlite::Error`) ne fuite ici :
//! les conversions `From` normalisent tout en messages français.

use std::io;
use std::path::Path;

use thiserror::Error;

/// Erreur métier du moteur CleanX.
#[derive(Debug, Clone, Error)]
pub enum CleanXError {
    /// Moteur utilisé avant `initialiser`.
    #[error("Moteur non initialisé : appelez initialiser() au démarrage de l'application")]
    MoteurNonInitialise,

    /// Fichier introuvable.
    #[error("Fichier introuvable : {chemin}")]
    FichierIntrouvable { chemin: String },

    /// Accès refusé (droits insuffisants, sandbox mobile, TCC macOS…).
    #[error("Permission refusée pour « {chemin} » : {detail}")]
    PermissionRefusee { chemin: String, detail: String },

    /// Fichier verrouillé par un autre processus (WouldBlock, partage refusé).
    #[error("Fichier verrouillé (en cours d'écriture ?) : {chemin}")]
    FichierVerrouille { chemin: String },

    /// Chemin invalide (non-UTF8, racine inexistante…).
    #[error("Chemin invalide : {chemin}")]
    CheminInvalide { chemin: String },

    /// Base SQLite illisible ou corrompue.
    #[error("Base locale corrompue : {detail}")]
    BaseCorrompue { detail: String },

    /// Échec d'une opération de quarantaine.
    #[error("Quarantaine impossible : {detail}")]
    Quarantaine { detail: String },

    /// Coffre OS inaccessible et aucun repli autorisé (refus explicite).
    #[error("Coffre de clés indisponible : {detail}")]
    CoffreIndisponible { detail: String },

    /// Échec de la surveillance temps réel.
    #[error("Surveillance impossible : {detail}")]
    Surveillance { detail: String },

    /// Scan interrompu par l'utilisateur (pas une erreur : signal de contrôle).
    #[error("Scan annulé par l'utilisateur")]
    ScanAnnule,

    /// Scan déjà en cours.
    #[error("Un scan est déjà en cours")]
    ScanDejaEnCours,

    /// Fonctionnalité non supportée dans cette compilation.
    #[error("Non supporté : {detail}")]
    NonSupporte { detail: String },

    /// Erreur interne inattendue (bug : à remonter avec les logs).
    #[error("Erreur interne : {detail}")]
    Interne { detail: String },
}

impl From<rusqlite::Error> for CleanXError {
    fn from(e: rusqlite::Error) -> Self {
        CleanXError::BaseCorrompue {
            detail: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for CleanXError {
    fn from(e: serde_json::Error) -> Self {
        CleanXError::Interne {
            detail: format!("JSON : {e}"),
        }
    }
}

/// Normalise une erreur d'entrée/sortie en erreur métier.
///
/// Distingue : introuvable / permission / verrouillage / autres, afin que
/// l'UI affiche un message actionnable plutôt qu'un code brut.
pub fn erreur_io(chemin: &Path, e: io::Error) -> CleanXError {
    let chemin = chemin.to_string_lossy().into_owned();
    match e.kind() {
        io::ErrorKind::NotFound => CleanXError::FichierIntrouvable { chemin },
        io::ErrorKind::PermissionDenied => CleanXError::PermissionRefusee {
            chemin,
            detail: "exécutez avec les droits requis ou accordez l'accès au dossier".to_string(),
        },
        io::ErrorKind::WouldBlock => CleanXError::FichierVerrouille { chemin },
        // Partage refusé sous Windows (ERROR_SHARING_VIOLATION = 32).
        _ if e.raw_os_error() == Some(32) => CleanXError::FichierVerrouille { chemin },
        _ => CleanXError::Interne {
            detail: format!("E/S sur « {chemin} » : {e}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalise_fichier_introuvable() {
        let e = erreur_io(
            Path::new("/inexistant/f.txt"),
            io::Error::from(io::ErrorKind::NotFound),
        );
        assert!(matches!(e, CleanXError::FichierIntrouvable { .. }));
    }

    #[test]
    fn normalise_permission_refusee() {
        let e = erreur_io(
            Path::new("x"),
            io::Error::from(io::ErrorKind::PermissionDenied),
        );
        assert!(matches!(e, CleanXError::PermissionRefusee { .. }));
    }

    #[test]
    fn normalise_verrouillage() {
        let e = erreur_io(Path::new("x"), io::Error::from(io::ErrorKind::WouldBlock));
        assert!(matches!(e, CleanXError::FichierVerrouille { .. }));
        // Violation de partage Windows (os error 32).
        let e = erreur_io(Path::new("x"), io::Error::from_raw_os_error(32));
        assert!(matches!(e, CleanXError::FichierVerrouille { .. }));
    }

    #[test]
    fn normalise_erreur_generique() {
        let e = erreur_io(Path::new("x"), io::Error::other("panne disque"));
        assert!(matches!(e, CleanXError::Interne { .. }));
    }
}
