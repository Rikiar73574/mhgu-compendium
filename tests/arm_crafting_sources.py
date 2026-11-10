"""A combining ingredient opens its own sources and back returns to the recipe."""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns
for ch in 'adamant pill':p.frame({'kind':'key','key':44 if ch==' ' else ord(ch)-97+4})
p.frame({'kind':'key','key':40});p.frame({'kind':'button','bits':1})
assert 'Adamant Pill' in n['texts'] and any('Combin' in t for t in n['texts']),n['texts']
p.frame({'kind':'button','bits':1})
assert 'Adamant Pill' not in n['texts'],n['texts']
p.frame({'kind':'button','bits':2})
assert 'Adamant Pill' in n['texts']
print('ARM recipe ingredient sources and return passed')
# A later trade page must survive the round trip too.
op=next(o for o in n['drawops'] if o[0]=='text' and o[2][1]=='Veggie Elder trades')
p.frame({'kind':'click','x':op[1][0]+3,'y':op[1][1]+3})
for _ in range(9):p.frame({'kind':'button','bits':1<<15})
before=n['texts'][:]
p.frame({'kind':'button','bits':1});assert 'Adamant Pill' not in n['texts']
p.frame({'kind':'button','bits':2});assert n['texts']==before,(before,n['texts'])
print('ARM later source page and selection restored')
