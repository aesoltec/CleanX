//! CleanX Core — moteur antivirus multiplateforme.
//!
//! Architecture : chaque module est indépendant et testable (`cargo test`).
//! Le module [`api`] expose la surface FFI consommée par Flutter via
//! flutter_rust_bridge (voir `flutter_rust_bridge.yaml` à la racine).
//!
//! Règles : aucun `unsafe`, erreurs typées ([`CleanXError`]), aucun panic
//! sur les chemins d'analyse (fichiers verrouillés, permissions, Unicode).
//!
//! `unexpected_cfgs` : la macro `#[frb]` émet du code sous `cfg(frb_expand)`
//! (utilisé par le codegen) ; l'allow est volontaire et documenté ici.

// La macro FRB génère des références `cfg(frb_expand)` : cfg attendu, pas une faute.
#![allow(unexpected_cfgs)]

pub mod api;
pub mod db;
pub mod error;
/// Bindings générés par flutter_rust_bridge (`frb_generated.rs`).
///
/// En attendant la génération (`flutter_rust_bridge_codegen generate`), ce
/// module fournit un `StreamSink` factice pour `cargo test`. Le codegen
/// remplacera ce fichier par les vrais bindings (conservez `pub mod`).
pub mod frb_generated;
pub mod heuristics;
pub mod logging;
pub mod mode;
pub mod quarantine;
pub mod rootkit;
pub mod scheduler;
pub mod self_defense;
pub mod signatures;
pub mod watcher;

pub use error::CleanXError;
