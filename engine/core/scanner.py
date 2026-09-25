"""Scan de signatures : calcul SHA-256 asynchrone + comparaison base locale."""

from __future__ import annotations

import asyncio
import hashlib
import os

from .database import is_known_malicious

# Lecture par blocs de 1 Mo (bon compromis disque / mémoire)
CHUNK_SIZE = 1024 * 1024


def _hash_sync(chemin: str) -> str:
    """Calcul SHA-256 bloquant (exécuté dans un thread via asyncio.to_thread)."""
    digest = hashlib.sha256()
    with open(chemin, "rb") as fh:
        while True:
            bloc = fh.read(CHUNK_SIZE)
            if not bloc:
                break
            digest.update(bloc)
    return digest.hexdigest()


async def calculer_sha256(chemin: str) -> str:
    """Calcule le SHA-256 sans bloquer la boucle asyncio.

    Lève :
        FileNotFoundError : fichier inexistant
        PermissionError : accès refusé
        OSError : fichier verrouillé / erreur I/O
    """
    if not os.path.isfile(chemin):
        raise FileNotFoundError(f"Fichier introuvable : {chemin}")
    # to_thread évite de bloquer l'event-loop pendant la lecture disque
    return await asyncio.to_thread(_hash_sync, chemin)


async def scanner_fichier_signatures(chemin: str) -> dict:
    """Scanne un fichier par signatures.

    Retour :
        {"chemin": str, "sha256": str|None, "menace": str|None,
         "erreur": str|None}
    Ne lève jamais : les erreurs (verrou, permissions) sont encapsulées.
    """
    try:
        sha = await calculer_sha256(chemin)
    except FileNotFoundError as exc:
        return {"chemin": chemin, "sha256": None, "menace": None, "erreur": str(exc)}
    except PermissionError:
        return {"chemin": chemin, "sha256": None, "menace": None,
                "erreur": "Permission refusée (exécutez en administrateur ?)"}
    except OSError as exc:
        return {"chemin": chemin, "sha256": None, "menace": None,
                "erreur": f"Fichier illisible/verrouillé : {exc}"}

    menace = is_known_malicious(sha)
    return {"chemin": chemin, "sha256": sha, "menace": menace, "erreur": None}
