# CleanX 2.0 — Antivirus multiplateforme (Flutter + Rust FFI)

UI Flutter (Material 3, FR/EN, clair/sombre, desktop + mobile) + moteur Rust
natif (`cleanx_core`) via **flutter_rust_bridge v2.13.0** (FFI direct, streams
temps réel). Persistance SQLite embarquée. Aucun runtime externe.

> Prototype historique Python/WebSocket conservé dans `engine/` (référence).
> La voie officielle est Rust + FFI (cf. `ADR/`).

## Arborescence
```
cleanX/
├── app/                     # Flutter (Riverpod, 6 écrans, i18n, FFI)
│   ├── lib/main.dart        # + src/{core/{engine,providers,theme,l10n},features/*,shared}
│   ├── lib/src/rust/        # Bindings GÉNÉRÉS (ne pas éditer)
│   └── flutter_rust_bridge.yaml
├── core/                    # Moteur Rust (signatures, heuristique, watcher,
│                            # quarantaine AES-GCM, scheduler, self-defense, logging, api FFI)
├── scripts/                 # build_{windows,linux,macos,android}.*, demo.ps1
├── rust_builder/            # Intégration mobile (notes cargokit)
├── ADR/                     # Décisions d'architecture
├── .github/workflows/ci.yml
├── JOURNAL.md DECISIONS.md ROADMAP.md BUGS.md AUDIT_SECURITE.md RAPPORT_FINAL.md
└── engine/                  # PROTOTYPE Python historique (hors build v2)
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

## Régénérer les bindings (après modif `core/src/api.rs`)
```powershell
cd app; flutter_rust_bridge_codegen generate
```

## Budgets
Scan > 20 000 fichiers/min · RAM < 150 Mo au repos · binaire < 50 Mo/plateforme.

## Licence
MIT OR Apache-2.0 (voir `core/Cargo.toml`).
