//! Surveillance temps réel cross-platform (crate `notify`).
//!
//! - Backends natifs : inotify (Linux), FSEvents (macOS), ReadDirectoryChangesW (Windows).
//! - Fonctionnement : [`surveiller_dossiers`] bloque jusqu'à `arret == true`,
//!   envoie chaque fichier créé/modifié sur `tx`, avec anti-rebond de 2 s
//!   (navigateurs/éditeurs écrivent en plusieurs passes).
//! - Fichiers temporaires ignorés : `.part`, `.crdownload`, `.tmp`, `~…`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::CleanXError;

/// Délai anti-rebond entre deux analyses du même fichier.
const ANTI_REBOND: Duration = Duration::from_secs(2);
/// Intervalle de scrutation du drapeau d'arrêt.
const SCRUTATION_ARRET: Duration = Duration::from_millis(500);

/// Extensions temporaires ignorées (téléchargements/écritures en cours).
fn est_temporaire(chemin: &Path) -> bool {
    let ext_tempo = matches!(
        chemin.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()),
        Some(ref e) if e == "part" || e == "crdownload" || e == "tmp" || e == "temp"
    );
    let tilde = chemin
        .file_name()
        .map(|n| n.to_string_lossy().starts_with('~'))
        .unwrap_or(false);
    ext_tempo || tilde
}

/// Filtre anti-rebond : `true` si le fichier peut être analysé maintenant.
pub struct FiltreAntiRebond {
    vus: HashMap<PathBuf, Instant>,
}

impl FiltreAntiRebond {
    pub fn new() -> Self {
        Self {
            vus: HashMap::new(),
        }
    }

    pub fn autoriser(&mut self, chemin: &Path) -> bool {
        let maintenant = Instant::now();
        match self.vus.get(chemin) {
            Some(&vu) if maintenant.duration_since(vu) < ANTI_REBOND => false,
            _ => {
                self.vus.insert(chemin.to_path_buf(), maintenant);
                // Nettoyage opportuniste (évite la croissance infinie).
                if self.vus.len() > 10_000 {
                    self.vus
                        .retain(|_, &mut t| maintenant.duration_since(t) < Duration::from_secs(60));
                }
                true
            }
        }
    }
}

impl Default for FiltreAntiRebond {
    fn default() -> Self {
        Self::new()
    }
}

/// Boucle de surveillance bloquante. Retourne quand `arret` passe à `true`
/// ou si aucun dossier valide n'est fourni.
///
/// Chaque fichier créé/modifié (non temporaire, anti-rebond OK) est envoyé
/// sur `tx` pour analyse par l'appelant (scan + quarantaine éventuelle).
pub fn surveiller_dossiers(
    dossiers: &[PathBuf],
    arret: &AtomicBool,
    tx: mpsc::Sender<PathBuf>,
) -> Result<(), CleanXError> {
    let (brut_tx, brut_rx) = mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(ev) = res {
                let _ = brut_tx.send(ev);
            }
        },
        Config::default(),
    )
    .map_err(|e| CleanXError::Surveillance {
        detail: e.to_string(),
    })?;

    let mut suivis = 0;
    for d in dossiers {
        if d.is_dir() {
            watcher.watch(d, RecursiveMode::NonRecursive).map_err(|e| {
                CleanXError::Surveillance {
                    detail: format!("{} : {e}", d.display()),
                }
            })?;
            suivis += 1;
        }
    }
    if suivis == 0 {
        return Err(CleanXError::Surveillance {
            detail: "aucun dossier valide à surveiller".into(),
        });
    }

    let mut filtre = FiltreAntiRebond::new();
    while !arret.load(Ordering::Relaxed) {
        match brut_rx.recv_timeout(SCRUTATION_ARRET) {
            Ok(ev) => {
                if !matches!(ev.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                    continue;
                }
                for chemin in ev.paths {
                    if est_temporaire(&chemin) || !filtre.autoriser(&chemin) {
                        continue;
                    }
                    // `send` échoue si l'appelant a abandonné : arrêt propre.
                    if tx.send(chemin).is_err() {
                        return Ok(());
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anti_rebond_filtre_doublons() {
        let mut f = FiltreAntiRebond::new();
        let p = Path::new("/tmp/a.exe");
        assert!(f.autoriser(p));
        assert!(!f.autoriser(p)); // < 2 s : bloqué
    }

    #[test]
    fn temporaires_ignores() {
        assert!(est_temporaire(Path::new("dl/fichier.part")));
        assert!(est_temporaire(Path::new("dl/fichier.crdownload")));
        assert!(est_temporaire(Path::new("dl/~brouillon.docx")));
        assert!(!est_temporaire(Path::new("dl/archive.zip")));
    }

    #[test]
    fn aucun_dossier_erreur_claire() {
        let arret = AtomicBool::new(false);
        let (tx, _rx) = mpsc::channel();
        let r = surveiller_dossiers(&[PathBuf::from("/dossier/inexistant/xyz")], &arret, tx);
        assert!(matches!(r, Err(CleanXError::Surveillance { .. })));
    }
}
