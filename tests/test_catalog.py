import sys,unittest,json
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
from catalog import ROOT,key,requirements,monster_sources,quest_sources
from bs4 import BeautifulSoup
class CatalogTests(unittest.TestCase):
 def test_quest_rewards_include_tickets_and_items_that_also_have_carves(self):
  import sqlite3
  c=sqlite3.connect(':memory:')
  c.executescript("create table items(_id,name,icon_name);create table hunting_rewards(item_id);create table quests(_id,name,hub,stars,rank);create table quest_rewards(quest_id,item_id,reward_slot,stack_size,percentage);insert into items values(1,'Commendation','icon_ticket'),(2,'Rathalos Scale','icon_scale'),(3,'Honey','icon_liquid');insert into hunting_rewards values(2);insert into quests values(7,'The Perilous Pair','Village',6,'LR'),(8,'G Hunt','Guild',3,'G');insert into quest_rewards values(7,1,'C',1,100),(7,2,'A',1,28),(8,3,'Sub',2,15);")
  result=quest_sources(c)
  self.assertEqual(result['Commendation'],[('The Perilous Pair','Village ★6 / Reward C','LR',1,100)])
  self.assertEqual(result['Rathalos Scale'],[('The Perilous Pair','Village ★6 / Reward A','LR',1,28)])
  self.assertEqual(result['Honey'],[('G Hunt','Guild ★3 / Reward Sub','GR',2,15)])
  c.close()
 def test_monster_rewards_keep_rank_condition_and_quantity(self):
  import sqlite3
  c=sqlite3.connect(':memory:')
  c.executescript("create table items(_id,name);create table monsters(_id,name);create table hunting_rewards(item_id,monster_id,condition,rank,stack_size,percentage);insert into items values(42,'Scale');insert into monsters values(7,'Rathalos');insert into hunting_rewards values(42,7,'Body Carve','LR',1,32),(42,7,'Capture','G',2,15);")
  self.assertCountEqual(monster_sources(c)['Scale'],[('Rathalos','Body Carve','LR',1,32),('Rathalos','Capture','GR',2,15)])
  self.assertEqual(monster_sources(c)['Unknown'],[])
  c.close()
 def test_hyper_materials_keep_rank_and_reward_slot(self):
  import sqlite3
  c=sqlite3.connect(':memory:')
  c.executescript("create table items(_id,name,icon_name);create table hunting_rewards(item_id);create table quests(_id,name,hub,stars,rank);create table quest_rewards(quest_id,item_id,reward_slot,stack_size,percentage);insert into items values(1,'Hyper Monster Bone','icon_bone'),(2,'Already Carved','icon_scale');insert into hunting_rewards values(2);insert into quests values(7,'Hyper Hunt','Hub',7,'G');insert into quest_rewards values(7,1,'A',2,25),(7,2,'B',1,10);")
  result=quest_sources(c)
  self.assertEqual(result['Hyper Monster Bone'],[('Hyper Hunt','Hub ★7 / Reward A','GR',2,25)])
  self.assertEqual(result['Already Carved'],[('Hyper Hunt','Hub ★7 / Reward B','GR',1,10)])
  c.close()
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
