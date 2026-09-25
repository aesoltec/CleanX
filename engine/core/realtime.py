"""Surveillance temps réel (watchdog) du dossier Téléchargements.

Le moniteur tourne dans un thread dédié (watchdog) et délègue
l'analyse lourde (hash + heuristique) à la boucle asyncio du serveur
via une file thread-safe, pour ne jamais bloquer la détection.
"""

from __future__ import annotations

import asyncio
import os
import queue
import threading
from pathlib import Path

from watchdog.events import FileSystemEventHandler
from watchdog.observers import Observer


def dossier_telechargements() -> str:
    """Retourne le dossier Téléchargements de l'utilisateur courant."""
    return str(Path.home() / "Downloads")


class _Handler(FileSystemEventHandler):
    """Transmet les créations/modifications de fichiers vers une file."""

    def __init__(self, file: queue.Queue[str]) -> None:
        super().__init__()
        self._file = file

    def on_created(self, event):  # noqa: ANN001, ANN202
        if not event.is_directory:
            self._file.put(event.src_path)

    def on_modified(self, event):  # noqa: ANN001, ANN202
        # Certains navigateurs écrivent en plusieurs passes (.part, .crdownload)
        # : on filtre les extensions temporaires.
        if not event.is_directory and not event.src_path.endswith((".part", ".crdownload", ".tmp")):
            self._file.put(event.src_path)


class MoniteurTempsReel:
    """Enveloppe Observer watchdog + boucle de consommation asyncio."""

    def __init__(self, dossier: str | None = None) -> None:
        self.dossier = dossier or dossier_telechargements()
        self._file: queue.Queue[str] = queue.Queue()
        self._observer = Observer()
        self._stop = threading.Event()
        self._thread: threading.Thread | None = None
        self.actif = False
        # Callback async : async def cb(chemin: str)
        self.on_fichier: object = None

    def demarrer(self) -> dict:
        """Démarre la surveillance. Idempotent."""
        if self.actif:
            return {"ok": True, "dossier": self.dossier, "deja_actif": True}
        try:
            os.makedirs(self.dossier, exist_ok=True)
            handler = _Handler(self._file)
            self._observer.schedule(handler, self.dossier, recursive=False)
            self._observer.start()
            self._stop.clear()
            self._thread = threading.Thread(target=self._consommer, daemon=True)
            self._thread.start()
            self.actif = True
            return {"ok": True, "dossier": self.dossier, "deja_actif": False}
        except OSError as exc:
            return {"ok": False, "erreur": f"Surveillance impossible : {exc}"}

    def arreter(self) -> dict:
        """Arrête proprement la surveillance. Idempotent."""
        if not self.actif:
            return {"ok": True, "deja_arrete": True}
        self._stop.set()
        self._observer.stop()
        try:
            self._observer.join(timeout=5)
        except Exception:
            pass
        # Réinitialise l'observer (non réutilisable après stop)
        self._observer = Observer()
        self.actif = False
        return {"ok": True}

    def _consommer(self) -> None:
        """Thread : dépile les chemins et les transmet à la boucle asyncio."""
        vus: dict[str, float] = {}
        import time
        while not self._stop.is_set():
            try:
                chemin = self._file.get(timeout=0.5)
            except queue.Empty:
                continue
            # Anti-rebond : ignore les doublons < 2 s (écritures multiples)
            maintenant = time.time()
            if maintenant - vus.get(chemin, 0) < 2.0:
                continue
            vus[chemin] = maintenant
            cb = self.on_fichier
            if cb is not None:
                try:
                    loop = asyncio.get_event_loop()
                    if loop.is_running():
                        asyncio.run_coroutine_threadsafe(cb(chemin), loop)  # type: ignore[arg-type]
                except RuntimeError:
                    pass  # boucle fermée (arrêt serveur) : on ignore
