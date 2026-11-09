"""Prepare native MHGU frames and a proportional font from an owned game dump.

Generated art/font bytes stay under build/. No texture or font is downloaded.
"""
import collections,io,math,os,struct,zlib
from pathlib import Path
from PIL import Image
from fontTools.fontBuilder import FontBuilder
from fontTools.pens.ttGlyphPen import TTGlyphPen
from paths import DATA,GENERATED
from extract_map_pngs import decode,decode_bytes

def font_bytes(glyphs,height=24,baseline=20):
    names={code:f'uni{code:04X}' for code in glyphs}
    fb=FontBuilder(height*64,isTTF=True)
    fb.setupGlyphOrder(['.notdef']+list(names.values()))
    fb.setupCharacterMap(names)
    shapes={'.notdef':TTGlyphPen(None).glyph()};metrics={'.notdef':(height*32,0)}
    for code,(mask,advance) in glyphs.items():
        filled={(x,y) for y in range(mask.height) for x in range(mask.width) if mask.getpixel((x,y))>=128}
        edges=collections.defaultdict(set)
        for x,y in filled:
            for a,b,neighbor in [((x,y),(x+1,y),(x,y-1)),((x+1,y),(x+1,y+1),(x+1,y)),
                                 ((x+1,y+1),(x,y+1),(x,y+1)),((x,y+1),(x,y),(x-1,y))]:
                if neighbor not in filled:edges[a].add(b)
        pen=TTGlyphPen(None)
        while edges:
            start=min(edges);points=[start];current=start
            while True:
                target=min(edges[current]);edges[current].remove(target)
                if not edges[current]:del edges[current]
                if target==start:break
                points.append(target);current=target
            corners=[]
            for i,p in enumerate(points):
                a=points[i-1];b=points[(i+1)%len(points)]
                if (p[0]-a[0])*(b[1]-p[1])!=(p[1]-a[1])*(b[0]-p[0]):corners.append(p)
            if len(corners)<3:continue
            pen.moveTo((corners[0][0]*64,(baseline-corners[0][1])*64))
            for x,y in corners[1:]:pen.lineTo((x*64,(baseline-y)*64))
            pen.closePath()
        shapes[names[code]]=pen.glyph();metrics[names[code]]=(advance*64,0)
    fb.setupGlyf(shapes);fb.setupHorizontalMetrics(metrics)
    fb.setupHorizontalHeader(ascent=baseline*64,descent=(baseline-height)*64)
    fb.setupNameTable({'familyName':'MHGU Field Guide','styleName':'Regular','uniqueFontIdentifier':'MHGU-local-glyphs',
                       'fullName':'MHGU Field Guide','psName':'MHGUFieldGuide'})
    fb.setupOS2(sTypoAscender=baseline*64,sTypoDescender=(baseline-height)*64,usWinAscent=baseline*64,usWinDescent=(height-baseline)*64)
    fb.setupPost();fb.setupMaxp()
    fb.font['head'].created=fb.font['head'].modified=2082844800
    out=io.BytesIO();fb.save(out);return out.getvalue()

def archive(path):
    data=path.read_bytes();assert data[:4]==b'ARC\0'
    result={}
    for i in range(struct.unpack_from('<H',data,6)[0]):
        start=12+i*80;name=data[start:start+64].split(b'\0')[0].decode()
        _,size,_,offset=struct.unpack_from('<4I',data,start+64)
        if 'font_festa_' in name:result[name]=data[offset:offset+size]
    return result

def font_page(blob):
    """Native TEX v160 RGBA8 font pages, with Tegra block-linear storage."""
    data=zlib.decompress(blob);magic,a,b,c,_=struct.unpack_from('<5I',data)
    assert magic==0x584554 and a&4095==160
    if c>>8&255 in (19,23):return decode_bytes(data).getchannel('A')
    assert c>>8&255==7
    w,h=(b>>6)&8191,b>>19;row=w*4;bh=min(16,1<<max(0,math.ceil(math.log2(max(1,h/8)))))
    src=data[20+4*(b&63):];out=bytearray(row*h);wg=(row+63)//64
    for y in range(h):
        for x in range(0,row,16):
            xi,yi=x%64,y%8
            at=(y//(8*bh))*wg*512*bh+(x//64)*512*bh+(y//8%bh)*512
            at+=(xi//32)*256+(yi//2)*64+(xi%32//16)*32+(yi%2)*16+xi%16
            out[y*row+x:y*row+x+16]=src[at:at+16]
    return Image.frombytes('RGBA',(w,h),bytes(out)).getchannel('A')

def prepare(game):
    root=game/'romfs/nativeNX';tex=root/'eng/GUI/99_texture'
    lobby=decode(tex/'HD_lobby01_BM_NOMIP.tex')
    lobby0=decode(tex/'HD_lobby00_BM_NOMIP.tex')
    cache=DATA/'game-ui';cache.mkdir(exist_ok=True)
    images=[lobby.crop((0,386,96,480)),lobby.crop((400,136,464,200)),lobby0.crop((0,272,64,336)),lobby0.crop((160,272,161,336)),lobby0.crop((328,272,424,336))]
    images += [lobby0.crop((160+i*28,64,188+i*28,92)) for i in range(4)]
    # Keep small native corner strips; stretch only their center in the renderer.
    out=["pub struct Sprite{pub w:u16,pub h:u16,pub runs:&'static[(u8,u8,u8,u8,u32)]}", 'pub static SPRITES:&[Sprite]=&[']
    for index,im in enumerate(images):
        im=im.convert('RGBA');im.save(cache/f'{index}.png')
        runs=[]
        # Native texture detail is retained at RGB565 precision. Alpha stays exact.
        im.putdata([((r>>3)*8,(g>>2)*4,(b>>3)*8,a) for r,g,b,a in im.get_flattened_data()])
        for y in range(im.height):
            x=0
            while x<im.width:
                color=im.getpixel((x,y));end=x+1
                while end<im.width and im.getpixel((end,y))==color and end-x<255:end+=1
                if color[3]:runs.append((x,y,end-x,int.from_bytes(bytes(color),'little')))
                x=end
        rectangles=[];previous={}
        for x,y,w,color in runs:
            key=(x,w,color);old=previous.get(key)
            if old is not None and rectangles[old][1]+rectangles[old][3]==y:
                rectangles[old][3]+=1
            else:
                old=len(rectangles);rectangles.append([x,y,w,1,color])
            previous[key]=old
        runs=[tuple(r) for r in rectangles]
        out.append(f'Sprite{{w:{im.width},h:{im.height},runs:&['+','.join(str(r).replace(' ','') for r in runs)+']},')
    out+= ['];']
    font_dir=Path(os.environ['MHGU_FONT_DIR']) if os.environ.get('MHGU_FONT_DIR') else None
    gfd=(font_dir/'font_festa.gfd' if font_dir else root/'eng/GUI/02_common/font/font_festa.gfd').read_bytes()
    assert gfd[:4]==b'GFD\0'
    height=struct.unpack_from('<I',gfd,20)[0];assert 16<=height<=96
    count=struct.unpack_from('<I',gfd,28)[0];start=len(gfd)-count*20
    pages=archive(root/'loc/arc/resident.arc') if not font_dir else {};decoded={};glyphs={};baseline=20
    chars=set(range(32,384))|{0x00d7,0x2014,0x2015,0x2019,0x2026,0x2190,0x2192,0x25b6,0x2605}
    for offset in range(start,len(gfd),20):
        code,pos,size,metric,_=struct.unpack_from('<5I',gfd,offset)
        if code not in chars:continue
        page=pos&255
        if page not in decoded:
            blob=(font_dir/f'font_festa_{page:02}_HQ.tex.z').read_bytes() if font_dir else pages[f'GUI\\02_common\\font\\font_festa_{page:02}_HQ']
            decoded[page]=font_page(blob)
        x,y=pos>>8&4095,pos>>20&4095;w,h=size&4095,size>>12&4095
        glyphs[code]=(decoded[page].crop((x,y,x+w,y+h)),metric&4095)
        if code==ord('A'):baseline=metric>>24
    font=font_bytes(glyphs,height,baseline);(cache/'native.ttf').write_bytes(font)
    out.append('pub static FONT:&[u8]=&['+','.join(map(str,font))+'];')
    (GENERATED/'game_ui.rs').write_text('\n'.join(out)+'\n')
    print('Native UI:',len(images),'sprites,',len(glyphs),'glyphs,',len(font),'font bytes')

if __name__=='__main__':prepare(Path(os.environ['MHGU_GAME_DIR']))
