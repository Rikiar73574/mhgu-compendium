"""Adding a set preserves selected pieces and adds the rest exactly once."""
import os,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory() as d:
 path=Path(d)/'wishlist.txt';path.write_text('armor:1310721 0\n')
 os.environ['COMPENDIUM_PREVIEW_WISHLIST']=str(path)
 sys.path.insert(0,str(ROOT/'tools'));import preview_app as p
 def press(b):p.frame({'kind':'button','bits':b});return p.ns['texts'][:]
 for _ in range(4):press(1<<7)
 for ch in 'leather headgear':p.frame({'kind':'key','key':44 if ch==' ' else ord(ch)-97+4})
 p.frame({'kind':'key','key':40});press(1)
 press(1<<2)
 rows=path.read_text().splitlines()
 assert len(rows)==5,rows
 press(1<<2);assert path.read_text().splitlines()==rows
 print('ARM full armor set wishlist is additive and idempotent')
