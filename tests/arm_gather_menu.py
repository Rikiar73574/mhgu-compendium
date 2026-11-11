"""Compiled checklist rendering and input against copied synthetic quest tables."""
import sys,struct
from pathlib import Path
from elftools.elf.elffile import ELFFile
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT/'tools'))
import preview_app as p
n=p.ns;u=n['u'];put=n['put'];M=n['MAIN'];H=n['HEAP']
u.mem_map(H+0x10000,0x40000)
with (ROOT/'mod/target/armv7a-none-eabi/release/libhunters_compendium.so').open('rb') as f:
 e=ELFFile(f);enabled=next(s['st_value'] for s in e.get_section_by_name('.symtab').iter_symbols() if 'gather_map7ENABLED' in s.name)
u.mem_write(n['BASE']+enabled,b'\1')
GC,SQ,QD,POP,IPL,PAY,POOL=[H+x for x in [0x8000,0x8100,0x8300,0x8500,0x9000,0xb000,0x10000]]
put(M+0x188ca48,GC);u.mem_write(GC+0x4c,b'\xff');put(M+0x188ad90,SQ)
put(SQ+0xa4,QD);put(SQ+0xe0,207);put(QD+8,207)
put(M+0x1889ec0,POP);put(POP,M+0x17524cc);put(POP+0x24,POOL);put(POP+0x20,IPL)
put(IPL,M+0x175245c);put(IPL+0x64,PAY);put(IPL+0x68,64)
def node(slot,kind,rows):
 raw=bytearray(128);struct.pack_into('<I3f',raw,0,M+0x17523e0,10.,20.,30.)
 struct.pack_into('<h',raw,0x4e,2);raw[0x55]=slot+1;raw[0x70]=3
 struct.pack_into('<II',raw,0x64,2 if kind==9 else 0,kind)
 at=PAY+slot*20;struct.pack_into('<I',raw,0x38,at);u.mem_write(POOL+slot*128,bytes(raw))
 u.mem_write(at,b''.join(struct.pack('<BBH',*r) for r in [*rows,(255,0,0)]))
node(0,9,[(80,1,318),(10,1,324),(10,2,324)])
node(1,9,[(60,1,318),(40,1,324)])
node(2,11,[(100,1,271)])
p.frame();texts=n['texts']
for label in ['Compendium','Mining','Iron Ore','Lightcrystal','Honey','0 / 3 selected','60% to 80%','20% to 40%']:
 assert label in texts,(label,texts)
def press(bits):p.frame({'kind':'button','bits':bits})
press(1);assert '2 / 3 selected' in n['texts'],'category must select all of its items'
press(1<<15);press(1);assert '1 / 3 selected' in n['texts'],'individual checkbox must remain independent'
press(2);assert 'Compendium' not in n['texts'],'back must close the checklist'
p.frame({'kind':'key','key':63});assert '1 / 3 selected' in n['texts'],'reopening lost selection'
# A mouse/touch click uses the same category/item actions as the controller.
op=next(o for o in n['drawops'] if o[0]=='text' and o[2][1]=='All gathering')
p.frame({'kind':'click','x':op[1][0]+4,'y':op[1][1]+4})
assert '3 / 3 selected' in n['texts'],'mouse select-all did not use checkbox action'
op=next(o for o in n['drawops'] if o[0]=='text' and o[2][1]=='All gathering')
p.frame({'kind':'click','x':op[1][0]+4,'y':op[1][1]+4})
assert '0 / 3 selected' in n['texts'],'mouse select-all did not clear selections'
press(1);assert '1 / 3 selected' in n['texts']
# Depletion retains the catalogue and checkbox state; native marker availability
# is tested separately by arm_gather_map.py.
u.mem_write(POOL+128+0x4e,b'\0\0')
for _ in range(65):p.frame()
assert '1 / 3 selected' in n['texts'] and 'Lightcrystal' in n['texts']
p.frame({'kind':'key','key':41});assert 'Compendium' not in n['texts']
print('PASS: native table odds/ranges, category and item toggles, controller close/reopen, depletion, keyboard Escape')
