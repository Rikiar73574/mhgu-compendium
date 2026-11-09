// Exercise the actual navigation with sparse columns and branching upgrades.
struct Ui { results: [u16; 5], count: usize, selected: usize }
mod world {
 pub struct Node { pub parent: i32, pub row: u16, pub depth: u16 }
 pub static TREE: [Node; 5] = [
  Node {parent:-1,row:0,depth:0},
  Node {parent:0,row:0,depth:1},
  Node {parent:1,row:1,depth:2},
  Node {parent:-1,row:5,depth:0},
  Node {parent:3,row:5,depth:1},
 ];
}
#[path="../mod/src/layout.rs"] mod layout;
fn ui(selected:usize)->Ui {Ui {results:[0,1,2,3,4],count:5,selected}}
#[test] fn vertical_skips_nearer_rows_in_other_columns(){
 let mut u=ui(0);layout::navigate(&mut u,1<<15);assert_eq!(u.selected,3);
 layout::navigate(&mut u,1<<13);assert_eq!(u.selected,0);
 let mut u=ui(1);layout::navigate(&mut u,1<<15);assert_eq!(u.selected,4);
 layout::navigate(&mut u,1<<13);assert_eq!(u.selected,1);
}
#[test] fn column_edge_stays_put(){
 let mut u=ui(2);layout::navigate(&mut u,1<<15);assert_eq!(u.selected,2);
 layout::navigate(&mut u,1<<13);assert_eq!(u.selected,2);
}
#[test] fn horizontal_follows_upgrade_links(){
 let mut u=ui(1);layout::navigate(&mut u,1<<14);assert_eq!(u.selected,2);
 layout::navigate(&mut u,1<<12);assert_eq!(u.selected,1);
}

#[test]fn triggers_jump_families_instead_of_traversing_every_upgrade(){
 let mut u=ui(1);layout::navigate(&mut u,1<<9);assert_eq!(u.selected,3);
 layout::navigate(&mut u,1<<8);assert_eq!(u.selected,0);
}
