# MISSION_STATUS.md — Tableau de bord CleanX

> **Dernière mise à jour :** 2026-09-26 — Cycle 2 (gouvernance P14-P24 + correctifs AGENTS)
> **Agent :** openCode
> **Branche active :** `mission/p14-prudent-par-defaut` (AGENTS.md §1.3 : pas de commit direct sur `main`)
> **Progression globale :** 5 / 24 points ✅ — **21%** (+1 ⚠️ atténué)

---

## 📊 Synthèse

| Catégorie | Total | ✅ | ⚠️ | 🔄/⏳ | ⬜/❌ |
|---|---|---|---|---|---|
| P0 — Vision produit & éthique (P14-P24) | 11 | 0 | 0 | 0 | 11 |
| P0 — Bloquant technique (Points 1-4) | 4 | 1 | 0 | 3 | 0 |
| P1 — Important (Points 5-7) | 3 | 2 | 1 | 0 | 0 |
| P2 — Qualité (Points 8-10) | 3 | 2 | 0 | 1 | 0 |
| P3 — Finition (Points 11-13) | 3 | 0 | 0 | 2 | 1 |
| **TOTAL** | **24** | **5** | **1** | **6** | **12** |

**Verdict provisoire :** ⏳ EN COURS — priorité P0 éthique (P14→P24) par ordre impératif.

**Ordre de traitement :** P14→P24 **EN PREMIER** (exigence 2026-09-26), puis fin P1→P13.

---

## 🔴 P0 — VISION PRODUIT & ÉTHIQUE (priorité absolue)

| # | Point | Statut | Preuve |
|---|---|---|---|
| 14 | Mode "Prudent" par défaut | ⬜ | — |
| 15 | Consentement utilisateur explicite | ⬜ | — |
| 16 | Explicabilité des détections | ⬜ | — |
| 17 | Traçabilité complète des actions | ⬜ | — |
| 18 | Protection des fichiers système | ⬜ | — |
| 19 | Quarantaine réversible 30 jours | ⬜ | — |
| 20 | Télémétrie opt-in (désactivée) | ⬜ | — |
| 21 | UI rivalisant avec Kaspersky | ⬜ | — |
| 22 | Pédagogie et messages clairs | ⬜ | — |
| 23 | Conformité RGPD / vie privée | ⬜ | — |
| 24 | Signaler faux positif en 1 clic | ⬜ | — |

**Progression P0 éthique :** 0 / 11 — **0%**

---

## 🔴 P0 — BLOQUANT TECHNIQUE

### Point 1 — CI réelle et verte
- **Statut :** ⏳ EN COURS
- **Preuve fournie (partielle) :**
  - Job `eicar-linux` ✅ sur 3 runs (36128936485, 36144874208, 36164233168)
  - Job `lint-workflows` ✅ ; `cargo deny check` ✅ en CI
  - Commande : `gh run list --limit 1` → dernier run `completed failure` (26/09)
- **Reste :** Analyse Flutter ×3, Tests ubuntu, coverages, supply complet, iOS, E2E.
- **Fichiers :** `.github/workflows/ci.yml`, `.github/workflows/release.yml`

---

### Point 2 — Chaîne d'audit sécurité complète
- **Statut :** ⏳ EN COURS
- **Preuve fournie (partielle) :**
  - `cargo deny check` : exit 0 local (2026-09-25) + step CI vert (run 36153856126)
  - `cargo audit --file core/Cargo.lock --json` : `vulns: 0, warnings: []`, exit 0 (2026-09-25)
  - Fichiers : `deny.toml` (schéma 0.20 validé), `docs/AUDIT_SECURITE.md`
- **Reste :** rapport geiger (outil retiré du gate, D23 — audit `unsafe` manuel versionné) ; artefact SBOM en CI.
- **Notes :** geiger incompatible prouvé (B12) : flag inexistant + exit 1 sur arbre sain (204 assets non-Rust).

---

### Point 3 — SBOM généré et publié
- **Statut :** ⏳ EN COURS
- **Preuve fournie (partielle) :**
  - Commande : `cargo-cyclonedx cyclonedx --format json --override-filename sbom` (cœur `core/`)
  - Résultat : 233 composants, spec 1.3, 295 Ko — validé localement 2026-09-25
  - Job CI `supply-chain` : step SBOM existant, artefact en attente run vert
- **Reste :** artefact `sbom.json` publié par un run vert.
- **Fichiers :** `.github/workflows/ci.yml`, `.github/workflows/release.yml`

---

### Point 4 — EICAR prouvé en CI Linux
- **Statut :** ✅ TERMINÉ
- **Preuve fournie :**
  - Runs : 36128936485, 36144874208, 36164233168 — job `eicar-linux` ✅ (~1m2x chacun)
  - Commande : `cargo test --manifest-path core/Cargo.toml --test eicar_disque -- --nocapture` (avec `CLEANX_EICAR_DISK=1`)
  - Test : `eicar_fichier_detecte_et_mis_en_quarantaine` (détection + quarantaine + restauration bit-à-bit)
  - Fichier : `core/tests/eicar_disque.rs`
- **Notes :** skip explicite local sans la variable (Defender os 225, BUGS.md B02).

---

## 🟠 P1 — IMPORTANT

### Point 5 — Nettoyer le dépôt
- **Statut :** ✅ TERMINÉ
- **Preuve fournie :**
  - Commande : `Test-Path engine` → `False` ; `git ls-files | Select-String '^engine/'` → vide (2026-09-26)
  - `docs/` contient : ADR/, JOURNAL.md, DECISIONS.md, BUGS.md, AUDIT_SECURITE.md, ROADMAP.md, RAPPORT_FINAL*.md, legacy-python/ (+README)
- **Fichiers :** `docs/legacy-python/README.md`

---

### Point 6 — Fichier `LICENSE` autonome
- **Statut :** ✅ TERMINÉ
- **Preuve fournie :**
  - Fichiers : `LICENSE-MIT`, `LICENSE-APACHE` (11 358 o, en-tête Apache 2.0 vérifié)
  - Commande : `gh api repos/aesoltec/CleanX/license --jq ...` → `{"license":"apache-2.0"}` (2026-09-26) — détection GitHub OK
  - `core/Cargo.toml` : `license = "MIT OR Apache-2.0"` (cohérent)
- **Notes :** pas de fichier `LICENSE` singulier (exigence du template) — les deux textes intégraux sont présents et détectés ; renommage possible si exigé.

---

### Point 7 — Bundle Windows < 50 Mo
- **Statut :** ⚠️ ATTÉNUÉ (justification ci-dessous)
- **Preuve fournie :**
  - `core/target/release/cleanx_core.dll` : 5,3 Mo (strip + LTO + panic=abort)
  - `dist/cleanx_ui.msix` : 32,4 Mo (distribution compressée)
  - Bundle installé : 89,5 Mo (dont `flutter_windows.dll` 20,3 Mo + AOT 57,9 Mo — incompressibles, framework)
  - Référence : `docs/ADR/007-taille-bundle.md`
- **Justification atténuation** : UPX rejeté (flaggé par les AV tiers) ; opt-level="z" révoqué après mesure (-40 % débit, ~0 octet gagné). Distribution < 50 Mo ✅, installé documenté.
- **Fichiers :** `core/Cargo.toml` ([profile.release]), `scripts/build_windows.ps1`

---

## 🟡 P2 — QUALITÉ PROJET

### Point 8 — README refondu
- **Statut :** ✅ TERMINÉ
- **Preuve fournie :**
  - Badge CI : `actions/workflows/ci.yml/badge.svg` → workflow existant ✅
  - Vérification liens 2026-09-26 (script Python) : 15/15 OK (`docs/*.md`, `CONTRIBUTING.md`, `LICENSE-*`, workflows)
- **Fichiers :** `README.md`

---

### Point 9 — `CONTRIBUTING.md` + `CODE_OF_CONDUCT.md`
- **Statut :** ✅ TERMINÉ
- **Preuve fournie :** fichiers présents (`CONTRIBUTING.md`, `CODE_OF_CONDUCT.md` adapté Covenant v2.1), templates `.github/ISSUE_TEMPLATE/*`, `.github/PULL_REQUEST_TEMPLATE.md`
- **Reste (non bloquant)** : détection par l'onglet Community de GitHub à confirmer visuellement.

---

### Point 10 — Clé quarantaine durcie
- **Statut :** ⏳ EN COURS
- **Preuve fournie (partielle) :**
  - Code : `keyring 4` (DPAPI/Keychain/Secret Service), `CoffreIndisponible`, `CLEANX_KEY_FALLBACK` explicite, évitement total du coffre si repli
  - Tests : 9/9 `quarantine::tests::*` verts local (2026-09-25)
  - Fichiers : `core/src/quarantine.rs`
- **Reste :** jobs ubuntu CI verts (run en cours).

---

## 🟢 P3 — FINITION

### Point 11 — Couverture mesurée et publiée
- **Statut :** ⏳ EN COURS
- **Preuve fournie (partielle) :**
  - Rust : **84,87 % lignes** (`cargo llvm-cov`, hors généré) ; Dart : **79,0 %** manuel (`lcov`, hors généré) — mesurés 2026-09-24/25
  - Jobs CI : `rust-coverage` (seuil 70), `dart-coverage` (seuil 60 + `check_coverage.py`)
- **Reste :** artefacts HTML/lcov publiés par un run vert.

---

### Point 12 — Processus de release
- **Statut :** ⏳ EN COURS
- **Preuve fournie (partielle) :**
  - `scripts/release.sh` créé : validation semver + garde arbre propre + `--dry-run`
  - Test : `bash -n` exit 0 ; garde prouvée (`Arbre git sale` sur arbre sale)
  - `CHANGELOG.md` (Keep a Changelog), `.github/workflows/release.yml` (jobs `sbom` + `build-windows`, YAML strict OK)
- **Reste :** exécution réelle sur tag (produit la release + artefacts).

---

### Point 13 — Rapport final V3
- **Statut :** ⬜ NON DÉMARRÉ
- **Preuve attendue :** `docs/RAPPORT_FINAL_V3.md` avec verdict
- **Condition :** tous les points ci-dessus traités (P14→P24 inclus).

---

## 📜 Journal des cycles

| Cycle | Date | Durée | Point traité | Résultat | Preuve |
|---|---|---|---|---|---|
| 1 | 2026-09-26 | — | Initialisation (fichier pré-existant) | Repris | — |
| 2 | 2026-09-26 | — | Gouvernance P14-P24 (MISSION.md, STATUS, dist, D24) | En cours | Cette branche |

---

## 🚦 Règles de mise à jour

1. **Mettre à jour ce fichier APRÈS CHAQUE POINT** traité (pas à la fin).
2. **Statuts possibles :**
   - ⬜ NON DÉMARRÉ
   - ⏳ EN COURS
   - ✅ TERMINÉ (avec preuve obligatoire)
   - ⚠️ ATTÉNUÉ (avec justification)
   - ❌ ÉCHEC (avec ticket BUGS.md)
3. **Aucun ✅ sans preuve** dans la section "Preuve fournie".
4. **Mettre à jour la synthèse** en haut après chaque changement.
5. **Mettre à jour le journal des cycles** à chaque fin de cycle.

---

## 🎯 Critère de fin de mission

La mission est terminée **uniquement** si :

- **24 / 24 points** sont ✅ (⚠️ accepté uniquement avec justification écrite type §4 AGENTS.md)
- **Chaque ✅ a une preuve** traçable
- **`RAPPORT_FINAL_V3.md`** est rédigé avec verdict explicite

**Progression actuelle :** 5 / 24 — **21%** (+1 ⚠️)
