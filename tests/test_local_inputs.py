import struct, tempfile, unittest
from pathlib import Path
import sys
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"tools"))
from game_names import names, export
class LocalInputsTest(unittest.TestCase):
 def test_reads_only_name_slots(self):
  with tempfile.TemporaryDirectory() as folder:
   path=Path(folder)/"fixture.gmd";header=bytearray(41);header[:4]=b"GMD\0"
   struct.pack_into("<I",header,36,0);path.write_bytes(header+b"zero\0description\0one\0description\0")
   self.assertEqual(names(path,2),{"0":"zero","1":"one"})
 def test_internal_shot_names_do_not_ambiguate_physical_ammo(self):
  with tempfile.TemporaryDirectory() as folder:
   table=Path(folder);header=bytearray(41);header[:4]=b"GMD\0"
   for kind in range(15):
    if kind!=5:(table/f"weapon{kind:02d}MsgData_eng.gmd").write_bytes(header+b"weapon\0"*6)
   rows=[str(i) for i in range(206)];rows[100]=rows[132]="physical ammo"
   (table/"itemData_eng.gmd").write_bytes(header+b"".join(name.encode()+b"\0description\0" for name in rows))
   _,items=export(table)
   self.assertEqual(items["100"],"physical ammo")
   self.assertNotIn("132",items);self.assertNotIn("204",items)
   self.assertIn("131",items);self.assertIn("205",items)
 def test_rejects_bad_or_truncated_gmd(self):
  with tempfile.TemporaryDirectory() as folder:
   path=Path(folder)/"fixture.gmd"
   for data in (b"",b"BAD!"+bytes(37)):
    path.write_bytes(data)
    with self.assertRaises(ValueError):names(path,2)
if __name__=="__main__":unittest.main()
