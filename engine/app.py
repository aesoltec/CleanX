"""CleanX Core — moteur de sécurité.

Exposition :
  - API REST FastAPI  : http://127.0.0.1:8765  (localhost uniquement)
  - WebSocket temps réel : ws://127.0.0.1:8765/ws  (événements push)

Protocole WS (JSON, serveur -> client) :
  {"type": "log",      "message": str}
  {"type": "progress", "scan_id": str, "fichier": str, "traites": int, "total": int}
  {"type": "threat",   "fichier": str, "menace": str, "action": str}
  {"type": "status",   "protection": bool, ...}

Sécurité : écoute 127.0.0.1 uniquement, CORS restreint, en-tête
optionnel X-CleanX-Token (voir JETON_API).

Packaging : `pyinstaller --noconsole --onefile app.py --name cleanx-core`
"""

from __future__ import annotations

import asyncio
import os
import uuid
from pathlib import Path

from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
from pydantic import BaseModel

from core.database import threat_count
from core.heuristics import analyser_heuristique
from core.quarantine import lister_quarantaine, mettre_en_quarantaine
from core.realtime import MoniteurTempsReel, dossier_telechargements
from core.scanner import scanner_fichier_signatures

# ---------------------------------------------------------------- Config
HOTE = "127.0.0.1"
PORT = 8765
JETON_API = os.environ.get("CLEANX_TOKEN", "cleanx-local-dev-token")

# Dossiers de scan rapide / complet (Windows)
HOME = Path.home()
DOSSIER_RAPIDE = [str(HOME / "Downloads"), str(HOME / "Desktop")]
RACINES_COMPLET = [str(HOME / "Documents"), str(HOME / "Downloads"), str(HOME / "Desktop")]

# ---------------------------------------------------------------- App
app = FastAPI(title="CleanX Core", version="1.0.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # trafic local uniquement (bind 127.0.0.1) ; resserrer en prod
    allow_methods=["*"],
    allow_headers=["*"],
)

# ---------------------------------------------------------------- Bus WS
class Bus:
    """Registre des WebSockets connectés + diffusion tolérante aux pannes."""

    def __init__(self) -> None:
        self._clients: set[WebSocket] = set()
        self._lock = asyncio.Lock()

    async def ajouter(self, ws: WebSocket) -> None:
        async with self._lock:
            self._clients.add(ws)

    async def retirer(self, ws: WebSocket) -> None:
        async with self._lock:
            self._clients.discard(ws)

    async def diffuser(self, message: dict) -> None:
        async with self._lock:
            clients = list(self._clients)
        morts = []
        for ws in clients:
            try:
                await ws.send_json(message)
            except Exception:
                morts.append(ws)
        for ws in morts:
            await self.retirer(ws)


bus = Bus()
moniteur = MoniteurTempsReel()
statut = {"protection": False, "menaces": 0, "fichiers_analyses": 0}

# État du scan en cours (un seul scan à la fois, piloté par l'UI).
# "task" garde la référence asyncio.Task pour permettre l'annulation.
etat_scan: dict = {"task": None, "scan_id": None, "annule": False}


# ---------------------------------------------------------------- Analyse unifiée
async def analyser_fichier(chemin: str, auto_quarantaine: bool = True) -> dict:
    """Pipeline complet : signatures -> heuristique -> quarantaine éventuelle.

    Retourne un dict verdict pour l'API et émet les événements WS.
    """
    statut["fichiers_analyses"] += 1

    resultat = await scanner_fichier_signatures(chemin)
    if resultat["erreur"]:
        await bus.diffuser({"type": "log", "message": f"⚠️ {chemin} : {resultat['erreur']}"})
        return {"fichier": chemin, "statut": "erreur", "detail": resultat["erreur"]}

    heur = await asyncio.to_thread(analyser_heuristique, chemin)

    menace: str | None = resultat["menace"]
    if heur["verdict"] == "menace":
        menace = f"Heuristique[{heur['score']}] : " + "; ".join(heur["signaux"][:3])
    elif heur["verdict"] == "suspect" and menace is None:
        # Suspect heuristique seul : log informatif, pas de quarantaine auto
        await bus.diffuser({"type": "log",
                            "message": f"🔍 Suspect ({heur['score']}) : {chemin}"})
        return {"fichier": chemin, "statut": "suspect", "score": heur["score"],
                "signaux": heur["signaux"], "sha256": resultat["sha256"]}

    if menace:
        statut["menaces"] += 1
        action = "signalé"
        if auto_quarantaine:
            q = await asyncio.to_thread(mettre_en_quarantaine, chemin, menace)
            action = f"mis en quarantaine ({q['destination']})" if q["ok"] else f"QUARANTAINE ÉCHOUÉE : {q['erreur']}"
        await bus.diffuser({"type": "threat", "fichier": chemin,
                            "menace": menace, "action": action})
        return {"fichier": chemin, "statut": "menace", "menace": menace,
                "action": action, "sha256": resultat["sha256"]}
    return {"fichier": chemin, "statut": "sain", "sha256": resultat["sha256"]}


async def _lister_fichiers(racines: list[str], limite: int = 5000) -> list[str]:
    """Liste les fichiers à scanner (récursif, tolérant aux permissions)."""
    fichiers: list[str] = []

    def _parcourir(racine: str) -> None:
        try:
            for dirpath, _, filenames in os.walk(racine, onerror=lambda e: None):
                for nom in filenames:
                    if len(fichiers) >= limite:
                        return
                    fichiers.append(os.path.join(dirpath, nom))
        except (OSError, PermissionError):
            pass

    await asyncio.to_thread(lambda: [_parcourir(r) for r in racines if os.path.isdir(r)])
    return fichiers


async def executer_scan(racines: list[str], scan_id: str) -> dict:
    """Scan avec progression temps réel via WS. Annulable via POST /scan/stop."""
    fichiers = await _lister_fichiers(racines)
    total = len(fichiers)
    menaces: list[dict] = []
    await bus.diffuser({"type": "log", "message": f"▶️ Scan {scan_id} : {total} fichiers"})
    try:
        for i, chemin in enumerate(fichiers, 1):
            # Vérifie l'annulation à chaque itération (bouton Stop de l'UI)
            if etat_scan.get("annule"):
                await bus.diffuser({"type": "scan_stopped", "scan_id": scan_id,
                                    "traites": i - 1, "total": total})
                await bus.diffuser({"type": "log",
                                    "message": f"⏹️ Scan {scan_id} arrêté par l'utilisateur "
                                               f"({i - 1}/{total} fichiers, {len(menaces)} menace(s))"})
                return {"scan_id": scan_id, "total": total, "menaces": menaces, "annule": True}
            verdict = await analyser_fichier(chemin)
            if verdict["statut"] == "menace":
                menaces.append(verdict)
            if i % 5 == 0 or i == total:  # throttle : évite de saturer le WS
                await bus.diffuser({"type": "progress", "scan_id": scan_id,
                                    "fichier": chemin, "traites": i, "total": total})
    except asyncio.CancelledError:
        # Annulation dure (task.cancel()) : notifie l'UI puis propage
        await bus.diffuser({"type": "scan_stopped", "scan_id": scan_id, "total": total})
        await bus.diffuser({"type": "log",
                            "message": f"⏹️ Scan {scan_id} interrompu"})
        raise
    await bus.diffuser({"type": "scan_done", "scan_id": scan_id, "total": total,
                        "menaces": len(menaces)})
    await bus.diffuser({"type": "log",
                        "message": f"✅ Scan {scan_id} terminé : {total} fichiers, {len(menaces)} menace(s)"})
    return {"scan_id": scan_id, "total": total, "menaces": menaces, "annule": False}


def _scan_termine(tache: asyncio.Task) -> None:
    """Callback : nettoie l'état même en cas d'erreur ou d'annulation."""
    etat_scan["task"] = None
    etat_scan["scan_id"] = None
    etat_scan["annule"] = False
    try:
        # Consomme l'exception éventuelle pour éviter "Task exception was never retrieved"
        tache.result()
    except (asyncio.CancelledError, Exception):
        pass


def _demarrer_scan(racines: list[str], prefixe: str) -> dict | JSONResponse:
    """Crée la tâche de scan si aucun scan ne tourne. Sinon 409."""
    tache = etat_scan.get("task")
    if isinstance(tache, asyncio.Task) and not tache.done():
        return JSONResponse(status_code=409, content={"erreur": "Un scan est déjà en cours"})
    scan_id = f"{prefixe}-{uuid.uuid4().hex[:6]}"
    etat_scan["annule"] = False
    etat_scan["scan_id"] = scan_id
    nouvelle = asyncio.create_task(executer_scan(racines, scan_id))
    nouvelle.add_done_callback(_scan_termine)
    etat_scan["task"] = nouvelle
    return {"scan_id": scan_id, "racines": racines}


# Callback temps réel : chaque nouveau fichier du dossier surveillé est analysé
async def _sur_nouveau_fichier(chemin: str) -> None:
    await bus.diffuser({"type": "log", "message": f"👁️ Nouveau fichier détecté : {chemin}"})
    # Petit délai : laisse le navigateur finir l'écriture (fichier non verrouillé)
    await asyncio.sleep(1.0)
    await analyser_fichier(chemin)

moniteur.on_fichier = _sur_nouveau_fichier  # type: ignore[assignment]


# ---------------------------------------------------------------- Routes REST
class CheminRequete(BaseModel):
    chemin: str


@app.get("/status")
async def get_status() -> dict:
    """État global pour le dashboard Flutter."""
    tache = etat_scan.get("task")
    scan_en_cours = isinstance(tache, asyncio.Task) and not tache.done()
    return {"protection": moniteur.actif or statut["protection"],
            "dossier_surveille": moniteur.dossier,
            "signatures": threat_count(),
            "menaces": statut["menaces"],
            "fichiers_analyses": statut["fichiers_analyses"],
            "scan_en_cours": scan_en_cours,
            "scan_id": etat_scan.get("scan_id") if scan_en_cours else None,
            "quarantaine": lister_quarantaine()}


@app.post("/scan/quick", response_model=None)
async def scan_rapide() -> dict | JSONResponse:
    return _demarrer_scan(DOSSIER_RAPIDE, "rapide")


@app.post("/scan/full", response_model=None)
async def scan_complet() -> dict | JSONResponse:
    return _demarrer_scan(RACINES_COMPLET, "complet")


@app.post("/scan/stop")
async def arreter_scan() -> dict:
    """Arrête le scan en cours (bouton Stop de l'UI). Idempotent."""
    tache = etat_scan.get("task")
    if not isinstance(tache, asyncio.Task) or tache.done():
        return {"ok": False, "message": "Aucun scan en cours"}
    etat_scan["annule"] = True  # la boucle s'arrête à l'itération suivante
    tache.cancel()  # interruption immédiate même pendant un hash/IO
    return {"ok": True, "scan_id": etat_scan.get("scan_id")}


@app.post("/scan/file")
async def scan_fichier(req: CheminRequete) -> dict:
    if not os.path.isfile(req.chemin):
        return JSONResponse(status_code=404, content={"erreur": "Fichier introuvable"})
    return await analyser_fichier(req.chemin)


@app.post("/protection/on")
async def protection_on() -> dict:
    res = await asyncio.to_thread(moniteur.demarrer)
    statut["protection"] = moniteur.actif
    await bus.diffuser({"type": "status", "protection": moniteur.actif})
    await bus.diffuser({"type": "log", "message": "🛡️ Protection temps réel ACTIVÉE"})
    return res


@app.post("/protection/off")
async def protection_off() -> dict:
    res = await asyncio.to_thread(moniteur.arreter)
    statut["protection"] = False
    await bus.diffuser({"type": "status", "protection": False})
    await bus.diffuser({"type": "log", "message": "⚠️ Protection temps réel DÉSACTIVÉE"})
    return res


@app.get("/quarantine")
async def get_quarantaine() -> dict:
    return {"fichiers": lister_quarantaine()}


# ---------------------------------------------------------------- WebSocket
@app.websocket("/ws")
async def websocket_endpoint(ws: WebSocket) -> None:
    await ws.accept()
    await bus.ajouter(ws)
    # Envoie l'état initial dès la connexion (résilience reconnexion Flutter)
    await ws.send_json({"type": "status", "protection": moniteur.actif,
                        "signatures": threat_count()})
    try:
        while True:
            # Canal bidirectionnel : le client peut envoyer {"ping": 1} ou des ordres
            msg = await ws.receive_json()
            if isinstance(msg, dict) and msg.get("ping"):
                await ws.send_json({"type": "pong"})
    except WebSocketDisconnect:
        pass
    finally:
        await bus.retirer(ws)


if __name__ == "__main__":
    import uvicorn
    uvicorn.run("app:app", host=HOTE, port=PORT, log_level="info")
