"""Execute the ARM native-row hook against mocked native layout calls."""
import ast,struct
from pathlib import Path
source=Path(__file__).with_name('arm_smoke.py')
tree=ast.parse(source.read_text());prefix=[]
for node in tree.body:
 if isinstance(node,ast.Expr) and isinstance(node.value,ast.Call) and isinstance(node.value.func,ast.Name) and node.value.func.id=='call':break
 prefix.append(node)
ns={'__file__':str(source)};exec(compile(ast.Module(body=prefix,type_ignores=[]),str(source),'exec'),ns)
u,put,get,call=map(ns.get,['u','put','get','call']);MAIN,STUB,HEAP=map(ns.get,['MAIN','STUB','HEAP'])
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import *
hooks={};rows=[];labels=[];native=STUB+0x8000;smith_original=STUB+0x8004
for p in [native,smith_original,MAIN+0x56e808]:u.mem_write(p,bytes.fromhex('1eff2fe1'))
def intercept(uc,at,size,ctx):
 r=[uc.reg_read(x) for x in [UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2,UC_ARM_REG_R3]]
 if at==native:
  rows.append([(get(r[0]+0x228+8*i),get(r[0]+0x22c+8*i))for i in range(4)])
  for glyph,text in rows[-1]:
   if glyph==132:labels.append(bytes(u.mem_read(text,80)).split(b'\0')[0].decode())
 elif at==MAIN+0x56e808:
  text=bytes(u.mem_read(r[3],80)).split(b'\0')[0].decode();labels.append((r[1],r[2],text))
u.hook_add(UC_HOOK_CODE,intercept,begin=STUB,end=STUB+0xffff)
u.hook_add(UC_HOOK_CODE,intercept,begin=MAIN+0x56e808,end=MAIN+0x56e808)
# Base callback runs first and modifies R0; handle hook creation before it instead.
def first(uc,at,size,ctx):
 if ns['imports'].get(at)=='forge_hook_create':
  r=[uc.reg_read(x)for x in [UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2,UC_ARM_REG_R3]];hooks[r[1]]=r[2];put(r[3],native if r[1]==MAIN+0x6439ec else smith_original);uc.reg_write(UC_ARM_REG_R0,r[0]);return True
 return False
ns['storage_callback']=first
# Exercise the actual ARM install guard. A Forge entry hook may precede us,
# but a foreign jump or a wrong-version prologue must never be chained.
entry=MAIN+0x6f1ae8;saved=(get(entry),get(entry+4))
for word,destination,accepted in [
 (0xe92d41f0,saved[1],True),
 (0xe51ff004,0x03200000,True),
 (0xe51ff004,0x03dffffc,True),
 (0xe51ff004,0x031ffffc,False),
 (0xe51ff004,0x03e00000,False),
 (0xe51ff004,0x03200001,False),
 (0xe51ff004,0,False),
 (0xea000000,0x03200000,False),
 (0xe92d4010,saved[1],False),
]:
 hooks.clear();put(entry,word);put(entry+4,destination)
 call(ns['symbols']['forge_onInit'])
 assert (entry in hooks)==accepted,(hex(word),hex(destination),accepted,hooks)
put(entry,saved[0]);put(entry+4,saved[1]);hooks.clear()
call(ns['symbols']['forge_onInit'])
obj=HEAP+0x4000;smith=HEAP+0x8000
put(obj,MAIN+0x1777c60);put(obj+0x4c,0x34);put(obj+0x50,33)
for i in range(4):put(obj+0x228+8*i,0xffffffff);put(obj+0x22c+8*i,0)
put(obj+0x228,15);put(obj+0x22c,HEAP+0x3000);u.mem_write(HEAP+0x3000,b'Help\0')
refresh=ns['symbols']['compendium_native_hint_refresh'];call(refresh,obj,0x34,33,1)
assert not labels
# Verified Petrified Lance Lv5 selection.
data=bytearray(0x3218);data[4:6]=bytes([7,25]);struct.pack_into('<H',data,0x5c,10);struct.pack_into('<H',data,0x1f78,5);struct.pack_into('<HH',data,0x28d4,8,1);data[0x28dd]=4;struct.pack_into('<H',data,0x2240,110);u.mem_write(smith,bytes(data))
call(hooks[MAIN+0x6f1ae8],smith)
assert labels[-1]=='Add to wishlist',labels
assert rows[-1][0]==(15,HEAP+0x3000) and rows[-1][1][0]==132,rows[-1]
assert get(obj+0x230)==0xffffffff and get(obj+0x234)==0,'temporary native slot leaked'
# Full/custom foreign rows and other layouts are never replaced.
n=len(labels);put(obj+0x230,3);put(obj+0x234,HEAP+0x3000);call(refresh,obj,0x34,33,1);assert len(labels)==n
put(obj+0x230,0xffffffff);put(obj+0x234,0);call(refresh,obj,0xcf,1,1);assert len(labels)==n
# Leaving the upgrade state removes the native hint.
struct.pack_into('<H',data,0x5c,0);u.mem_write(smith,bytes(data));call(hooks[MAIN+0x6f1ae8],smith)
assert rows[-1][1][0]==0xffffffff and len(labels)==n
print('PASS: stock/Forge entry guards, foreign jumps refused, native row insertion, Help preserved, temporary state restored, full/foreign menus excluded, exit cleanup')
