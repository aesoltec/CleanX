# Roadmap — CleanX

## Fait
- [x] Toolchain Rust 1.98.1 + clippy/rustfmt (Cycle 0)
- [x] Crate `cleanx_core` : error, db, signatures (+Ed25519), heuristics
      (+entropie), watcher (notify), quarantine (AES-GCM + Argon2),
      scheduler, self_defense, logging (tracing JSON + rotation), api FFI (Cycle 1)
- [x] 22 tests Rust verts, clippy/fmt stricts (Cycle 1)
- [x] Bindings FRB 2.13.0 générés (11 fichiers) (Cycle 2)
- [x] Flutter : contrat + simule + pont FRB, 8 providers, 6 écrans, i18n FR/EN,
      analyze 0 + 6 tests verts (Cycle 3)
- [x] Gouvernance : JOURNAL, DECISIONS, ROADMAP, BUGS, AUDIT_SECURITE, ADR/001-004

## Reste (Phase 2 — état final)
- [x] R1 couverture : Rust 84,0 % / Dart 79,0 % (seuils CI en place)
- [x] R2 EICAR : test + job CI Ubuntu (exécution au premier push)
- [x] R3 bundle : DLL 5,3 Mo, MSIX 32,4 Mo (installé 89,5 Mo : NO-GO strict documenté)
- [x] R4 CI matrix : YAML 7 jobs validé (exécution au premier push)
- [x] R5 watcher : 0,35 s mesuré + testé
- [x] Android APK 63 Mo (`dist/`) ; iOS : procédure + CI + runner scaffoldé
- [x] Fuzz 100k déterministe 0 crash ; clé keyring ; dépôt HTTPS+Ed25519 ; WCAG AA
- [ ] Premier push GitHub → CI réelle + builds macOS/Linux/iOS + signatures (voir RAPPORT_FINAL_V2)

## Estimation restante
- ~5-6 cycles ≈ 4-6 h de travail autonome (hors compilations longues).
