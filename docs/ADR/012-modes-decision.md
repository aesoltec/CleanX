# ADR-012 — Modes de décision : Prudent par défaut (P14, éthique)

- **Statut :** accepté (2026-09-26, Phase 4 Cycle P14).
- **Contexte :** AGENTS.md §4bis.2 impose le consentement explicite ; or le
  moteur mettait en quarantaine automatiquement toute menace (`auto=true`
  codé en dur dans scan + watcher). Non conforme par construction.
- **Décision :**
  - Nouveau module `core/src/mode.rs` : `enum ModeDecision { Prudent,
    Automatique, Agressif, Silencieux }`, `Default = Prudent`, seuils
    documentés (Automatique : menace avérée ; Agressif : score ≥ 50 ;
    Prudent/Silencieux : jamais d'action auto).
  - Global `MODE_DECISION: AtomicU8` (défaut 0 = Prudent), FFI
    `definir_mode` / `mode_actuel` (régénération bindings incluse).
  - `analyser_fichier` prend le mode en paramètre ; `auto = !MODE_JEU`
    conservé comme surcouche (mode jeu ⇒ surveillance seule).
  - `EvenementMoteur::Menace` enrichi pour P15/P16 : `score: u8`,
    `signaux: Vec<String>`, `critique: bool` (chemins système :
    `C:\Windows\`, `/System/`, `/usr/lib/`, `/etc/`, insensible à la casse).
  - UI : sélecteur dans Paramètres, Prudent coché par défaut, persistance
    locale (provider + lecture à l'init).
- **Conséquences :** `watcher_latence` passe en Automatique (comportement
  historique préservé par opt-in explicite) ; `no_action_without_user_confirmation`
  prouve le défaut Prudent ; aucun changement du format de quarantaine.
- **Alternatives écartées :** file d'attente Rust persistante (complexité ;
  la file d'attente vit côté UI P15), suppression définitive en Agressif
  (interdite : toujours via quarantaine 30 j, P19).
