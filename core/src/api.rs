//! Surface FFI du moteur, consommée par Flutter via flutter_rust_bridge v2.
//!
//! Conventions :
//! - `#[frb(sync)]` : requêtes rapides (statut, config) exécutées inline.
//! - Fonctions bloquantes (scan, protection) : tournent sur le pool FRB,
//!   l'UI reste fluide ; la progression part via `StreamSink`.
//! - Les fonctions de streaming retournent `Result<(), CleanXError>` et
//!   poussent tout (y compris le rapport final) dans le sink : côté Dart,
//!   elles deviennent de simples `Stream<EvenementMoteur>`.
//! - Un seul scan à la fois (`SCAN_EN_COURS`), annulable (`arreter_scan`),
//!   suspendable (`suspendre_scan`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc, Mutex, OnceLock,
};
use std::time::Duration;

use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use tokio::runtime::Runtime;

use crate::{
    heuristics, logging, quarantine, scheduler, self_defense, signatures, watcher, CleanXError,
};

// ================================================================ Types exposés

/// État global pour le dashboard.
#[derive(Debug, Clone)]
pub struct StatutGlobal {
    pub protection: bool,
    pub signatures: u32,
    pub menaces: u64,
    pub fichiers_analyses: u64,
    pub scan_en_cours: bool,
    pub dossiers: Vec<String>,
}

/// Événements poussés du Rust vers Dart.
#[derive(Debug, Clone)]
pub enum EvenementMoteur {
    /// Ligne de journal (info).
    Journal { message: String },
    /// Avancement d'un scan.
    Progression {
        scan_id: String,
        fichier: String,
        traites: u64,
        total: u64,
    },
    /// Menace détectée (avec action effectuée + empreinte pour audit).
    Menace {
        fichier: String,
        menace: String,
        action: String,
        sha256: Option<String>,
    },
    /// Fin de scan (naturelle ou annulée).
    ScanTermine {
        scan_id: String,
        total: u64,
        menaces: u64,
        annule: bool,
    },
    /// Changement d'état de la protection temps réel.
    Protection { active: bool },
}

/// Verdict unitaire d'analyse (usage interne scan + watcher).
#[derive(Debug, Clone)]
struct VerdictFichier {
    fichier: String,
    statut: &'static str, // "sain" | "suspect" | "menace" | "erreur"
    menace: Option<String>,
    action: Option<String>,
    sha256: Option<String>,
    score: u8,
}

// ================================================================ État global

/// Configuration résolue à `initialiser` (clonée hors mutex avant tout `block_on`).
#[derive(Clone)]
struct EtatMoteur {
    db: PathBuf,
    quarantaine_dir: PathBuf,
    cle: [u8; 32],
}

static ETAT: OnceLock<Mutex<EtatMoteur>> = OnceLock::new();
/// `Result` stocké car `OnceLock::get_or_try_init` est instable sur stable :
/// l'erreur de construction est conservée et rejouée à chaque appel.
static RUNTIME: OnceLock<Result<Runtime, String>> = OnceLock::new();
static SCAN_EN_COURS: AtomicBool = AtomicBool::new(false);
static SCAN_ANNULE: AtomicBool = AtomicBool::new(false);
static SCAN_PAUSE: AtomicBool = AtomicBool::new(false);
/// Mode jeu : quand actif, pas de quarantaine automatique (surveillance seule).
static MODE_JEU: AtomicBool = AtomicBool::new(false);
static PROTECTION_ACTIVE: AtomicBool = AtomicBool::new(false);
static PROTECTION_STOP: AtomicBool = AtomicBool::new(false);
static COMPTEUR_MENACES: AtomicU64 = AtomicU64::new(0);
static COMPTEUR_FICHIERS: AtomicU64 = AtomicU64::new(0);

/// Runtime tokio partagé (8 workers). Erreur propagée (jamais de panic) :
/// un échec ici signifie un système à genoux (plus de threads/RLIMIT).
fn runtime() -> Result<&'static Runtime, CleanXError> {
    RUNTIME
        .get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(8)
                .thread_name("cleanx-scan")
                .enable_all()
                .build()
                .map_err(|e| format!("runtime tokio : {e}"))
        })
        .as_ref()
        .map_err(|e| CleanXError::Interne { detail: e.clone() })
}

fn etat() -> Result<EtatMoteur, CleanXError> {
    ETAT.get()
        .ok_or(CleanXError::MoteurNonInitialise)?
        .lock()
        .map(|e| e.clone())
        .map_err(|_| CleanXError::Interne {
            detail: "mutex d'état empoisonné".into(),
        })
}

/// Dossier personnel (HOME Unix / USERPROFILE Windows / temp en repli).
fn dossier_personnel() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

/// Racines du scan rapide : Téléchargements + Bureau (existants uniquement).
fn racines_rapides() -> Vec<PathBuf> {
    let home = dossier_personnel();
    [
        home.join("Downloads"),
        home.join("Desktop"),
        home.join("Téléchargements"),
        home.join("Bureau"),
    ]
    .into_iter()
    .filter(|p| p.is_dir())
    .collect()
}

/// Racines du scan complet : dossier personnel entier (repli : scan rapide).
fn racines_completes() -> Vec<PathBuf> {
    let home = dossier_personnel();
    if home.is_dir() {
        vec![home]
    } else {
        racines_rapides()
    }
}

// ================================================================ Cycle de vie

/// Initialise le moteur : dossiers, base SQLite + seed, clé, logs, dossiers suivis.
///
/// À appeler une fois au démarrage de l'app (chemin base fourni par Dart via
/// path_provider). Idempotent : ré-appelable sans effet destructeur.
#[frb(sync)]
pub fn initialiser(base: String) -> Result<StatutGlobal, CleanXError> {
    let base = PathBuf::from(base);
    std::fs::create_dir_all(&base).map_err(|e| crate::error::erreur_io(&base, e))?;
    runtime()?; // pré-chauffe le pool de workers (erreur fatale propagée)
    let db = base.join("cleanx.db");
    crate::db::ouvrir(&db)?; // crée + migre + seed
    let quarantaine_dir = base.join("quarantaine");
    std::fs::create_dir_all(&quarantaine_dir)
        .map_err(|e| crate::error::erreur_io(&quarantaine_dir, e))?;
    // Clé STRICTEMENT depuis le coffre ; repli fichier uniquement si
    // explicitement autorisé (dev/CI : CLEANX_KEY_FALLBACK=1), sinon refus.
    let (cle, provenance) = match quarantine::charger_ou_creer_cle_trace(&base.join("cle.key")) {
        Ok(ok) => ok,
        Err(CleanXError::CoffreIndisponible { .. }) if quarantine::repli_fichier_autorise() => {
            quarantine::charger_ou_creer_cle_avec_repli(&base.join("cle.key"))?
        }
        Err(e) => return Err(e),
    };
    logging::initialiser(&base.join("logs"), "info")?;

    // Dossiers suivis : config persistée, sinon défauts.
    let dossiers = charger_dossiers(&db)?;
    let dossiers = if dossiers.is_empty() {
        // Téléchargements + Bureau s'ils existent (CI : absents → repli
        // sur le dossier personnel lui-même, jamais une liste vide qui
        // désactiverait silencieusement la protection).
        let mut defaut: Vec<String> = racines_rapides()
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        let home = dossier_personnel();
        if defaut.is_empty() && home.is_dir() {
            defaut.push(home.to_string_lossy().into_owned());
        }
        sauver_dossiers(&db, &defaut)?;
        defaut
    } else {
        dossiers
    };

    // `get_or_init` prend un FnOnce : on clone avant, aucun emprunt résiduel.
    let neuf = EtatMoteur {
        db: db.clone(),
        quarantaine_dir,
        cle,
    };
    let amorce = neuf.clone();
    *ETAT
        .get_or_init(|| Mutex::new(amorce))
        .lock()
        .map_err(|_| CleanXError::Interne {
            detail: "mutex d'état empoisonné".into(),
        })? = neuf;

    let _ = logging::journaliser(
        &db,
        "info",
        &format!("Moteur CleanX initialisé (clé quarantaine : {provenance:?})"),
    );
    let _ = dossiers; // (lu via statut())
    statut()
}

/// État global instantané (dashboard).
#[frb(sync)]
pub fn statut() -> Result<StatutGlobal, CleanXError> {
    let e = etat()?;
    let signatures = crate::db::avec_connexion(&e.db, |conn| crate::db::compter_signatures(conn))?;
    Ok(StatutGlobal {
        protection: PROTECTION_ACTIVE.load(Ordering::Relaxed),
        signatures,
        menaces: COMPTEUR_MENACES.load(Ordering::Relaxed),
        fichiers_analyses: COMPTEUR_FICHIERS.load(Ordering::Relaxed),
        scan_en_cours: SCAN_EN_COURS.load(Ordering::Relaxed),
        dossiers: charger_dossiers(&e.db)?,
    })
}

// ================================================================ Pipeline d'analyse

/// Analyse complète d'un fichier : signatures → heuristique → quarantaine auto.
async fn analyser_fichier(
    db: &Path,
    quarantaine_dir: &Path,
    cle: &[u8; 32],
    chemin: &Path,
    auto_quarantaine: bool,
) -> VerdictFichier {
    COMPTEUR_FICHIERS.fetch_add(1, Ordering::Relaxed);
    let chemin_txt = chemin.to_string_lossy().into_owned();

    let sig = match signatures::verifier_signature(db, chemin).await {
        Ok(v) => v,
        Err(e) => {
            return VerdictFichier {
                fichier: chemin_txt,
                statut: "erreur",
                menace: None,
                action: None,
                sha256: None,
                score: 0,
            }
            .avec_detail(e.to_string())
        }
    };
    if let Some(err) = sig.erreur {
        return VerdictFichier {
            fichier: chemin_txt,
            statut: "erreur",
            menace: None,
            action: None,
            sha256: sig.sha256,
            score: 0,
        }
        .avec_detail(err);
    }

    let chemin2 = chemin.to_path_buf();
    let heur = tokio::task::spawn_blocking(move || {
        heuristics::analyser(&chemin2, heuristics::Seuils::default())
    })
    .await
    .unwrap_or(heuristics::AnalyseHeuristique {
        score: 0,
        signaux: vec!["analyse heuristique interrompue".into()],
        verdict: heuristics::VerdictHeuristique::Sain,
    });

    let mut menace = sig.menace.clone();
    if heur.verdict == heuristics::VerdictHeuristique::Menace {
        menace = Some(format!(
            "Heuristique[{}] : {}",
            heur.score,
            heur.signaux
                .iter()
                .take(3)
                .cloned()
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }

    if let Some(m) = menace {
        COMPTEUR_MENACES.fetch_add(1, Ordering::Relaxed);
        let action = if auto_quarantaine {
            let (db2, dir2, cle2, ch2, m2) = (
                db.to_path_buf(),
                quarantaine_dir.to_path_buf(),
                *cle,
                chemin.to_path_buf(),
                m.clone(),
            );
            match tokio::task::spawn_blocking(move || {
                quarantine::mettre_en_quarantaine(&db2, &dir2, &cle2, &ch2, &m2, heur.score)
            })
            .await
            {
                Ok(Ok(id)) => format!("mis en quarantaine (#{id})"),
                Ok(Err(e)) => format!("QUARANTAINE ÉCHOUÉE : {e}"),
                Err(e) => format!("worker quarantaine : {e}"),
            }
        } else {
            "signalé (quarantaine auto désactivée)".to_string()
        };
        VerdictFichier {
            fichier: chemin_txt,
            statut: "menace",
            menace: Some(m),
            action: Some(action),
            sha256: sig.sha256,
            score: heur.score,
        }
    } else if heur.verdict == heuristics::VerdictHeuristique::Suspect {
        VerdictFichier {
            fichier: chemin_txt,
            statut: "suspect",
            menace: None,
            action: Some("surveillé".into()),
            sha256: sig.sha256,
            score: heur.score,
        }
    } else {
        VerdictFichier {
            fichier: chemin_txt,
            statut: "sain",
            menace: None,
            action: None,
            sha256: sig.sha256,
            score: heur.score,
        }
    }
}

impl VerdictFichier {
    fn avec_detail(mut self, detail: String) -> Self {
        self.action = Some(detail);
        self
    }
}

/// Liste les fichiers à scanner (marche en profondeur, sans suivre les liens
/// symboliques, tolérante aux permissions, chemins Unicode intacts).
fn lister_fichiers_sync(racines: &[PathBuf], limite: usize) -> Vec<PathBuf> {
    let mut fichiers = Vec::new();
    let mut pile: Vec<PathBuf> = racines.to_vec();
    while let Some(dir) = pile.pop() {
        if fichiers.len() >= limite {
            break;
        }
        let entrees = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue, // permission refusée : on ignore la branche
        };
        for entree in entrees.flatten() {
            let chemin = entree.path();
            if fichiers.len() >= limite {
                break;
            }
            // Pas de suivi des symlinks (boucles + sorties de périmètre).
            match entree.file_type() {
                Ok(t) if t.is_dir() && !t.is_symlink() => pile.push(chemin),
                Ok(t) if t.is_file() => fichiers.push(chemin),
                _ => continue,
            }
        }
    }
    fichiers
}

/// Exécute un scan avec progression temps réel. Vérifie annulation + pause.
async fn executer_scan(racines: Vec<PathBuf>, scan_id: String, sink: &StreamSink<EvenementMoteur>) {
    let e = match etat() {
        Ok(e) => e,
        Err(err) => {
            let _ = sink.add(EvenementMoteur::Journal {
                message: format!("Scan impossible : {err}"),
            });
            return;
        }
    };
    let fichiers = tokio::task::spawn_blocking(move || lister_fichiers_sync(&racines, 50_000))
        .await
        .unwrap_or_default();
    let total = fichiers.len() as u64;
    let _ = sink.add(EvenementMoteur::Journal {
        message: format!("Scan {scan_id} : {total} fichiers"),
    });
    let mut menaces = 0u64;

    for (i, chemin) in fichiers.iter().enumerate() {
        if SCAN_ANNULE.load(Ordering::Relaxed) {
            let _ = sink.add(EvenementMoteur::ScanTermine {
                scan_id: scan_id.clone(),
                total,
                menaces,
                annule: true,
            });
            let _ = logging::journaliser(
                &e.db,
                "info",
                &format!("Scan {scan_id} arrêté ({i}/{total})"),
            );
            return;
        }
        // Pause coopérative (bouton pause de l'UI, ou mode jeu).
        while SCAN_PAUSE.load(Ordering::Relaxed) && !SCAN_ANNULE.load(Ordering::Relaxed) {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        // Mode jeu : détection sans quarantaine auto (zéro E/S disque).
        let auto = !MODE_JEU.load(Ordering::Relaxed);
        let v = analyser_fichier(&e.db, &e.quarantaine_dir, &e.cle, chemin, auto).await;
        match v.statut {
            "menace" => {
                menaces += 1;
                let _ = sink.add(EvenementMoteur::Menace {
                    fichier: v.fichier.clone(),
                    menace: v.menace.clone().unwrap_or_default(),
                    action: v.action.clone().unwrap_or_default(),
                    sha256: v.sha256.clone(),
                });
                let _ = logging::journaliser(
                    &e.db,
                    "quarantaine",
                    &format!("{} — {}", v.fichier, v.action.unwrap_or_default()),
                );
            }
            "suspect" => {
                let _ = sink.add(EvenementMoteur::Journal {
                    message: format!("Suspect ({}) : {}", v.score, v.fichier),
                });
            }
            "erreur" => {
                let _ = sink.add(EvenementMoteur::Journal {
                    message: format!("{} : {}", v.fichier, v.action.unwrap_or_default()),
                });
            }
            _ => {}
        }
        let traites = (i + 1) as u64;
        if traites % 5 == 0 || traites == total {
            let _ = sink.add(EvenementMoteur::Progression {
                scan_id: scan_id.clone(),
                fichier: v.fichier,
                traites,
                total,
            });
        }
    }
    let _ = sink.add(EvenementMoteur::ScanTermine {
        scan_id: scan_id.clone(),
        total,
        menaces,
        annule: false,
    });
    let _ = logging::journaliser(
        &e.db,
        "info",
        &format!("Scan {scan_id} terminé : {total} fichiers, {menaces} menace(s)"),
    );
}

/// Démarre un scan (un seul à la fois). Le `sink` reçoit progression + rapport.
fn demarrer_scan(
    racines: Vec<PathBuf>,
    prefixe: &str,
    sink: StreamSink<EvenementMoteur>,
) -> Result<(), CleanXError> {
    etat()?; // moteur initialisé ?
    if SCAN_EN_COURS.swap(true, Ordering::SeqCst) {
        return Err(CleanXError::ScanDejaEnCours);
    }
    SCAN_ANNULE.store(false, Ordering::Relaxed);
    // Un scan démarré en mode jeu reste suspendu d'emblée.
    SCAN_PAUSE.store(MODE_JEU.load(Ordering::Relaxed), Ordering::Relaxed);
    let scan_id = format!("{prefixe}-{}", &uuid::Uuid::new_v4().to_string()[..8]);
    runtime()?.block_on(executer_scan(racines, scan_id, &sink));
    SCAN_EN_COURS.store(false, Ordering::SeqCst);
    Ok(())
}

/// Scan rapide : Téléchargements + Bureau.
pub fn scan_rapide(sink: StreamSink<EvenementMoteur>) -> Result<(), CleanXError> {
    demarrer_scan(racines_rapides(), "rapide", sink)
}

/// Scan complet : dossier personnel.
pub fn scan_complet(sink: StreamSink<EvenementMoteur>) -> Result<(), CleanXError> {
    demarrer_scan(racines_completes(), "complet", sink)
}

/// Scan personnalisé sur `racines` (chemins existants, sinon erreur claire).
pub fn scan_personnalise(
    racines: Vec<String>,
    sink: StreamSink<EvenementMoteur>,
) -> Result<(), CleanXError> {
    let mut valides = Vec::new();
    for r in racines {
        let p = PathBuf::from(&r);
        if p.is_dir() {
            valides.push(p);
        } else {
            return Err(CleanXError::CheminInvalide { chemin: r });
        }
    }
    demarrer_scan(valides, "perso", sink)
}

/// Demande l'arrêt du scan en cours. `true` si un scan tournait.
#[frb(sync)]
pub fn arreter_scan() -> Result<bool, CleanXError> {
    if SCAN_EN_COURS.load(Ordering::Relaxed) {
        SCAN_ANNULE.store(true, Ordering::Relaxed);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Suspend (`true`) ou reprend (`false`) le scan en cours.
#[frb(sync)]
pub fn suspendre_scan(suspendre: bool) -> Result<(), CleanXError> {
    SCAN_PAUSE.store(suspendre, Ordering::Relaxed);
    Ok(())
}

/// Mode jeu : scans suspendus + temps réel en surveillance seule.
///
/// Quand actif : les scans en cours se mettent en pause (comme
/// `suspendre_scan(true)`) et la protection temps réel DÉTECTE sans mettre
/// en quarantaine automatiquement (aucune E/S disque intempestive pendant
/// une partie). Retourne l'état précédent. V1 manuelle (bascule UI) ;
/// détection automatique de plein écran ticketée (spécifique par OS).
#[frb(sync)]
pub fn mode_jeu(actif: bool) -> Result<bool, CleanXError> {
    let precedent = MODE_JEU.swap(actif, Ordering::SeqCst);
    SCAN_PAUSE.store(actif, Ordering::Relaxed);
    Ok(precedent)
}

/// État du mode jeu.
#[frb(sync)]
pub fn mode_jeu_actif() -> Result<bool, CleanXError> {
    Ok(MODE_JEU.load(Ordering::Relaxed))
}

// ================================================================ Protection temps réel

/// Active la surveillance : bloque jusqu'à `desactiver_protection`.
///
/// Chaque nouveau fichier est analysé (1 s de grâce pour fin d'écriture) puis
/// mis en quarantaine auto si menace. Les alertes partent dans `sink`.
pub fn activer_protection(sink: StreamSink<EvenementMoteur>) -> Result<(), CleanXError> {
    let e = etat()?;
    if PROTECTION_ACTIVE.swap(true, Ordering::SeqCst) {
        return Ok(()); // déjà active : idempotent
    }
    PROTECTION_STOP.store(false, Ordering::Relaxed);
    let _ = sink.add(EvenementMoteur::Protection { active: true });
    let _ = logging::journaliser(&e.db, "info", "Protection temps réel ACTIVÉE");

    let (tx, rx) = mpsc::channel();
    let dossiers = charger_dossiers(&e.db)?
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    // Le watcher vit dans son thread ; cette fonction consomme les alertes.
    let guetteur =
        std::thread::spawn(move || watcher::surveiller_dossiers(&dossiers, &PROTECTION_STOP, tx));
    let mut rebond: HashMap<PathBuf, std::time::Instant> = HashMap::new();

    while !PROTECTION_STOP.load(Ordering::Relaxed) {
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(chemin) => {
                // Grâce d'écriture (navigateurs) + anti-rebond local.
                let maintenant = std::time::Instant::now();
                if rebond
                    .get(&chemin)
                    .map(|t| maintenant.duration_since(*t) < Duration::from_secs(2))
                    .unwrap_or(false)
                {
                    continue;
                }
                rebond.insert(chemin.clone(), maintenant);
                // Grâce d'écriture courte (300 ms) : les petits fichiers sont
                // complets en quelques ms ; les téléchargements en cours
                // émettent des Modify qui re-déclenchent l'analyse finale
                // (anti-rebond watcher : 2 s). Vise un verdict < 1 s (R5).
                std::thread::sleep(Duration::from_millis(300));
                if !chemin.is_file() {
                    continue;
                }
                let _ = sink.add(EvenementMoteur::Journal {
                    message: format!("Nouveau fichier : {}", chemin.display()),
                });
                // Mode jeu : détection sans quarantaine auto (lu à chaque
                // fichier : bascule dynamique pendant la surveillance).
                let auto = !MODE_JEU.load(Ordering::Relaxed);
                let v = runtime()?.block_on(analyser_fichier(
                    &e.db,
                    &e.quarantaine_dir,
                    &e.cle,
                    &chemin,
                    auto,
                ));
                if v.statut == "menace" {
                    let _ = sink.add(EvenementMoteur::Menace {
                        fichier: v.fichier.clone(),
                        menace: v.menace.clone().unwrap_or_default(),
                        action: v.action.clone().unwrap_or_default(),
                        sha256: v.sha256.clone(),
                    });
                    let _ = logging::journaliser(
                        &e.db,
                        "quarantaine",
                        &format!("temps réel : {}", v.fichier),
                    );
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    let _ = guetteur.join();
    PROTECTION_ACTIVE.store(false, Ordering::SeqCst);
    let _ = sink.add(EvenementMoteur::Protection { active: false });
    let _ = logging::journaliser(&e.db, "info", "Protection temps réel DÉSACTIVÉE");
    Ok(())
}

/// Désactive la protection. `true` si elle était active.
#[frb(sync)]
pub fn desactiver_protection() -> Result<bool, CleanXError> {
    if PROTECTION_ACTIVE.load(Ordering::Relaxed) {
        PROTECTION_STOP.store(true, Ordering::Relaxed);
        Ok(true)
    } else {
        Ok(false)
    }
}

// ================================================================ Dossiers surveillés (persistés)

fn charger_dossiers(db: &Path) -> Result<Vec<String>, CleanXError> {
    let txt: Option<String> = crate::db::avec_connexion(db, |conn| {
        Ok(conn
            .query_row(
                "SELECT valeur FROM config WHERE cle = 'dossiers'",
                [],
                |r| r.get(0),
            )
            .ok())
    })?;
    Ok(txt
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default())
}

fn sauver_dossiers(db: &Path, dossiers: &[String]) -> Result<(), CleanXError> {
    crate::db::avec_connexion(db, |conn| {
        conn.execute(
            "INSERT INTO config(cle, valeur) VALUES ('dossiers', ?1)
             ON CONFLICT(cle) DO UPDATE SET valeur = excluded.valeur",
            [serde_json::to_string(dossiers)?],
        )?;
        Ok(())
    })
}

/// Dossiers surveillés.
#[frb(sync)]
pub fn dossiers_surveilles() -> Result<Vec<String>, CleanXError> {
    charger_dossiers(&etat()?.db)
}

/// Ajoute un dossier (doit exister, sinon `CheminInvalide`).
#[frb(sync)]
pub fn ajouter_dossier(dossier: String) -> Result<Vec<String>, CleanXError> {
    let e = etat()?;
    if !Path::new(&dossier).is_dir() {
        return Err(CleanXError::CheminInvalide { chemin: dossier });
    }
    let mut liste = charger_dossiers(&e.db)?;
    if !liste.contains(&dossier) {
        liste.push(dossier);
        sauver_dossiers(&e.db, &liste)?;
    }
    Ok(liste)
}

/// Retire un dossier de la surveillance.
#[frb(sync)]
pub fn retirer_dossier(dossier: String) -> Result<Vec<String>, CleanXError> {
    let e = etat()?;
    let mut liste = charger_dossiers(&e.db)?;
    liste.retain(|d| d != &dossier);
    sauver_dossiers(&e.db, &liste)?;
    Ok(liste)
}

// ================================================================ Quarantaine / logs / planification

/// Met en quarantaine (score heuristique calculé automatiquement).
#[frb(sync)]
pub fn mettre_en_quarantaine(chemin: String, raison: String) -> Result<i64, CleanXError> {
    let e = etat()?;
    let p = PathBuf::from(&chemin);
    let score = heuristics::analyser(&p, heuristics::Seuils::default()).score;
    let id =
        quarantine::mettre_en_quarantaine(&e.db, &e.quarantaine_dir, &e.cle, &p, &raison, score)?;
    let _ = logging::journaliser(
        &e.db,
        "quarantaine",
        &format!("{chemin} isolé (#{id}) : {raison}"),
    );
    Ok(id)
}

/// Liste la quarantaine.
#[frb(sync)]
pub fn lister_quarantaine() -> Result<Vec<quarantine::FichierQuarantaine>, CleanXError> {
    quarantine::lister(&etat()?.db)
}

/// Restaure un fichier isolé (faux positif).
#[frb(sync)]
pub fn restaurer_quarantaine(id: i64, destination: String) -> Result<(), CleanXError> {
    let e = etat()?;
    quarantine::restaurer(&e.db, &e.cle, id, Path::new(&destination))?;
    let _ = logging::journaliser(
        &e.db,
        "info",
        &format!("quarantaine #{id} restaurée vers {destination}"),
    );
    Ok(())
}

/// Supprime définitivement une entrée.
#[frb(sync)]
pub fn supprimer_quarantaine(id: i64) -> Result<(), CleanXError> {
    let e = etat()?;
    quarantine::supprimer_definitivement(&e.db, &e.quarantaine_dir, id)?;
    let _ = logging::journaliser(&e.db, "info", &format!("quarantaine #{id} supprimée"));
    Ok(())
}

/// Inventaire des processus + signaux de dissimulation (base anti-rootkit
/// userland v1 — voir `rootkit.rs` pour les limites assumées).
#[frb(sync)]
pub fn analyser_processus_api() -> Result<Vec<crate::rootkit::ProcessusAnalyse>, CleanXError> {
    Ok(crate::rootkit::analyser_processus())
}

/// Limites documentées de la détection (affichables dans l'UI).
#[frb(sync)]
pub fn limites_rootkit_api() -> Result<Vec<String>, CleanXError> {
    Ok(crate::rootkit::limites())
}

/// Libère les ressources natives : vide le pool SQLite (ferme les fichiers).
///
/// À appeler avant suppression du dossier de base ou à l'arrêt de l'app.
/// Sans appel, les fichiers `.db` restent verrouillés le temps du processus
/// (comportement pool standard, documenté ici et dans `db::vider_pool`).
#[frb(sync)]
pub fn liberer_ressources() -> Result<(), CleanXError> {
    crate::db::vider_pool();
    Ok(())
}

/// Derniers événements (UI filtrable).
#[frb(sync)]
pub fn lister_logs(limite: u32) -> Result<Vec<crate::logging::EntreeLog>, CleanXError> {
    crate::logging::lister(&etat()?.db, limite)
}

/// Exporte les logs au format CSV (contenu texte, l'UI choisit la destination).
#[frb(sync)]
pub fn exporter_logs_csv() -> Result<String, CleanXError> {
    let logs = lister_logs(5000)?;
    let mut csv = String::from("id,date,niveau,message\n");
    for l in logs {
        let msg = l.message.replace('"', "\"\"");
        csv.push_str(&format!(
            "{},\"{}\",\"{}\",\"{}\"\n",
            l.id, l.date, l.niveau, msg
        ));
    }
    Ok(csv)
}

/// Crée une planification (retourne son id).
#[frb(sync)]
pub fn ajouter_planification(
    nom: String,
    racines: Vec<String>,
    intervalle_secs: u64,
) -> Result<i64, CleanXError> {
    scheduler::ajouter(&etat()?.db, &nom, &racines, intervalle_secs)
}

/// Liste les planifications.
#[frb(sync)]
pub fn lister_planifications() -> Result<Vec<scheduler::Planification>, CleanXError> {
    scheduler::lister(&etat()?.db)
}

/// Supprime une planification.
#[frb(sync)]
pub fn supprimer_planification(id: i64) -> Result<(), CleanXError> {
    scheduler::supprimer(&etat()?.db, id)
}

// ================================================================ Utilitaires exposés

/// SHA-256 d'un fichier (pur, sans initialisation requise).
#[frb(sync)]
pub fn calculer_sha256(chemin: String) -> Result<String, CleanXError> {
    runtime()?.block_on(signatures::sha256_fichier(Path::new(&chemin)))
}

/// Analyse heuristique d'un fichier (pur, sans initialisation requise).
#[frb(sync)]
pub fn analyser_heuristique_api(
    chemin: String,
) -> Result<heuristics::AnalyseHeuristique, CleanXError> {
    Ok(heuristics::analyser(
        Path::new(&chemin),
        heuristics::Seuils::default(),
    ))
}

/// Empreinte SHA-256 du binaire (auto-protection).
#[frb(sync)]
pub fn integrite_binaire() -> Result<String, CleanXError> {
    self_defense::empreinte_binaire()
}

/// Vérifie l'intégrité contre une empreinte de référence (`None` = pas de référence).
#[frb(sync)]
pub fn verifier_integrite_api(attendu: Option<String>) -> Result<bool, CleanXError> {
    self_defense::verifier_integrite(attendu.as_deref())
}

/// Dérive une clé hex via Argon2 (testable sans initialisation).
#[frb(sync)]
pub fn deriver_cle_api(phrase: String, sel_b64: String) -> Result<String, CleanXError> {
    quarantine::deriver_cle(&phrase, &sel_b64).map(hex::encode)
}

/// Vérifie un paquet de signatures Ed25519 (pur).
#[frb(sync)]
pub fn verifier_paquet_api(
    paquet: Vec<u8>,
    signature_hex: String,
    cle_publique_hex: String,
) -> Result<bool, CleanXError> {
    signatures::verifier_paquet(&paquet, &signature_hex, &cle_publique_hex)
}

/// Met à jour les signatures depuis un dépôt HTTPS signé Ed25519.
///
/// Import transactionnel : tout ou rien (rollback si signature invalide ou
/// entrée malformée). Retourne le nombre de signatures AJOUTÉES.
pub async fn mettre_a_jour_signatures(
    url_depot: String,
    cle_publique_hex: String,
) -> Result<u32, CleanXError> {
    let e = etat()?;
    let n = signatures::mettre_a_jour_depuis_depot(&e.db, &url_depot, &cle_publique_hex).await?;
    let _ = logging::journaliser(&e.db, "info", &format!("Signatures : +{n} depuis le dépôt"));
    Ok(n)
}
