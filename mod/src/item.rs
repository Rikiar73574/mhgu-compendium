use super::{Ui,item_data};
pub fn active(u:&Ui)->bool{u.detail<item_data::ITEMS.len()}
pub fn group(u:&mut Ui,index:usize){u.link=index.min(item_data::ITEMS[u.detail].groups.len().saturating_sub(1));u.scroll=0;}
pub fn scroll(u:&mut Ui,down:bool,step:usize){let max=item_data::ITEMS[u.detail].groups.get(u.link).map(|g|g.rows.len().saturating_sub(1)).unwrap_or(0);u.scroll=if down{(u.scroll+step).min(max)}else{u.scroll.saturating_sub(step)};}
pub fn input(u:&mut Ui,p:u32){if p&(1<<14)!=0{group(u,u.link+1);}if p&(1<<12)!=0{group(u,u.link.saturating_sub(1));}if p&(1<<15)!=0{scroll(u,true,1);}if p&(1<<13)!=0{scroll(u,false,1);}}
