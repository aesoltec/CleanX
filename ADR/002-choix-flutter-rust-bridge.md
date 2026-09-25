# ADR-002 — flutter_rust_bridge v2 pour le FFI

- **Statut** : accepté (2026-09-23).
- **Contexte** : relier Flutter (Dart) au moteur Rust en bidirectionnel
  (appels + streams de progression/alertes).
- **Décision** : flutter_rust_bridge v2.13.0 épinglé (lib Rust, package Dart,
  codegen — l'alignement des trois est obligatoire).
- **Conséquences** : `StreamSink<T>` → `Stream<T>` côté Dart, erreurs `Result<T, E>`
  mappées ; `frb_generated.rs` versionné après génération ; stub local avant codegen
  (cf. DECISIONS.md D04).
- **Alternatives écartées** : dart:ffi manuel (coût, erreurs de codec), MethodChannel
  + binaire externe (latence, processus à superviser), Protobuf/gRPC local (lourd).
