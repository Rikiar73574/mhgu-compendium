"""Exercise Wishlist removal in the compiled ARM UI with isolated file fixtures."""
import ast,os,re,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
if len(sys.argv)==1:
 for case in ['middle','last','filtered','recipe','empty','failed','mouse','mouse_failed']:
  subprocess.run([sys.executable,__file__,case],check=True)
 sys.exit(0)
case=sys.argv[1]
weapons=[line for line in (ROOT/'build/generated/weapon_data.rs').read_text().splitlines() if line.startswith('Weapon{')]
nodes=[line for line in (ROOT/'build/generated/world.rs').read_text().splitlines() if line.startswith('Node{')]
key=lambda i:int(re.search(r'key:(\d+)',weapons[i])[1])
names=[re.search(r'name:"([^"]+)"',line)[1] for line in nodes]
other=next(i for i,name in enumerate(names) if name!=names[0])
fixtures=[(key(i),0) for i in [0,other,1]]
if case=='recipe':
 multi=next(i for i,line in enumerate(weapons) if line.count('Recipe{')>1)
 fixtures=[(key(multi),0),(key(multi),1),(key(1),0)]
with tempfile.TemporaryDirectory(prefix='compendium-remove-') as folder:
 path=Path(folder)/'wishlist.txt'
 initial=fixtures[:1] if case in ['empty','failed','mouse'] else fixtures
 path.write_text(''.join(f'{k} {r}\n' for k,r in initial))
 os.environ['COMPENDIUM_PREVIEW_WISHLIST']=str(path)
 sys.path.insert(0,str(ROOT/'tools'))
 import preview_app as preview
 ns=preview.ns
 def press(bits):preview.frame({'kind':'button','bits':bits});return ns['texts'][:]
 def remaining():return [tuple(map(int,line.split())) for line in path.read_text().splitlines() if line.strip()]
 for _ in range(3):press(1<<7)
 assert 'Wishlist' in ns['texts'],ns['texts']
 if case=='filtered':
  for c in names[0].lower():
   preview.frame({'kind':'key','key':44 if c==' ' else ord(c)-ord('a')+4})
  preview.frame({'kind':'key','key':40})
 if case in ['middle','filtered','last','recipe','mouse_failed']:
  steps=weapons[0].count('Material{')
  if case=='recipe':steps=weapons[multi].split('Recipe{')[1].count('Material{')
  if case=='middle':steps+=1
  if case=='mouse_failed':steps+=weapons[other].count('Material{')-1
  if case=='last':steps+=weapons[other].count('Material{')
  for _ in range(steps):press(1<<15)
 if case=='middle':
  source=ROOT/'tests/arm_smoke.py'
  image=next(node for node in ast.parse(source.read_text()).body if isinstance(node,ast.FunctionDef) and node.name=='preview')
  exec(compile(ast.Module(body=[image],type_ignores=[]),str(source),'exec'),ns)
  ns['preview']('wishlist-remove-preview.png')
 if case in ['failed','mouse_failed']:
  original=ns['storage_callback']
  from unicorn.arm_const import UC_ARM_REG_R0,UC_ARM_REG_R2
  def failing(uc,at,size,ctx):
   if ns['imports'].get(at)=='_ZN2nn2fs8OpenFileEPNS0_10FileHandleEPKci' and uc.reg_read(UC_ARM_REG_R2)==6:
    uc.reg_write(UC_ARM_REG_R0,0x1f4002);return True
   return original(uc,at,size,ctx)
  ns['storage_callback']=failing
 if case in ['mouse','mouse_failed']:
  preview.frame({'kind':'click','x':176+1160*.85,'y':110+164*.85})
 else:press(1<<5)
 expected={'middle':[fixtures[0],fixtures[2]],'recipe':[fixtures[0],fixtures[2]],'last':fixtures[:2],
           'filtered':fixtures[:2],'empty':[],'failed':initial,'mouse':[],'mouse_failed':initial}[case]
 assert remaining()==expected,(case,remaining(),expected,ns['texts'])
 if case=='mouse_failed':
  assert 'Wishlist save failed' in ns['texts'],ns['texts']
  assert ns['texts'].count('Wishlist save failed')==1 and ns['texts'].count('Remove')==2,ns['texts']
  out=press(1);assert 'ITEM SOURCES' in out and 'Earth Crystal' in out,out
  press(2);ns['storage_callback']=original
  press(1<<5);assert remaining()==fixtures[1:],remaining()
 elif case=='failed':
  assert 'Wishlist save failed' in ns['texts'],ns['texts']
  ns['storage_callback']=original
  press(1<<5);assert remaining()==[],remaining()
 elif case in ['empty','mouse']:
  press(1<<5);assert remaining()==[]
  assert 'Your wishlist is empty' in ns['texts'],ns['texts']
 else:
  press(1<<5)
  assert remaining()==([fixtures[0]] if case in ['middle','last','recipe'] else [fixtures[1]]),(case,remaining())
  if case=='filtered':
   assert 'No wishlist recipes match your search' in ns['texts'],ns['texts']
   press(1<<5);assert remaining()==[fixtures[1]],remaining()
 print('ARM Wishlist removal passed:',case)
