//! Détection rootkit BASIQUE (userland, v1 honnête).
//!
//! Périmètre assumé : sans pilote noyau, on ne voit que ce que l'OS veut bien
//! montrer. Ce module fournit donc un _socle_ : inventaire des processus via
//! `sysinfo` + heuristiques de dissimulation simples (nom vide, exécutable
//! manquant/inaccessible, mémoire incohérente). La détection de hooks
//! (SSDT, inline-hooking, DKOM) exige un driver signé par plateforme —
//! ticketée (voir `limites()`), pas prétendue ici.

use sysinfo::{ProcessesToUpdate, System};

/// Un processus avec ses signaux de dissimulation.
#[derive(Debug, Clone)]
pub struct ProcessusAnalyse {
    /// PID.
    pub pid: u32,
    /// Nom tel que rapporté (peut être vide = suspect).
    pub nom: String,
    /// Exécutable résolu, `None` si inaccessible (suspect hors processus système).
    pub exe: Option<String>,
    /// Mémoire résidente (octets).
    pub memoire: u64,
    /// Raisons de suspicion (vide = rien à signaler).
    pub signaux: Vec<String>,
}

/// PIDs considérés système (0/1/4 : idle, init, System NT — exe absent normal).
fn pid_systeme(pid: u32) -> bool {
    pid <= 4
}

/// Invente les processus et applique les heuristiques de dissimulation.
///
/// Ne panique jamais ; une erreur d'accès snapshot vaut liste vide + signal
/// global via le premier élément ? Non : retourne simplement ce qui est
/// lisible (principe : visibilité partielle > échec total).
pub fn analyser_processus() -> Vec<ProcessusAnalyse> {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    let mut resultat = Vec::new();
    for (pid, proc_) in sys.processes() {
        let pid_n = pid.as_u32();
        let nom = proc_.name().to_string_lossy().into_owned();
        let exe = proc_.exe().map(|p| p.to_string_lossy().into_owned());
        let mut signaux = Vec::new();
        if nom.trim().is_empty() && !pid_systeme(pid_n) {
            signaux.push("nom de processus vide (dissimulation ?)".to_string());
        }
        if exe.is_none() && !pid_systeme(pid_n) {
            signaux.push("exécutable inaccessible (droits ? injection ?)".to_string());
        }
        if proc_.memory() == 0 && !nom.trim().is_empty() && !pid_systeme(pid_n) {
            signaux.push("mémoire nulle avec nom présent (anomalie)".to_string());
        }
        resultat.push(ProcessusAnalyse {
            pid: pid_n,
            nom,
            exe,
            memoire: proc_.memory(),
            signaux,
        });
    }
    resultat.sort_by_key(|p| p.pid);
    resultat
}

/// Compte les processus suspects (signaux non vides).
pub fn compter_suspects() -> usize {
    analyser_processus()
        .iter()
        .filter(|p| !p.signaux.is_empty())
        .count()
}

/// Limites documentées de la v1 (affichables dans l'UI "À propos").
pub fn limites() -> Vec<String> {
    vec![
        "Sans pilote noyau : un rootkit noyau peut masquer ce que voit userland.".into(),
        "Détection de hooks (SSDT/inline, DKOM) : driver signé requis par OS.".into(),
        "Faux positifs possibles : processus protégés (AV, DRM) sans exe lisible.".into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventaire_non_vide_et_coherent() {
        let liste = analyser_processus();
        // Le processus de test lui-même doit s'y trouver.
        let moi = std::process::id();
        assert!(
            liste.iter().any(|p| p.pid == moi),
            "processus courant introuvable"
        );
        for p in &liste {
            // Invariant : PID 0/1/4 exemptés, les autres ont un nom.
            if !pid_systeme(p.pid) && p.signaux.iter().any(|s| s.contains("vide")) {
                assert!(p.nom.trim().is_empty());
            }
        }
    }

    #[test]
    fn limites_documentees() {
        assert!(limites().len() >= 3);
    }
}
