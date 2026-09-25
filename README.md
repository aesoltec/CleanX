# CleanX 2.0 — Antivirus multiplateforme (Flutter + Rust FFI)

![CI](https://github.com/aesoltec/CleanX/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.98%2B-orange) ![Flutter](https://img.shields.io/badge/flutter-3.47%2B-blue)

UI Flutter (Material 3, FR/EN, clair/sombre, desktop + mobile) + moteur Rust
natif (`cleanx_core`) via **flutter_rust_bridge v2.13.0** (FFI direct, streams
temps réel). Persistance SQLite embarquée. Aucun runtime externe.

> Prototype historique Python/WebSocket archivé dans `docs/legacy-python/`
> (référence uniquement). La voie officielle est Rust + FFI (cf. `docs/ADR/`).

## Arborescence
```
cleanX/
├── app/                     # Flutter (Riverpod, 6 écrans, i18n, FFI)
│   ├── lib/main.dart        # + src/{core/{engine,providers,theme,l10n},features/*,shared}
│   ├── lib/src/rust/        # Bindings GÉNÉRÉS (ne pas éditer)
│   └── flutter_rust_bridge.yaml
├── core/                    # Moteur Rust (signatures, heuristique, watcher,
│                            # quarantaine AES-GCM, scheduler, self-defense,
│                            # logging, rootkit, api FFI)
├── scripts/                 # build_{windows,linux,macos,android,ios}.*,
│                            # demo.ps1, check_coverage.py
├── rust_builder/            # Intégration mobile (notes cargokit)
├── docs/                    # Gouvernance : ADR, JOURNAL, DECISIONS, BUGS,
│                            # AUDIT_SECURITE, ROADMAP, RAPPORT_*, legacy-python
├── dist/                    # Artefacts (MSIX, APK, DLL)
├── deny.toml                # Supply chain (cargo-deny)
└── .github/workflows/       # ci.yml (matrix 3 OS) + release.yml
```

## Démarrage rapide (Windows)
```powershell
# 1. Prérequis : Rust stable (rustup), Flutter stable
# 2. Moteur : tests + build natif
cargo test --manifest-path core/Cargo.toml
.\scripts\build_windows.ps1          # cleanx_core.dll -> build/.../Debug|Release
# 3. UI démo (sans lib native)
cd app; flutter pub get; flutter run -d windows
# 4. UI + vrai moteur Rust
flutter run -d windows --dart-define=CLEANX_FRB=true
# 5. Démo reproductible (sans UI) : .\scripts\demo.ps1  (Unix : make demo)
```

Préconisé dev : exclure `core\target` du scan temps réel de votre antivirus
hôte (verrous `os error 5` pendant l'édition de lien — cf. `docs/BUGS.md` B08).

## Régénérer les bindings (après modif `core/src/api.rs`)
```powershell
cd app; flutter_rust_bridge_codegen generate
```

## Budgets mesurés
Scan ~100k fichiers/min (2k) · DLL 5,3 Mo · MSIX 32,4 Mo · APK 63 Mo.

## Docs
[Gouvernance](docs/JOURNAL.md) · [Décisions](docs/DECISIONS.md) ·
[Rapport final](docs/RAPPORT_FINAL_V2.md) · [Audit sécurité](docs/AUDIT_SECURITE.md) ·
[Contribuer](CONTRIBUTING.md)

## Licence
MIT OR Apache-2.0 — voir [LICENSE-MIT](LICENSE-MIT) et [LICENSE-APACHE](LICENSE-APACHE).
