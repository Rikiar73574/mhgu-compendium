#[path = "../mod/src/gather_filter.rs"]
mod gather_filter;
use gather_filter::{Catalogue, CheckState, DropTable, Selection};

fn rows(rows: &[(u8,u8,u16)]) -> Vec<u8> {
    rows.iter().flat_map(|&(weight,qty,item)| {
        let [lo,hi]=item.to_le_bytes(); [weight,qty,lo,hi]
    }).collect()
}
fn table(rows_in: &[(u8,u8,u16)]) -> DropTable {
    let mut raw=rows(rows_in); raw.extend_from_slice(&[255,0,0,0]);
    DropTable::parse(&raw).unwrap()
}

#[test]
fn duplicated_items_add_probability_instead_of_quantity() {
    let t=table(&[(20,1,318),(30,5,318),(50,2,319)]);
    assert_eq!(t.entries().len(),2);
    assert_eq!(t.rate(318),Some(500));
    assert_eq!(t.rate(319),Some(500));
    assert_eq!(t.rate(320),None);
    assert_eq!(table(&[(1,1,1),(2,1,2)]).rate(1),Some(333));
}
#[test]
fn parser_requires_complete_sentinel_and_rejects_invalid_positive_rows() {
    for raw in [vec![],rows(&[(100,1,1)]),vec![100,1,1,0,255],
        rows(&[(100,0,1),(255,0,0)]),rows(&[(100,100,1),(255,0,0)]),
        rows(&[(100,1,0),(255,0,0)]),rows(&[(100,1,2991),(255,0,0)]),
        rows(&[(0,1,1),(255,0,0)]),rows(&[(255,0,0)])] {
        assert!(DropTable::parse(&raw).is_none(),"{raw:?}");
    }
    let mut too_long=rows(&vec![(1,1,1);64]); too_long.extend_from_slice(&[255,0,0,0]);
    assert!(DropTable::parse(&too_long).is_none());
    let max=table(&vec![(1,1,1);63]); assert_eq!(max.rate(1),Some(1000));
    let mut trailing=rows(&[(100,1,2990),(255,0,0)]); trailing.push(1);
    assert_eq!(DropTable::parse(&trailing).unwrap().rate(2990),Some(1000));
}
#[test]
fn zero_weight_padding_never_changes_rates_or_validation() {
    let t=table(&[(0,0,0),(0,255,65535),(20,1,1),(80,99,2)]);
    assert_eq!(t.entries().len(),2);
    assert_eq!(t.rate(1),Some(200));
}
#[test]
fn forced_fallback_validates_native_item_range() {
    for item in [0,2991,65535] { assert!(DropTable::from_forced(item).is_none()); }
    for item in [1,2990] { assert_eq!(DropTable::from_forced(item).unwrap().rate(item),Some(1000)); }
}
#[test]
fn catalogue_ranges_only_include_nodes_where_the_item_exists() {
    let mut c=Catalogue::new();
    assert!(c.add_table(1,&table(&[(20,1,318),(80,1,319)])));
    assert!(c.add_table(1,&table(&[(60,1,318),(40,1,320)])));
    assert!(c.add_table(2,&table(&[(100,1,318)])));
    let choices=c.entries();
    assert_eq!(choices.len(),4);
    let ore=choices.iter().find(|e|e.category==1&&e.item==318).unwrap();
    assert_eq!((ore.min_rate,ore.max_rate),(200,600));
    let rare=choices.iter().find(|e|e.category==1&&e.item==319).unwrap();
    assert_eq!((rare.min_rate,rare.max_rate),(800,800));
    let other=choices.iter().find(|e|e.category==2).unwrap();
    assert_eq!((other.min_rate,other.max_rate),(1000,1000));
}
#[test]
fn matching_uses_actual_items_and_category_and_defaults_to_no_points() {
    let t=table(&[(95,1,318),(5,1,319)]);
    let mut s=Selection::new();
    assert!(!s.matches(1,&t));
    assert!(s.toggle(1,319));
    assert!(s.selected(1,319));
    assert!(s.matches(1,&t));
    assert!(!s.matches(2,&t));
    assert!(!s.matches(1,&table(&[(100,1,318)])));
    assert!(s.toggle(1,319));
    assert!(!s.matches(1,&t));
    assert!(!s.set(1,0,true));
    assert!(!s.set(1,2991,true));
}
#[test]
fn category_checkboxes_have_partial_state_and_preserve_other_categories() {
    let mut c=Catalogue::new(); c.add_table(1,&table(&[(50,1,1),(50,1,2)]));
    c.add_table(2,&table(&[(100,1,1)]));
    let mut s=Selection::new();
    assert_eq!(s.category_state(&c,1),CheckState::None);
    s.set(1,1,true); assert_eq!(s.category_state(&c,1),CheckState::Some);
    s.set_category(&c,1,true); assert_eq!(s.category_state(&c,1),CheckState::All);
    assert!(!s.selected(2,1));
    s.set(2,1,true); s.set_category(&c,1,false);
    assert_eq!(s.category_state(&c,1),CheckState::None);
    assert!(s.selected(2,1));
    assert_eq!(s.category_state(&c,77),CheckState::None);
    s.clear(); assert!(!s.selected(2,1));
}
#[test]
fn refresh_or_depletion_keeps_selections_keyed_to_items() {
    let mut c=Catalogue::new(); c.add_table(1,&table(&[(50,1,1),(50,1,2)]));
    let mut s=Selection::new(); s.set(1,1,true);
    c.clear(); c.add_table(1,&table(&[(100,1,2)]));
    assert_eq!(s.category_state(&c,1),CheckState::None);
    assert!(s.selected(1,1));
    c.clear(); c.add_table(1,&table(&[(100,1,1)]));
    assert_eq!(s.category_state(&c,1),CheckState::All);
    assert!(s.matches(1,&table(&[(100,1,1)])));
}
#[test]
fn bounded_catalogue_and_selection_fail_atomically() {
    let mut c=Catalogue::new(); let mut s=Selection::new();
    for item in 1..=512 { assert!(c.add_table(1,&table(&[(100,1,item)]))); assert!(s.set(1,item,true)); }
    assert!(!c.add_table(1,&table(&[(50,1,1),(50,1,513)])));
    assert_eq!(c.entries()[0].min_rate,1000);
    assert_eq!(c.entries().len(),512);
    assert!(!s.set(1,513,true)); assert!(!s.selected(1,513));
    assert!(s.set(1,2,false)); assert!(s.set(1,513,true));
    assert!(s.selected(1,513));
    let mut new=Catalogue::new(); new.add_table(2,&table(&[(50,1,1),(50,1,2)]));
    assert!(!s.set_category(&new,2,true));
    assert!(!s.selected(2,1)); assert!(!s.selected(2,2));
}
