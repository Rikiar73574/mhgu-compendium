use super::{Ui,world,weapon_data,enter};
pub fn active(u:&Ui)->bool{u.detail!=usize::MAX&&u.detail_node<world::TREE.len()&&world::TREE[u.detail_node].entry as usize==u.detail}
// Xbox B selects and A backs out throughout weapon details and their item links.
pub fn controls(u:&Ui)->bool{active(u)||u.history[..u.depth].iter().any(|h|h.node!=usize::MAX)}
pub fn select(u:&mut Ui,id:usize){if id>=world::TREE.len(){return;}u.detail_node=id;u.detail=world::TREE[id].entry as usize;u.recipe=0;u.link=0;u.scroll=0;}
pub fn recipe(u:&Ui)->Option<&'static weapon_data::Recipe>{weapon_data::WEAPONS[u.detail_node].recipes.get(u.recipe)}
pub fn change_recipe(u:&mut Ui,forward:bool){let n=weapon_data::WEAPONS[u.detail_node].recipes.len();if n>0{u.recipe=if forward{(u.recipe+1)%n}else{(u.recipe+n-1)%n};u.link=0;}}
pub fn open_material(u:&mut Ui,i:usize){if let Some(item)=recipe(u).and_then(|r|r.items.get(i)){if item.entry!=65535{u.link=i;enter(u,item.entry as usize);}}}
pub fn tree_range(id:usize)->(usize,usize){
 let mut root=id;while world::TREE[root].parent>=0{root=world::TREE[root].parent as usize;}
 let end=(root+1..world::TREE.len()).find(|i|world::TREE[*i].parent<0).unwrap_or(world::TREE.len());(root,end)
}
pub fn visible_lane(row:u16,root:usize,end:usize,min_depth:u8)->usize{
 let mut count=0;let mut last=u16::MAX;
 for node in &world::TREE[root..end]{if node.row>=row{continue;}if node.depth>=min_depth&&node.depth<=min_depth.saturating_add(4)&&node.row!=last{count+=1;last=node.row;}}
 count
}
pub fn input(u:&mut Ui,p:u32){
 if !u.panel_entered{
  if p&((1<<13)|(1<<12))!=0{u.tree_focus=true;}
  if p&((1<<15)|(1<<14))!=0{u.tree_focus=false;u.panel_entered=true;u.link=0;return;}
  if p&1!=0{u.panel_entered=true;if !u.tree_focus{u.link=0;}}
  return;
 }
 if p&(1<<9)!=0{change_recipe(u,true);}if p&(1<<8)!=0{change_recipe(u,false);}
 if u.tree_focus{
  if p&1!=0{u.tree_focus=false;u.link=0;return;}
  let node=&world::TREE[u.detail_node];let(root,end)=tree_range(u.detail_node);
  if p&(1<<12)!=0&&node.parent>=0{select(u,node.parent as usize);}
  else if p&(1<<14)!=0{if let Some(i)=(root..end).find(|i|world::TREE[*i].parent==u.detail_node as i32){select(u,i);}}
  else if p&((1<<13)|(1<<15))!=0{
   let down=p&(1<<15)!=0;
   let found=(root..end).filter(|i|{let other=&world::TREE[*i];other.depth==node.depth&&if down{other.row>node.row}else{other.row<node.row}}).min_by_key(|i|world::TREE[*i].row.abs_diff(node.row));
   if let Some(i)=found{select(u,i);}
  }
 }else if u.link==usize::MAX{
  if p&(1<<13)!=0{u.tree_focus=true;u.panel_entered=false;}
  else if p&(1<<15)!=0||p&1!=0{u.link=0;}
  else if p&(1<<12)!=0{change_recipe(u,false);u.link=usize::MAX;}
  else if p&(1<<14)!=0{change_recipe(u,true);u.link=usize::MAX;}
 }else{
  let n=recipe(u).map(|r|r.items.len()).unwrap_or(0);
  if p&(1<<15)!=0{u.link=(u.link+1).min(n.saturating_sub(1));}
  if p&(1<<13)!=0{if u.link==0{u.link=usize::MAX;}else{u.link-=1;}}
  if p&1!=0{open_material(u,u.link);}
 }
}
