"""Analyse heuristique initiale.

Principe : sans exécuter le fichier, on attribue un score de suspicion
(0-100) à partir de signaux faibles :
  1. Extension à risque / double extension (ex: facture.pdf.exe)
  2. Taille anormale (très petit exécutable, gros script)
  3. Chaînes suspectes (appels système, obfuscation, exfiltration)

Un score >= 70 => menace heuristique.
Un score >= 40 => suspect (alerte informative).
"""

from __future__ import annotations

import os
import re

# Extensions exécutables / scriptées à risque
RISKY_EXTENSIONS = frozenset({
    ".exe", ".scr", ".com", ".bat", ".cmd", ".ps1", ".vbs", ".vbe",
    ".js", ".jse", ".wsf", ".wsh", ".jar", ".msi", ".dll", ".lnk",
    ".hta", ".cpl", ".gadget", ".apk",
})

# Extensions doubles typiques d'arnaque
DOCUMENT_EXTS = frozenset({".pdf", ".doc", ".docx", ".xls", ".xlsx", ".jpg", ".png", ".mp4", ".zip"})

# Motifs suspects (regex insensibles à la casse, lecture en mode texte tolérant)
SUSPICIOUS_PATTERNS: list[tuple[str, int, str]] = [
    (r"powershell\s+-[eE]nc", 30, "PowerShell encodé (-enc)"),
    (r"frombase64string|toBase64String", 15, "Encodage Base64 API"),
    (r"CreateRemoteThread|VirtualAllocEx|WriteProcessMemory", 30, "Injection de processus"),
    (r"mimikatz|sekurlsa|lsass", 35, "Vol d'identifiants"),
    (r"HKLM.*\\Run|CurrentVersion\\Run", 15, "Persistance registre"),
    (r"bitsadmin|certutil\s+-urlcache|Invoke-Mimikatz|Invoke-Shellcode", 25, "Téléchargement furtif"),
    (r"eval\s*\(|exec\s*\(|obfuscate|chr\s*\(\s*\d+", 10, "Obfuscation possible"),
    (r"curl.+\|\s*(ba)?sh|wget.+\|\s*(ba)?sh", 20, "Pipe shell distant"),
    (r"ransom|decrypt.*bitcoin|vssadmin\s+delete\s+shadows", 35, "Comportement rançongiciel"),
    (r"keylog|SetWindowsHookEx|GetAsyncKeyState", 25, "Keylogger potentiel"),
]

_COMPILED = [(re.compile(p, re.IGNORECASE), pts, label) for p, pts, label in SUSPICIOUS_PATTERNS]

# Taille max lue pour la recherche de chaînes (2 Mo suffisent pour l'heuristique)
MAX_READ_BYTES = 2 * 1024 * 1024


def analyser_heuristique(chemin: str) -> dict:
    """Analyse un fichier et retourne un verdict heuristique.

    Retour :
        {"score": int, "signaux": [str], "verdict": "sain"|"suspect"|"menace"}
    Ne lève jamais d'exception : toute erreur I/O est rapportée comme signal.
    """
    signaux: list[str] = []
    score = 0

    try:
        nom = os.path.basename(chemin)
        _, ext = os.path.splitext(nom)
        ext = ext.lower()

        # 1. Extension à risque
        if ext in RISKY_EXTENSIONS:
            score += 20
            signaux.append(f"Extension exécutable à risque : {ext}")

        # 2. Double extension (ex: doc.pdf.exe)
        parties = nom.lower().split(".")
        if len(parties) >= 3 and ("." + parties[-2] in DOCUMENT_EXTS) and ext not in DOCUMENT_EXTS:
            score += 25
            signaux.append(f"Double extension trompeuse : {nom}")

        # 3. Taille anormale
        try:
            taille = os.path.getsize(chemin)
            if ext in RISKY_EXTENSIONS and taille < 10 * 1024:
                score += 10
                signaux.append(f"Exécutable anormalement petit ({taille} octets)")
        except OSError as exc:
            signaux.append(f"Taille illisible : {exc}")

        # 4. Chaînes suspectes (lecture binaire tolérante)
        try:
            with open(chemin, "rb") as fh:
                contenu = fh.read(MAX_READ_BYTES).decode("utf-8", errors="ignore")
            for regex, points, label in _COMPILED:
                if regex.search(contenu):
                    score += points
                    signaux.append(f"Motif suspect : {label}")
        except (OSError, PermissionError) as exc:
            # Fichier verrouillé / permission refusée : on ne bloque pas le scan
            signaux.append(f"Contenu illisible (verrouillé ?) : {exc}")

        score = min(score, 100)
        if score >= 70:
            verdict = "menace"
        elif score >= 40:
            verdict = "suspect"
        else:
            verdict = "sain"

        return {"score": score, "signaux": signaux, "verdict": verdict}

    except Exception as exc:  # garde-fou : l'heuristique ne doit jamais crasher le scan
        return {"score": 0, "signaux": [f"Erreur heuristique : {exc}"], "verdict": "sain"}
