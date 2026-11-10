"""A scrap source opens the precise weapon level rather than its family's first node."""
import re,sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns
for ch in 'hyper rathalos scrap':p.frame({'kind':'key','key':44 if ch==' ' else ord(ch)-97+4})
p.frame({'kind':'key','key':40})
def click(text):
 op=next(o for o in n['drawops'] if o[0]=='text' and o[2][1]==text)
 p.frame({'kind':'click','x':op[1][0]+3,'y':op[1][1]+3})
click('Hyper Rathalos Scrap');click('Equipment')
for _ in range(20):
 label=next((o[2][1] for o in n['drawops'] if o[0]=='text' and o[2][0]==0xff65d7f2 and re.search(r' LV[2-9]$',o[2][1])),None)
 if label:break
 p.frame({'kind':'key','key':78})
assert label,n['texts'];level=label.rsplit(' LV',1)[1]
click(label)
assert 'Weapon stats' in n['texts'] and any(o[0]=='text' and o[1][2]>22 and o[2][1].endswith('Lv'+level) for o in n['drawops']),n['texts']
p.frame({'kind':'button','bits':2});assert 'Hyper Rathalos Scrap' in n['texts']
print('ARM precise weapon-level byproduct source passed')
