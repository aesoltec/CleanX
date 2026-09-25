# Contribuer à CleanX

Merci de contribuer ! Ce guide résume le strict minimum pour une PR acceptable.

## Démarrage (Windows)
```powershell
# Rust + Flutter à jour (voir README.md)
cargo test --manifest-path core/Cargo.toml
cd app; flutter pub get; flutter analyze; flutter test
```

## Règles non négociables
- Rust : `cargo fmt`, `cargo clippy --all-targets -- -D warnings` verts ;
  **aucun `unwrap`/`expect` en code production** (tests uniquement).
- Dart : `flutter analyze` 0 incident, conventions Effective Dart.
- Après toute modification de `core/src/api.rs` :
  `flutter_rust_bridge_codegen generate` (depuis `app/`) + commit des bindings.
- UI : aucune string hardcodée (ARB FR/EN via `flutter gen-l10n`).
- Sécurité : pas de secret en clair, pas de télémétrie, erreurs en français
  remontées à l'UI. Toute CVE HIGH/CRITICAL bloque la PR (`cargo audit`).
- Supply chain : `cargo deny check` vert ; nouvelle dépendance = justification
  dans la PR + vérification licence (allow-list dans `deny.toml`).
- `Ne PAS activer --features yara` (B05 : ré-audit préalable obligatoire).
- Épinglage FRB 2.13.0 triple (Rust + Dart + codegen) à conserver.

## Processus PR
1. Décrivez le problème (lien BUGS.md ou issue) et la preuve
   (commandes + résultats, couverture si applicable).
2. Mettez à jour `docs/JOURNAL.md` (cycle) et `docs/DECISIONS.md` si arbitrage.
3. La CI doit être verte sur les 3 OS avant merge (pas d'exception).

## Gouvernance
Cycle Plan → Conception (ADR) → Implémentation → Tests → Audit → Go/No-Go →
Journal. Voir `docs/JOURNAL.md` pour les exemples.
