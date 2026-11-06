pub const START:u32=1<<10;
pub const LEFT:u32=1<<12;
pub struct Gate{pub open:bool,pub passive:bool,combo:bool,drain:bool,pending_start:bool}
impl Gate{
 pub const fn new()->Self{Self{open:false,passive:false,combo:false,drain:false,pending_start:false}}
 pub fn filter(&mut self,b:u32)->u32{
  let combo=b&(START|LEFT)==START|LEFT;
  if combo&&!self.combo {self.open=!self.open;self.drain=true;self.pending_start=false;}
  self.combo=combo;
  if self.open&&!self.passive {self.pending_start=false;return 0;}
  if self.drain {if b==0{self.drain=false;}return 0;}
  if b&START!=0 {self.pending_start=true;return b&!START;}
  if self.pending_start {self.pending_start=false;return b|START;}
  b
 }
 pub fn capturing(&self)->bool{(self.open&&!self.passive)||self.drain}
 pub fn close(&mut self){self.open=false;self.drain=true;self.pending_start=false;}
}
#[cfg(test)] mod tests{
 use super::*;
 #[test] fn chord_consumed_and_not_repeated(){let mut g=Gate::new();assert_eq!(g.filter(START),0);assert_eq!(g.filter(START|LEFT),0);assert!(g.open);g.filter(START|LEFT);assert!(g.open);g.filter(0);g.filter(START|LEFT);assert!(!g.open);assert_eq!(g.filter(LEFT),0);g.filter(0);assert_eq!(g.filter(1),1);}
 #[test] fn passive_tracker_passes_gameplay(){let mut g=Gate::new();g.passive=true;g.filter(START|LEFT);assert!(g.open);g.filter(0);assert_eq!(g.filter(1|LEFT),1|LEFT);assert!(!g.capturing());g.filter(START|LEFT);assert!(!g.open);}
 #[test] fn normal_start_replayed_on_release(){let mut g=Gate::new();assert_eq!(g.filter(START),0);assert_eq!(g.filter(START),0);assert_eq!(g.filter(0),START);assert_eq!(g.filter(0),0);}
 #[test] fn close_drains_buttons(){let mut g=Gate::new();g.open=true;g.close();assert_eq!(g.filter(2),0);g.filter(0);assert_eq!(g.filter(2),2);}
}
