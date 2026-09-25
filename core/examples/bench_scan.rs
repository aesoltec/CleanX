//! Micro-benchmark de débit (outil dev, pas un test) :
//! `cargo run --release --manifest-path core/Cargo.toml --example bench_scan [N]`
//!
//! Génère N fichiers bénins (défaut 2000) et mesure le hot-path réel d'un scan
//! (SHA-256 async + lookup SQLite + heuristique complète, workers tokio),
//! sans le sink FFI (événements) ni la quarantaine (voie rare).
//! Affiche fichiers/min (objectif : > 20 000).

use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);
    let base = std::env::temp_dir().join(format!("cleanx-bench-{}", std::process::id()));
    let fixtures = base.join("fixtures");
    std::fs::create_dir_all(&fixtures)?;
    let db_path = base.join("cleanx.db");

    // Contenus variés et bénins (tailles ~0.5–30 Ko).
    for i in 0..n {
        let contenu = format!("Fichier de benchmark CleanX #{i}\n").repeat(20 + (i % 60));
        std::fs::write(fixtures.join(format!("fichier_{i:05}.txt")), contenu)?;
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;
    // Phase 0 : coût d'ouverture SQLite seule (piste d'optimisation).
    let t0 = Instant::now();
    for _ in 0..500 {
        let _ = cleanx_core::db::ouvrir(&db_path)?;
    }
    println!(
        "OUVERTURE-BASE : 500 ouvertures en {:.2}s",
        t0.elapsed().as_secs_f64()
    );
    let debut = Instant::now();
    rt.block_on(async {
        // Base partagée : une connexion par worker via spawn_blocking.
        let mut fichiers = Vec::with_capacity(n);
        let mut pile = vec![fixtures.clone()];
        while let Some(dir) = pile.pop() {
            for e in std::fs::read_dir(dir)?.flatten() {
                let p = e.path();
                if let Ok(t) = e.file_type() {
                    if t.is_dir() {
                        pile.push(p);
                    } else if t.is_file() {
                        fichiers.push(p);
                    }
                }
            }
        }
        let mut groupe = tokio::task::JoinSet::new();
        let fichiers = std::sync::Arc::new(fichiers);
        for w in 0..8 {
            let fichiers = fichiers.clone();
            let db = db_path.clone();
            groupe.spawn(async move {
                let total = fichiers.len();
                let mut i = w;
                while i < total {
                    let chemin = &fichiers[i];
                    if let Ok(v) = cleanx_core::signatures::verifier_signature(&db, chemin).await {
                        if v.erreur.is_none() {
                            let c = chemin.clone();
                            let _ = tokio::task::spawn_blocking(move || {
                                cleanx_core::heuristics::analyser(
                                    &c,
                                    cleanx_core::heuristics::Seuils::default(),
                                )
                            })
                            .await;
                        }
                    }
                    i += 8;
                }
            });
        }
        while groupe.join_next().await.is_some() {}
        std::io::Result::Ok(())
    })?;
    let secs = debut.elapsed().as_secs_f64();
    println!(
        "BENCH : {n} fichiers en {secs:.2}s = {:.0} fichiers/min",
        n as f64 / secs * 60.0
    );
    std::fs::remove_dir_all(&base).ok();
    Ok(())
}
