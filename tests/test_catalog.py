import sys,unittest,json
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
from catalog import ROOT,key,requirements
from bs4 import BeautifulSoup
class CatalogTests(unittest.TestCase):
 def test_search(self):
  self.assertEqual(key("King's Blade"),key('KINGS BLADE'))
 def test_recipe_semantics(self):
  html='<td><small><div><a href="https://mhgu.kiranico.com/item/a">Ore</a> x4</div><div><a href="https://mhgu.kiranico.com/item/category/b">Jaggi LV1</a>: 10</div><div>―</div><div><a href="https://mhgu.kiranico.com/item/c">Scrap</a> x2</div></small></td>'
  r=requirements(BeautifulSoup(html,'html.parser').td)
  self.assertEqual(len(r['ingredients']),2)
  self.assertEqual(r['ingredients'][1]['kind'],'material_points')
  self.assertEqual(r['ingredients'][1]['quantity'],10)
  self.assertEqual(r['byproducts'][0]['name'],'Scrap')
 def test_export(self):
  d=json.loads((ROOT/'build/data/catalog.json').read_text())
  self.assertEqual(d['coverage']['weapon_classes'],14)
  w=d['weapons'];thane=w['weapon/48dae']
  self.assertEqual(thane['levels'][0]['attack'],80)
  self.assertEqual(thane['recipes'][0]['price_z'],1200)
  self.assertEqual(thane['recipes'][0]['upgrade']['ingredients'][0]['quantity'],4)
  self.assertEqual(thane['recipes'][0]['upgrade']['byproducts'][0]['name'],'Ore Scrap')
  self.assertTrue(any(e['from_level']==2 and e['to']=='weapon/48dae' for e in w['weapon/4be0a']['branches']))
  for f in w.values():
   self.assertEqual(len({x['level'] for x in f['levels']}),len(f['levels']))
   for b in f['branches']:
    self.assertIn(b['to'],w)
    self.assertIn(b['from_level'],[x['level'] for x in f['levels']])
  iron=d['items']['item/5d436']
  self.assertTrue(any(x['kind']=='Map' and x['rank']=='LR' for x in iron['sources']))
if __name__=='__main__':unittest.main()
