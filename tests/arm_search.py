"""Search retains weapon scope and every drawn controller key accepts pointer input."""
import os,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
ns=p.ns
def press(bits):p.frame({'kind':'button','bits':bits});return ns['texts'][:]
def click(label):
 op=next(o for o in ns['drawops'] if o[0]=='text' and o[2][1]==label)
 p.frame({'kind':'click','x':op[1][0]+3,'y':op[1][1]+3})
press(1<<7);press(1<<15);press(1<<15);press(1)
assert 'Lance' in ns['texts']
press(1<<3)
assert 'Space' in ns['texts'] and 'Done' in ns['texts'],ns['texts']
click('P');click('E');click('T');click('Done')
assert 'Lance' in ns['texts'] and any('Petrified Lance' in s for s in ns['texts']),ns['texts']
assert not any('Petrified Blade' in s for s in ns['texts'])
# Searching with a physical keyboard stays inside the selected class too.
press(1<<3);click('Clear');click('Done')
for ch in 'obsidian':p.frame({'kind':'key','key':ord(ch)-97+4})
p.frame({'kind':'key','key':40})
assert any('Obsidian Lance' in s for s in ns['texts']),ns['texts']
print('ARM pointer keyboard and scoped controller/physical search passed')
