# ADR-010 — Supply chain : suppression yara-x, deny, SBOM

- **Statut** : accepté (2026-09-25, Phase 3).
- **Contexte** : `cargo audit` rapportait 18 avis (dont 2 CRITICAL wasmtime)
  via la dépendance OPTIONNELLE `yara-x`, jamais compilée (0 référence dans
  `src/`, feature jamais activée).
- **Décision** :
  1. SUPPRIMER `yara-x` du manifeste (pas d'ignore-list : le risque disparaît
     au lieu d'être masqué). Résultat : **0 vulnérabilité, 0 warning**.
  2. `deny.toml` (schéma cargo-deny 0.20) : allow-list permissive stricte,
     `wildcards = deny`, registres inconnus refusés, `multiple-versions = warn`.
  3. SBOM CycloneDX (`cargo-cyclonedx`, 233 composants) en artefact CI/release.
  4. `cargo-geiger` en CI (rapport unsafe) ; audit manuel local (0 `unsafe`
     manuscrit, glue FRB générée uniquement — B12 : geiger inutilisable sur
     MSVC récent).
- **Conséquences** : toute réintroduction YARA exige wasmtime corrigé +
  ré-audit + implémentation réelle (B05 maintenu comme garde-fou).
- **Alternatives écartées** : 18 entrées `ignore` dans audit.toml (masquage,
  maintenance fragile, faux sentiment de sécurité).
