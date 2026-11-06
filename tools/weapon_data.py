from paths import ROOT,DATA,GENERATED,database
"""Emit per-level stats and separate, linked craft/upgrade recipes."""
import collections,json,re,zlib

def emit(root,ordered,nodes,ids,c):
 rows=collections.defaultdict(list)
 for r in c.execute('select w.*,i.name,i.rarity from weapons w join items i on i._id=w._id'):
  rows[r['name'],r['wtype'],r['attack']].append(dict(r))
 parts=collections.defaultdict(list)
 for r in c.execute('select c.*,i.name,i.type item_type from components c join items i on i._id=c.component_item_id'):
  parts[r['created_item_id']].append(dict(r))
 catalog=json.loads((DATA/'catalog.json').read_text())
 item_ids={r['name']:ids[r['id']] for r in catalog['items'].values()}
 lit=lambda s:json.dumps(str(s),ensure_ascii=False)
 out=['// Generated from exact Kiranico levels and MHGenDatabase matches.',
 'pub struct Material{pub entry:u16,pub name:&\'static str,pub quantity:u16,pub points:bool,pub key:bool}',
 'pub struct Recipe{pub label:&\'static str,pub price:&\'static str,pub items:&\'static [Material]}',
 'pub struct Weapon{pub key:u32,pub stats:&\'static [(&\'static str,&\'static str)],pub recipes:&\'static [Recipe]}',
 'pub static WEAPONS:&[Weapon]=&[']
 for k,_,_ in ordered:
  family,lv=nodes[k];candidates=rows[f"{lv['name']} {lv['level']}",family['type'],lv['attack']]
  row=candidates[0] if len(candidates)==1 else None
  stats=[];recipes=[]
  if row:
   stats=[('Affinity',str(row['affinity'] or 0)+'%'),('Slots',str(row['num_slots'] or 0)),('Rarity',str(row['rarity']))]
   for e,a in [('element','element_attack'),('element_2','element_2_attack')]:
    if row[e] and row[a]:stats.append((row[e],str(row[a])))
   if row['defense']:stats.append(('Defense',f"+{row['defense']}"))
   for label,key in [('Shelling','shelling_type'),('Phial','phial'),('Recoil','recoil'),('Reload','reload_speed'),('Deviation','deviation'),('Charges','charges'),('Coatings','coatings'),('Notes','horn_notes')]:
    if row.get(key):stats.append((label,str(row[key])))
  else:
   # Preserve source facts when an exact database-level match is unavailable.
   rare=next((s for s in lv['stats_text'] if s.startswith('Rare ')),None)
   if rare:stats.append(('Rarity',rare[5:]))
   for i,s in enumerate(lv['stats_text']):
    if s and not s.startswith('Rare '):stats.append(('Stats' if i<2 else 'Slots',s))
  recipe=next((r for r in family.get('recipes',[]) if r['level']==lv['level']),None)
  if recipe:
   for mode,label in [('create','Forge'),('upgrade','Upgrade')]:
    ingredients=recipe[mode]['ingredients']
    if ingredients:recipes.append((label,f"{recipe['price_z']}z",[(ids.get(i['id'],65535),i['name'],i['quantity'],i['kind']=='material_points',False) for i in ingredients]))
  elif row:
   for mode,label in [('Create','Forge'),('Create B','Forge alternative'),('Improve','Upgrade')]:
    ingredients=[i for i in parts[row['_id']] if i['type']==mode and i['item_type']!='Weapon']
    cost=row['upgrade_cost'] if mode=='Improve' else row['creation_cost']
    if ingredients:recipes.append((label,f'{cost}z' if cost is not None else '',[(item_ids.get(i['name'],65535),i['name'],i['quantity'],i['item_type']=='Materials',bool(i['key'])) for i in ingredients]))
  rs=[]
  for label,price,ingredients in recipes:
   ms=[f'Material{{entry:{entry},name:{lit(name)},quantity:{qty},points:{str(points).lower()},key:{str(key).lower()}}}' for entry,name,qty,points,key in ingredients]
   rs.append(f'Recipe{{label:{lit(label)},price:{lit(price)},items:&['+','.join(ms)+']}')
  out.append('Weapon{key:'+str(zlib.crc32(f'{k[0]}:{k[1]}'.encode()))+',stats:&['+','.join('('+lit(a)+','+lit(b)+')' for a,b in stats)+'],recipes:&['+','.join(rs)+']},')
 out.append('];')
 game=json.loads((DATA/'game-weapons.json').read_text())['weapons']
 classes={0:'Great Sword',1:'Sword & Shield',2:'Hammer',3:'Lance',4:'Heavy Bowgun',6:'Light Bowgun',7:'Long Sword',8:'Switch Axe',9:'Gunlance',10:'Bow',11:'Dual Blades',12:'Hunting Horn',13:'Insect Glaive',14:'Charge Blade'}
 mapping=[]
 for i,(k,_,_) in enumerate(ordered):
  family,lv=nodes[k]
  for kind,cls in classes.items():
   if family['type']!=cls:continue
   matched=[int(id) for id,name in game[str(kind)].items() if name==family['name']]
   if len(matched)==1:mapping.append((kind,matched[0],lv['level'],i))
 out.append('pub static GAME_WEAPONS:&[(u16,u16,u8,usize)]=&['+','.join('('+','.join(map(str,row))+')' for row in mapping)+'];')
 # Game IDs are independent from catalog/Kiranico IDs. Only unique exact names map.
 game_items=collections.defaultdict(list)
 for key,name in json.loads((DATA/'game-items.json').read_text()).items():
  if 0<int(key)<4096:game_items[name].append(int(key))
 item_mapping=sorted((entry,game_items[name][0]) for name,entry in item_ids.items() if len(game_items[name])==1)
 out.append('pub static GAME_ITEMS:&[(u16,u16)]=&['+','.join(f'({entry},{game})' for entry,game in item_mapping)+'];')
 (GENERATED/'weapon_data.rs').write_text('\n'.join(out))
 print('Structured weapon details:',len(ordered),'levels')
