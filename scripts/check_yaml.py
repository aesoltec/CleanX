"""Détecte les clés dupliquées dans les workflows (yaml.safe_load les ignore)."""
import sys


class UniqueLoader(__import__("yaml").SafeLoader):
    pass


def _duplique(loader, node, deep=False):
    mapping = {}
    for k, v in node.value:
        key = loader.construct_object(k, deep=True)
        if key in mapping:
            raise ValueError(f"clé dupliquée : {key!r} (ligne {k.start_mark.line + 1})")
        mapping[key] = loader.construct_object(v, deep=True)
    return mapping


UniqueLoader.add_constructor(
    __import__("yaml").resolver.BaseResolver.DEFAULT_MAPPING_TAG, _duplique
)

ok = True
for f in sys.argv[1:]:
    try:
        __import__("yaml").load(open(f, encoding="utf-8"), Loader=UniqueLoader)
        print(f"OK : {f}")
    except ValueError as e:
        ok = False
        print(f"KO : {f} -> {e}")
sys.exit(0 if ok else 1)
