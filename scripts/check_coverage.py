"""Vérifie la couverture Dart manuelle (hors code généré) depuis un lcov.

Usage : python3 scripts/check_coverage.py app/coverage/lcov.info 60
Exclus : lib/src/rust/* (bindings FRB), lib/**/l10n/arb/* (gen-l10n).
Sortie non-zéro si sous le seuil. Utilisé par la CI (job dart-coverage).
"""
import sys


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: check_coverage.py <lcov.info> <seuil>")
        return 2
    chemin, seuil = sys.argv[1], float(sys.argv[2])
    tot_f = tot_h = 0
    cur = ""
    for ligne in open(chemin, encoding="utf-8"):
        ligne = ligne.strip().replace("\\", "/")
        if ligne.startswith("SF:"):
            cur = ligne[3:]
        elif ligne.startswith("LF:"):
            f = int(ligne[3:])
        elif ligne.startswith("LH:"):
            h = int(ligne[3:])
            if "/src/rust/" in cur or "/l10n/arb/" in cur:
                continue
            tot_f += f
            tot_h += h
    pct = 100 * tot_h / max(tot_f, 1)
    print(f"couverture Dart manuelle : {pct:.1f}% ({tot_h}/{tot_f}), seuil {seuil:.0f}%")
    return 0 if pct >= seuil else 1


if __name__ == "__main__":
    sys.exit(main())
