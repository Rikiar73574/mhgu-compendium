"""Progression ordering retains edges while rarity orders independent families."""
import re,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
class ProgressionTest(unittest.TestCase):
 def test_tree_has_rarity_and_root_order(self):
  data=(ROOT/'build/generated/world.rs').read_text()
  rows=re.findall(r'Node\{entry:(\d+),level:(\d+),name:("(?:[^"\\]|\\.)*"),attack:\d+,kind:(\d+),parent:(-?\d+),depth:\d+,row:\d+,rarity:(\d+)',data)
  self.assertEqual(len(rows),10877,'every upgrade must retain its source rarity')
  last={}
  for entry,level,name,kind,parent,rarity in rows:
   if parent=='-1':
    self.assertGreaterEqual(int(rarity),last.get(kind,0));last[kind]=int(rarity)
