"""Local-only generated inputs; no dependency on another project checkout."""
import os, sqlite3
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
DATA=ROOT/'build/data'
GENERATED=ROOT/'build/generated'

def database():
    path=Path(os.environ['MHGU_DATABASE']).resolve()
    return sqlite3.connect(path.as_uri()+'?mode=ro',uri=True)
