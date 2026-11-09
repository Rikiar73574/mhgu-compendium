#!/usr/bin/env python3
"""Offline-first, cached Kiranico adapter. No game process or saves are accessed."""
import argparse, json, re, time, unicodedata
from pathlib import Path
from urllib.parse import urlparse
import requests
from bs4 import BeautifulSoup
from paths import ROOT,DATA
DATA.mkdir(parents=True,exist_ok=True)
BASE='https://mhgu.kiranico.com'
def text(node): return ' '.join(node.get_text(' ',strip=True).split())
def key(s): return ''.join(c for c in unicodedata.normalize('NFKD',s.casefold()) if c.isalnum())
def identity(url): return urlparse(url).path.strip('/')
def fetch(path,offline=False):
    if path.startswith(BASE): path=path[len(BASE):]
    if not path.startswith('/') or '..' in path: raise ValueError(path)
    cache=DATA/'cache'/((path.strip('/') or 'index').replace('/','_')+'.html')
    if not cache.exists():
        if offline: raise FileNotFoundError(cache)
        time.sleep(.35)
        r=requests.get(BASE+path,timeout=40);r.raise_for_status()
        cache.parent.mkdir(parents=True,exist_ok=True);cache.write_text(r.text)
    return BeautifulSoup(cache.read_text(),'html.parser')
def item_index(soup):
    items={}
    for a in soup.select('a[href]'):
        if re.fullmatch(r'https://mhgu.kiranico.com/item/[a-z0-9]+',a['href']):
            name=text(a)
            if name: items[identity(a['href'])]={'id':identity(a['href']),'name':name,'search':key(name),'source_url':a['href'],'details_loaded':False}
    return items

def weapon_index(soup,kind):
    families={};current=None
    for row in soup.select('table.table-bordered tr'):
        cells=row.find_all('td',recursive=False)
        if not cells:continue
        branch=cells[0] if cells[0].has_attr('rowspan') else None
        if branch:
            a=branch.find('a',href=True)
            if not a:continue
            current={'id':identity(a['href']),'name':text(a),'type':kind,'source_url':a['href'],'levels':[],'branches':[],'details_loaded':False}
            families[current['id']]=current
            for div in branch.find_all('div',recursive=False)[1:]:
                a=div.find('a',href=True);lv=re.search(r'LV(\d+)',text(div))
                if a and lv:current['branches'].append({'from_level':int(lv[1]),'to':identity(a['href'])})
            cells=cells[1:]
        if not current or not cells:continue
        lv=re.search(r' LV(\d+)$',text(cells[0]))
        if not lv:continue
        current['levels'].append({'level':int(lv[1]),'name':text(cells[0]).rsplit(' LV',1)[0], 'attack':int(text(cells[1])),'stats_text':[text(c) for c in cells[2:]],'sharpness_display':[[{'color':sp.get('class',[''])[0],'width_px':int(re.search(r'width:(\d+)px',sp.get('style',''))[1])} for sp in bar.select('span')] for bar in row.select('.sharpness-bar')]})
    for f in families.values():f['search']=key(f['name']+' '+ ' '.join(l['name'] for l in f['levels']))
    return families

def requirements(cell):
    ingredients=[];byproducts=[];after_separator=False
    for div in cell.select('small > div'):
        value=text(div)
        if value=='―':after_separator=True;continue
        a=div.find('a',href=True)
        if not a:continue
        count=re.search(r'(?:x|:)\s*(\d+)',value)
        if not count:raise ValueError('Unparsed requirement: '+value)
        row={'id':identity(a['href']),'name':text(a),'quantity':int(count[1]),'kind':'material_points' if '/category/' in a['href'] else 'item'}
        (byproducts if after_separator else ingredients).append(row)
    return {'ingredients':ingredients,'byproducts':byproducts}

def weapon_details(soup):
    recipes=[]
    for table in soup.select('table'):
        if [text(t) for t in table.select('thead th')]!=['','Price','Create','Upgrade']:continue
        for tr in table.select('tr'):
            cells=tr.find_all('td',recursive=False)
            if len(cells)!=4:continue
            # The unlinked div identifies this level; linked divs are parents/branches.
            target=next((text(d) for d in cells[0].find_all('div',recursive=False) if not d.find('a') and re.search(r' LV\d+$',text(d))),None)
            if not target:raise ValueError('Missing recipe level')
            recipes.append({'level':int(re.search(r' LV(\d+)$',target)[1]),'price_z':int(re.sub(r'\D','',text(cells[1]))),'create':requirements(cells[2]),'upgrade':requirements(cells[3])})
    if not recipes:raise ValueError('No recipes found')
    return recipes

def item_details(soup):
    sources=[]
    for table in soup.select('table'):
        if table.find('ais-pagination'):continue
        heading=table.find_previous(['h2','h3','h4','h5'])
        section=text(heading) if heading else ''
        if section not in ['Quest','Monster','Map','Hunters for Hire','Meownster Hunters']:continue
        for tr in table.select('tr'):
            cells=tr.find_all('td',recursive=False)
            if not cells:continue
            fields=[text(c) for c in cells]
            sources.append({'kind':section,'fields':fields,'rank':{'Low Rank':'LR','High Rank':'HR','G Rank':'GR'}.get(fields[0]),'links':[{'id':identity(a['href']),'name':text(a),'url':a['href']} for a in tr.select('a[href]')]})
    return sources

def monster_sources(connection):
    """Use database item identities, retaining each condition and rank separately."""
    from collections import defaultdict
    result=defaultdict(list)
    for name,monster,condition,rank,qty,rate in connection.execute(
            'select i.name,m.name,r.condition,r.rank,r.stack_size,r.percentage '
            'from hunting_rewards r join items i on i._id=r.item_id '
            'join monsters m on m._id=r.monster_id order by m.name,r.rank,r.condition'):
        result[name].append((monster,condition,'GR' if rank=='G' else rank,qty,rate))
    return result

def quest_sources(connection):
    """All quest rewards, even when an item also has gathering or monster sources."""
    from collections import defaultdict
    result=defaultdict(list)
    sql=('select i.name,q.name,q.hub,q.stars,q.rank,r.reward_slot,r.stack_size,r.percentage '
         'from quest_rewards r join quests q on q._id=r.quest_id join items i on i._id=r.item_id '
         'order by q.name,q.hub,q.stars,r.reward_slot')
    for item,quest,hub,stars,rank,slot,qty,rate in connection.execute(sql):
        result[item].append((quest,f'{hub} ★{stars} / Reward {slot}','GR' if rank=='G' else rank,qty,rate))
    return result

def build(offline=False):
    items=item_index(fetch('/item',offline));weapons={}
    index=fetch('/item',offline)
    # Read authoritative class URLs rather than guessing their spelling.
    links={text(a):a['href'] for a in index.select('a[href]') if text(a) in ['Great Sword','Long Sword','Sword & Shield','Dual Blades','Hammer','Hunting Horn','Lance','Gunlance','Switch Axe','Charge Blade','Insect Glaive','Light Bowgun','Heavy Bowgun','Bow']}
    assert len(links)==14
    for name,url in links.items():
        parsed=weapon_index(fetch(url,offline),name)
        if not parsed:raise ValueError('No weapons: '+name)
        weapons.update(parsed)
    # Seed a complete recipe example and its concrete materials for the first UI pass.
    f=weapons['weapon/48dae'];f['recipes']=weapon_details(fetch(f['source_url'],offline));f['details_loaded']=True
    seeds={'Iron Ore','Earth Crystal','Jaggi Scale','Machalite Ore','Bealite Ore','Potion','Honey','Great Maccao Scale'}
    material_ids={i['id'] for r in f['recipes'] for mode in ['create','upgrade'] for i in r[mode]['ingredients'] if i['kind']=='item'}
    for i in items.values():
        if i['name'] in seeds or i['id'] in material_ids:
            i['sources']=item_details(fetch(i['source_url'],offline));i['details_loaded']=True
    missing=[b['to'] for f in weapons.values() for b in f['branches'] if b['to'] not in weapons]
    if missing:raise ValueError('Dangling weapon branches: '+str(missing[:10]))
    result={'schema_version':1,'source':'Kiranico MHGU','items':items,'weapons':weapons,'coverage':{'weapon_classes':len(links),'items':len(items),'weapon_families':len(weapons),'weapon_levels':sum(len(f['levels']) for f in weapons.values()),'item_details':sum(i['details_loaded'] for i in items.values()),'weapon_recipe_families':1}}
    out=DATA/'catalog.json';out.write_text(json.dumps(result,ensure_ascii=False,separators=(',',':'))+'\n');print(json.dumps(result['coverage'],indent=2))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('command',choices=['build','search']);p.add_argument('query',nargs='?',default='');p.add_argument('--offline',action='store_true');args=p.parse_args()
    if args.command=='build':build(args.offline)
    else:
        data=json.loads((DATA/'catalog.json').read_text());q=key(args.query)
        matches=[{'id':r['id'],'name':r['name'],'source_url':r['source_url'],'details_loaded':r['details_loaded']} for collection in ['items','weapons'] for r in data[collection].values() if q in r['search']]
        print(json.dumps(matches[:30],ensure_ascii=False,indent=2))
