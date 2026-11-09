"""Compiled planner excludes locked quests and returns from sources to its route."""
import os,re,sys,tempfile,ast
from pathlib import Path
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0,UC_ARM_REG_LR,UC_ARM_REG_PC
ROOT=Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory() as d:
 path=Path(d)/'wishlist';line=next(s for s in (ROOT/'build/generated/weapon_data.rs').read_text().splitlines() if s.startswith('Weapon{'))
 path.write_text(re.search(r'key:(\d+)',line)[1]+' 0\narmor:1310721 0\n');os.environ['COMPENDIUM_PREVIEW_WISHLIST']=str(path)
 sys.path.insert(0,str(ROOT/'tools'));import preview_app as p
 n=p.ns;put=n['put'];M=n['MAIN'];H=n['HEAP']
 # A complete valid inventory snapshot with no stock, plus town context.
 for off,ptr,vt in [(0x1858438,H+0x7000,0x173f5b4),(0x18584b8,H+0xa000,0x173f5f8),(0x1897f78,H+0xc000,0x176af48)]:put(M+off,ptr);put(ptr,M+vt)
 put(H+0x701a,318|(3<<16));put(M+0x1885b58,H+0x3000);put(H+0x301c,H+0x4000);put(M+0x188ca48,H+0x5000);put(M+0x188ae38,H+0x6000)
 checked=[];entry_checked=[]
 def unlock(uc,at,size,ctx):
  q=uc.reg_read(UC_ARM_REG_R0);checked.append(q)
  uc.reg_write(UC_ARM_REG_R0,int(q in (106,1110103)));uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
 n['u'].hook_add(UC_HOOK_CODE,unlock,begin=M+0x54af8c,end=M+0x54af8c)
 # Downloaded HR4 event: story/download unlocked, rank requirement still fails.
 put(H+0x2864,1);put(H+0x286c,H+0x2900);n['u'].mem_write(H+0x2916,b'\x03')
 def lookup(uc,at,size,ctx):
  uc.reg_write(UC_ARM_REG_R0,H+0x2800);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
 n['u'].hook_add(UC_HOOK_CODE,lookup,begin=M+0x3ba50c,end=M+0x3ba50c)
 def eligibility(uc,at,size,ctx):
  resource=uc.reg_read(UC_ARM_REG_R0)
  data=n['get'](resource+0x6c)
  assert n['get'](resource+0x64)==1
  # The character is HR1. Entry requirement 3 (HR4+) must be refused.
  entry=bytes(uc.mem_read(data+0x16,1))[0];entry_checked.append(entry);allowed=entry==0
  uc.reg_write(UC_ARM_REG_R0,1 if allowed else 3);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
 n['u'].hook_add(UC_HOOK_CODE,eligibility,begin=M+0x54ce08,end=M+0x54ce08)
 # Close/reopen to refresh inventory.
 def press(b):p.frame({'kind':'button','bits':b});return n['texts'][:]
 press((1<<10)|(1<<12));press((1<<10)|(1<<12))
 for _ in range(3):press(1<<7)
 press(1<<2)
 for _ in range(100):p.frame()
 assert 0 in entry_checked and 3 in entry_checked,'planner skipped native HR eligibility'
 assert checked and len(checked)==len(set(checked)),len(checked)
 assert 'Wipe Out!' in n['texts'],n['texts']
 assert 'Advanced: Invoking Rath' not in n['texts']
 assert 'Need 2' in n['texts'] and 'Need 1' in n['texts'],n['texts']
 assert 'ITEM SOURCES' in press(1),n['texts']
 assert 'Wipe Out!' in press(2),n['texts']
 source=ROOT/'tests/arm_smoke.py';fn=next(x for x in ast.parse(source.read_text()).body if isinstance(x,ast.FunctionDef)and x.name=='preview')
 exec(compile(ast.Module(body=[fn],type_ignores=[]),str(source),'exec'),n);n['preview']('quest-planner.png')
 print('ARM verified-unlock gate, missing inventory, route and material return passed',len(checked))
