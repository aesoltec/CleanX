"""Quarantaine : isole les fichiers malveillants de façon réversible."""

from __future__ import annotations

import json
import os
import shutil
import time
from pathlib import Path

# Dossier de quarantaine à côté du moteur (créé au besoin)
QUARANTINE_DIR = Path(__file__).resolve().parent.parent / "quarantine"
QUARANTINE_DIR.mkdir(parents=True, exist_ok=True)


def mettre_en_quarantaine(chemin: str, raison: str) -> dict:
    """Déplace un fichier en quarantaine et écrit un manifeste .json.

    Retour : {"ok": bool, "destination": str|None, "erreur": str|None}
    """
    try:
        src = Path(chemin)
        if not src.is_file():
            return {"ok": False, "destination": None, "erreur": "Fichier introuvable"}

        dest_nom = f"{int(time.time())}_{src.name}.quar"
        dest = QUARANTINE_DIR / dest_nom
        shutil.move(str(src), str(dest))

        manifeste = dest.with_suffix(dest.suffix + ".json")
        manifeste.write_text(
            json.dumps({"origine": str(src), "raison": raison,
                        "date": time.strftime("%Y-%m-%d %H:%M:%S")},
                       ensure_ascii=False, indent=2),
            encoding="utf-8",
        )
        return {"ok": True, "destination": str(dest), "erreur": None}

    except PermissionError:
        return {"ok": False, "destination": None,
                "erreur": "Permission refusée : lancez CleanX en administrateur"}
    except OSError as exc:
        return {"ok": False, "destination": None,
                "erreur": f"Échec quarantaine (fichier verrouillé ?) : {exc}"}


def restaurer(nom_quar: str, destination: str) -> dict:
    """Restaure un fichier depuis la quarantaine (faux-positif)."""
    try:
        src = QUARANTINE_DIR / nom_quar
        if not src.is_file():
            return {"ok": False, "erreur": "Entrée de quarantaine introuvable"}
        shutil.move(str(src), destination)
        # Nettoie le manifeste associé s'il existe
        manifeste = src.with_suffix(src.suffix + ".json")
        if manifeste.exists():
            manifeste.unlink(missing_ok=True)
        return {"ok": True, "erreur": None}
    except OSError as exc:
        return {"ok": False, "erreur": f"Restauration impossible : {exc}"}


def lister_quarantaine() -> list[dict]:
    """Liste les fichiers en quarantaine (sans lever d'exception)."""
    try:
        return [{"nom": p.name, "taille": p.stat().st_size}
                for p in QUARANTINE_DIR.glob("*.quar")]
    except OSError:
        return []
