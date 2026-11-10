import struct,sys,unittest
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
from item_sources import native_inventory_sources
class NativeSourceTests(unittest.TestCase):
 def names(self):
  names=['','']*205
  for i,n,d in [(40,'Powercharm','Boosts attack.'),(32,'Dust of Life','Healing.'),(164,'Blast S Lv1','Program Data'),(100,'Normal S Lv1','Unlimited shots.'),(132,'Normal S Lv1','Program Data')]:names[i*2:i*2+2]=[n,d]
  return names
 def test_real_stock_membership_without_guessing_gates_cost_or_quantity(self):
  shop=struct.pack('<6I',0x3f800000,1,0,40,7,15)
  coin=struct.pack('<IIIBIBB',0x3f800000,1,32,15,22,0,0)
  rows,internal=native_inventory_sources({'shopList00.slt':shop,'coinTradeList.ctl':coin},self.names())
  self.assertIn('Item shop',rows['Powercharm'])
  self.assertIn('Coin exchange',rows['Dust of Life'])
  self.assertTrue(all(not row[3] and not row[4] for groups in rows.values() for values in groups.values() for row in values))
  self.assertIn('Blast S Lv1',internal);self.assertNotIn('Normal S Lv1',internal)
 def test_malformed_stock_does_not_get_silently_parsed(self):
  with self.assertRaises(ValueError):native_inventory_sources({'shopList00.slt':b'bad'},self.names())

 def test_incomplete_name_pairs_never_reclassify_a_real_item(self):
  for names in ([],['','', 'Entry','Program Data','Entry']):
   with self.assertRaises(ValueError):native_inventory_sources({},names)
