"""Compiled gathering callbacks using synthetic native layouts, nodes and ownership."""
import ast, struct
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import *
source=Path(__file__).with_name('arm_smoke.py')
tree=ast.parse(source.read_text());prefix=[]
for node in tree.body:
 if isinstance(node,ast.Expr) and isinstance(node.value,ast.Call) and isinstance(node.value.func,ast.Name) and node.value.func.id=='call':break
 prefix.append(node)
ns={'__file__':str(source)};exec(compile(ast.Module(body=prefix,type_ignores=[]),str(source),'exec'),ns)
u,put,get,call=(ns[k] for k in ('u','put','get','call'))
MAIN,HEAP,STUB=(ns[k] for k in ('MAIN','HEAP','STUB'))
symbols=ns['symbols']
for name in ('compendium_gather_update','compendium_gather_request','compendium_gather_drop_layout'):
 assert name in symbols, f'missing compiled export {name}'
u.mem_map(HEAP+0x10000,0x40000)
hooks={};created=[];mutations=[];items=[];positions=[];accepted=[];originals=[];destroyed=[]
objects={};parents={};foreign=set()
UPDATE,DROP=MAIN+0x619494,MAIN+0x91b284
ORIGINAL_UPDATE,ORIGINAL_DROP=STUB+0x8000,STUB+0x8004
for at in (ORIGINAL_UPDATE,ORIGINAL_DROP):u.mem_write(at,bytes.fromhex('1eff2fe1'))
def storage_callback(uc,at,size,ctx):
 if ns['imports'].get(at)!='forge_hook_create':return False
 target=uc.reg_read(UC_ARM_REG_R1);detour=uc.reg_read(UC_ARM_REG_R2);out=uc.reg_read(UC_ARM_REG_R3)
 hooks[target]=detour;put(out,ORIGINAL_DROP if target==DROP else ORIGINAL_UPDATE)
 return True
ns['storage_callback']=storage_callback
UI,MAP,VTABLE=HEAP+0x7000,HEAP+0x8000,MAIN+0x1750000
LAYOUT,NEW_LAYOUT,PARENT=HEAP+0x3000,HEAP+0x3800,HEAP+0x9000
GC,SQ,QD,POP,POOL=HEAP+0x4000,HEAP+0x5000,HEAP+0x6000,HEAP+0xa000,HEAP+0x10000
IPL,PAYLOAD=HEAP+0xc000,HEAP+0xd000
put(MAIN+0x188ca48,GC);u.mem_write(GC+0x4c,b'\xff')
put(MAIN+0x188ad90,SQ);put(SQ+0xa4,QD);put(SQ+0xe0,207);put(QD+8,207)
put(UI+0x24,LAYOUT);put(UI+0xcc,MAP);put(UI+0xd0,2);put(UI+0xd4,0)
u.mem_write(UI+0x4c,b'\xff'*128)
put(MAP,VTABLE);put(VTABLE+0x64,MAIN+0x61bda8);put(VTABLE+0x6c,MAIN+0x61bde8)
put(MAIN+0x1889ec0,POP);put(POP,MAIN+0x17524cc);put(POP+0x24,POOL)
put(POP+0x20,IPL);put(IPL,MAIN+0x175245c);put(IPL+0x64,PAYLOAD);put(IPL+0x68,12)
u.mem_write(PAYLOAD,struct.pack('<BBHBBHBBH',80,1,271,20,1,318,255,0,0))
parents[LAYOUT]=PARENT;parents[NEW_LAYOUT]=PARENT+0x100

def f32(a):return struct.unpack('<f',u.mem_read(a,4))[0]
def set_node(slot,kind,area,xyz,uses=2):
 raw=bytearray(0x80);struct.pack_into('<I3f',raw,0,MAIN+0x17523e0,*xyz)
 struct.pack_into('<h',raw,0x4e,uses);raw[0x55]=area;raw[0x70]=3
 struct.pack_into('<I',raw,0x68,kind);u.mem_write(POOL+slot*0x80,bytes(raw))
set_node(0,11,1,(10.,20.,30.));set_node(1,9,7,(-40.,50.,60.))
put(POOL+0x38,PAYLOAD);put(POOL+0x80+0x38,PAYLOAD)

def native(uc,at,size,ctx):
 r=[uc.reg_read(reg) for reg in (UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2,UC_ARM_REG_R3)]
 result=0
 if at==ORIGINAL_UPDATE:originals.append(r[0])
 elif at==ORIGINAL_DROP:destroyed.append(r[0])
 elif at==MAIN+0x544fe8:
  result=parents.get(r[0],0) if r[1]==0 else objects.get((r[0],r[1]),0)
 elif at==MAIN+0x54501c:
  assert r[1:]==[2,4,0],('create resource/template/parent',r)
  assert get(uc.reg_read(UC_ARM_REG_SP))==MAIN+0x2123290,'wrong sprite DTI'
  assert r[0] in parents
  result=1000+len(created);obj=HEAP+0x20000+len(created)*0x500
  objects[r[0],result]=obj;put(obj+0x68,parents[r[0]]);put(obj+0xd8,obj+0x200);put(obj+0x204,obj+0x300)
  created.append((r[0],result,obj))
 elif at==MAIN+0x546f48:
  assert r[2:]==[1,2],('common item atlas, not map-marker atlas',r);mutations.append(('texture',r[0],r[1],r[3]))
 elif at==MAIN+0xa4fbf4:
  assert [f32(r[1]),f32(r[1]+4)]==[18.,18.],'readable sprite dimensions'
  assert any(r[0]==obj+0x300 for _,_,obj in created),'foreign sprite resized'
 elif at==MAIN+0x546e84:
  assert r[2]==50,'sprite priority';mutations.append(('priority',r[0],r[1],r[2]))
 elif at==MAIN+0x54e908:u.mem_write(r[0],bytes(32))
 elif at==MAIN+0x54e96c:
  assert r[1:3]==[UI,0]
  item,ident=get(r[0]+4),get(r[0]+8);assert get(r[0]+16)==1
  items.append((ident,item));mutations.append(('item',get(UI+0x24),ident,item))
 elif at==MAIN+0x545b78:
  position=struct.unpack('<2f',u.mem_read(r[2],8));positions.append((r[0],r[1],position));mutations.append(('position',r[0],r[1],position))
 elif at==MAIN+0x54533c:mutations.append(('show',r[0],r[1],r[2]))
 elif at==MAIN+0x61bda8:
  assert r[0]==MAP;accepted.append(r[1]);result=int(r[1]<=12)
 elif at==MAIN+0x61bde8:
  assert r[0]==MAP
  xyz=struct.unpack('<4f',u.mem_read(r[1],16));assert xyz[3]==1.
  # The native projection includes the map slider; the callback preserves it.
  result_pos=(xyz[0]*.1+r[3]*100+13,xyz[2]*.1+r[3]*50+17,0.,1.)
  u.mem_write(r[2],struct.pack('<4f',*result_pos))
 else:return
 uc.reg_write(UC_ARM_REG_R0,result);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
for at in [ORIGINAL_UPDATE,ORIGINAL_DROP,MAIN+0x544fe8,MAIN+0x54501c,MAIN+0x546f48,
 MAIN+0xa4fbf4,MAIN+0x546e84,MAIN+0x54e908,MAIN+0x54e96c,MAIN+0x545b78,
 MAIN+0x54533c,MAIN+0x61bda8,MAIN+0x61bde8]:
 u.hook_add(UC_HOOK_CODE,native,begin=at,end=at)
# Fingerprint rejection precedes either gathering hook installation.
fingerprint=get(MAIN+0x54501c);put(MAIN+0x54501c,0)
call(symbols['forge_onInit']);assert UPDATE not in hooks and DROP not in hooks
assert not created
put(MAIN+0x54501c,fingerprint);hooks.clear();call(symbols['forge_onInit'])
assert hooks.get(UPDATE)==symbols['compendium_gather_update']
assert hooks.get(DROP)==symbols['compendium_gather_drop_layout']
update=lambda:call(hooks[UPDATE],UI)
request=lambda value:call(symbols['compendium_gather_request'],value)
request(0);update();assert not created,'disabled gathering allocated sprites'
request(1);update()
assert not created,'no selected items must show no gathering spots'
select=lambda category,item:call(symbols['compendium_gather_select'],category,item)
select(4,271);select(0,318);update()
assert len(created)==2,created
assert items==[(1000,271),(1001,57)],items
assert set(accepted)=={1,7},'whole-map gathering incorrectly filters the current area'
assert positions[:2]==[(LAYOUT,1000,(105.,61.)),(LAYOUT,1001,(700.,364.))],positions
assert originals==[UI,UI,UI],'native update did not execute before overlay'
update();assert len(created)==2 and len(items)==2,'second frame reallocated sprites'
# A respawned mining point may switch type in the same slot. Repaint with the
# native gold/grey pickaxe entry without allocating or changing its loot filter.
put(POOL+0x80+0x68,10);update()
assert len(created)==2 and items[-1]==(1001,59),'gold mining point retained grey pickaxe'
put(POOL+0x80+0x68,9);update()
assert len(created)==2 and items[-1]==(1001,57),'normal mining point retained gold pickaxe'
# Two different spot types at the same projected position must BOTH remain
# visible. The first stays anchored; only the overlapping icon moves aside.
u.mem_write(POOL+0x80+4,struct.pack('<3f',10.,20.,30.));u.mem_write(POOL+0x80+0x55,b'\1');update()
placed=[p[2]for p in positions[-2:]]
assert placed[0]==(105.,61.) and (abs(placed[0][0]-placed[1][0])>=16 or abs(placed[0][1]-placed[1][1])>=16),placed
assert len(created)==2,'spreading recreated/merged a native point'
u.mem_write(POOL+0x80+4,struct.pack('<3f',-40.,50.,60.));u.mem_write(POOL+0x80+0x55,b'\7');update()
assert positions[-1][2]==(700.,364.),'an uncrowded marker did not return to its true position'
# Walking players and moving monsters must not displace gathering icons.
stock=HEAP+0xe100;objects[LAYOUT,2000]=stock;put(stock+0x54,3);put(stock+0x68,PARENT);put(UI+0x4c,2000)
u.mem_write(stock+0x90,struct.pack('<4f',709.,373.,0.,1.));update()
assert positions[-1][2]==(700.,364.),'a visible stock marker displaced a gathering icon'
for x,y in [(700.,364.),(0.,0.),(709.,373.)]:
 u.mem_write(stock+0x90,struct.pack('<4f',x,y,0.,1.));update();assert positions[-1][2]==(700.,364.),'a moving stock marker displaced a gathering icon'
put(stock+0x54,1);update();assert positions[-1][2]==(700.,364.),'an invisible stock marker reserved space'
put(UI+0x4c,0xffffffff);update();assert positions[-1][2]==(700.,364.)
assert not [m for m in mutations if m[2]==2000],'native marker was mutated'
# Only the depleted node is hidden; unrelated stock markers are never written.
u.mem_write(POOL+0x4e,b'\0\0');start=len(mutations);update()
assert [m for m in mutations[start:] if m[0]=='show']==[('show',LAYOUT,1000,0)]
request(0);start=len(mutations);update()
assert [m for m in mutations[start:] if m[0]=='show']==[('show',LAYOUT,1001,0)]
# A resolver/ID reuse invalidates our ownership before any mutation.
u.mem_write(POOL+0x4e,struct.pack('<h',-1));request(1);update()
objects[LAYOUT,1000]=PARENT+0x800;foreign.add((LAYOUT,1000))
start=len(mutations);update()
assert not [m for m in mutations[start:] if m[1:3]==(LAYOUT,1000)],'foreign ID was mutated'
update();assert len(created)==3,'lost sprite ownership did not permit a fresh allocation'
# Invalid singleton or pool hides only sprites still owned by this callback.
put(POP+0x24,0);start=len(mutations);update()
assert [m for m in mutations[start:] if m[0]=='show']==[('show',LAYOUT,1002,0),('show',LAYOUT,1001,0)]
put(POP+0x24,POOL);update();put(POP,0);start=len(mutations);update()
assert [m for m in mutations[start:] if m[0]=='show']==[('show',LAYOUT,1002,0),('show',LAYOUT,1001,0)]
put(POP,MAIN+0x17524cc);update()
# The native drop list handles a type-zero honey node through bounded rIPL data.
set_node(2,0,3,(70.,80.,90.));put(POOL+2*0x80+0x38,PAYLOAD)
update();assert items[-1]==(1003,271),'generic honey list did not decode from rIPL'
put(POOL+2*0x80+0x38,PAYLOAD+12);start=len(mutations);update()
assert ('show',LAYOUT,1003,0) in mutations[start:],'out-of-resource drop pointer was accepted'
put(POOL+2*0x80+0x38,PAYLOAD);put(IPL+0x68,8);start=len(mutations);update()
assert not [m for m in mutations[start:] if m[1:3]==(LAYOUT,1003)],'unterminated bounded drop list was accepted'
put(IPL+0x68,12);update()
# Layout cleanup discards metadata without touching children already owned by native destruction.
start=len(mutations);call(hooks[DROP],LAYOUT)
assert destroyed==[LAYOUT] and len(mutations)==start
put(UI+0x24,NEW_LAYOUT);start=len(mutations);update()
assert len(created)==7,'new layout inherited stale sprite IDs'
assert all(m[1]==NEW_LAYOUT for m in mutations[start:]),'old layout metadata leaked'
# Reusing the destroyed layout address is safe after another native cleanup.
call(hooks[DROP],NEW_LAYOUT);put(UI+0x24,LAYOUT);start=len(mutations);update()
assert len(created)==10
assert all(m[2]>=1007 for m in mutations[start:]),'reused layout inherited old sprite IDs'
assert all(m[2]>=1000 for m in mutations),'stock animation ID was mutated'
print('PASS: compiled ARM hooks, fingerprint gate, whole-map native sprites, item atlas/size/priority, projection/slider, reuse, depletion, toggle, ownership loss, bounded drops, invalid pool/singleton and layout cleanup')
