//! Compact, offline whole-quest-map gathering preview.
use super::{Ui,world,input};
use forge::input::{Input,Key};
pub fn rows(u:&Ui,area:usize,out:&mut [u16;64])->usize{
 let Some(&zone)=u.zones.get(area).filter(|_|area<u.zone_count)else{return 0;};let mut n=0;
 for(i,r)in world::GATHER.iter().enumerate(){if !r.supply&&r.loc as i16==u.location&&r.rank==u.rank&&r.zone==zone&&!out[..n].iter().any(|&old|world::GATHER[old as usize].name==r.name){if n<out.len(){out[n]=i as u16;n+=1;}}}
 out[..n].sort_unstable_by_key(|&id|world::GATHER[id as usize].name);n
}
pub fn input(u:&mut Ui,mut p:u32){
 for(key,bit)in[(Key::Up,13),(Key::Down,15),(Key::Left,12),(Key::Right,14),(Key::Enter,0)]{if Input::is_key_pressed(key){p|=1<<bit;}}
 if p&2!=0||Input::is_key_pressed(Key::Escape){input::close();return;}
 if p&(1<<3)!=0||Input::is_key_pressed(Key::Tab){u.quick_map=false;return;}
 if u.zone_count==0{return;}
 if p&(1<<14)!=0{u.zone=(u.zone+1)%u.zone_count;u.selected=0;}
 if p&(1<<12)!=0{u.zone=(u.zone+u.zone_count-1)%u.zone_count;u.selected=0;}
 let mut ids=[0;64];let count=rows(u,u.zone,&mut ids);u.selected=u.selected.min(count.saturating_sub(1));
 if p&(1<<15)!=0&&count>0{u.selected=(u.selected+1).min(count-1);}
 if p&(1<<13)!=0{u.selected=u.selected.saturating_sub(1);}
 if p&1!=0&&count>0{let entry=world::GATHER[ids[u.selected]as usize].entry;if entry!=65535{super::enter(u,entry as usize);}}
}
