import collections,json,re,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
class LayoutTests(unittest.TestCase):
 def test_every_area_fits_without_scrolling(self):
  areas=collections.defaultdict(lambda:collections.defaultdict(set))
  for r in json.loads((ROOT/'build/data/locations.json').read_text())['gathering']:
   kind=r['site'].split(' (')[0]
   areas[r['location_id'],r['rank'],r['area']][kind].add(r['name'])
  for key,groups in areas.items():
   self.assertLessEqual(len(groups),4,key)
   for names in groups.values():self.assertLessEqual(len(names),23,key)
 def test_tree_levels_share_lanes_and_branches_connect(self):
  source=(ROOT/'build/generated/world.rs').read_text()
  nodes=[tuple(map(int,m)) for m in re.findall(r'Node\{entry:(\d+),level:(\d+),name:.*?,attack:\d+,kind:(\d+),parent:(-?\d+),depth:(\d+),row:(\d+),rarity:\d+,sharp:',source)]
  self.assertEqual(len(nodes),10877)
  occupied=set()
  for i,(entry,level,kind,parent,depth,row) in enumerate(nodes):
   position=(kind,depth,row);self.assertNotIn(position,occupied);occupied.add(position)
   if parent>=0:
    pe,pl,pk,_,pd,pr=nodes[parent]
    self.assertLess(parent,i);self.assertEqual(pk,kind);self.assertEqual(depth,pd+1)
    if pe==entry:self.assertEqual(row,pr);self.assertEqual(level,pl+1)
    else:self.assertGreater(row,pr)
