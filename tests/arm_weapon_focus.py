"""A mouse-selected material retains material focus when returning from sources."""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns
def press(b):p.frame({'kind':'button','bits':b});return n['texts'][:]
press(1<<7);press(1);press(1)
op=next(o for o in n['drawops'] if o[0]=='text' and o[2][1]=='Iron Ore')
p.frame({'kind':'click','x':op[1][0]+3,'y':op[1][1]+3})
assert 'ITEM SOURCES' in n['texts']
press(2)
assert 'ITEM SOURCES' in press(1),'return lost material focus'
print('ARM mouse material return focus passed')
