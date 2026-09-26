# MISSION — CleanX : professionnalisation production-grade (consolidé 2026-09-26)

> Généré par l'agent : aucun `MISSION.md` n'existait. Consolidation exhaustive
> des exigences (prompts Phase 2/3, `docs/RAPPORT_FINAL_V2.md`, `docs/ROADMAP.md`,
> `docs/BUGS.md`, état CI du jour). Protocole par point : code modifié
> (chemin + extrait) → test (nom + résultat) → preuve (log/output).
> Ne passer au point suivant qu'après preuve. Tableau final en bas.

## Légende statuts
- ✅ prouvé (log CI local ou GitHub cité) · ⚠️ partiel/documenté
- ❌ ouvert · 🔄 en cours de validation CI

## ⛔ Ordre de priorité impératif (mise à jour 2026-09-26)
Les points **P14→P24 (vision produit & éthique, § P3 ci-dessous) sont PRIORITÉ
P0 et se traitent EN PREMIER**, avant les points techniques restants.
Raison : un antivirus qui agit arbitrairement, même parfait techniquement,
est un produit dangereux et inacceptable. L'éthique d'abord, la technique ensuite.

## P0 — Bloquant production

### M1 — CI verte matrix [ubuntu, macos, windows] 🔄
- Jobs : rust ×3, flutter ×3, lint, coverage ×2, eicar, supply, ios, windows-e2e.
- Preuve attendue : run GitHub vert + liens.

### M2 — `flutter analyze` 0 incident sur 3 OS 🔄
- Cause trouvée (B13) : `quarantine_page.dart` jamais commité ; fix poussé.
- Preuve attendue : steps Analyse verts sur le run en cours.

### M3 — Tests Rust verts sur 3 OS (dont ubuntu, ex-keyring) 🔄
- Cause trouvée : tests clé stricts sur OS sans coffre + cascade PoisonError.
- Fix : chemin env-gaté + verrous anti-empoisonnement (+ repli sans toucher
  le coffre en headless). Preuve attendue : run en cours.

### M4 — Couverture Rust ≥ 70 % mesurée en CI 🔄
- Local : 84,9 % lignes (llvm-cov, hors généré). CI : job dédié + seuil.
- Preuve attendue : step `rust-coverage` vert.

### M5 — Couverture Dart ≥ 60 % mesurée en CI 🔄
- Local : 79,0 % manuel. CI : job + `scripts/check_coverage.py`.
- Preuve attendue : step `dart-coverage` vert.

### M6 — EICAR disque réel en CI Ubuntu ✅
- Preuve : jobs `eicar-linux` verts (runs 36128936485, 36144874208,
  36164233168 — step « Test EICAR bout-en-bout » success, ~1m2x).

### M7 — Chaîne audit : deny ✅ / audit ✅ / geiger 🔄 / SBOM 🔄
- `cargo deny check` vert local + CI ; `cargo audit` 0/0 local.
- geiger : échec CI en cours d'analyse (logs) ; SBOM validé local (233).
- Preuves attendues : steps supply-chain verts + artefact SBOM.

## P1 — Important

### M8 — Bundle Windows : DLL 5,3 Mo ✅ ; MSIX 32,4 Mo ✅ ; installé 89,5 Mo ⚠️
- Cause externe documentée (framework Flutter). Pas d'action restante
  raisonnable (UPX rejeté, ADR-007).

### M9 — Dépôt propre : `docs/` + legacy archivé + LICENSE ✅
- Preuve : arborescence + `git ls-files` (aucun `engine/` à la racine).

### M10 — Clé quarantaine stricte, repli explicite ✅ (code) 🔄 (CI)
- `CoffreIndisponible`, `CLEANX_KEY_FALLBACK`, évitement total du coffre
  si repli (anti-hang). Preuve attendue : jobs ubuntu verts.

## P2 — Qualité projet

### M11 — README badges + docs/ + CONTRIBUTING + CoC + templates ✅
- Vérifier : badges pointent vers workflows existants ; liens docs/ valides.

### M12 — iOS : `.a` compilés en CI 🔄 / liaison `.app` documentée ✅
- Job `ios-unsigned` réduit aux libs ; échec `Permission denied` analysé
  (bit +x manquant → corrigé via `git update-index`).
- Preuve attendue : step vert sur le run en cours.

### M13 — Android APK ✅ (63 Mo, `dist/`, 3 ABI)

### M14 — Watcher < 1 s ✅ (0,35 s mesuré, test `watcher_latence`)

### M15 — Fuzz 100k déterministe 0 crash ✅ (+ cible cargo-fuzz pour CI Linux)

### M16 — Dépôt HTTPS + Ed25519 + delta + rollback ✅ (tests v1/v2/v0)

### M17 — WCAG AA ✅ (10 paires en test + Semantics, B07 corrigé)

### M18 — Mode jeu, rapport PDF, rootkit v1 ✅ (tests verts)

## Tableau récapitulatif
| Point | Statut | Preuve |
|---|---|---|
| M1 CI verte complète | 🔄 | run en cours (voir § verdict) |
| M2 Analyse Flutter ×3 | 🔄 | B13 prouvé local (clone frais) ; CI en cours |
| M3 Tests Rust ×3 | 🔄 | fix keyring poussé ; CI en cours |
| M4 Couverture Rust CI | 🔄 | 84,9 % local ; job en cours |
| M5 Couverture Dart CI | 🔄 | 79,0 % local ; job en cours |
| M6 EICAR CI | ✅ | 3 runs verts (steps ~1m2x) |
| M7 Audit/deny/geiger/SBOM | 🔄→✅ partiel | deny ✅ CI, audit 0/0, SBOM 233 ✅ ; geiger retiré documenté (B12/D23) |
| M8 Bundle | ✅/⚠️ | DLL 5,3 Mo, MSIX 32,4 Mo ; installé 89,5 Mo documenté |
| M9 Dépôt propre | ✅ | arborescence + git |
| M10 Clé stricte | ✅/🔄 | code + tests locaux verts ; CI en cours |
| M11 Docs/contrib | ✅ | fichiers présents (liens à re-vérifier) |
| M12 iOS | 🔄 | bit +x corrigé ; CI en cours |
| M13 APK | ✅ | `dist/cleanx-2.0.0-release.apk` 63 Mo |
| M14 Watcher | ✅ | 0,35 s (`watcher_latence`) |
| M15 Fuzz | ✅ | 100k/0 crash |
| M16 Dépôt HTTPS | ✅ | tests v1/v2/v0 + rollback |
| M17 WCAG AA | ✅ | `accessibilite_test.dart` vert |
| M18 Bonus UI | ✅ | 21/21 tests Dart |


## 🔴 P0 — NOUVEAUX POINTS CRITIQUES (vision produit & éthique)

### Point 14 — Mode "Prudent" par défaut
- **Statut cible :** ✅
- **Preuve exigée :** capture d'écran + code prouvant que le mode par défaut
  est "Demander avant toute action"
- **Livrables :**
  - Enum `ModeDecision { Prudent, Automatique, Agressif, Silencieux }`
  - Constante `DEFAULT_MODE = ModeDecision::Prudent`
  - UI : sélecteur dans Paramètres, "Prudent" coché par défaut
  - Test unitaire : `fn default_mode_is_prudent()`
  - Test d'intégration : détection → notification, pas d'action auto

### Point 15 — Consentement utilisateur explicite
- **Statut cible :** ✅
- **Preuve exigée :** test E2E prouvant qu'aucune action n'est prise sans
  confirmation utilisateur en mode Prudent
- **Livrables :**
  - Dialogue modal à chaque détection (mode Prudent)
  - 4 boutons : Ignorer / Quarantaine / Supprimer / Analyser plus
  - Timeout par défaut : aucune action (le fichier reste en place)
  - Test : `fn no_action_without_user_confirmation()`

### Point 16 — Explicabilité des détections
- **Statut cible :** ✅
- **Preuve exigée :** chaque détection expose règle, score, signature
- **Livrables :**
  - Structure `DetectionReason { rule, score, signature, timestamp }`
  - UI : bouton "En savoir plus" sur chaque alerte
  - Message pédagogique (ex. "Ce fichier tente de modifier le registre système")
  - Test : `fn detection_exposes_reason()`

### Point 17 — Traçabilité complète des actions
- **Statut cible :** ✅
- **Preuve exigée :** logs structurés JSON + tableau de bord "Historique"
- **Livrables :**
  - Table SQLite `actions_log` (timestamp, file, action, mode, user_decision)
  - Écran "Historique des décisions" dans l'UI
  - Export CSV possible
  - Test : `fn all_actions_are_logged()`

### Point 18 — Protection des fichiers système
- **Statut cible :** ✅
- **Preuve exigée :** test prouvant qu'aucune action auto n'est prise sur
  `C:\Windows\`, `/System/`, `/usr/lib/`, `/etc/`
- **Livrables :**
  - Fonction `is_system_critical_path(path) -> bool`
  - Double confirmation obligatoire pour ces chemins
  - Whitelist signée Microsoft/Apple/Google
  - Test : `fn system_files_require_double_confirmation()`

### Point 19 — Quarantaine réversible
- **Statut cible :** ✅
- **Preuve exigée :** test prouvant qu'un fichier en quarantaine peut être
  restauré pendant 30 jours minimum
- **Livrables :**
  - Politique de rétention 30 jours (configurable)
  - Bouton "Restaurer" dans l'UI
  - Suppression définitive uniquement après confirmation explicite
  - Test : `fn quarantine_is_restorable_for_30_days()`

### Point 20 — Télémétrie opt-in (désactivée par défaut)
- **Statut cible :** ✅
- **Preuve exigée :** code + test prouvant qu'aucune donnée n'est envoyée
  sans consentement
- **Livrables :**
  - Constante `TELEMETRY_ENABLED = false` par défaut
  - Écran Paramètres : toggle explicite + lien politique de confidentialité
  - Aucune requête réseau si désactivé
  - Test : `fn no_network_without_consent()`

### Point 21 — Interface rivalisant avec Kaspersky
- **Statut cible :** ✅
- **Preuve exigée :** captures d'écran + audit UX (fluidité 60 FPS, animations)
- **Livrables :**
  - Dashboard épuré (jauges animées, cards Material 3)
  - Animations fluides (`flutter_animate` ou `AnimationController`)
  - Thèmes clair/sombre soignés
  - Icônes cohérentes (Material Symbols ou custom)
  - Test : `flutter test --profile` avec métriques 60 FPS
  - Audit visuel : comparer avec captures Kaspersky/Bitdefender

### Point 22 — Pédagogie et messages clairs
- **Statut cible :** ✅
- **Preuve exigée :** chaque message d'alerte est relu pour clarté
- **Livrables :**
  - Guide de style des messages (`docs/STYLE_MESSAGES.md`)
  - Aucun jargon non expliqué
  - Bouton "En savoir plus" sur chaque écran
  - Test : `fn all_messages_are_i18n_and_clear()`

### Point 23 — Conformité RGPD / vie privée
- **Statut cible :** ✅
- **Preuve exigée :** `docs/CONFIDENTIALITE.md` + code prouvant opt-in
- **Livrables :**
  - Politique de confidentialité rédigée
  - Aucune donnée envoyée sans opt-in
  - Aucune publicité, aucune vente de données
  - Conformité RGPD documentée
  - Test : `fn no_telemetry_without_consent()`

### Point 24 — Mode "Signaler faux positif" en 1 clic
- **Statut cible :** ✅
- **Preuve exigée :** bouton visible sur chaque alerte + log de signalement
- **Livrables :**
  - Bouton "Signaler faux positif" dans chaque dialogue
  - Enregistrement local + option d'envoi opt-in
  - Amélioration continue de la whitelist
  - Test : `fn false_positive_can_be_reported()`

---

## 🎯 Critères globaux mis à jour

Le projet est **GO PRODUCTION** uniquement si :

- **Tous les points P1→P13** sont ✅ (critères techniques)
- **Tous les points P14→P24** sont ✅ (critères produit & éthique)
- **L'UI rivalise visuellement avec Kaspersky** (audit UX comparatif)
- **Aucune action automatique** sans opt-in explicite
- **Chaque détection est explicable** à un utilisateur non technique
- **Télémétrie désactivée par défaut**
- **Aucune publicité, aucune vente de données**
- **Quarantaine toujours réversible** (30 jours minimum)