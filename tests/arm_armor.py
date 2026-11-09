"""Navigate the compiled armor browser and persist a mixed weapon/armor wishlist."""
import ast,os,re,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
weapons=(ROOT/'build/generated/weapon_data.rs').read_text()
key=re.search(r'Weapon\{key:(\d+)',weapons)[1]
with tempfile.TemporaryDirectory(prefix='compendium-armor-') as directory:
    wishlist=Path(directory)/'wishlist.txt';wishlist.write_text(f'{key} 0\n')
    os.environ['COMPENDIUM_PREVIEW_WISHLIST']=str(wishlist)
    sys.path.insert(0,str(ROOT/'tools'))
    import preview_app as preview
    ns=preview.ns
    def press(bits):
        preview.frame({'kind':'button','bits':bits})
        return ns['texts'][:]
    for _ in range(4):press(1<<7)
    assert 'Armor' in ns['texts'] and 'Chest' in ns['texts'],ns['texts']
    press(1)
    for character in 'leather headgear':
        preview.frame({'kind':'key','key':44 if character==' ' else ord(character)-ord('a')+4})
    preview.frame({'kind':'key','key':40})
    out=press(1)
    assert 'Leather Headgear' in out and 'Full set preview' in out and 'Stats & skill points' in out and 'Crafting materials' in out,out
    assert 'Leather Vest' in out and 'Leather Trousers' in out,out
    if os.environ.get('COMPENDIUM_SCREENSHOTS'):
        source=ROOT/'tests/arm_smoke.py'
        node=next(n for n in ast.parse(source.read_text()).body if isinstance(n,ast.FunctionDef) and n.name=='preview')
        exec(compile(ast.Module(body=[node],type_ignores=[]),str(source),'exec'),ns)
        ns['preview']('armor-details.png')
    press(1<<5)
    assert wishlist.read_text()==f'{key} 0\narmor:1310721 0\n',wishlist.read_text()
    # Open a material, return, then navigate a set piece without changing equipment.
    assert 'ITEM SOURCES' in press(1),ns['texts']
    assert 'Leather Headgear' in press(2),ns['texts']
    assert 'Leather Vest' in press(1<<14),ns['texts']
    press(2);press(1<<6)
    assert 'Wishlist' in ns['texts'] and 'Leather Headgear' in ns['texts'],ns['texts']
    # Up/Down reaches the armor's materials directly, without opening its details.
    weapon_count=next(line for line in weapons.splitlines() if line.startswith('Weapon{')).split('Recipe{',1)[1].split('Recipe{')[0].count('Material{')
    for _ in range(weapon_count):press(1<<15)
    assert 'ITEM SOURCES' in press(1),ns['texts']
    press(2);press(1<<5)
    assert wishlist.read_text()==f'{key} 0\n',wishlist.read_text()
    # The same item view exposes monster conditions, ranks, quantities and rates.
    press(1<<7)  # Armor
    press(1<<7)  # Items
    for character in 'rathalos scale':
        preview.frame({'kind':'key','key':44 if character==' ' else ord(character)-ord('a')+4})
    preview.frame({'kind':'key','key':40})
    out=press(1)
    assert 'Hyper Rathalos Scale' in out and 'Quest rewards' in out and 'HR' in out,out
    press(2);press(1<<15);out=press(1)
    assert 'Rathalos' in out and 'Body Carve' in out and 'LR' in out,out
    if os.environ.get('COMPENDIUM_SCREENSHOTS'):ns['preview']('monster-material-sources.png')
    print('ARM armor browsing, set links, material sources and mixed wishlist persistence passed')
