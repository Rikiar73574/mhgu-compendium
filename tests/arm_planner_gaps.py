"""Incomplete recipes, no eligible route and refreshed eligibility retain honest state."""
import os,re,subprocess,sys,tempfile
from pathlib import Path
if len(sys.argv)==1:
 for case in ('empty','zero','no_route','stale','partial'):subprocess.run([sys.executable,__file__,case],check=True)
 raise SystemExit
from unicorn import UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0,UC_ARM_REG_LR,UC_ARM_REG_PC
ROOT=Path(__file__).resolve().parents[1];case=sys.argv[1]
with tempfile.TemporaryDirectory() as d:
 path=Path(d)/'wishlist';text=''
 if case=='zero':
  armors=[s for s in (ROOT/'build/generated/armor_data.rs').read_text().splitlines() if s.startswith('Armor{') and 'material_count:0,' in s]
  assert armors,'expected real armor fixture with no recorded recipe'
  text='armor:'+re.search(r'key:(\d+)',armors[0])[1]+' 0\n'
 elif case in ('no_route','stale','partial'):
  if case=='stale':pattern='name:"LR Ore Materials",quantity:4,points:true'
  else:pattern='name:"Disc Stone",quantity:2,points:false'
  line=next(s for s in (ROOT/'build/generated/weapon_data.rs').read_text().splitlines() if pattern in s and 'LR Ore Materials' in s)
  text=re.search(r'key:(\d+)',line)[1]+' 0\n'
 path.write_text(text);os.environ['COMPENDIUM_PREVIEW_WISHLIST']=str(path)
 sys.path.insert(0,str(ROOT/'tools'));import preview_app as p
 n=p.ns;put=n['put'];M=n['MAIN'];H=n['HEAP']
 for off,ptr,vt in [(0x1858438,H+0x7000,0x173f5b4),(0x18584b8,H+0xa000,0x173f5f8),(0x1897f78,H+0xc000,0x176af48)]:put(M+off,ptr);put(ptr,M+vt)
 put(M+0x1885b58,H+0x3000);put(H+0x301c,H+0x4000);put(M+0x188ca48,H+0x5000);put(M+0x188ae38,H+0x6000)
 allowed=case in ('stale','partial')
 def unlock(uc,at,size,ctx):
  uc.reg_write(UC_ARM_REG_R0,int(allowed and uc.reg_read(UC_ARM_REG_R0)==106));uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
 def eligible(uc,at,size,ctx):
  uc.reg_write(UC_ARM_REG_R0,1);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
 n['u'].hook_add(UC_HOOK_CODE,unlock,begin=M+0x54af8c,end=M+0x54af8c)
 n['u'].hook_add(UC_HOOK_CODE,eligible,begin=M+0x54ce08,end=M+0x54ce08)
 def press(b):p.frame({'kind':'button','bits':b});return n['texts'][:]
 press((1<<10)|(1<<12));press((1<<10)|(1<<12))
 for _ in range(3):press(1<<7)
 press(1<<2)
 for _ in range(100):p.frame()
 if case=='empty':
  assert any('wishlist is empty' in t for t in n['texts']),n['texts']
  assert not any('supplies all' in t for t in n['texts'])
 elif case=='zero':
  assert any('incomplete' in t.lower() for t in n['texts']),n['texts']
  assert not any('supplies all' in t for t in n['texts'])
 elif case=='no_route':
  assert 'Disc Stone' in n['texts'] and 'LR Ore Materials' in n['texts'],n['texts']
  assert 'Need 2' in n['texts'] and any('pt unallocated' in t for t in n['texts']),n['texts']
  assert not any('Wipe Out!' in t for t in n['texts'])
  press(1);assert 'ITEM SOURCES' not in n['texts'] and 'Eligible point materials' not in n['texts']
 elif case=='partial':
  assert 'Wipe Out!' in n['texts'],n['texts']
  assert 'Disc Stone' in n['texts'] and 'LR Ore Materials' in n['texts'],n['texts']
  assert any('Not available from this quest' in t for t in n['texts']),n['texts']
 else:
  assert 'Wipe Out!' in n['texts'],n['texts']
  allowed=False;press(1<<2);press(1)
  assert 'Eligible point materials' not in n['texts'] and 'ITEM SOURCES' not in n['texts'],n['texts']
  assert 'Wipe Out!' not in n['texts'],n['texts']
  for _ in range(100):p.frame()
  assert 'LR Ore Materials' in n['texts'] and any('pt unallocated' in t for t in n['texts']),n['texts']
 print('ARM planner '+case+' passed')
