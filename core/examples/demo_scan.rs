//! Démo reproductible (`cargo run --example demo_scan --manifest-path core/Cargo.toml`).
//!
//! Initialise le moteur dans un dossier temporaire, analyse un fichier sain
//! et un fichier suspect, affiche les verdicts. Aucun contenu dangereux
//! (compatible avec un antivirus hôte actif).

use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = std::env::temp_dir().join(format!("cleanx-demo-{}", std::process::id()));
    std::fs::create_dir_all(&base)?;

    // Base + seed.
    let db = base.join("cleanx.db");
    let conn = cleanx_core::db::ouvrir(&db)?;
    println!(
        "Base OK : {} signatures",
        cleanx_core::db::compter_signatures(&conn)?
    );

    // Fixtures.
    let sain = base.join("bonjour.txt");
    std::fs::write(&sain, b"Document de demonstration CleanX.")?;
    let suspect = base.join("note.txt.exe");
    let mut f = std::fs::File::create(&suspect)?;
    f.write_all(b"Contenu inoffensif, extension trompeuse pour la demo.")?;

    // Signatures (sync via runtime mono-thread de l'exemple).
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    for cible in [&sain, &suspect] {
        let verdict = rt.block_on(cleanx_core::signatures::verifier_signature(&db, cible))?;
        println!(
            "{} -> menace={:?} erreur={:?}",
            cible.display(),
            verdict.menace,
            verdict.erreur
        );
        let h =
            cleanx_core::heuristics::analyser(cible, cleanx_core::heuristics::Seuils::default());
        println!(
            "  heuristique : score={} verdict={:?} signaux={:?}",
            h.score, h.verdict, h.signaux
        );
    }
    println!("Démo terminée (base : {})", base.display());
    Ok(())
}
