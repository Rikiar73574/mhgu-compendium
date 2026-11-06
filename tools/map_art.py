from paths import ROOT,DATA,GENERATED,database
"""Bake cached location artwork and local game minimaps for Forge."""
import json
from pathlib import Path
from PIL import Image,ImageOps
root=Path(__file__).resolve().parents[1]
locations=json.loads((DATA/'locations.json').read_text())['locations']
def rectangles(im):
 active={};rects=[];w,h=im.size
 for y in range(h):
  row={};x=0
  while x<w:
   rgba=im.getpixel((x,y));end=x+1
   while end<w and im.getpixel((end,y))==rgba:end+=1
   key=(x,end,rgba)
   if rgba[3]:
    if key in active:rect=active.pop(key);rect[3]+=1
    else:rect=[x,y,end-x,1,rgba[0]|rgba[1]<<8|rgba[2]<<16|rgba[3]<<24]
    row[key]=rect
   x=end
  rects.extend(active.values());active=row
 rects.extend(active.values());return rects
lines=['// Generated offline; source attribution is in data/location-art and location-maps.']
for name in ['ART','MAPS']:
 lines.append(f'pub static {name}:&[&[(u8,u8,u8,u8,u32)]]=&[')
 for loc in locations:
  id=loc['_id']%100
  path=DATA/(f"location-art/{loc['_id']}.img" if name=='ART' else f'location-maps/{id}.png')
  rects=[]
  if path.exists():
   if name=='ART':im=ImageOps.fit(Image.open(path).convert('RGB'),(96,32)).quantize(colors=24).convert('RGBA')
   else:im=Image.open(path).convert('RGBA').resize((96,96),Image.Resampling.NEAREST)
   rects=rectangles(im)
  lines.append('&['+','.join('('+','.join(map(str,r))+')' for r in rects)+'],')
 lines.append('];')
(GENERATED/'map_pixels.rs').write_text('\n'.join(lines))
print('Map art and minimaps baked')
