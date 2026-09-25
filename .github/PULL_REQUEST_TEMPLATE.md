## Objet
<!-- Lien issue/BUGS.md, résumé -->

## Preuves
<!-- Commandes + résultats : cargo test/clippy/fmt, flutter analyze/test, couverture -->

## Checklist
- [ ] `cargo clippy --all-targets -- -D warnings` vert
- [ ] `flutter analyze` 0 incident
- [ ] Aucun unwrap/expect en prod, aucune string UI hardcodée
- [ ] Bindings régénérés si `api.rs` touché
- [ ] `docs/JOURNAL.md` mis à jour (et `DECISIONS.md` si arbitrage)
- [ ] `cargo audit` + `cargo deny check` verts (si dépendances touchées)
