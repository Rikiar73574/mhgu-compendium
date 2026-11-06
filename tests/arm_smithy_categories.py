"""Wishlist capture across native equipment categories, flags, and menu guards."""
import os,sys,tempfile,struct,json,zlib
from pathlib import Path
folder=tempfile.TemporaryDirectory();path=Path(folder.name)/'wishlist.txt'
os.environ['COMPENDIUM_PREVIEW_WISHLIST']=str(path)
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
import preview_app as model
ns=model.ns;model.frame({'kind':'key','key':63});obj=ns['HEAP']+0x8000
catalog=json.loads((model.ROOT/'build/data/catalog.json').read_text())
def capture(category,flags,id,level,attack,state=7,substate=25):
 data=bytearray(0x3218);data[4:6]=bytes([state,substate]);struct.pack_into('<H',data,0x5c,category)
 struct.pack_into('<H',data,0x1f78,1);struct.pack_into('<HH',data,0x28d4,flags,id);data[0x28dd]=level-1
 struct.pack_into('<H',data,0x2240,attack)
 ns['u'].mem_write(obj,bytes(data));ns['call'](ns['symbols']['compendium_smithy_capture'],obj)
 ns['hid'](32);ns['render']();ns['hid'](0);ns['render']()
def keys():return set(int(s.split()[0]) for s in path.read_text().splitlines()) if path.exists() else set()
def key(name,level):
 family=next(w for w in catalog['weapons'].values() if w['name']==name)
 return zlib.crc32(f"{family['id']}:{level}".encode())
# Actual live Hammer selection plus original Lance and reported Hunting Horn.
for cat,flags,id,lv,atk,name in [(9,8,7,1,80,'Type 42 Shellbuster'),(10,8,1,5,110,'Petrified Lance'),(19,8,8,1,70,'Larinoth Ocarina')]:
 capture(cat,flags,id,lv,atk);assert key(name,lv) in keys(),name
# Availability changes must not change identity. Zero flags = craftable.
capture(9,0,7,1,80);assert key('Type 42 Shellbuster',1) not in keys()
capture(9,4,7,1,80);assert key('Type 42 Shellbuster',1) in keys()
expected=keys()
for args in [(9,0x210,7,1,80),(9,8,7,1,999),(1,8,7,1,80),(9,8,7,1,80,2),(9,8,7,1,80,7,24)]:
 capture(*args);assert keys()==expected,args
print('PASS: Hammer, Lance, Hunting Horn, availability flags, locked rows, wrong attack/category/menu guards')
