"""Armor type/gender filters work with pointer and controller, retaining common gear."""
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
click('Head');click('Type: All');assert 'Type: Blademaster' in n['texts']
press(1<<2);assert 'Type: Gunner' in n['texts']
click('Gender: All');assert 'Gender: Male' in n['texts']
press(1<<4);assert 'Gender: Female' in n['texts']
press(1)
assert 'Gunner' in n['texts'] or 'Both' in n['texts'],n['texts']
assert 'Female' in n['texts'] or 'Both' in n['texts'],n['texts']
print('ARM armor hunter and gender filters passed')
