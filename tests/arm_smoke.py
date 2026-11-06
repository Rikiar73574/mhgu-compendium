"""Run the built ARM NRO's ELF against actual main NSO import metadata and mocked HID/UI."""
import sys,struct
from pathlib import Path
from elftools.elf.elffile import ELFFile
from unicorn import Uc,UC_ARCH_ARM,UC_MODE_ARM,UC_HOOK_CODE
from unicorn.arm_const import *
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
from nso import Nso
ROOT=Path(__file__).resolve().parents[1]
BASE,MAIN,STUB,STACK,HEAP,STOP=0x4000000,0x10000000,0x3000000,0xa000000,0xb000000,0x3100000
u=Uc(UC_ARCH_ARM,UC_MODE_ARM)
u.ctl_set_cpu_model(UC_CPU_ARM_MAX)
for address,size in [(BASE,0x1000000),(MAIN,0x2200000),(STUB,0x10000),(STACK,0x10000),(HEAP,0x10000),(STOP,0x1000)]:u.mem_map(address,size)
u.reg_write(UC_ARM_REG_C1_C0_2,0xf00000);u.reg_write(UC_ARM_REG_FPEXC,0x40000000)
def put(a,v):u.mem_write(a,struct.pack('<I',v))
def get(a):return struct.unpack('<I',u.mem_read(a,4))[0]
import os
n=Nso(str(Path(os.environ['MHGU_GAME_DIR'])/'exefs/main'));u.mem_write(MAIN,bytes(n.image))
imports={};symbols={};raw=0;key_pressed=set();touch=None;texts=[];drawops=[]
with (ROOT/'mod/target/armv7a-none-eabi/release/libhunters_compendium.so').open('rb') as f:
 e=ELFFile(f)
 for s in e.iter_segments():
  if s['p_type']=='PT_LOAD':u.mem_write(BASE+s['p_vaddr'],s.data())
 dyn=e.get_section_by_name('.dynsym')
 for s in dyn.iter_symbols():
  if not s.name:continue
  if s['st_shndx']=='SHN_UNDEF':
   at=STUB+4*len(imports);imports[at]=s.name;symbols[s.name]=at;u.mem_write(at,bytes.fromhex('1eff2fe1'))
  else:symbols[s.name]=BASE+s['st_value']
 for section in ['.rel.dyn','.rel.plt']:
  rs=e.get_section_by_name(section)
  if rs is None:continue
  for r in rs.iter_relocations():
   at=BASE+r['r_offset'];typ=r['r_info_type']
   if typ==23:value=BASE+get(at)
   elif typ in [21,22]:value=symbols[dyn.get_symbol(r['r_info_sym']).name]
   else:raise AssertionError(typ)
   put(at,value)
slots=[]
for off,typ,idx,add in n.relocs:
 if 'GetNpadState' in n.syms[idx][0]:
  at=STUB+0x1000+len(slots)*4;imports[at]='HID';u.mem_write(at,bytes.fromhex('1eff2fe1'));put(MAIN+off,at);slots.append(MAIN+off)
IO=HEAP+0x1000
u.mem_write(IO+8,struct.pack('<fffff',1440.,900.,1.,1.,1/60))
def callback(uc,at,size,ctx):
 if 'storage_callback' in globals() and storage_callback(uc,at,size,ctx):return
 name=imports.get(at)
 if not name:return
 r0=uc.reg_read(UC_ARM_REG_R0);r1=uc.reg_read(UC_ARM_REG_R1);sp=uc.reg_read(UC_ARM_REG_SP);out=0
 def fl(v):return struct.unpack('<f',struct.pack('<I',v))[0]
 if name in ['ImDrawList_AddRectFilled','ImDrawList_AddRect','ImDrawList_AddLine','ImDrawList_PushClipRect']:
  coords=[fl(r1),fl(uc.reg_read(UC_ARM_REG_R2)),fl(uc.reg_read(UC_ARM_REG_R3)),fl(get(sp))]
  drawops.append((name,coords,get(sp+4)))
 if name=='ImDrawList_PopClipRect':drawops.append((name,[],0))
 if name=='forge_mem_getMainTextAddr':out=MAIN
 elif name=='forge_input_isDown':out=int(bool(raw&(1<<r0)))
 elif name in ['forge_input_getStickL','forge_input_getStickR']:put(r0,0);put(r1,0)
 elif name=='forge_input_isKeyPressed':out=int(r0 in key_pressed)
 elif name=='forge_input_getTouch':
  if touch is not None:
   uc.mem_write(r0,struct.pack('<f',touch[0]));uc.mem_write(r1,struct.pack('<f',touch[1]));out=1
 elif name=='igGetIO_Nil':out=IO
 elif name in ['igGetForegroundDrawList_Nil','igGetFont']:out=HEAP+0x2000
 elif name=='HID':
  uc.mem_write(r0,bytes(40));put(r0+8,raw);put(r0+16,1000);put(r0+32,1)
 elif name=='ImDrawList_AddText_FontPtr':
  begin,end=get(sp+8),get(sp+12)
  if end>=begin and end-begin<1000:
   value=bytes(uc.mem_read(begin,end-begin)).decode();texts.append(value)
   drawops.append(('text',[fl(uc.reg_read(UC_ARM_REG_R3)),fl(get(sp)),fl(uc.reg_read(UC_ARM_REG_R2))],(get(sp+4),value)))
 uc.reg_write(UC_ARM_REG_R0,out)
u.hook_add(UC_HOOK_CODE,callback,begin=STUB,end=STUB+0xffff)
def call(at,*args):
 u.reg_write(UC_ARM_REG_SP,STACK+0xf000);u.reg_write(UC_ARM_REG_LR,STOP)
 for reg,value in zip([UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2,UC_ARM_REG_R3],args):u.reg_write(reg,value)
 try:u.emu_start(at,STOP,count=10000000)
 except Exception:
  pc=u.reg_read(UC_ARM_REG_PC);print('Failed at',hex(pc),'opcode',bytes(u.mem_read(pc,4)).hex());raise
 assert u.reg_read(UC_ARM_REG_PC)==STOP,'instruction budget exhausted'
def hid(bits):
 global raw
 raw=bits;put(HEAP+0x100,0);call(get(slots[0]),HEAP,HEAP+0x100);return get(HEAP+8)
def render():
 texts.clear();drawops.clear();call(symbols['forge_onImGuiFreeRender']);return texts[:]
call(symbols['forge_onInit'])
assert all(BASE<=get(s)<BASE+0x1000000 for s in slots),'not all input imports redirected'
assert hid(1<<10)==0
assert hid((1<<10)|(1<<12))==0
assert get(HEAP+16)==0,'stick leaked while open'
def press(b):
 hid(0);render();hid(b);return render()
def preview(name):
 from PIL import Image,ImageDraw,ImageFont
 im=Image.new('RGB',(1440,900),'#12181b');clips=[(0,0,1440,900)]
 def color(c):return (c&255,(c>>8)&255,(c>>16)&255,(c>>24)&255)
 for kind,coords,arg in drawops:
  if kind=='ImDrawList_PushClipRect':
   a=coords;b=clips[-1];clips.append((max(a[0],b[0]),max(a[1],b[1]),min(a[2],b[2]),min(a[3],b[3])));continue
  if kind=='ImDrawList_PopClipRect':clips.pop();continue
  layer=Image.new('RGBA',im.size);paint=ImageDraw.Draw(layer)
  if kind=='text':
   x,y,size=coords;font=ImageFont.truetype('/usr/share/fonts/TTF/DejaVuSans.ttf',max(1,int(size)))
   paint.text((x,y),arg[1],font=font,fill=color(arg[0]),anchor='lt')
  elif kind=='ImDrawList_AddLine':paint.line(coords,fill=color(arg),width=1)
  elif kind=='ImDrawList_AddRect':paint.rectangle(coords,outline=color(arg),width=1)
  elif kind=='ImDrawList_AddRectFilled':
   if coords[2]>=coords[0] and coords[3]>=coords[1]:paint.rectangle(coords,fill=color(arg))
  box=tuple(int(v) for v in clips[-1])
  if box[2]>box[0] and box[3]>box[1]:im.paste(layer.crop(box),box,layer.crop(box))
 im.save(ROOT/'build'/name)
SELECT,BACK=1,2 # Physical Xbox B / A
hid(0);out=render();assert 'MHGU Compendium' in out and 'Item list' in out,out
press(SELECT) # Open supplies category.
out=press(SELECT);assert 'ITEM SOURCES' in out,out
out=press(BACK);assert 'ITEM SOURCES' not in out
out=press(1<<7);assert 'Weapon trees' in out and 'Great Sword' in out
# Choose Lance (type position 6) using two grid-row moves.
press(1<<15);press(1<<15);out=press(SELECT);assert 'Lance' in out and any('Lv1' in s for s in out)
preview('weapon-tree-preview.png')
out=press(SELECT);assert 'Crafting materials' in out and 'Weapon stats' in out
press(BACK)
# Locations list then Jurassic Frontier via current-quest detection on opening.
press(1<<7);assert 'Locations' in render()
press((1<<10)|(1<<12));assert render()==[]
gc,sq,qd=HEAP+0x4000,HEAP+0x5000,HEAP+0x6000
put(MAIN+0x188ca48,gc);u.mem_write(gc+0x4c,b'\xff');put(MAIN+0x188ad90,sq);put(sq+0xa4,qd);put(sq+0xe0,207);put(qd+8,207)
out=press((1<<10)|(1<<12));assert 'Jurassic Frontier' in out and 'Current quest location' in out,out
preview('location-preview.png')
# Cycle ranks and zones, open a gathering item and navigate back.
press(1<<9);out=press(1<<14);assert any('Area' in s for s in out)
out=press(SELECT);assert 'ITEM SOURCES' in out
press(BACK);out=press(1<<3);assert 'Controller search' in out
press(SELECT);out=press(BACK);assert any('Search: A' in s for s in out)
press((1<<10)|(1<<12));assert render()==[]
assert hid(1)==0;hid(0);assert hid(1)==1
print('ARM smoke passed: type selection, connected tree, context map/rank, zones, gathering links, search, input drain')

# Desktop path on the existing recomp: raw keyboard and mouse-as-touch API.
hid(0)
def key(code):
 key_pressed.add(code);render();key_pressed.clear();return render()
def click(x,y):
 global touch
 # 1440x900 window with a centered 1440x810 game picture.
 touch=((176+x*.85)/1.125,(110+y*.85-45)/1.125)
 render();touch=None;return render()
key(63) # F6 opens
out=click(70,120);assert 'Item list' in out
click(800,70) # Search focus
for code in [19,18,23,12,18,17]:key(code) # potion
out=render();assert any('Search: potion_' in t for t in out),out
key(40) # Enter exits editing
out=click(300,145);assert 'ITEM SOURCES' in out,out
out=key(41);assert 'ITEM SOURCES' not in out
out=click(70,170);assert 'Weapon trees' in out
out=click(250,150);assert any('Lv1' in t for t in out)
click(340,20) # Back
out=click(70,225);assert 'Locations' in out
if not any('Area' in t for t in out):out=click(250,150)
assert any('gathering records' in t for t in out) or any('Area' in t for t in out),out
out=click(560,70) # shared HR selector
assert any('Area' in t for t in out),out
key(63);assert render()==[]
print('Desktop ARM smoke passed: F6, typed search, mouse items/type/tree/location links, Escape/back')

# Typing directly must leave the controller keyboard and suppress mapped WASD.
key(63);click(70,120)
out=press(1<<3);assert 'Controller search' in out
hid(1<<13);out=key(26) # W also arrives as the game's Up binding.
hid(0);out=render();assert 'Controller search' not in out and 'Search: w_' in out,out
for code in [4,22,7]:key(code)
assert 'Search: wasd_' in render()
key(42);assert 'Search: was_' in render()
key(40);out=render();assert 'Search: was' in out
key(63);assert render()==[]
print('Direct typing passed: physical WASD edits search and dismisses controller keyboard')

# Render the exact Arctic Ridge area from the reported screenshot.
import sqlite3,json
cx=sqlite3.connect(Path(os.environ['MHGU_DATABASE']).resolve().as_uri()+'?mode=ro',uri=True)
qid=cx.execute("select _id from quests where location_id=3 and rank='LR' limit 1").fetchone()[0]
put(sq+0xe0,qid);put(qd+8,qid);key(63)
out=render();assert 'Arctic Ridge' in out,out
assert not any('(Fixed)' in t or '(Rare)' in t for t in out),out
rows=json.loads((ROOT/'build/data/locations.json').read_text())['gathering']
expected={r['name'] for r in rows if r['location_id']==3 and r['rank']=='LR' and r['area']=='1'}
# Areas in the source are labeled 'Area 1'.
expected={r['name'] for r in rows if r['location_id']==3 and r['rank']=='LR' and r['area']=='Area 1'}
assert expected and expected.issubset(set(out)),expected-set(out)
preview('location-preview.png')
key(63)
print('Arctic Ridge: every Area 1 item visible together, no Fixed/Rare headings')
