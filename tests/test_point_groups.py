"""Point groups preserve physical identity and reject incomplete input."""
import sqlite3,sys,unittest,subprocess,tempfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
import quest_data
class PointGroups(unittest.TestCase):
 def db(self):
  d=sqlite3.connect(':memory:');self.addCleanup(d.close);d.executescript('CREATE TABLE items(_id INTEGER,name TEXT);CREATE TABLE item_to_material(item_id INTEGER,material_item_id INTEGER,amount INTEGER);INSERT INTO items VALUES(10,"Shell"),(11,"Bone"),(100,"A Materials"),(101,"B Materials");INSERT INTO item_to_material VALUES(10,100,4),(11,100,2),(10,101,4);');return d
 def test_shared_member_is_same_catalog_item_in_both_groups(self):
  self.assertTrue(hasattr(quest_data,'point_groups'),'point group generator is not implemented')
  groups=quest_data.point_groups(self.db(),{'Shell':0,'Bone':1})
  self.assertEqual(groups,[('A Materials',[(1,2),(0,4)]),('B Materials',[(0,4)])])
 def test_missing_member_and_nonpositive_points_fail_closed(self):
  self.assertTrue(hasattr(quest_data,'point_groups'),'point group generator is not implemented')
  with self.assertRaises(ValueError):quest_data.point_groups(self.db(),{'Shell':0})
  d=self.db();d.execute('update item_to_material set amount=0 where item_id=10')
  with self.assertRaises(ValueError):quest_data.point_groups(d,{'Shell':0,'Bone':1})
 def test_dangling_database_member_is_not_silently_dropped(self):
  d=self.db();d.execute('INSERT INTO item_to_material VALUES(999,100,2)')
  with self.assertRaises(ValueError):quest_data.point_groups(d,{'Shell':0,'Bone':1})
 def test_only_reviewed_exact_recipe_aliases_are_emitted(self):
  self.assertTrue(hasattr(quest_data,'point_aliases'),'reviewed aliases are not implemented')
  d=self.db();names=['LR Jaggi Materials','G Jaggi Materials','LR Ore Materials','HR Ore Materials','G Ore Materials'];keys=[1966118,1966120,1966084,1966085,1966086]
  d.executemany('insert into items values(?,?)',zip(keys,names));groups=[(n,[]) for n in names]
  self.assertEqual(dict(quest_data.point_aliases(d,groups)),{'Jaggi LV1':0,'Jaggi LV3':1,'Ore LV1':2,'Ore LV2':3,'Ore LV3':4})
  d.execute('update items set name="Unrelated" where _id=1966118')
  with self.assertRaises(ValueError):quest_data.point_aliases(d,groups)
 def test_generated_rust_group_tables_type_check(self):
  d=self.db();names=['LR Jaggi Materials','G Jaggi Materials','LR Ore Materials','HR Ore Materials','G Ore Materials'];keys=[1966118,1966120,1966084,1966085,1966086]
  d.executemany('insert into items values(?,?)',zip(keys,names));d.executemany('insert into item_to_material values(10,?,4)',[(k,) for k in keys])
  d.executescript('CREATE TABLE quests(_id,name,location_id,rank,stars,hub,hunter_type,goal);CREATE TABLE monsters(_id,name,class);CREATE TABLE monster_to_quest(monster_id,quest_id,unstable);CREATE TABLE hunting_rewards(item_id,monster_id,rank,percentage);CREATE TABLE quest_rewards(item_id,quest_id,percentage);')
  with tempfile.TemporaryDirectory() as t:
   p=Path(t);generated=quest_data.emit(d,{'items':{'a':{'name':'Shell'},'b':{'name':'Bone'}}},{'locations':[],'gathering':[]},p)
   types='mod quest{pub struct Quest{pub entry:[u8;3],pub id:u32,pub name:&\'static str,pub loc:u8,pub rank:u8,pub stars:u8,pub hub:&\'static str,pub prowler:bool,pub targets:&\'static[u8],pub monsters:&\'static[u8]}pub struct Source{pub item:u16,pub entity:u16,pub rank:u8,pub kind:u8,pub zone:u8}pub struct PointGroup{pub name:&\'static str,pub members:&\'static[(u16,u16)]}}'
   (p/'groups.rs').write_text(types+'mod data{'+generated+'}')
   result=subprocess.run(['rustc','+nightly','--edition','2024','--crate-type','lib','--emit','metadata',str(p/'groups.rs'),'-o',str(p/'groups.rmeta')],capture_output=True,text=True)
   self.assertEqual(result.returncode,0,result.stderr)
if __name__=='__main__':unittest.main()
