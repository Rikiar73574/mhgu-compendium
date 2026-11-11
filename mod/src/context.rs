// MHGU 1.4.0 quest identity: same verified fields used by HuntLog.
fn valid(p:u32)->bool{p>=0x08000000&&p<0xa7ffff00&&p%4==0}
fn word(p:u32)->Option<u32>{if valid(p){Some(unsafe{core::ptr::read_volatile(p as *const u32)})}else{None}}
pub fn identity()->Option<(u32,usize,u8)>{
 let base=forge::mem::text_addr();let gc=unsafe{core::ptr::read_volatile(base.checked_add(0x188ca48)? as *const u32)};
 if !valid(gc)||unsafe{core::ptr::read_volatile((gc+0x4c) as *const u8)}!=255{return None;}
 let sq=unsafe{core::ptr::read_volatile(base.checked_add(0x188ad90)? as *const u32)};let qd=word(sq.checked_add(0xa4)?)?;let id=word(sq.checked_add(0xe0)?)?;
 if id==0||word(qd.checked_add(8)?)?!=id{return None;}
 crate::world::QUESTS.binary_search_by_key(&id,|q|q.0).ok().map(|i|{let q=crate::world::QUESTS[i];(id,q.1 as usize,q.2)})
}
pub fn location()->Option<(usize,u8)>{identity().map(|(_,loc,rank)|(loc,rank))}
