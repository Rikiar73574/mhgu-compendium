"""Isolated ARM native Start-menu lifecycle; never starts/attaches the game."""
import ast,struct
from pathlib import Path
source=Path(__file__).with_name('arm_smoke.py');tree=ast.parse(source.read_text());prefix=[]
for node in tree.body:
 if isinstance(node,ast.Expr) and isinstance(node.value,ast.Call) and isinstance(node.value.func,ast.Name) and node.value.func.id=='call':break
 prefix.append(node)
# Use the actual game's low main base; an artificial high base hides bad
# guards that confuse executable/data pointers with allocated heap objects.
for node in prefix:
 if isinstance(node,ast.Assign) and isinstance(node.targets[0],ast.Tuple):
  names=[n.id for n in node.targets[0].elts if isinstance(n,ast.Name)]
  if 'MAIN' in names:node.value.elts[names.index('MAIN')]=ast.Constant(0x200000)
ast.fix_missing_locations(tree)
ns={'__file__':str(source)};exec(compile(ast.Module(body=prefix,type_ignores=[]),str(source),'exec'),ns)
u,put,get,invoke=map(ns.get,['u','put','get','call']);MAIN,STUB,HEAP=map(ns.get,['MAIN','STUB','HEAP'])
from unicorn.arm_const import *
from unicorn import UC_HOOK_CODE
def call(at,*args):
 invoke(at,*args);return u.reg_read(UC_ARM_REG_R0)
hooks={};freed=[];allocations=[];calls=[];fail_alloc=0;fail_hook=0
native=[STUB+0x8000+i*4 for i in range(7)]
native[4:6]=[MAIN+0x800100,MAIN+0x800104]
ctor,init,fill,action,alloc,free,heap_for=native
heap,table=HEAP+0x3000,MAIN+0x17d0000
put(heap,table);put(table+0x1c,alloc);put(table+0x34,free)
for p in native:u.mem_write(p,bytes.fromhex('1eff2fe1'))
# The native page-count setter is a leaf; execute it unchanged, without game globals.
# The rest of the native lifecycle is mocked so assertions can isolate the hook.
def intercept(uc,at,size,ctx):
 global fail_alloc
 r=[uc.reg_read(x)for x in [UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2]]
 if at==ctor:uc.reg_write(UC_ARM_REG_R0,r[0])
 elif at==init:calls.append(('init',r[0]))
 elif at==fill:
  calls.append(('fill',r[0]));rows=get(r[0]+0xa8)
  for i in range(get(r[0]+0xac)):put(rows+i*8,0);put(rows+i*8+4,0)
  for i in range(13):put(rows+i*8,1);put(rows+i*8+4,HEAP+0x2000+i*4)
 elif at==action:calls.append(('action',r[0]));uc.reg_write(UC_ARM_REG_R0,123)
 elif at==heap_for:
  assert r[0]==MAIN+0x191eb18;uc.reg_write(UC_ARM_REG_R0,heap)
 elif at==alloc:
  assert r[0]==heap and r[2]==16
  allocations.append(r[1]);n=len(allocations)
  out=0 if fail_alloc==n else HEAP+0x7000+n*0x200
  uc.reg_write(UC_ARM_REG_R0,out)
 elif at==free:
  assert r[0]==heap;freed.append(r[1])
u.hook_add(UC_HOOK_CODE,intercept,begin=STUB+0x8000,end=STUB+0x9000)
u.hook_add(UC_HOOK_CODE,intercept,begin=alloc,end=free)
def first(uc,at,size,ctx):
 if ns['imports'].get(at)=='forge_hook_create':
  r=[uc.reg_read(x)for x in [UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2,UC_ARM_REG_R3]]
  if r[1]==fail_hook:
   uc.reg_write(UC_ARM_REG_R0,0);return True
  hooks[r[1]]=r[2]
  originals={MAIN+0x677b48:ctor,MAIN+0x677d0c:init,MAIN+0x678e80:fill,MAIN+0x677ee0:action}
  put(r[3],originals.get(r[1],STUB+0x9000));u.mem_write(STUB+0x9000,bytes.fromhex('1eff2fe1'));uc.reg_write(UC_ARM_REG_R0,r[0]);return True
 return False
ns['storage_callback']=first
# Route the exact native allocator function to its isolated mock.
u.mem_write(MAIN+0x7aa47c,struct.pack('<II',0xe51ff004,heap_for))
# Restore allocator signature for installation then use mock after.
put(MAIN+0x7aa47c,0xe59f1010)
call(ns['symbols']['forge_onInit'])
expected={MAIN+x for x in [0x677b48,0x677d0c,0x678e80,0x677ee0]}
assert expected<=hooks.keys(),hooks
u.mem_write(MAIN+0x7aa47c,struct.pack('<II',0xe51ff004,heap_for))
obj=HEAP+0x4000;old_rows=HEAP+0x5000;old_hints=HEAP+0x6000
row_bytes=bytes((i*7+11)&255 for i in range(13*8));hint_bytes=bytes((i*3+5)&255 for i in range(13*12))
def reset(count=13):
 allocations.clear();freed.clear();calls.clear();put(obj,MAIN+0x177aeb8);put(obj+0xa8,old_rows);put(obj+0xac,count);put(obj+0xc0,old_hints);put(obj+0xc4,count)
 put(obj+0x48,0);put(obj+0x4c,0);u.mem_write(old_rows,row_bytes);u.mem_write(old_hints,hint_bytes);u.mem_write(obj+0xd8,bytes(0x24));u.mem_write(obj+0xe6,b'\x07')
def hook(off,*args):return call(hooks[MAIN+off],*args)
reset();assert hook(0x677b48,obj)==obj
rows,hints=get(obj+0xa8),get(obj+0xc0)
assert get(obj+0xac)==14 and get(obj+0xc4)==14
assert bytes(u.mem_read(rows,13*8))==row_bytes,'stock rows (including Pause) modified'
assert bytes(u.mem_read(hints,13*12))==hint_bytes,'stock hints modified'
assert allocations==[14*8,14*12] and freed==[old_rows,old_hints]
assert get(rows+13*8)==1
label=get(rows+13*8+4);assert bytes(u.mem_read(label,11))==b'Compendium\0'
assert bytes(u.mem_read(hints+13*12+4,8))==b'\xff'*8
hook(0x677d0c,obj)
assert struct.unpack('<H',u.mem_read(obj+0xec,2))[0]==14
assert struct.unpack('<H',u.mem_read(obj+0xea,2))[0]==2,'two native pages retained'
hook(0x678e80,obj)
assert get(rows+13*8+4)==label and get(rows+13*8)==1
assert [get(rows+i*8+4)for i in range(13)]==[HEAP+0x2000+i*4 for i in range(13)]
take=ns['symbols']['compendium_start_take_open'];assert call(take)==0
put(obj+0x4c,13);assert hook(0x677ee0,obj)==4
assert call(take)==1 and call(take)==0,'request must be consumed once'
put(obj+0x4c,12);assert hook(0x677ee0,obj)==123 and call(take)==0,'stock action intercepted'
put(obj+0x4c,13);put(obj+0x48,3);assert hook(0x677ee0,obj)==123,'non-top internal action intercepted'
# Foreign row fourteen, wrong vtable and unexpected counts are untouched.
for field,value in [(0xac,12),(0xc4,14),(0,MAIN+0x177aebc)]:
 reset();put(obj+field,value);hook(0x677b48,obj);assert not allocations
reset(14);hook(0x677b48,obj);assert not allocations
put(old_rows+13*8+4,HEAP+0x2000);put(obj+0x4c,13);assert hook(0x677ee0,obj)==123
# Heap vtables must be main's relocated data; callbacks must be main code.
for address,value in [(heap,HEAP+0x3100),(table+0x1c,STUB+0x8000),(table+0x34,table)]:
 reset();saved=get(address);put(address,value);hook(0x677b48,obj);assert not allocations;put(address,saved)
# Both allocation failures preserve all original pointers/counts and free only new memory.
for failed in [1,2]:
 reset();fail_alloc=failed;hook(0x677b48,obj)
 assert get(obj+0xa8)==old_rows and get(obj+0xc0)==old_hints and get(obj+0xac)==13 and get(obj+0xc4)==13
 assert bytes(u.mem_read(old_rows,13*8))==row_bytes and bytes(u.mem_read(old_hints,13*12))==hint_bytes
 assert freed==([]if failed==1 else[HEAP+0x7200])
# A foreign hook/version signature refuses the entire extension before creation.
put(MAIN+0x7aa47c,0xe59f1010)
for offset in [0x677b48,0x677d0c,0x678e80,0x677ee0,0x54ee34,0x54f9f0,0x7aa47c]:
 at=MAIN+offset;saved=get(at);put(at,0);hooks.clear();call(ns['symbols']['forge_onInit'])
 assert not(expected & hooks.keys()),hex(offset)
 put(at,saved)
# Failure of a dependency never enables the allocating constructor.
for offset in [0x677d0c,0x678e80,0x677ee0]:
 hooks.clear();fail_hook=MAIN+offset;call(ns['symbols']['forge_onInit']);assert MAIN+0x677b48 not in hooks
print('PASS: additive native row, all13 stock rows/hints preserved, native pagination, refill, action/one-shot request, foreign layouts, allocation rollback, version guards, partial-install fallback')
