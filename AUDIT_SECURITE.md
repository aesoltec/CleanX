# Audit sécurité — CleanX (STRIDE, vivant)

## Menaces par module (Cycle 1 — code review)

| Module | Spoofing | Tampering | Repudiation | InfoDisclosure | DoS | EoP |
|---|---|---|---|---|---|---|
| signatures | — (hash fort) | paquet vérifié Ed25519 ✅ | logs horodatés | chemins en logs ⚠️* | fichier 10 Go : lecture streamée ✅ | — |
| heuristics | — | — | — | contenu lu ≤ 2 Mo, jamais exfiltré ✅ | regex bornées, lecture capée ✅ | — |
| watcher | — | symlink non suivis ✅ | journal temps réel | — | anti-rebond + cap ✅ | — |
| quarantine | — | AES-GCM authentifié ✅, nonce 12 o OsRng ✅ | lignes base | clé `cle.key` en clair ⚠️** | blob avant suppression ✅ | restauration sandboxée (dest. choisie par UI) |
| scheduler | — | racines validées (`is_dir`) ✅ | — | — | — | — |
| api FFI | types FRB, pas de parsing manuel ✅ | — | — | `sha256` exposé (non sensible) | 1 scan à la fois, cap 50k fichiers ✅ | moindre privilège (userland assumé, cf. ADR) |
| logging | — | rotation append-only | horodatage + niveaux | * chemins complets en logs locaux (accepté : usage local seul) | rotation quotidienne ✅ | — |

\* Accepté : logs 100 % locaux, aucune télémétrie (contrainte §6).
\** Accepté v1 (documenté) : clé 32 o aléatoire par installation, fichier local.
Durcissement prévu : DPAPI (Windows) / Keychain (macOS) / Keystore (Android) —
ticket Cycle 6.

## Checklist contraintes §6 (état Phase 2 finale)
- [x] Aucun `unwrap`/`expect` en prod (tests uniquement ; `runtime()` → `Result`,
      regex → `filter_map` documenté)
- [x] Aucun `unsafe` manuscrit (glue FFI générée + allow ciblé documenté)
- [x] Aucune injection SQL (paramétrées + transaction dépôt)
- [x] Aucun secret en clair (keyring first, repli tracé ; clés test documentées)
- [x] `cargo audit` 0.22.2 : 18 avis TOUS hors arbre compilé (delta reqwest/keyring = 0)
- [x] Fuzz 100k déterministe 0 crash (cargo-fuzz : cible Linux documentée, B09)
- [x] i18n complet (56 clés FR/EN) ; WCAG AA prouvé en test (10 paires + Semantics)
- [x] Perf mesurée : 73–108k fichiers/min (garde-fou bench, marge 3–5×)
