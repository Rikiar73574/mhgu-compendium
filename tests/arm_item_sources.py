"""Verify item sources through the compiled ARM UI, without running the game."""
import ast,sys,tempfile,os
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
with tempfile.TemporaryDirectory(prefix='compendium-sources-') as temporary:
 os.environ['COMPENDIUM_PREVIEW_WISHLIST']=str(Path(temporary)/'wishlist')
 import preview_app as preview
 ns=preview.ns
 def press(bits):preview.frame({'kind':'button','bits':bits});return ns['texts'][:]
 def click(label):
  op=next(op for op in ns['drawops'] if op[0]=='text' and op[2][1]==label)
  preview.frame({'kind':'click','x':op[1][0]+3,'y':op[1][1]+3})
 def open_item(name):
  click('Items')
  for letter in name.lower():preview.frame({'kind':'key','key':44 if letter==' ' else ord(letter)-ord('a')+4})
  preview.frame({'kind':'key','key':40})
  click(name)
  assert 'ITEM SOURCES' in ns['texts'],ns['texts']
 def row(expected):
  for _ in range(100):
   texts=ns['texts'][:]
   if any(tuple(texts[i:i+5])==expected for i in range(len(texts))):return
   preview.frame({'kind':'key','key':78})
  raise AssertionError((expected,ns['texts']))
 open_item('Commendation');click('Quest rewards')
 row(('The Perilous Pair','Village ★6 / Reward C','LR','×1','100%'))
 # Rasterize the actual ARM draw calls for review.
 source=ROOT/'tests/arm_smoke.py'
 image=next(n for n in ast.parse(source.read_text()).body if isinstance(n,ast.FunctionDef)and n.name=='preview')
 exec(compile(ast.Module(body=[image],type_ignores=[]),str(source),'exec'),ns)
 ns['preview']('quest-item-sources.png')
 open_item('Rathalos Scale');click('Rathalos')
 row(('Body Carve','','LR','×1','40%'))
 ns['preview']('monster-item-sources.png')
 click('Quest rewards')
 row(('A King, Robed in Smoke','Village ★5 / Reward A','LR','×1','28%'))
 open_item('Hyper Monster Bone');click('Quest rewards')
 row(('Born of Darkness','Guild ★6 / Reward B','HR','×1','25%'))
print('ARM item sources passed: tickets, monster carves, quest rewards for carved items, Hyper materials, paging and compact text decoding')
