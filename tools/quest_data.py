"""Offline quest coverage joins; source IDs are never game item IDs."""
import json,collections,os,struct
from pathlib import Path
from paths import DATA,GENERATED,database

def emit(db,catalog,locations,quest_dir):
    item_ids={r['name']:i for i,r in enumerate(sorted(catalog['items'].values(),key=lambda r:r['name']))}
    locations_by_id={r['_id']:i for i,r in enumerate(locations['locations'])}
    ranks={'LR':0,'HR':1,'G':2}
    quests=[r for r in db.execute('select _id,name,location_id,rank,stars,hub from quests order by _id') if r[2] in locations_by_id and r[3] in ranks]
    assert len(quests)<=1500
    indices={q[0]:i for i,q in enumerate(quests)}
    monster_rows=list(db.execute('select _id,name from monsters order by _id'))
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
         'use super::quest::{Quest,Source};','pub static QUESTS:&[Quest]=&[']
    for id,name,loc,rank,stars,hub in quests:
        roster=','.join(map(str,sorted(stable[id])))
        p=quest_dir/f'questData_{id:07d}.ext'
        data=p.read_bytes() if p.is_file() else b''
        entry=list(data[0x1a:0x1d]) if len(data)>=29 and data[:4]==b'\x00\x00KC' and struct.unpack_from('<I',data,12)[0]==id else [255,255,255]
        out.append(f'Quest{{entry:{entry},id:{id},name:{q(name)},loc:{locations_by_id[loc]},rank:{ranks[rank]},stars:{stars},hub:{q(hub)},monsters:&[{roster}]}},')
    out+= ['];','pub static SOURCES:&[Source]=&[']
    out += [f'Source{{item:{i},entity:{e},rank:{r},kind:{k},zone:{z}}},' for i,e,r,k,z in sorted(sources)]
    out+= ['];','pub static MONSTERS:&[&str]=&['+','.join(q(n) for _,n in monster_rows)+'];']
    return '\n'.join(out)+'\n'

if __name__=='__main__':
    result=emit(database(),json.loads((DATA/'catalog.json').read_text()),json.loads((DATA/'locations.json').read_text()),Path(os.environ['MHGU_GAME_DIR'])/'romfs/nativeNX/loc/quest/questData')
    (GENERATED/'quest_data.rs').write_text(result)
    print('Generated quest coverage facts')
