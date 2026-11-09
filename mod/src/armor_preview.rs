//! Reuse the game's paper doll and RomFS textures, outside Forge's NRO region.
use core::{mem,ptr};
use super::armor_data;
#[unsafe(no_mangle)]pub static mut COMPENDIUM_PREVIEW_REQUEST:u32=0;
#[unsafe(no_mangle)]pub static mut COMPENDIUM_PREVIEW_OBJECT:u32=0;
static mut ENABLED:bool=false;
static mut ACTIVE:u16=u16::MAX;
static mut MANAGER:u32=0;
static mut ACTOR:u32=0;
static mut RESTORE:bool=false;
static mut CLOSING:bool=false;
static mut CAMERA:u32=0;
static mut FLAGS:[u32;3]=[0;3];
static mut RECT:[i32;4]=[0;4];
static mut TURN:u32=0;
#[repr(C,align(16))]struct Appearance([u32;0x560/4]);
static mut BACKUP:Appearance=Appearance([0;0x560/4]);
static mut ORIGINAL:u32=0;
static mut INITIALIZE:u32=0;
static mut TICK:u32=0;
static mut GET_ACTOR:u32=0;
static mut RELEASE_VIEW:u32=0;
#[repr(C)]struct Hook([u32;19]);
static mut HOOK:Hook=Hook([0;19]);
static mut INIT_HOOK:Hook=Hook([0;19]);
static mut TICK_HOOK:Hook=Hook([0;19]);
static mut ACTOR_HOOK:Hook=Hook([0;19]);
static mut VIEW_HOOK:Hook=Hook([0;19]);
unsafe extern "C"{#[link_name="forge_hook_create"]fn hook(out:*mut Hook,target:u32,detour:u32,original:*mut u32)->*mut Hook;}
fn address(pc:u32)->u32{forge::mem::text_addr()+pc-0x200000}
unsafe fn read(p:u32,off:u32)->u32{unsafe{ptr::read_unaligned((p+off)as *const u32)}}
unsafe fn write(p:u32,off:u32,v:u32){unsafe{ptr::write_unaligned((p+off)as *mut u32,v);}}
unsafe fn call1(pc:u32,p:u32)->u32{unsafe{let f:unsafe extern "C" fn(u32)->u32=mem::transmute(address(pc));f(p)}}
unsafe fn call2(pc:u32,a:u32,b:u32)->u32{unsafe{let f:unsafe extern "C" fn(u32,u32)->u32=mem::transmute(address(pc));f(a,b)}}
unsafe fn call4(pc:u32,a:u32,b:u32,c:u32,d:u32)->u32{unsafe{let f:unsafe extern "C" fn(u32,u32,u32,u32)->u32=mem::transmute(address(pc));f(a,b,c,d)}}
unsafe fn manager()->u32{unsafe{read(address(0x1a85b58),0)}}
unsafe fn renderer()->u32{unsafe{read(address(0x1a4d750),0)}}
unsafe fn owner()->bool{unsafe{
 let r=renderer();let m=manager();let p=COMPENDIUM_PREVIEW_OBJECT;
 m==MANAGER&&r!=0&&(read(m,0x2c)==0||read(m,0x2c)==ACTOR)&&(read(r,0xde8)==0||read(r,0xde8)==read(p,4))
}}
unsafe fn close()->bool{unsafe{
 let p=COMPENDIUM_PREVIEW_OBJECT;if p==0{return true;}
 // The release filters protect resources acquired by a newer scene/menu.
 let restore=RESTORE&&owner();CAMERA=read(p,4);CLOSING=true;
 COMPENDIUM_PREVIEW_OBJECT=0;ACTIVE=u16::MAX;
 let destroy:unsafe extern "C" fn(u32)=mem::transmute(read(read(p,0),4));destroy(p);
 CLOSING=false;
 if restore{call2(0x46e600,MANAGER+0x30dc,(&raw mut BACKUP)as u32);write(MANAGER+0x30dc,0x504,FLAGS[0]);write(MANAGER+0x30dc,0x55c,FLAGS[1]);ptr::write((MANAGER+0x35e2)as *mut u8,FLAGS[2]as u8);}
 call1(0x46e50c,(&raw mut BACKUP)as u32);ACTOR=0;RESTORE=false;true
}}
unsafe extern "C" fn get_actor(m:u32)->u32{unsafe{
 if CLOSING&&(m!=MANAGER||read(m,0x2c)!=ACTOR){return 0;}
 let f:unsafe extern "C" fn(u32)->u32=mem::transmute(GET_ACTOR);f(m)
}}
unsafe extern "C" fn release_view(r:u32){unsafe{
 if CLOSING&&read(r,0xde8)!=0&&read(r,0xde8)!=CAMERA{return;}
 let f:unsafe extern "C" fn(u32)=mem::transmute(RELEASE_VIEW);f(r)
}}
unsafe extern "C" fn initialize(p:u32){unsafe{
 // This also covers controllers embedded in native menus (no allocator call).
 COMPENDIUM_PREVIEW_REQUEST=0;close();
 let f:unsafe extern "C" fn(u32)=mem::transmute(INITIALIZE);f(p)
}}
unsafe extern "C" fn tick(p:u32){unsafe{
 if p!=COMPENDIUM_PREVIEW_OBJECT&&read(p,0x3c)==1{COMPENDIUM_PREVIEW_REQUEST=0;close();}
 let f:unsafe extern "C" fn(u32)=mem::transmute(TICK);f(p)
}}
unsafe fn create(id:usize)->bool{unsafe{
 let m=manager();let r=renderer();if m<0x8000000||r==0||read(m,0x2c)!=0||read(r,0xde8)!=0||ptr::read((r+0xa80)as *const u8)!=0{return false;}
 let a=&armor_data::ARMOR[id];let mut pieces=[0u16;5];
 for piece in armor_data::ARMOR.iter().filter(|p|p.family==a.family){if piece.native==u16::MAX{return false;}pieces[piece.slot as usize]=piece.native;}
 let p=call2(0x6ab984,0x690,16);if p==0{return false;}
 MANAGER=m;FLAGS=[read(m+0x30dc,0x504),read(m+0x30dc,0x55c),ptr::read((m+0x35e2)as *const u8)as u32];
 call1(0x46e354,(&raw mut BACKUP)as u32);call2(0x46e600,(&raw mut BACKUP)as u32,m+0x30dc);RESTORE=true;
 call1(0x6ab9c0,p);write(p,0x24,0);let init:unsafe extern "C" fn(u32)=mem::transmute(INITIALIZE);init(p);
 // Mode 0 keeps our temporary loadout; mode 3 reloads equipped armor instead.
 call2(0x46e600,p+0x124,m+0x7c);
 if a.gender<2{ptr::write((p+0x124+0x4d8)as *mut u8,a.gender);}
 let mut record=[0u32;11];
 for (slot,series) in pieces.into_iter().enumerate(){
  call1(0x2daba4,record.as_mut_ptr()as u32);
  ptr::write_unaligned(record.as_mut_ptr().cast::<u8>().add(4).cast::<u16>(),slot as u16+1);
  ptr::write_unaligned(record.as_mut_ptr().cast::<u8>().add(6).cast::<u16>(),series);
  call4(0x46e964,p+0x124,record.as_mut_ptr()as u32,0xffff,0);
  call1(0x2dad24,record.as_mut_ptr()as u32);
 }
 write(p,0x48,1);write(p,0x4c,pieces[0]as u32);write(p,0x78,read(p,0x78)|1);write(p,0x3c,1);
 COMPENDIUM_PREVIEW_OBJECT=p;ACTIVE=a.family;ACTOR=read(m,0x2c);true
}}
unsafe extern "C" fn update(object:u32){unsafe{
 let requested=COMPENDIUM_PREVIEW_REQUEST as usize;
 let desired=requested.checked_sub(1).and_then(|i|armor_data::ARMOR.get(i));
 // Release before game input/scene updates can open a native equipment menu.
 if desired.is_none_or(|a|a.family!=ACTIVE){close();}
 let f:unsafe extern "C" fn(u32)=mem::transmute(ORIGINAL);f(object);
 let requested=COMPENDIUM_PREVIEW_REQUEST as usize;
 let desired=requested.checked_sub(1).and_then(|i|armor_data::ARMOR.get(i));
 if desired.is_none(){return;}
 if COMPENDIUM_PREVIEW_OBJECT==0&&!create(requested-1){return;}
 let p=COMPENDIUM_PREVIEW_OBJECT;
 if !owner(){RESTORE=false;close();return;}
 write(p,0x58,TURN);call1(0x6ad0e8,p);ACTOR=read(MANAGER,0x2c);
 let camera=read(p,4);let r=renderer();
 if camera!=0&&read(r,0xde8)==camera{
  call4(0x21a010,r,camera,(&raw const RECT)as u32,0);
  // Native's 30-degree equipment lens crops a full hunter in a portrait card.
  write(camera,0x3c,55.0f32.to_bits());call1(0x22a440,camera);
 }
}}
pub fn clear(){unsafe{COMPENDIUM_PREVIEW_REQUEST=0;}}
pub fn status()->&'static str{unsafe{
 if !ENABLED{return "Native preview unavailable";}
 if COMPENDIUM_PREVIEW_REQUEST==0{return "No model mapping for this set";}
 if COMPENDIUM_PREVIEW_OBJECT==0{return "Preview view is busy";}
 "Loading model..."
}}
pub fn request(id:usize,rect:[i32;4])->bool{unsafe{
 if !ENABLED||armor_data::ARMOR.iter().any(|a|a.family==armor_data::ARMOR[id].family&&a.native==u16::MAX){return false;}
 let(x,_)=super::Input::get_stick_r();TURN=if x< -0.25{0x10}else if x>0.25{0x20}else{0};
 RECT=rect;COMPENDIUM_PREVIEW_REQUEST=id as u32+1;true
}}
pub fn ready()->bool{unsafe{let p=COMPENDIUM_PREVIEW_OBJECT;ENABLED&&p!=0&&owner()&&ACTOR!=0&&read(ACTOR,0x13d0)==3&&ptr::read((ACTOR+0x2501)as *const u8)&2!=0&&ptr::read((ACTOR+0x13d4)as *const u8)!=0}}
pub fn install(){unsafe{
 for (pc,word) in [(0x5da8fc,0xe92d4ff0),(0x6ab984,0xe92d4070),(0x6ab9c0,0xe92d4070),(0x6abd14,0xe92d4070),(0x6ad0e8,0xe92d48f0),(0x6ac7e8,0xe92d41f0),(0x46e964,0xe92d4bf0),(0x476790,0xe590002c),(0x21a154,0xe92d4830)]{if read(address(pc),0)!=word{forge::log::prelude::error!("Armor preview disabled: native code mismatch at {:#x}",pc);return;}}
 hook(&raw mut INIT_HOOK,address(0x6abd14),initialize as *const () as u32,&raw mut INITIALIZE);
 hook(&raw mut TICK_HOOK,address(0x6ad0e8),tick as *const () as u32,&raw mut TICK);
 hook(&raw mut ACTOR_HOOK,address(0x476790),get_actor as *const () as u32,&raw mut GET_ACTOR);
 hook(&raw mut VIEW_HOOK,address(0x21a154),release_view as *const () as u32,&raw mut RELEASE_VIEW);
 hook(&raw mut HOOK,address(0x5da8fc),update as *const () as u32,&raw mut ORIGINAL);ENABLED=true;
}}
