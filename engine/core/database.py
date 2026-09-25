"""Base de signatures locale simulée.

En production, cette base serait synchronisée depuis un serveur
(Delta-update) et stockée en SQLite. Ici on simule avec un set
de SHA-256 connus + le hash EICAR standard.
"""

from __future__ import annotations

import hashlib

# Chaîne de test EICAR (standard antivirus, inoffensive)
EICAR_STRING = b"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*"
EICAR_SHA256 = hashlib.sha256(EICAR_STRING).hexdigest()

# Quelques hashs de test simulés (valeurs fictives, format valide)
_FAKE_TEST_HASHES: set[str] = {
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",  # sha256("") vide -> suspect test
    "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",  # sha256("test")
    EICAR_SHA256,
}

# Mapping hash -> nom de menace pour l'affichage
THREAT_NAMES: dict[str, str] = {
    EICAR_SHA256: "EICAR-Test-File (test inoffensif)",
    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855": "Test.Empty-File-Suspect",
    "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08": "Test.Hash-Demo",
}


def is_known_malicious(sha256: str) -> str | None:
    """Retourne le nom de la menace si le hash est connu, sinon None."""
    key = sha256.lower()
    if key in _FAKE_TEST_HASHES:
        return THREAT_NAMES.get(key, "Menace.Connue.Generique")
    return None


def threat_count() -> int:
    """Nombre de signatures chargées (pour le dashboard)."""
    return len(_FAKE_TEST_HASHES)
