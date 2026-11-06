"""Interactive browser preview using the compiled ARM mod and mocked game input."""
import ast,json,threading
from http.server import BaseHTTPRequestHandler,HTTPServer
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
source=ROOT/'tests/arm_smoke.py'
# Only host-import stubs need Python callbacks; let Unicorn JIT the ARM code.
harness=source.read_text().replace('u.hook_add(UC_HOOK_CODE,callback)', 'u.hook_add(UC_HOOK_CODE,callback,begin=STUB,end=STUB+0xffff)')
tree=ast.parse(harness)
prefix=[]
for node in tree.body:
 if isinstance(node,ast.Expr) and isinstance(node.value,ast.Call) and isinstance(node.value.func,ast.Name) and node.value.func.id=='call':break
 prefix.append(node)
ns={'__file__':str(source),'__name__':'compendium_preview'}
exec(compile(ast.Module(body=prefix,type_ignores=[]),str(source),'exec'),ns)
import os,preview_storage
preview_storage.install(ns,Path(os.environ.get('COMPENDIUM_PREVIEW_WISHLIST',str(ROOT/'build/preview-wishlist.txt'))))
ns['call'](ns['symbols']['forge_onInit'])
ns['hid'](1<<10);ns['hid']((1<<10)|(1<<12));ns['render']();ns['hid'](0);ns['render']()
def frame(action=None):
 if action:
  kind=action.get('kind')
  if kind=='button':
   ns['hid'](int(action['bits']));ns['render']();ns['hid'](0)
  elif kind=='key':
   ns['key_pressed'].add(int(action['key']));ns['render']();ns['key_pressed'].clear()
  elif kind=='click':
   x,y=float(action['x']),float(action['y'])
   ns['touch']=(x/1.125,(y-45)/1.125);ns['render']();ns['touch']=None
  elif kind=='wheel':
   ns['u'].mem_write(ns['IO']+232,ns['struct'].pack('<f',float(action['delta'])));ns['render']();ns['u'].mem_write(ns['IO']+232,bytes(4))
 ns['render']()
 # Qt paints asynchronously. Never expose the mutable list cleared by render().
 return list(ns['drawops'])
PAGE='''<!doctype html><meta charset="utf-8"><title>Compendium Preview</title><style>
*{box-sizing:border-box}body{margin:0;background:#12181b;color:#e6ecee;font:14px system-ui}header{padding:12px 20px;display:flex;align-items:center;gap:16px;border-bottom:1px solid #35474a}h1{font-size:16px;margin:0;color:#45ccef}span{color:#9badb1}button{background:#242d30;border:1px solid #35474a;color:#e6ecee;border-radius:4px;padding:7px 12px;cursor:pointer}button:hover{border-color:#45ccef}canvas{display:block;width:min(100%,calc((100vh - 110px)*1.6),1088px);height:auto;margin:auto;outline:0}#state{margin-left:auto}footer{text-align:center;color:#9badb1;padding:8px}</style>
<header><h1>Compendium Preview</h1><span>Compiled mod renderer • simulated game input</span><button onclick="send({kind:'button',bits:64})">Previous section</button><button onclick="send({kind:'button',bits:128})">Next section</button><button onclick="send({kind:'button',bits:2})">Back</button><span id="state">Ready</span></header>
<canvas id="screen" width="1088" height="680" tabindex="0"></canvas><footer>Click to browse. Type to search. Enter selects, Esc goes back. Arrow keys navigate. [ and ] change rank. F6 toggles.</footer>
<script>
const canvas=document.querySelector('canvas'),ctx=canvas.getContext('2d');let chain=Promise.resolve();
function color(n){return `rgba(${n&255},${(n>>>8)&255},${(n>>>16)&255},${(n>>>24)/255})`}
function draw(ops){ctx.setTransform(1,0,0,1,0,0);ctx.fillStyle='#12181b';ctx.fillRect(0,0,1088,680);ctx.translate(-176,-110);for(const[k,a,b]of ops){if(k==='ImDrawList_PushClipRect'){ctx.save();ctx.beginPath();ctx.rect(a[0],a[1],a[2]-a[0],a[3]-a[1]);ctx.clip();continue}if(k==='ImDrawList_PopClipRect'){ctx.restore();continue}if(k==='text'){ctx.font=`${a[2]}px sans-serif`;ctx.textBaseline='top';ctx.fillStyle=color(b[0]);ctx.fillText(b[1],a[0],a[1]);continue}ctx.fillStyle=ctx.strokeStyle=color(b);ctx.lineWidth=1;if(k==='ImDrawList_AddLine'){ctx.beginPath();ctx.moveTo(a[0],a[1]);ctx.lineTo(a[2],a[3]);ctx.stroke()}else if(k==='ImDrawList_AddRectFilled'){ctx.fillRect(a[0],a[1],a[2]-a[0],a[3]-a[1])}else if(k==='ImDrawList_AddRect'){ctx.strokeRect(a[0],a[1],a[2]-a[0],a[3]-a[1])}}}
function send(a){chain=chain.then(async()=>{document.querySelector('#state').textContent='Rendering';const r=await fetch('/frame',{method:'POST',body:JSON.stringify(a)});if(!r.ok)throw Error(await r.text());draw(await r.json());document.querySelector('#state').textContent='Ready'}).catch(e=>document.querySelector('#state').textContent=e.message)}
canvas.addEventListener('click',e=>{canvas.focus();const r=canvas.getBoundingClientRect();send({kind:'click',x:176+(e.clientX-r.left)*1088/r.width,y:110+(e.clientY-r.top)*680/r.height})});
const keys={Enter:40,Escape:41,Backspace:42,Tab:43,' ':44,'-':45,"'":52,ArrowRight:79,ArrowLeft:80,ArrowDown:81,ArrowUp:82,PageUp:75,PageDown:78,F6:63};
window.addEventListener('keydown',e=>{if(e.ctrlKey||e.metaKey||e.altKey)return;const s=e.key.toLowerCase();let code=keys[e.key];if(/^[a-z]$/.test(s))code=s.charCodeAt(0)-97+4;if(/^[0-9]$/.test(s))code=s==='0'?39:29+Number(s);if(s==='['||s===']'){e.preventDefault();send({kind:'button',bits:s==='['?256:512});return}if(code!==undefined){e.preventDefault();send({kind:'key',key:code})}});
canvas.addEventListener('wheel',e=>{e.preventDefault();send({kind:'wheel',delta:e.deltaY>0?-1:1})},{passive:false});send({});
</script>'''
class Handler(BaseHTTPRequestHandler):
 def do_GET(self):
  self.send_response(200);self.send_header('Content-Type','text/html; charset=utf-8');self.end_headers();self.wfile.write(PAGE.encode())
 def do_POST(self):
  try:
   data=json.loads(self.rfile.read(int(self.headers.get('Content-Length','0'))))
   out=json.dumps(frame(data)).encode();self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(out)
  except Exception as e:
   self.send_response(500);self.end_headers();self.wfile.write(str(e).encode())
 def log_message(self,*args):pass
if __name__=='__main__':
 print('Compendium preview: http://127.0.0.1:8770',flush=True)
 HTTPServer(('127.0.0.1',8770),Handler).serve_forever()
