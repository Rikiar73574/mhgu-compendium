from paths import ROOT,DATA,GENERATED,database
"""Generate typed gathering and weapon-tree data; never equate source IDs."""
import json,sqlite3,collections
from pathlib import Path
from PIL import Image
root=Path(__file__).resolve().parents[1]
d=json.loads((DATA/'catalog.json').read_text());records=sorted(d['items'].values(),key=lambda r:r['name'])+sorted(d['weapons'].values(),key=lambda r:(r['type'],r['name']))
ids={r['id']:i for i,r in enumerate(records)}
items=collections.defaultdict(list)
for r in d['items'].values():items[r['name'].casefold()].append(ids[r['id']])
def lit(x):return json.dumps(x,ensure_ascii=False)
c=database();c.row_factory=sqlite3.Row
locations=[dict(r) for r in c.execute('select _id,name from locations order by name')]
locids={r['_id']:i for i,r in enumerate(locations)}
ranks={'LR':0,'HR':1,'G':2}
gather=[]
for r in c.execute('select g.*,i.name from gathering g join items i on i._id=g.item_id order by location_id,rank,area,site,group_num,fixed desc,rare,percentage desc'):
 x=dict(r);x['entry']=items[x['name'].casefold()][0] if len(items[x['name'].casefold()])==1 else 65535;gather.append(x)
# Supply containers can also yield normal items (e.g. Might Seed). Classify
# the complete source group so those do not make a supply-only arena eligible.
def source_group(r):return tuple(r[k] for k in ('location_id','rank','area','site','group_num','fixed','rare'))
supply_names={'First-aid Med','First-aid Med+','Mini Whetstone','Ration','Ballista Ammo','Cannon Ammo','One-shot Binder'}
supply_groups={source_group(r) for r in gather if r['name'].startswith('EZ ') or r['name'] in supply_names}
for r in gather:r['supply']=source_group(r) in supply_groups
zones=sorted({r['area'] for r in gather},key=lambda s:(int(''.join(filter(str.isdigit,s)) or 0),s));zoneids={z:i for i,z in enumerate(zones)}
lines=['// Generated from MHGenDatabase (MIT) and Kiranico; source IDs remain separate.','pub struct Location{pub name:&\'static str}','pub struct Gather{pub loc:u8,pub zone:u8,pub rank:u8,pub name:&\'static str,pub site:&\'static str,pub supply:bool,pub qty:u16,pub rate:i16,pub fixed:bool,pub rare:bool,pub group:u16,pub entry:u16}', 'pub static LOCATIONS:&[Location]=&['+','.join(f'Location{{name:{lit(r["name"])}}}' for r in locations)+'];', 'pub static ZONES:&[&str]=&['+','.join(map(lit,zones))+'];','pub static GATHER:&[Gather]=&[']
for r in gather:lines.append(f'Gather{{loc:{locids[r["location_id"]]},zone:{zoneids[r["area"]]},rank:{ranks[r["rank"]]},name:{lit(r["name"])},site:{lit(r["site"])},supply:{str(r["supply"]).lower()},qty:{r["quantity"] or 0},rate:{r["percentage"] if r["percentage"] is not None else -1},fixed:{str(bool(r["fixed"])).lower()},rare:{str(bool(r["rare"])).lower()},group:{r["group_num"] or 0},entry:{r["entry"]}}},')
lines+= ['];','pub static QUESTS:&[(u32,u8,u8)]=&[']
for r in c.execute('select _id,location_id,rank from quests order by _id'):
 if r['location_id'] in locids and r['rank'] in ranks:lines.append(f'({r["_id"]},{locids[r["location_id"]]},{ranks[r["rank"]]}),')
lines += ['];']
classes=['Great Sword','Long Sword','Sword & Shield','Dual Blades','Hammer','Hunting Horn','Lance','Gunlance','Switch Axe','Charge Blade','Insect Glaive','Light Bowgun','Heavy Bowgun','Bow']
iconids=[0,7,1,11,2,12,3,9,8,14,13,6,4,10]
lines+=['pub static CLASSES:&[&str]=&['+','.join(map(lit,classes))+'];','pub struct Node{pub entry:u16,pub level:u8,pub name:&\'static str,pub attack:u16,pub kind:u8,pub parent:i32,pub depth:u8,pub row:u16,pub sharp:[[u8;7];3]}','pub static TREE:&[Node]=&[']
nodeparents={};nodes={}
for w in d['weapons'].values():
 for lv in w['levels']:
  k=(w['id'],lv['level']);nodes[k]=(w,lv)
  if lv['level']>1:nodeparents[k]=(w['id'],lv['level']-1)
 for b in w['branches']:
  k=(b['to'],1);assert k not in nodeparents;nodeparents[k]=(w['id'],b['from_level'])
assert all(p in nodes for p in nodeparents.values())
children=collections.defaultdict(list)
for k in nodes:children[nodeparents.get(k)].append(k)
ordered=[];seen=set()
def walk(k,parent,depth):
 assert k not in seen;seen.add(k);i=len(ordered);ordered.append((k,parent,depth))
 for child in sorted(children[k],key=lambda x:(x[0]!=k[0],nodes[x][1]['name'],x[1])):walk(child,i,depth+1)
for kind in classes:
 for k in sorted(children[None],key=lambda x:(not nodes[x][1]['name'].startswith('Petrified'),nodes[x][1]['name'])):
  if nodes[k][0]['type']==kind:walk(k,-1,0)
assert len(ordered)==len(nodes)
lanes={};next_lane=collections.defaultdict(int)
for k,parent,depth in ordered:
 w,lv=nodes[k]
 if w['id'] not in lanes:lanes[w['id']]=next_lane[w['type']];next_lane[w['type']]+=1
 sharp=[[part['width_px'] for part in row] for row in lv.get('sharpness_display',[])][:3]
 sharp=[(row+[0]*7)[:7] for row in sharp]
 sharp=(sharp+[[0]*7]*3)[:3]
 lines.append(f'Node{{entry:{ids[w["id"]]},level:{lv["level"]},name:{lit(lv["name"])},attack:{lv["attack"]},kind:{classes.index(w["type"])},parent:{parent},depth:{depth},row:{lanes[w["id"]]},sharp:{str(sharp)}}},')
lines+= ['];','pub static ICONS:&[&[u32]]=&[']
for icon in iconids:
 im=Image.open(root/f'assets/classic/{icon}.png').convert('RGBA').resize((24,24))
 pixels=[r|(g<<8)|(b<<16)|(a<<24) for r,g,b,a in im.get_flattened_data()]
 lines.append('&['+','.join(map(str,pixels))+'],')
lines+= ['];']
(GENERATED/'world.rs').write_text('\n'.join(lines)+'\n')
(DATA/'locations.json').write_text(json.dumps({'source':'MHGenDatabase','locations':locations,'gathering':gather},ensure_ascii=False,separators=(',',':')))
print(f'{len(locations)} maps, {len(gather)} sources, {len(ordered)} connected level nodes; {sum(r["entry"]==65535 for r in gather)} sources without exact item links')

from weapon_data import emit
emit(root,ordered,nodes,ids,c)
