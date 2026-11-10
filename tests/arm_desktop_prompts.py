"""Hints follow real pointer/keyboard versus controller activity."""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns
op=next(o for o in n['drawops'] if o[0]=='text' and o[2][1]=='Items')
p.frame({'kind':'click','x':op[1][0]+3,'y':op[1][1]+3})
assert 'Enter  Select' in n['texts'],n['texts']
assert 'Esc  Back' in n['texts']
p.frame({'kind':'button','bits':1<<7})
assert 'Select' in n['texts'] and 'Enter  Select' not in n['texts']
print('ARM active-input prompts passed')
