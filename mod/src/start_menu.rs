//! Additive MHGU 1.4 quest Start-menu entry. All thirteen stock rows survive.
//! Runtime addresses in the RE notes use image + 0x200000. Forge uses image.
use core::{mem,ptr,sync::atomic::{AtomicBool,Ordering}};
const STOCK:u32=13;
const TOTAL:u32=14;
const VTABLE:u32=0x177aeb8;
static OPEN:AtomicBool=AtomicBool::new(false);
#[repr(C)]struct Hook([u32;19]);
static mut HOOKS:[Hook;4]=[const{Hook([0;19])};4];
static mut ORIGINAL:[u32;4]=[0;4];
unsafe extern "C"{#[link_name="forge_hook_create"]fn create(out:*mut Hook,target:u32,detour:u32,original:*mut u32)->*mut Hook;}
fn label()->u32{c"Compendium".as_ptr()as u32}
fn valid(p:u32)->bool{(0x08000000..0xa7ff0000).contains(&p)&&p&3==0}
unsafe fn read(p:u32,off:u32)->u32{unsafe{ptr::read_unaligned((p+off)as *const u32)}}
unsafe fn write(p:u32,off:u32,v:u32){unsafe{ptr::write_unaligned((p+off)as *mut u32,v)}}
unsafe fn object(p:u32)->bool{unsafe{valid(p)&&read(p,0)==forge::mem::text_addr()+VTABLE}}
unsafe fn owned(p:u32)->bool{unsafe{object(p)&&read(p,0xac)==TOTAL&&read(p,0xc4)==TOTAL&&valid(read(p,0xa8))&&valid(read(p,0xc0))&&read(read(p,0xa8),STOCK*8+4)==label()}}
unsafe fn row(p:u32){unsafe{let rows=read(p,0xa8);write(rows,STOCK*8,1);write(rows,STOCK*8+4,label());}}
/// Only the game update thread calls these hooks; the request crosses to render.
pub fn take_open()->bool{compendium_start_take_open()}
#[unsafe(no_mangle)]pub extern "C" fn compendium_start_take_open()->bool{OPEN.swap(false,Ordering::AcqRel)}
#[unsafe(no_mangle)]pub unsafe extern "C" fn compendium_start_ctor(p:u32)->u32{unsafe{
 let original:unsafe extern "C" fn(u32)->u32=mem::transmute(ORIGINAL[0]);let result=original(p);
 if !object(p)||read(p,0xac)!=STOCK||read(p,0xc4)!=STOCK{return result;}
 let old_rows=read(p,0xa8);let old_hints=read(p,0xc0);
 if !valid(old_rows)||!valid(old_hints){return result;}
 // Exact allocator used by both vectors in 0x877b48 and freed by 0x86e3ac.
 let heap_for:unsafe extern "C" fn(u32)->u32=mem::transmute(forge::mem::text_addr()+0x7aa47c);
 let heap=heap_for(forge::mem::text_addr()+0x191eb18);if !valid(heap){return result;}
 // Heap objects are in the heap; their relocated vtables are in main's data
 // segment, below 0x08000000 when main loads at its real 0x00200000 base.
 let base=forge::mem::text_addr();let table=read(heap,0);
 if table&3!=0||!(base+0x172f000..=base+0x184ad78-0x38).contains(&table){return result;}
 let allocate=read(table,0x1c);let release=read(table,0x34);
 if [allocate,release].iter().any(|&f|f&3!=0||!(base..base+0x13f1dec).contains(&f)){return result;}
 let alloc:unsafe extern "C" fn(u32,u32,u32)->u32=mem::transmute(allocate);
 let free:unsafe extern "C" fn(u32,u32)=mem::transmute(release);
 // Allocate both before publishing either. Allocation failure leaves stock intact.
 let rows=alloc(heap,TOTAL*8,16);if rows==0{return result;}
 let hints=alloc(heap,TOTAL*12,16);if hints==0{free(heap,rows);return result;}
 ptr::copy_nonoverlapping(old_rows as *const u8,rows as *mut u8,(STOCK*8)as usize);
 ptr::copy_nonoverlapping(old_hints as *const u8,hints as *mut u8,(STOCK*12)as usize);
 // cUIUnderGuideData is vtable + eight value bytes; its destructor 0x74f9f0
 // is a no-op. Type 0xff disables hints (0x86fda8), so no stock help is reused.
 write(hints,STOCK*12,read(old_hints,0));write(hints,STOCK*12+4,u32::MAX);write(hints,STOCK*12+8,u32::MAX);
 write(p,0xa8,rows);write(p,0xac,TOTAL);write(p,0xc0,hints);write(p,0xc4,TOTAL);row(p);
 free(heap,old_rows);free(heap,old_hints);result
}}
#[unsafe(no_mangle)]pub unsafe extern "C" fn compendium_start_init(p:u32){unsafe{
 let ours=owned(p);let original:unsafe extern "C" fn(u32)=mem::transmute(ORIGINAL[1]);original(p);
 if ours&&owned(p){
  // Same native page-control setter as 0x87006c. Seven rows per page is
  // retained; row fourteen occupies the existing second page's spare slot.
  let count:unsafe extern "C" fn(u32,u32,u32)=mem::transmute(forge::mem::text_addr()+0x54ee34);count(p+0xd8,TOTAL,3);
 }
}}
#[unsafe(no_mangle)]pub unsafe extern "C" fn compendium_start_fill(p:u32){unsafe{
 let ours=owned(p);let original:unsafe extern "C" fn(u32)=mem::transmute(ORIGINAL[2]);original(p);
 // Native refill clears the whole vector, then writes exactly stock indices.
 if ours&&object(p)&&read(p,0xac)==TOTAL&&read(p,0xc4)==TOTAL{row(p);}
}}
#[unsafe(no_mangle)]pub unsafe extern "C" fn compendium_start_action(p:u32)->u32{unsafe{
 if owned(p)&&read(p,0x4c)==STOCK&&read(p,0x48)==0{
  OPEN.store(true,Ordering::Release);
  return 4; // Native close result, handled by 0x86f574 -> virtual +0x70.
 }
 let original:unsafe extern "C" fn(u32)->u32=mem::transmute(ORIGINAL[3]);original(p)
}}
pub fn install(){unsafe{
 let base=forge::mem::text_addr();
 let entries=[(0x677b48,0xe92d48f0,compendium_start_ctor as *const ()as u32),(0x677d0c,0xe92d47f0,compendium_start_init as *const ()as u32),(0x678e80,0xe92d4bf0,compendium_start_fill as *const ()as u32),(0x677ee0,0xe92d4830,compendium_start_action as *const ()as u32)];
 // Refuse foreign hooks or another game version before installing any hook.
 if entries.iter().any(|&(at,word,_)|ptr::read_unaligned((base+at)as *const u32)!=word){return;}
 // Static leaf routines and allocator layout are part of this compatibility gate.
 if read(base+0x54ee34,0)!=0xe1c011b4||read(base+0x54f9f0,0)!=0xe12fff1e||read(base+0x7aa47c,0)!=0xe59f1010{return;}
 // Install the allocating constructor last. A failed dependency leaves every
 // stock object untouched; already-installed callbacks only pass it through.
 for i in [1,2,3,0]{let(at,_,detour)=entries[i];if create(&raw mut HOOKS[i],base+at,detour,&raw mut ORIGINAL[i]).is_null()||ORIGINAL[i]==0{return;}}
}}
