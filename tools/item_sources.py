"""Verified acquisition sources absent from the ordinary reward tables.

Buy prices are deliberately never interpreted as shop inventories. Database
component recipes belong to separate Create/Create B alternatives. Recipe
rows show inputs; their quantities are not the number of decorations produced.
"""
from collections import defaultdict
import re
from urllib.parse import urlparse


# Each label was cross-checked on 2026-11-09 against the Kiranico item page
# for an item occurring in exactly one database node type: Iron Ore, Whetfish,
# Herb, Scatternut, Bone Husk, Thunder Ambrosia, Honey, Flashbug, Blue Mushroom.
MEOWNSTER_NODES = ('Stone', 'Fish', 'Grass', 'Nuts', 'Bone', 'Bait', 'Honey', 'Insect', 'Mushroom')


def database_sources(connection):
    result = defaultdict(lambda: defaultdict(list))
    sql = '''SELECT output.name, a.name, b.name, c.amount_made_min,
                    c.amount_made_max, c.percentage
             FROM combining c JOIN items output ON output._id=c.created_item_id
             JOIN items a ON a._id=c.item_1_id JOIN items b ON b._id=c.item_2_id
             ORDER BY c._id'''
    for name, a, b, minimum, maximum, rate in connection.execute(sql):
        quantity = str(minimum) if minimum == maximum else f'{minimum}–{maximum}'
        recipe = 1 + sum(group.startswith('Combining recipe ') for group in result[name])
        result[name][f'Combining recipe {recipe}'].extend(
            (ingredient, 'Input ×1', '', f'×{quantity}', f'{rate}%') for ingredient in (a, b))
    sql = '''SELECT output.name, c.type, ingredient.name, c.quantity
             FROM components c JOIN items output ON output._id=c.created_item_id
             JOIN items ingredient ON ingredient._id=c.component_item_id
             WHERE output.type='Decoration' AND c.type IN ('Create','Create B')
             ORDER BY c.created_item_id,c.type,c._id'''
    for name, recipe, ingredient, quantity in connection.execute(sql):
        label = 'Craft at smithy recipe ' + ('A' if recipe == 'Create' else 'B')
        result[name][label].append((ingredient, '', '', f'×{quantity}', ''))
    sql = '''SELECT i.name, l.name, r.rank, r.rare, r.node_type,
                    r.stack_size, r.percentage
             FROM meow_rewards r JOIN items i ON i._id=r.item_id
             JOIN meow_locations l ON l._id=r.meow_location_id
             ORDER BY l.name,r.rank,r.rare,r.node_type,i.name,r.stack_size,r.percentage'''
    for name, location, rank, rare, node, quantity, rate in connection.execute(sql):
        if not 0 <= node < len(MEOWNSTER_NODES):raise ValueError(f'Unknown Meownster node type: {node}')
        method = MEOWNSTER_NODES[node] + ' / ' + ('Rare' if rare else 'Normal')
        result[name]['Meownster Hunters'].append((location, method, 'GR' if rank == 'G' else rank,
                                                 f'×{quantity}', f'{rate}%'))
    return result


def byproduct_sources(catalog):
    result = defaultdict(lambda: defaultdict(list))
    for weapon in catalog.get('weapons', {}).values():
        levels = {level['level']: level['name'] for level in weapon.get('levels', [])}
        for recipe in weapon.get('recipes', []):
            level = recipe['level']
            for mode in ('create', 'upgrade'):
                for item in recipe.get(mode, {}).get('byproducts', []):
                    result[item['name']]['Crafting byproducts'].append(
                        (f"{levels.get(level, weapon['name'])} LV{level}", mode.title(), '',
                         f"×{item['quantity']}", ''))
    return result


def description_sources(metadata):
    """Use only explicit acquisition statements, without guessing from item names."""
    result = defaultdict(lambda: defaultdict(list))
    for name, item in metadata.items():
        description = item.get('description') or ''
        match = re.fullmatch(r'An unappraised (.+) material obtained by a Prowler using Pilfer or Plunderang\.', description)
        if match:
            result[name]['Prowler gathering'].append(('Pilfer or Plunderang', match[1], '', '', ''))
        if description == 'Junk created when a combination attempt fails.':
            result[name]['Combining'].append(('Failed combination', '', '', '', ''))
        if '(Supply Item)' in description:
            result[name]['Quest supplies'].append(('Supply item', 'Availability depends on quest', '', '', ''))
        match = re.match(r'Received for (?:great )?contributions to (.+?)\.', description)
        if match:
            result[name]['Contribution rewards'].append((match[1], 'Contribution reward; threshold not recorded', '', '', ''))
        match = re.match(r"Given to the (.+?)(?:'s|') best(?: and favorite)? customers\.", description)
        if match:
            result[name]['Service rewards'].append((match[1].capitalize(), 'Best customers; threshold not recorded', '', '', ''))
        if description.startswith('A ticket awarded for hunting with friends.'):
            result[name]['Guild rewards'].append(('Hunting with friends', 'Award threshold not recorded', '', '', ''))
        elif description.startswith(('A ticket awarded in recognition of contributions to the Guild.',
                                     'A ticket awarded to Guild contributors.')):
            result[name]['Guild rewards'].append(('Guild contributions', 'Award threshold not recorded', '', '', ''))
        elif re.match(r"A ticket awarded to (?:one|a warrior) who(?:'s| has) challenged G-rank Quests\.", description):
            result[name]['Guild rewards'].append(('Challenge G-rank quests', 'Award threshold not recorded', 'GR', '', ''))
    return result


def parse_exchanges(soup):
    """Read directed Veggie Elder trades and separate probabilistic gifts."""
    result = []
    for table in soup.select('table'):
        if table.find('ais-pagination'):
            continue
        for tr in table.select('tr'):
            cells = tr.find_all('td', recursive=False)
            if len(cells) not in (4, 5) or cells[2].get_text(strip=True) != '→':
                continue
            def item(cell):
                link = cell.find('a', href=True)
                if not link:
                    return None
                identity = urlparse(link['href']).path.strip('/')
                if not identity.startswith('item/'):
                    raise ValueError('Non-item exchange link: ' + identity)
                return {'id': identity, 'name': link.get_text(' ', strip=True)}
            output = item(cells[3])
            if output is None:
                raise ValueError('Exchange lacks output item')
            result.append({'location': cells[0].get_text(' ', strip=True),
                           'input': item(cells[1]), 'output': output,
                           'rate': cells[4].get_text(strip=True) if len(cells) == 5 else ''})
    return result


def exchange_sources(snapshot, catalog):
    result = defaultdict(lambda: defaultdict(list))
    for exchange in snapshot['exchanges']:
        output = exchange['output']
        # Identity comes from the source href, never a fuzzy name match.
        target = catalog['items'].get(output['id'])
        if target is None:
            raise ValueError('Unknown exchange output: ' + output['id'])
        location = exchange['location'] or 'Common'
        required = exchange['input']
        if required:
            source = catalog['items'].get(required['id'])
            if source is None:
                raise ValueError('Unknown exchange input: ' + required['id'])
            row = (source['name'], location, '', '', exchange['rate'])
            kind = 'Veggie Elder trades'
        else:
            row = (location, 'Gift; no exchange item', '', '', exchange['rate'])
            kind = 'Veggie Elder gifts'
        result[target['name']][kind].append(row)
    return result


def parse_item_acquisitions(soup, scrap=False):
    """Import only acquisition tables; equipment-use tables are not item sources.

    Scraps are the exception: Weapon/Armor rows are reverse byproduct indexes,
    verified against recipe cells where scraps follow the explicit separator.
    Palico rows consume scraps and are deliberately excluded.
    """
    groups = defaultdict(list)
    ranks = {'Low Rank': 'LR', 'High Rank': 'HR', 'G Rank': 'GR'}
    allowed = {'Quest', 'Monster', 'Map', 'Hunters for Hire', 'Meownster Hunters'}
    for table in soup.select('table'):
        if table.find('ais-pagination'):
            continue
        heading = table.find_previous(['h2', 'h3', 'h4', 'h5'])
        kind = heading.get_text(' ', strip=True) if heading else ''
        byproduct = scrap and kind in ('Weapon', 'Armor')
        if not byproduct and kind not in allowed:
            continue
        for tr in table.select('tr'):
            cells = tr.find_all('td', recursive=False)
            fields = [' '.join(cell.get_text(' ', strip=True).split()) for cell in cells]
            if not fields:
                continue
            quantity = next((f.replace('x', '×', 1) for f in fields if re.fullmatch(r'x\d+', f)), '')
            rate = next((f for f in fields if re.fullmatch(r'\d+%', f)), '')
            if byproduct:
                if len(fields) != 2 or not quantity:
                    raise ValueError('Unrecognized scrap byproduct row: ' + repr(fields))
                groups['Equipment byproducts'].append((fields[0], kind + ' smithy', '', quantity, ''))
                continue
            rank = ranks.get(fields[0], '')
            content = [f for f in fields if f not in ranks and not re.fullmatch(r'x\d+|\d+%', f)]
            if content:
                groups[{'Quest': 'Quest rewards', 'Monster': 'Monster drops'}.get(kind, kind)].append(
                    (content[0], ' / '.join(content[1:]), rank, quantity, rate))
    return dict(groups)


def native_inventory_sources(tables, names):
    """Read exact stock membership; unknown gate/currency/quantity fields stay unused."""
    import struct
    if not names or len(names)%2:raise ValueError('Incomplete native item name/description pairs')
    specs = {'shopList00.slt': (0x3f800000,16,'Item shop'),
             'shopList01.slt': (0x3f800000,16,'Item shop'),
             'coinTradeList.ctl': (0x3f800000,11,'Coin exchange'),
             'lobby/tradePointItemList.tpil': (0x3f9ae148,8,'Trader point exchange'),
             'lobby/tradePointItemSpList.tpil': (0x3f9ae148,8,'Trader point exchange'),
             'lobby/tradeItemList.tril': (0x3f9ae148,16,'Trader')}
    result=defaultdict(lambda:defaultdict(list))
    for file,raw in tables.items():
        version,size,label=specs[file]
        if len(raw)<8:raise ValueError('Truncated native inventory: '+file)
        actual,count=struct.unpack_from('<II',raw)
        if actual!=version or len(raw)!=8+count*size:raise ValueError('Unsupported native inventory: '+file)
        for index in range(count):
            row=raw[8+index*size:8+(index+1)*size]
            if file.startswith('shop'):
                position,item,_,_=struct.unpack('<4I',row)
                if position!=index:raise ValueError('Invalid native shop row order')
                ids=[item]
            elif file.endswith('.tril'):ids=struct.unpack('<8H',row)[::2]
            elif file.endswith('.tpil'):ids=[struct.unpack_from('<H',row)[0]]
            else:ids=[struct.unpack_from('<I',row)[0]]
            for item in ids:
                if item*2+1>=len(names):raise ValueError('Native stock item outside English name table')
                if not item or names[item*2] in ('','(None)','DUMMY'):continue
                value=(label,'Availability depends on stock and progression','','','')
                if value not in result[names[item*2]][label]:result[names[item*2]][label].append(value)
    identities=defaultdict(list)
    for i in range(0,len(names)-1,2):
        if names[i]:identities[names[i]].append(names[i+1])
    internal={name for name,descriptions in identities.items() if all(d=='Program Data' for d in descriptions)}
    for name in internal:result[name]['Internal game data'].append(('Internal game entry','Not a regular pouch item','','',''))
    return result,internal


def owned_inventory_sources(game):
    from game_names import strings
    root=game/'romfs/nativeNX'
    paths=['shopList00.slt','shopList01.slt','coinTradeList.ctl','lobby/tradePointItemList.tpil',
           'lobby/tradePointItemSpList.tpil','lobby/tradeItemList.tril']
    return native_inventory_sources({p:(root/'table'/p).read_bytes() for p in paths},strings(root/'eng/table/itemData_eng.gmd'))
