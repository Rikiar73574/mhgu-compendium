import sqlite3
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'tools'))
import item_sources


class ItemSourceTests(unittest.TestCase):
    def connection(self):
        db = sqlite3.connect(':memory:')
        self.addCleanup(db.close)
        db.executescript('''
            CREATE TABLE items(_id, name, type, buy);
            CREATE TABLE combining(_id, created_item_id, item_1_id, item_2_id, amount_made_min, amount_made_max, percentage);
            CREATE TABLE components(_id, created_item_id, component_item_id, quantity, type);
            CREATE TABLE meow_locations(_id, name);
            CREATE TABLE meow_rewards(meow_location_id, item_id, rank, rare, node_type, stack_size, percentage);
            INSERT INTO items VALUES (1,'Potion','Item',66),(2,'Herb','Item',20),(3,'Blue Mushroom','Item',NULL),
              (4,'Antidote Jwl 2','Decoration',200),(5,'Lazurite Jewel','Item',NULL),(6,'Old Book','Item',1000);
        ''')
        return db

    def test_combining_retains_alternatives_output_range_and_success_rate(self):
        db = self.connection()
        db.executescript('''INSERT INTO combining VALUES (1,1,2,3,1,3,95),(2,1,3,2,1,1,90);''')
        groups = item_sources.database_sources(db)['Potion']
        self.assertEqual(groups.get('Combining recipe 1', []),
                         [('Herb', 'Input ×1', '', '×1–3', '95%'),
                          ('Blue Mushroom', 'Input ×1', '', '×1–3', '95%')])
        self.assertEqual(groups.get('Combining recipe 2', []),
                         [('Blue Mushroom', 'Input ×1', '', '×1', '90%'),
                          ('Herb', 'Input ×1', '', '×1', '90%')])

    def test_decoration_recipes_keep_create_b_as_an_alternative(self):
        db = self.connection()
        db.executescript('''INSERT INTO components VALUES (1,4,5,1,'Create'),(2,4,2,2,'Create'),(3,4,3,3,'Create B');''')
        groups = item_sources.database_sources(db)['Antidote Jwl 2']
        self.assertEqual(groups.get('Craft at smithy recipe A', []), [('Lazurite Jewel','','','×1',''),('Herb','','','×2','')])
        self.assertEqual(groups.get('Craft at smithy recipe B', []), [('Blue Mushroom','','','×3','')])

    def test_buy_price_does_not_create_a_shop_source(self):
        self.assertFalse(item_sources.database_sources(self.connection())['Old Book'])

    def test_meownster_rewards_keep_area_rank_rare_node_quantity_rate(self):
        db = self.connection()
        db.executescript("INSERT INTO meow_locations VALUES (0,'Forest'); INSERT INTO meow_rewards VALUES (0,2,'G',1,8,3,25);")
        rows = item_sources.database_sources(db)['Herb'].get('Meownster Hunters', [])
        self.assertEqual(rows, [('Forest', 'Mushroom / Rare', 'GR', '×3', '25%')])

    def test_description_sources_only_use_explicit_acquisition_statements(self):
        metadata={'Rathalos Item':{'description':'An unappraised Rathalos material obtained by a Prowler using Pilfer or Plunderang.'},
                  'Red Scale':{'description':'A scale that may be useful.'},
                  'Garbage':{'description':'Junk created when a combination attempt fails.'}}
        result=item_sources.description_sources(metadata)
        self.assertEqual(result['Rathalos Item'].get('Prowler gathering', []),
                         [('Pilfer or Plunderang','Rathalos','','','')])
        self.assertFalse(result['Red Scale'])
        self.assertEqual(result['Garbage'].get('Combining', []), [('Failed combination','','','','')])

    def test_service_and_guild_rewards_keep_the_documented_acquisition_route(self):
        result=item_sources.description_sources({
            'Airship Ticket':{'description':"Given to the airship's best customers. Collect enough and it might pay off."},
            'Bistro Ticket':{'description':"Given to the Bistros' best customers. Collect enough and it might pay off."},
            'VE Bronze Ticket':{'description':'A ticket awarded for hunting with friends. Trade it with the Veggie Elder.'}})
        self.assertEqual(result['Airship Ticket'].get('Service rewards',[]),
                         [('Airship','Best customers; threshold not recorded','','','')])
        self.assertEqual(result['Bistro Ticket'].get('Service rewards',[]),
                         [('Bistros','Best customers; threshold not recorded','','','')])
        self.assertEqual(result['VE Bronze Ticket'].get('Guild rewards',[]),
                         [('Hunting with friends','Award threshold not recorded','','','')])

    def test_exchange_parser_keeps_direction_location_and_gift_probability(self):
        from bs4 import BeautifulSoup
        html='<table><tr><td>Dunes</td><td><a href="/item/t">VE Ticket</a></td><td>→</td><td><a href="/item/r">Ruby</a></td></tr></table><table><tr><td>Forest</td><td></td><td>→</td><td><a href="/item/h">Herb</a></td><td>15%</td></tr></table>'
        self.assertEqual(item_sources.parse_exchanges(BeautifulSoup(html,'html.parser')),
                         [{'location':'Dunes','input':{'id':'item/t','name':'VE Ticket'},'output':{'id':'item/r','name':'Ruby'},'rate':''},
                          {'location':'Forest','input':None,'output':{'id':'item/h','name':'Herb'},'rate':'15%'}])

    def test_exchange_sources_link_input_and_preserve_uncertain_gifts(self):
        snapshot={'exchanges':[{'location':'Dunes','input':{'id':'item/t','name':'Ticket'},
                              'output':{'id':'item/r','name':'Ruby'},'rate':''},
                             {'location':'Forest','input':None,'output':{'id':'item/r','name':'Ruby'},'rate':'15%'}]}
        catalog={'items':{'item/t':{'name':'Ticket'},'item/r':{'name':'Ruby'}}}
        rows=item_sources.exchange_sources(snapshot,catalog)['Ruby']
        self.assertEqual(rows.get('Veggie Elder trades', []), [('Ticket','Dunes','','','')])
        self.assertEqual(rows.get('Veggie Elder gifts', []), [('Forest','Gift; no exchange item','','','15%')])
        self.assertFalse(item_sources.exchange_sources(snapshot,catalog)['Ticket'])

    def test_scrap_pages_reverse_equipment_sources_but_not_palico_consumption(self):
        from bs4 import BeautifulSoup
        html='<h5>Weapon</h5><table><tr><td><a href="/weapon/a">Wyvern Lovers LV5</a></td><td>x1</td></tr></table><h5>Palico</h5><table><tr><td><a href="/palico/a">F Rathalos Helm R</a></td><td>x2</td></tr></table>'
        soup=BeautifulSoup(html,'html.parser')
        self.assertEqual(item_sources.parse_item_acquisitions(soup,scrap=True),
                         {'Equipment byproducts':[('Wyvern Lovers LV5','Weapon smithy','','×1','')]})
        self.assertEqual(item_sources.parse_item_acquisitions(soup,scrap=False),{})

    def test_item_page_import_preserves_secret_meownster_rewards(self):
        from bs4 import BeautifulSoup
        html='<h5>Meownster Hunters</h5><table><tr><td>Low Rank</td><td>Wetlands</td><td>Secret</td><td>Rare</td><td>x1</td><td>100%</td></tr></table>'
        self.assertEqual(item_sources.parse_item_acquisitions(BeautifulSoup(html,'html.parser')),
                         {'Meownster Hunters':[('Wetlands','Secret / Rare','LR','×1','100%')]})

    def test_catalog_scrap_byproduct_retains_weapon_level_and_crafting_mode(self):
        catalog={'weapons':{'weapon/a':{'name':'Sword','levels':[{'level':2,'name':'Sword+'}],
                 'recipes':[{'level':2,'upgrade':{'byproducts':[{'name':'Ore Scrap','quantity':2}]},
                             'create':{'byproducts':[]}}]}}}
        self.assertEqual(item_sources.byproduct_sources(catalog)['Ore Scrap'].get('Crafting byproducts', []),
                         [('Sword+ LV2','Upgrade','','×2','')])


if __name__ == '__main__':
    unittest.main()
