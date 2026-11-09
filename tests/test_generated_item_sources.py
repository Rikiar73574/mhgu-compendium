"""Exercise generated Rust data, including source navigation targets."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]


@unittest.skipUnless(os.environ.get('MHGU_DATABASE') and shutil.which('rustc'),
                     'Requires local database and Rust compiler')
class GeneratedItemSourceTests(unittest.TestCase):
    def test_recipe_inputs_and_weapon_byproducts_open_exact_catalog_targets(self):
        subprocess.run(['python', str(ROOT/'tools/item_data.py')], check=True,
                       capture_output=True, text=True)
        catalog = json.loads((ROOT/'build/data/catalog.json').read_text())
        items = sorted(catalog['items'].values(), key=lambda item: item['name'])
        names = {item['name']: index for index, item in enumerate(items)}
        weapons = sorted(catalog['weapons'].values(), key=lambda weapon: (weapon['type'], weapon['name']))
        weapon = len(items) + next(index for index, w in enumerate(weapons) if w['id'] == 'weapon/48dae')
        program = '''#![allow(dead_code)]
include!("GENERATED");
fn main() {
 let potion=&ITEMS[POTION];
 let recipe=potion.groups.iter().find(|g|g.name=="Combining recipe 1").unwrap();
 let herb=recipe.rows.iter().find(|r|r.text()[0]=="Herb").unwrap();
 assert_eq!(herb.link(),Some(HERB));
 assert_eq!(herb.text()[4],"95%");
 let decoration=&ITEMS[DECORATION];
 let recipe=decoration.groups.iter().find(|g|g.name=="Craft at smithy recipe A").unwrap();
 let material=recipe.rows.iter().find(|r|r.text()[0]=="Sunspire Jewel").unwrap();
 assert_eq!(material.link(),Some(SUNSPIRE));
 let scraps=&ITEMS[SCRAP];
 let source=scraps.groups.iter().find(|g|g.name=="Crafting byproducts").unwrap();
 assert!(source.rows.iter().any(|r|r.link()==Some(WEAPON)));
 assert!(ITEMS.iter().flat_map(|i|i.groups).flat_map(|g|g.rows).all(|r|r.link().map_or(true,|id|id<ENTRY_COUNT)));
}
'''
        replacements={'GENERATED':str(ROOT/'build/generated/item_data.rs'),
                      'POTION':str(names['Potion']), 'HERB':str(names['Herb']),
                      'DECORATION':str(names['Antidote Jwl 2']), 'SUNSPIRE':str(names['Sunspire Jewel']),
                      'SCRAP':str(names['Ore Scrap']), 'WEAPON':str(weapon),
                      'ENTRY_COUNT':str(len(items)+len(weapons))}
        for key,value in replacements.items():
            program=program.replace(key,value)
        with tempfile.TemporaryDirectory() as directory:
            source=Path(directory)/'test.rs'; source.write_text(program)
            executable=Path(directory)/'test'
            compiled=subprocess.run(['rustc','+stable','--edition=2021',str(source),'-o',str(executable)],
                                    capture_output=True,text=True)
            self.assertEqual(compiled.returncode,0,compiled.stderr)
            result=subprocess.run([str(executable)],capture_output=True,text=True)
            self.assertEqual(result.returncode,0,result.stderr)


if __name__ == '__main__':
    unittest.main()
