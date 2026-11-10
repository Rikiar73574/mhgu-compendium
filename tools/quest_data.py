"""Offline quest coverage joins; source IDs are never game item IDs."""
import json,collections,os,struct,re
from pathlib import Path
from paths import DATA,GENERATED,database

# Nine exact Thane/Knight/Cavalry Lance upgrade recipes match DB Improve rows:
# weapon name/type/attack, named co-ingredients, and point quantity all agree.
# These five reviewed labels are explicit exceptions, not an LV-to-rank rule.
POINT_ALIASES={
    'Jaggi LV1':(1966118,'LR Jaggi Materials'),
    'Jaggi LV3':(1966120,'G Jaggi Materials'),
    'Ore LV1':(1966084,'LR Ore Materials'),
    'Ore LV2':(1966085,'HR Ore Materials'),
    'Ore LV3':(1966086,'G Ore Materials'),
}

def point_aliases(db,groups):
    names={name:i for i,(name,_) in enumerate(groups)};out=[]
    for alias,(key,name) in POINT_ALIASES.items():
        row=db.execute('select name from items where _id=?',(key,)).fetchone()
        if not row or row[0]!=name or name not in names:raise ValueError(f'Reviewed point alias no longer matches: {alias}')
        out.append((alias,names[name]))
    return out

def point_groups(db,item_ids):
    """Real catalog members only; shared members retain the same stock identity."""
    if len(item_ids)>=0x8000:raise ValueError('Catalog exceeds physical planner namespace')
    groups=collections.defaultdict(dict);names={};membership=collections.Counter()
    for item,name,group,label,amount in db.execute('select m.item_id,i.name,m.material_item_id,g.name,m.amount from item_to_material m left join items i on i._id=m.item_id left join items g on g._id=m.material_item_id order by m.material_item_id,m.item_id'):
        if not label or name not in item_ids or not isinstance(amount,int) or not 0<amount<=65535:
            raise ValueError(f'Incomplete point group {label}: {name} ({amount})')
        entry=item_ids[name]
        if not 0<=entry<0x8000 or entry in groups[group]:raise ValueError(f'Duplicate/invalid point member: {label}: {name}')
        names[group]=label;groups[group][entry]=amount;membership[entry]+=1
    if len(groups)>=0x7fff or len(set(names.values()))!=len(names):raise ValueError('Ambiguous/excessive point groups')
    return [(names[g],sorted(m.items(),key=lambda r:(membership[r[0]],r[0]))) for g,m in sorted(groups.items())]

def target_monsters(goal,roster,monsters):
    """Exact objective names only; incidental and unstable spawns are not targets."""
    all_large=re.fullmatch(r'(?:Hunt|Slay|Capture) all large monsters(?:, not including (.+))?',goal)
    if all_large:
        excluded=all_large[1]
        return [m for m in sorted(roster) if monsters[m][2]=='0' and monsters[m][1]!=excluded]
    if not re.match(r'^(?:Hunt|Slay|Capture|Repel)\b',goal):return []
    matches=[(m,hit.span()) for m in sorted(roster) for hit in re.finditer(r'(?<!\w)'+re.escape(monsters[m][1])+r'(?!\w)',goal)]
    return sorted({m for m,(a,b) in matches if not any(c<=a and b<=d and (a,b)!=(c,d) for _,(c,d) in matches)})

def emit(db,catalog,locations,quest_dir):
    item_ids={r['name']:i for i,r in enumerate(sorted(catalog['items'].values(),key=lambda r:r['name']))}
    locations_by_id={r['_id']:i for i,r in enumerate(locations['locations'])}
    ranks={'LR':0,'HR':1,'G':2}
    quests=[r for r in db.execute('select _id,name,location_id,rank,stars,hub,hunter_type,goal from quests order by _id') if r[2] in locations_by_id and r[3] in ranks]
    assert len(quests)<=1500
    indices={q[0]:i for i,q in enumerate(quests)}
    monster_rows=list(db.execute('select _id,name,class from monsters order by _id'))
    monsters={r[0]:i for i,r in enumerate(monster_rows)}
    assert len(monsters)<256
    stable=collections.defaultdict(set)
    for m,q,unstable in db.execute('select monster_id,quest_id,unstable from monster_to_quest'):
        if str(unstable)=='0' and m in monsters:stable[q].add(monsters[m])
    sources=set()
    for r in locations['gathering']:
        if not r['supply'] and r['name'] in item_ids:
            sources.add((item_ids[r['name']],locations_by_id[r['location_id']],ranks[r['rank']],0,r['area']))
    zones=sorted({r['area'] for r in locations['gathering']},key=lambda s:(int(''.join(filter(str.isdigit,s)) or 0),s))
    zone_ids={z:i for i,z in enumerate(zones)}
    sources={(i,e,r,k,zone_ids[z]) for i,e,r,k,z in sources}
    for name,m,rank,rate in db.execute('select i.name,h.monster_id,h.rank,h.percentage from hunting_rewards h join items i on i._id=h.item_id'):
        if name in item_ids and m in monsters and rank in ranks and rate and rate>0:
            sources.add((item_ids[name],monsters[m],ranks[rank],1,0))
    for name,q,rate in db.execute('select i.name,r.quest_id,r.percentage from quest_rewards r join items i on i._id=r.item_id'):
        if name in item_ids and q in indices and rate and rate>0:
            sources.add((item_ids[name],indices[q],ranks[quests[indices[q]][3]],2,0))
    q=lambda s:json.dumps(s,ensure_ascii=False)
    out=['// Generated local facts from MHGenDatabase; availability is checked live.',
         'use super::quest::{Quest,Source,PointGroup};','pub static QUESTS:&[Quest]=&[']
    for id,name,loc,rank,stars,hub,hunter_type,goal in quests:
        if hunter_type not in (0,1):raise ValueError(f"Unknown hunter type for quest {id}: {hunter_type}")
        roster=','.join(map(str,sorted(stable[id])))
        targets=','.join(map(str,target_monsters(goal,stable[id],monster_rows)))
        p=quest_dir/f'questData_{id:07d}.ext'
        data=p.read_bytes() if p.is_file() else b''
        entry=list(data[0x1a:0x1d]) if len(data)>=29 and data[:4]==b'\x00\x00KC' and struct.unpack_from('<I',data,12)[0]==id else [255,255,255]
        out.append(f'Quest{{entry:{entry},id:{id},name:{q(name)},loc:{locations_by_id[loc]},rank:{ranks[rank]},stars:{stars},hub:{q(hub)},prowler:{str(hunter_type==1).lower()},targets:&[{targets}],monsters:&[{roster}]}},')
    out+= ['];','pub static SOURCES:&[Source]=&[']
    out += [f'Source{{item:{i},entity:{e},rank:{r},kind:{k},zone:{z}}},' for i,e,r,k,z in sorted(sources)]
    out+= ['];','pub static MONSTERS:&[&str]=&['+','.join(q(n) for _,n,_ in monster_rows)+'];']
    out+= ['pub static POINT_GROUPS:&[PointGroup]=&[']
    groups=point_groups(db,item_ids)
    for name,members in groups:
        out.append(f'PointGroup{{name:{q(name)},members:&['+','.join(f'({i},{v})' for i,v in members)+']},')
    out+= ['];']
    out+= ['pub static POINT_ALIASES:&[(&str,u16)]=&['+','.join(f'({q(label)},{i})' for label,i in point_aliases(db,groups))+'];']
    return '\n'.join(out)+'\n'

if __name__=='__main__':
    result=emit(database(),json.loads((DATA/'catalog.json').read_text()),json.loads((DATA/'locations.json').read_text()),Path(os.environ['MHGU_GAME_DIR'])/'romfs/nativeNX/loc/quest/questData')
    (GENERATED/'quest_data.rs').write_text(result)
    print('Generated quest coverage facts')
