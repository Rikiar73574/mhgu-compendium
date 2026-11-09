"""Set browsing and rank filters share the same armor detail/wishlist flow."""
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns
def press(b):p.frame({'kind':'button','bits':b})
def click(label):
 op=next(o for o in n['drawops'] if o[0]=='text' and o[2][1]==label)
 p.frame({'kind':'click','x':op[1][0]+3,'y':op[1][1]+3})
for _ in range(4):press(1<<7)
assert 'Armor sets' in n['texts'],n['texts']
click('Armor sets');click('LR')
assert all(not s.startswith(('HR  R','GR  R')) for s in n['texts']),n['texts']
press(1)
assert 'Full set preview' in n['texts'] and 'Y  Add full set to wishlist' in n['texts'],n['texts']
press(2);press(2)
assert 'Armor sets' in n['texts'],'Back from a set list must return to armor categories'
print('ARM armor sets, rank filtering and direct set details passed')
