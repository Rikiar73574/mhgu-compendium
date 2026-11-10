use super::{Ui,item_data};
pub fn active(u:&Ui)->bool{u.detail<item_data::ITEMS.len()}
pub fn group(u:&mut Ui,index:usize){u.link=index.min(item_data::ITEMS[u.detail].groups.len().saturating_sub(1));u.scroll=0;}
pub fn scroll(u:&mut Ui,down:bool,step:usize){let max=item_data::ITEMS[u.detail].groups.get(u.link).map(|g|g.rows.len().saturating_sub(1)).unwrap_or(0);u.scroll=if down{(u.scroll+step).min(max)}else{u.scroll.saturating_sub(step)};}
pub fn open_source(u:&mut Ui,index:usize){
 let Some(row)=item_data::ITEMS.get(u.detail).and_then(|item|item.groups.get(u.link)).and_then(|g|g.rows.get(index))else{return;};
 let Some(target)=row.link().filter(|&id|id<super::catalog::ENTRIES.len())else{return;};
 let fields=row.text();let level=fields[0].rsplit_once(" LV").and_then(|(_,v)|v.parse::<u8>().ok());u.scroll=index;super::enter(u,target);
 if target>=item_data::ITEMS.len(){if let Some(id)=level.and_then(|level|super::world::TREE.iter().position(|n|n.entry as usize==target&&n.level==level)){super::weapon::select(u,id);if fields[1]=="Upgrade"{if let Some(recipe)=super::weapon_data::WEAPONS[id].recipes.iter().position(|r|r.label=="Upgrade"){u.recipe=recipe;}}}}
}
pub fn input(u:&mut Ui,p:u32){if p&(1<<14)!=0{group(u,u.link+1);}if p&(1<<12)!=0{group(u,u.link.saturating_sub(1));}if p&(1<<15)!=0{scroll(u,true,1);}if p&(1<<13)!=0{scroll(u,false,1);}if p&1!=0{open_source(u,u.scroll);}}
