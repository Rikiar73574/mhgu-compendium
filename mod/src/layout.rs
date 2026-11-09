use super::{Ui,world};
pub fn category(site:&str)->usize{match site{"Mine"=>0,"Bug"=>1,"Gather"=>3,_=>2}}
pub const CATEGORIES:[&str;4]=["Mining","Bugs","Fishing","Gathering"];
pub fn navigate(u:&mut Ui,p:u32){if u.count==0{return;}let id=u.results[u.selected]as usize;let n=&world::TREE[id];
 if p&(1<<12)!=0&&n.parent>=0{if let Some(i)=u.results[..u.count].iter().position(|id|*id as i32==n.parent){u.selected=i;}return;}
 if p&(1<<14)!=0{if let Some(i)=u.results[..u.count].iter().position(|candidate|world::TREE[*candidate as usize].parent==id as i32){u.selected=i;}return;}
 if p&((1<<8)|(1<<9))!=0{let down=p&(1<<9)!=0;
  if let Some(i)=u.results[..u.count].iter().enumerate().filter(|(_,id)|{let q=&world::TREE[**id as usize];q.parent<0&&if down{q.row>n.row}else{q.row<n.row}}).min_by_key(|(_,id)|world::TREE[**id as usize].row.abs_diff(n.row)).map(|(i,_)|i){u.selected=i;}return;}
 let down=p&(1<<15)!=0;let up=p&(1<<13)!=0;if down||up{let mut best=None;let mut score=u32::MAX;for(i,id)in u.results[..u.count].iter().enumerate(){let other=&world::TREE[*id as usize];if other.depth==n.depth&&((down&&other.row>n.row)||(up&&other.row<n.row)){let d=other.row.abs_diff(n.row)as u32;if d<score{score=d;best=Some(i);}}}if let Some(i)=best{u.selected=i;}}
}
