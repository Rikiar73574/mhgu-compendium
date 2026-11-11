//! Independent native item sprites under the game's own map parent.
//! No monster slots, map textures or Map Assist hooks are replaced.
use core::{mem,ptr};
use crate::gather;
const NONE:u32=u32::MAX;
#[derive(Clone,Copy)]struct Icon{id:u32,object:u32,item:u16,shown:bool}
const EMPTY_ICON:Icon=Icon{id:NONE,object:0,item:0,shown:false};
#[derive(Clone,Copy)]struct Map{layout:u32,parent:u32,resource:u32,icons:[Icon;gather::CAPACITY]}
const EMPTY_MAP:Map=Map{layout:0,parent:0,resource:0,icons:[EMPTY_ICON;gather::CAPACITY]};
static mut MAPS:[Map;3]=[EMPTY_MAP;3];
static mut SHOW:bool=false;
static mut ENABLED:bool=false;
static mut UPDATE:u32=0;
static mut DROP_LAYOUT:u32=0;
#[repr(C)]struct Hook([u32;19]);
static mut UPDATE_HOOK:Hook=Hook([0;19]);
static mut DROP_HOOK:Hook=Hook([0;19]);
unsafe extern "C"{#[link_name="forge_hook_create"]fn hook(out:*mut Hook,target:u32,detour:u32,original:*mut u32)->*mut Hook;}
fn address(pc:u32)->u32{forge::mem::text_addr()+pc-0x200000}
fn valid(p:u32,size:u32)->bool{p>=0x08000000&&p&3==0&&p.checked_add(size).is_some_and(|end|end<=0xa8000000)}
unsafe fn read(p:u32,offset:u32)->u32{unsafe{ptr::read_unaligned((p+offset)as *const u32)}}
unsafe fn call1(pc:u32,a:u32)->u32{unsafe{let f:unsafe extern "C" fn(u32)->u32=mem::transmute(address(pc));f(a)}}
unsafe fn call2(pc:u32,a:u32,b:u32)->u32{unsafe{let f:unsafe extern "C" fn(u32,u32)->u32=mem::transmute(address(pc));f(a,b)}}
unsafe fn call3(pc:u32,a:u32,b:u32,c:u32)->u32{unsafe{let f:unsafe extern "C" fn(u32,u32,u32)->u32=mem::transmute(address(pc));f(a,b,c)}}
unsafe fn call4(pc:u32,a:u32,b:u32,c:u32,d:u32)->u32{unsafe{let f:unsafe extern "C" fn(u32,u32,u32,u32)->u32=mem::transmute(address(pc));f(a,b,c,d)}}
#[unsafe(no_mangle)]pub extern "C" fn compendium_gather_request(show:u32){unsafe{SHOW=show!=0;}}
unsafe fn owns(layout:u32,parent:u32,icon:&Icon)->bool{unsafe{
 icon.id!=NONE&&valid(icon.object,0xdc)&&call2(0x744fe8,layout,icon.id)==icon.object&&read(icon.object,0x68)==call2(0x744fe8,layout,parent)
}}
unsafe fn hide_icon(layout:u32,parent:u32,icon:&mut Icon){unsafe{
 if !owns(layout,parent,icon){*icon=EMPTY_ICON;return;}
 if icon.shown{call3(0x74533c,layout,icon.id,0);icon.shown=false;}
}}
unsafe fn hide(map:&mut Map){unsafe{for icon in &mut map.icons{hide_icon(map.layout,map.parent,icon);}}}
/// Native layout destruction also owns all child animation instances.
#[unsafe(no_mangle)]pub unsafe extern "C" fn compendium_gather_drop_layout(layout:u32){unsafe{
 for map in &mut *(&raw mut MAPS){if map.layout==layout{*map=EMPTY_MAP;}}
 let f:unsafe extern "C" fn(u32)=mem::transmute(DROP_LAYOUT);f(layout);
}}
unsafe fn drop_rows(pop:u32,node:u32)->Option<&'static [u8]>{unsafe{
 let list=read(pop,0x20);if !valid(list,0x88)||read(list,0)!=address(0x195245c){return None;}
 let start=read(list,0x64);let size=read(list,0x68);if !valid(start,size){return None;}
 let data=read(node,0x38);if data<start||data>=start+size||data&3!=0{return None;}
 Some(core::slice::from_raw_parts(data as *const u8,(start+size-data).min(256)as usize))
}}
unsafe fn map_for(layout:u32,parent:u32,resource:u32)->Option<&'static mut Map>{unsafe{
 let maps=&mut *(&raw mut MAPS);
 let index=maps.iter().position(|m|m.layout==layout&&m.parent==parent&&m.resource==resource).or_else(||maps.iter().position(|m|m.layout==0))?;
 let map=&mut maps[index];if map.layout==0{map.layout=layout;map.parent=parent;map.resource=resource;}Some(map)
}}
unsafe fn create(map:&mut Map,slot:usize,item:u16,ui:u32)->bool{unsafe{
 let icon=&mut map.icons[slot];
 if icon.id!=NONE&&!owns(map.layout,map.parent,icon){*icon=EMPTY_ICON;return false;}
 if icon.id==NONE{
  let f:unsafe extern "C" fn(u32,u32,u32,u32,u32)->u32=mem::transmute(address(0x74501c));
  let id=f(map.layout,map.resource,4,map.parent,address(0x2323290));
  let object=call2(0x744fe8,map.layout,id);if !valid(object,0xdc){return false;}
  *icon=Icon{id,object,item:0,shown:false};
  let parts=read(object,0xd8);let sprite=if valid(parts,8){read(parts,4)}else{0};
  if !valid(sprite,0x108){call3(0x74533c,map.layout,id,0);icon.item=u16::MAX;return false;}
  // Item atlas, a smaller real quad, and a priority below player/monster markers.
  call4(0x746f48,map.layout,id,1,3);
  let size=[12.0f32,12.0];call2(0xc4fbf4,sprite,size.as_ptr()as u32);
  call3(0x746e84,map.layout,id,50);
 }
 if icon.item==u16::MAX||!owns(map.layout,map.parent,icon){return false;}
 if icon.item!=item{
  let mut helper=[0u32;8];call1(0x74e908,helper.as_mut_ptr()as u32);
  helper[1]=item as u32;helper[2]=icon.id;helper[4]=1;
  call3(0x74e96c,helper.as_mut_ptr()as u32,ui,0);icon.item=item;
 }
 true
}}
#[unsafe(no_mangle)]pub unsafe extern "C" fn compendium_gather_update(ui:u32){unsafe{
 let original:unsafe extern "C" fn(u32)=mem::transmute(UPDATE);original(ui);
 if !ENABLED||!valid(ui,0xe0){return;}
 let layout=read(ui,0x24);let object=read(ui,0xcc);if !valid(layout,0x29c)||!valid(object,0x98){return;}
 let table=read(object,0);let main=forge::mem::text_addr();
 if table<main+0x1400000||table>main+0x2140000-0x70{return;}
 let project=read(table,0x6c);if project!=address(0x81bde8)&&project!=address(0x81cacc){return;}
 let Some(map)=map_for(layout,read(ui,0xd4),read(ui,0xd0))else{return;};
 if !SHOW||crate::context::location().is_none(){hide(map);return;}
 let pop=read(address(0x1a89ec0),0);
 if !valid(pop,0xe8)||read(pop,0)!=address(0x19524cc){hide(map);return;}
 let pool=read(pop,0x24);if !valid(pool,0x80*gather::CAPACITY as u32){hide(map);return;}
 let accepts:unsafe extern "C" fn(u32,u32)->u32=mem::transmute(read(table,0x64));
 let project:unsafe extern "C" fn(u32,*const f32,*mut f32,u32)=mem::transmute(project);
 for slot in 0..gather::CAPACITY{
  let node=pool+slot as u32*0x80;
  let mut raw=[0u8;0x80];ptr::copy_nonoverlapping(node as *const u8,raw.as_mut_ptr(),raw.len());
  let point=gather::point(&raw,address(0x19523e0));
  let point=point.filter(|p|accepts(object,p.area as u32)!=0);
  let item=point.as_ref().and_then(|p|gather::icon(p,drop_rows(pop,node)));
  if let (Some(point),Some(item))=(point,item){
   let mut position=[0.0f32;4];project(object,point.pos.as_ptr(),position.as_mut_ptr(),point.area as u32);
   if position[..2].iter().all(|v|v.is_finite()&&v.abs()<100000.0)&&create(map,slot,item,ui){
    let icon=&mut map.icons[slot];call3(0x745b78,layout,icon.id,position.as_ptr()as u32);
    if !icon.shown{call3(0x74533c,layout,icon.id,1);icon.shown=true;}continue;
   }
  }
  hide_icon(layout,map.parent,&mut map.icons[slot]);
 }
}}
pub fn install(){unsafe{
 // Check every native entry used here before installing either callback.
 for (pc,word) in [(0x819494,0xe92d4ff0),(0xb1b284,0xe92d4830),(0x74501c,0xe92d41f0),(0x745b78,0xe92d4830),(0x74e96c,0xe92d4bf0),(0x744fe8,0xe5902298),(0x74533c,0xe5903298),(0x746f48,0xe590c298),(0xc4fbf4,0xe92d4010),(0x746e84,0xe5903298),(0x74e908,0xe59f1048)]{
  if read(address(pc),0)!=word{forge::log::prelude::error!("Gathering map disabled: native mismatch at {:#x}",pc);return;}
 }
 hook(&raw mut DROP_HOOK,address(0xb1b284),compendium_gather_drop_layout as *const ()as u32,&raw mut DROP_LAYOUT);
 hook(&raw mut UPDATE_HOOK,address(0x819494),compendium_gather_update as *const ()as u32,&raw mut UPDATE);
 ENABLED=DROP_LAYOUT!=0&&UPDATE!=0;
}}
