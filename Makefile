# CleanX — Makefile (Unix : Linux/macOS ; Windows : voir scripts/*.ps1)

.PHONY: demo demo-moteur gen bindings audit test-rust test-dart build-win

# Démo reproductible : scan réel du moteur + rappel UI.
demo: demo-moteur
	@echo ''
	@echo '=== UI (autre terminal) ==='
	@echo '  cd app && flutter run -d linux  # mode démo (simulé)'
	@echo '  ./scripts/build_linux.sh        # lib native release'
	@echo '  cd app && flutter run -d linux --dart-define=CLEANX_FRB=true'

demo-moteur:
	cargo run --manifest-path core/Cargo.toml --example demo_scan

# Régénère les bindings Dart après modification de core/src/api.rs
gen:
	cd app && flutter_rust_bridge_codegen generate

audit:
	cargo audit --file core/Cargo.lock

test-rust:
	cargo test --manifest-path core/Cargo.toml
	cargo clippy --manifest-path core/Cargo.toml --all-targets -- -D warnings
	cargo fmt --manifest-path core/Cargo.toml --check

test-dart:
	cd app && flutter analyze && flutter test
