//! Native MHGU 1.4 cUIOUnderKeyAssign row, not an ImGui imitation.
//! The smithy uses two table-driven actions plus a custom Help row. The fourth
//! row is reserved only while a verified weapon-upgrade selection is active.
use core::{mem,ptr};
static mut ORIGINAL:u32=0;
static mut OBJECT:u32=0;
static mut LABEL:u8=0;
static mut SHOWN:u8=0;
#[repr(C)]struct Hook([u32;19]);
static mut HOOK:Hook=Hook([0;19]);
unsafe extern "C"{#[link_name="forge_hook_create"]fn create(out:*mut Hook,target:u32,detour:u32,original:*mut u32)->*mut Hook;}
fn label(n:u8)->*const u8{match n{2=>c"Remove from wishlist".as_ptr().cast(),3=>c"Wishlist save failed".as_ptr().cast(),_=>c"Add to wishlist".as_ptr().cast()}}
unsafe fn read(object:u32,offset:u32)->u32{unsafe{ptr::read_unaligned((object+offset)as *const u32)}}
unsafe fn write(object:u32,offset:u32,value:u32){unsafe{ptr::write_unaligned((object+offset)as *mut u32,value)}}
unsafe fn valid(object:u32)->bool{unsafe{(0x08000000..0xa8000000).contains(&object)&&object&3==0&&read(object,0)==forge::mem::text_addr()+0x1777c60}}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn compendium_native_hint_refresh(object:u32,mode:u32,flags:u32,force:u32){unsafe{
 let original:unsafe extern "C" fn(u32,u32,u32,u32)=mem::transmute(ORIGINAL);
 if !valid(object){original(object,mode,flags,force);return;}
 OBJECT=object;
 let desired=LABEL;
 // 0x34 is the live smithy upgrade help layout. Never extend other menus.
 let mut slot=None;let mut occupied=0;
 for i in 0..4{if read(object,0x228+i*8)==u32::MAX{if slot.is_none(){slot=Some(i);}}else{occupied+=1;}}
 let insert=if desired!=0&&mode==0x34&&flags&4==0&&occupied==1{slot}else{None};
 let saved=insert.map(|i|(read(object,0x228+i*8),read(object,0x22c+i*8)));
 if let Some(i)=insert{
  // Message 132 resolves <ICON ANALOG_RS> through the active font/icon mod.
  write(object,0x228+i*8,132);write(object,0x22c+i*8,label(desired)as u32);
 }
 original(object,mode,flags,if insert.is_some()||SHOWN!=0{1}else{force});
 if let Some(i)=insert{
  let (kind,text)=saved.unwrap();write(object,0x228+i*8,kind);write(object,0x22c+i*8,text);
  SHOWN=desired;
 }else{SHOWN=0;}
}}
/// Called on the game update thread, after the smithy's own update.
pub fn sync(selection:Option<(usize,usize)>){unsafe{
 let next=if super::input::open(){0}else if let Some((node,recipe))=selection{if super::wishlist::failed(){3}else if super::wishlist::contains(node,recipe){2}else{1}}else{0};
 let changed=LABEL!=next;LABEL=next;
 if ORIGINAL!=0&&valid(OBJECT)&&(changed||SHOWN!=next){let object=OBJECT;compendium_native_hint_refresh(object,read(object,0x4c),read(object,0x50),1);}
}}
/// Expire a departed smithy without invoking game UI code on the render thread.
pub fn expire(){unsafe{LABEL=0;}}
pub fn install(){unsafe{
 let target=forge::mem::text_addr()+0x6439ec;
 if ptr::read_unaligned(target as *const u32)!=0xe92d4ff0{return;}
 create(&raw mut HOOK,target,compendium_native_hint_refresh as *const () as u32,&raw mut ORIGINAL);
}}
