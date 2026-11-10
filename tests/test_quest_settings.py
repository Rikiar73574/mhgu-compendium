"""Quest flags and exact objectives must not confuse roaming monsters with targets."""
import re,sys,unittest,json,tempfile,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import quest_data
class QuestSettings(unittest.TestCase):
 def test_delivery_monster_is_incidental(self):
  self.assertEqual(quest_data.target_monsters('Deliver 3 Popo Tongues',{0},[(83,'Gammoth','0')]),[])
  self.assertEqual(quest_data.target_monsters('Deliver a Gammoth Pelt',{0},[(83,'Gammoth','0')]),[])
 def test_exact_hunt_and_multi_target_objectives(self):
  monsters=[(83,'Gammoth','0'),(10,'Seltas','0'),(4110,'Popo','1')]
  self.assertEqual(quest_data.target_monsters('Hunt a Gammoth',{0,1},monsters),[0])
  self.assertEqual(quest_data.target_monsters('Hunt all large monsters, not including Seltas',{0,1,2},monsters),[0])
 def test_subspecies_name_does_not_include_base_monster(self):
  self.assertEqual(quest_data.target_monsters('Hunt a Dreadqueen Rathian',{0,1},[(1,'Rathian','0'),(2,'Dreadqueen Rathian','0')]),[1])
 def test_generated_flags_keep_quest_identity(self):
  rows=(ROOT/'build/generated/quest_data.rs').read_text().splitlines()
  prowler=next(l for l in rows if re.search(r'\bid:100303,',l));self.assertIn('prowler:true',prowler)
  popo=next(l for l in rows if re.search(r'\bid:324,',l));self.assertIn('prowler:false',popo);self.assertIn('targets:&[]',popo)
  hunt=next(l for l in rows if re.search(r'\bid:508,',l));self.assertRegex(hunt,r'targets:&\[\d+\]')
 def test_real_gammoth_pelt_route_prefers_unlocked_target_hunt(self):
  items=json.loads((ROOT/'build/data/catalog.json').read_text())['items'].values()
  item=next(i for i,r in enumerate(sorted(items,key=lambda r:r['name'])) if r['name']=='Gammoth Pelt')
  source=f'''#![allow(dead_code)]
#[path={json.dumps(str(ROOT/'mod/src/quest.rs'))}]mod quest;
mod data{{include!({json.dumps(str(ROOT/'build/generated/quest_data.rs'))});}}
#[test]fn gammoth_pelt(){{
 let mut allowed=vec![false;data::QUESTS.len()];
 let popo=data::QUESTS.iter().position(|q|q.id==324).unwrap();
 let hunt=data::QUESTS.iter().position(|q|q.id==508).unwrap();
 allowed[popo]=true;allowed[hunt]=true;
 let needs=[quest::Need{{item:{item},quantity:3}}];
 assert_eq!(quest::best_groups(data::QUESTS,data::SOURCES,&needs,&allowed,&[],data::POINT_GROUPS,false).unwrap().0,hunt);
 allowed[hunt]=false;
 assert_eq!(quest::best_groups(data::QUESTS,data::SOURCES,&needs,&allowed,&[],data::POINT_GROUPS,false).unwrap().0,popo);
}}
'''
  with tempfile.TemporaryDirectory() as d:
   p=Path(d);(p/'real.rs').write_text(source)
   result=subprocess.run(['rustc','+nightly','--edition','2024','--test',str(p/'real.rs'),'-o',str(p/'real')],capture_output=True,text=True)
   self.assertEqual(result.returncode,0,result.stderr)
   result=subprocess.run([str(p/'real')],capture_output=True,text=True)
   self.assertEqual(result.returncode,0,result.stdout+result.stderr)
if __name__=='__main__':unittest.main()
