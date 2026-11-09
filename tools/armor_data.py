"""Compact armor facts; all IDs stay in their source namespace."""
import collections,json,sqlite3,os
from paths import DATA,GENERATED,database

def emit(db,catalog):
    db.row_factory=sqlite3.Row
    native=json.loads((DATA/'game-armor.json').read_text())
    ids={r['name']:i for i,r in enumerate(sorted(catalog['items'].values(),key=lambda r:r['name']))}
    parts=collections.defaultdict(list)
    for r in db.execute('select c.*,i.name,i.type item_type from components c join items i on i._id=c.component_item_id'):
        if r['type']=='Create':parts[r['created_item_id']].append(r)
    skills=collections.defaultdict(list)
    for r in db.execute('select s.item_id,t.name,s.point_value from item_to_skill_tree s join skill_trees t on t._id=s.skill_tree_id'):
        skills[r['item_id']].append((r['name'],r['point_value']))
    rows=list(db.execute('select a.*,i.name,i.rarity from armor a join items i using(_id) order by i.rarity,i.name,a._id'))
    families=collections.defaultdict(list)
    for a in rows:families[a['family']].append(a)
    names=sorted({m['name'] for a in rows for m in parts[a['_id']]});ni={n:i for i,n in enumerate(names)}
    skill_names=sorted({n for a in rows for n,_ in skills[a['_id']]});si={n:i for i,n in enumerate(skill_names)}
    q=lambda s:json.dumps(str(s),ensure_ascii=False)
    slots=['Head','Body','Arms','Waist','Legs'];materials=[];points=[]
    leaders={min(group,key=lambda a:slots.index(a['slot']))['_id'] for group in families.values()}
    out=['// Generated from MHGenDatabase; IDs are not game equipment IDs.',
         "pub struct Armor{pub key:u32,pub name:&'static str,pub family:u16,pub native:u16,pub slot:u8,pub rarity:u8,pub leader:bool,pub defense:u16,pub max_defense:u16,pub slots:u8,pub hunter:u8,pub gender:u8,pub resist:[i8;5],pub material:u16,pub material_count:u8,pub skill:u16,pub skill_count:u8}",
         'pub static ARMOR:&[Armor]=&[']
    for a in rows:
        ms=len(materials);ss=len(points)
        for m in parts[a['_id']]:materials.append((ids.get(m['name'],65535),ni[m['name']],m['quantity'],int(m['item_type']=='Materials')|int(bool(m['key']))<<1))
        points += [(si[n],v) for n,v in skills[a['_id']]]
        slot=slots.index(a['slot']);game_id=native[str(slot)].get(a['name'],65535)
        gender=1-a['gender'] if a['gender']<2 else 2 # Database female=0, male=1; native male=0, female=1.
        out.append(f"Armor{{key:{a['_id']},name:{q(a['name'])},family:{a['family']},native:{game_id},slot:{slot},rarity:{a['rarity']},leader:{str(a['_id'] in leaders).lower()},defense:{a['defense']},max_defense:{a['max_defense']},slots:{a['num_slots']},hunter:{a['hunter_type']},gender:{gender},resist:{[a[e+'_res'] for e in ['fire','water','thunder','ice','dragon']]},material:{ms},material_count:{len(materials)-ms},skill:{ss},skill_count:{len(points)-ss}}},")
    assert len(materials)<65536 and len(points)<65536 and len(skill_names)<256
    out+= ['];',"pub static SET_NAMES:&[(u16,&str)]=&["+','.join(f'({family},{q(os.path.commonprefix([a["name"] for a in group]).strip() or group[0]["name"])})' for family,group in sorted(families.items()))+'];',"pub static MATERIAL_NAMES:&[&str]=&["+','.join(map(q,names))+'];',
           'pub static MATERIALS:&[(u16,u16,u16,u8)]=&['+','.join(str(m).replace(' ','') for m in materials)+'];',
           'pub static SKILL_NAMES:&[&str]=&['+','.join(map(q,skill_names))+'];',
           'pub static SKILLS:&[(u8,i8)]=&['+','.join(str(m).replace(' ','') for m in points)+'];']
    return '\n'.join(out)+'\n'

if __name__=='__main__':
    (GENERATED/'armor_data.rs').write_text(emit(database(),json.loads((DATA/'catalog.json').read_text())))
    print('Generated compact armor data')
