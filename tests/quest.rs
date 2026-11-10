#[path="../mod/src/quest.rs"]mod quest;
use quest::{Quest,Source,Need};
static QUESTS:[Quest;3]=[
 Quest{entry:[0;3],id:101,name:"Locked ideal",loc:0,rank:1,stars:4,hub:"Guild",monsters:&[1,2]},
 Quest{entry:[0;3],id:102,name:"Available pair",loc:0,rank:1,stars:4,hub:"Guild",monsters:&[1,2]},
 Quest{entry:[0;3],id:103,name:"Wrong rank",loc:0,rank:0,stars:2,hub:"Village",monsters:&[1]},
];
static SOURCES:[Source;4]=[
 Source{item:10,entity:0,rank:1,kind:0,zone:2}, // gather on map 0
 Source{item:11,entity:1,rank:1,kind:1,zone:0}, // stable monster
 Source{item:12,entity:2,rank:1,kind:1,zone:0},
 Source{item:13,entity:1,rank:1,kind:2,zone:0}, // quest index 1
];
#[test]fn locked_and_unknown_candidates_never_win(){
 let needs=[Need{item:10,quantity:5},Need{item:11,quantity:2},Need{item:12,quantity:1}];
 assert_eq!(quest::best(&QUESTS,&SOURCES,&needs,&[false,true,false],&[]),Some((1,3)));
 assert_eq!(quest::best(&QUESTS,&SOURCES,&needs,&[false;3],&[]),None);
}
#[test]fn rank_and_specific_reward_identity_are_enforced(){
 assert!(!quest::covers(2,&QUESTS[2],10,&SOURCES));
 assert!(quest::covers(1,&QUESTS[1],13,&SOURCES));
 assert!(!quest::covers(0,&QUESTS[0],13,&SOURCES));
}
#[test]fn route_covers_remaining_materials_without_repeating_a_quest(){
 let needs=[Need{item:10,quantity:5}];
 assert_eq!(quest::best(&QUESTS,&SOURCES,&needs,&[false,true,false],&[1]),None);
}

static GROUPS:[quest::PointGroup;3]=[
 quest::PointGroup{name:"A",members:&[(10,4),(11,4)]},
 quest::PointGroup{name:"B",members:&[(10,4)]},
 quest::PointGroup{name:"Five",members:&[(12,5)]},
];
#[test]fn point_coverage_uses_real_member_sources_and_counts_one_obligation(){
 let needs=[Need{item:0x8000,quantity:4}];
 assert!(quest::covers_group(1,&QUESTS[1],0x8000,&SOURCES,&GROUPS));
 assert!(!quest::covers_group(2,&QUESTS[2],0x8000,&SOURCES,&GROUPS));
 assert!(!quest::covers_group(1,&QUESTS[1],65535,&SOURCES,&GROUPS));
 assert_eq!(quest::best_groups(&QUESTS,&SOURCES,&needs,&[false,true,false],&[],&GROUPS),Some((1,1)));
}
#[test]fn named_stock_is_reserved_before_overlapping_point_groups(){
 let mut stock=[Some(0);14];stock[10]=Some(3);
 let mut needs=[Need{item:0x8000,quantity:4},Need{item:10,quantity:2},Need{item:0x8001,quantity:4}];
 assert!(quest::allocate(&mut needs,&mut stock,&GROUPS));
 assert_eq!(needs[1].quantity,0);
 assert_eq!(needs[0].quantity+needs[2].quantity,4);
 assert_eq!(stock[10],Some(0));
}
#[test]fn separate_operations_cannot_share_a_single_indivisible_unit(){
 let mut stock=[Some(0);14];stock[12]=Some(1);
 let mut needs=[Need{item:0x8002,quantity:2},Need{item:0x8002,quantity:3}];
 assert!(quest::allocate(&mut needs,&mut stock,&GROUPS));
 assert_eq!((needs[0].quantity,needs[1].quantity),(0,3));
 stock[12]=Some(2);needs[0].quantity=2;needs[1].quantity=3;
 assert!(quest::allocate(&mut needs,&mut stock,&GROUPS));
 assert!(needs.iter().all(|n|n.quantity==0));
}
#[test]fn constrained_groups_get_shared_stock_before_flexible_groups(){
 let mut stock=[Some(0);14];stock[10]=Some(1);stock[11]=Some(1);
 let mut needs=[Need{item:0x8000,quantity:4},Need{item:0x8001,quantity:4}];
 assert!(quest::allocate(&mut needs,&mut stock,&GROUPS));
 assert!(needs.iter().all(|n|n.quantity==0));
}
#[test]fn unknown_or_invalid_stock_does_not_mutate_demands(){
 let mut stock=[Some(0);14];stock[10]=None;
 let mut needs=[Need{item:0x8000,quantity:4}];
 assert!(!quest::allocate(&mut needs,&mut stock,&GROUPS));assert_eq!(needs[0].quantity,4);
 needs[0].item=65535;assert!(!quest::allocate(&mut needs,&mut stock,&GROUPS));
 let bad=[quest::PointGroup{name:"Bad",members:&[(10,0)]}];needs[0].item=0x8000;
 stock[10]=Some(1);assert!(!quest::allocate(&mut needs,&mut stock,&bad));
}
#[test]fn maximum_demand_rounding_does_not_overflow(){
 let mut stock=[Some(0);14];stock[10]=Some(u32::MAX);
 let mut needs=[Need{item:0x8001,quantity:u32::MAX}];
 assert!(quest::allocate(&mut needs,&mut stock,&GROUPS));
 assert_eq!(needs[0].quantity,0);assert_eq!(stock[10],Some(u32::MAX-1_073_741_824));
}
#[test]fn point_obligations_keep_owners_and_capacity_overflow_is_explicit(){
 let mut needs=[Need{item:65535,quantity:0};2];let mut owners=[65535;2];let mut count=0;
 assert!(quest::add_need(&mut needs,&mut owners,&mut count,0x8000,2,0));
 assert!(quest::add_need(&mut needs,&mut owners,&mut count,0x8000,3,256));
 assert_eq!(count,2);assert_eq!(owners,[0,256]);
 assert!(!quest::add_need(&mut needs,&mut owners,&mut count,0x8001,1,512));assert_eq!(count,2);
 let mut named=[Need{item:10,quantity:u32::MAX};1];let mut own=[65535];let mut len=1;
 assert!(!quest::add_need(&mut named,&mut own,&mut len,10,1,65535));assert_eq!(named[0].quantity,u32::MAX);
}
#[test]fn reviewed_aliases_resolve_exactly_without_guessing_lv_labels(){
 assert_eq!(quest::point_id("A",&GROUPS,&[]),Some(0x8000));
 assert_eq!(quest::point_id("Reviewed LV1",&GROUPS,&[("Reviewed LV1",1)]),Some(0x8001));
 assert_eq!(quest::point_id("Reviewed LV2",&GROUPS,&[("Reviewed LV1",1)]),None);
 assert_eq!(quest::point_id("Broken",&GROUPS,&[("Broken",900)]),None);
}
#[test]fn route_slot_requires_current_length_and_valid_quest_index(){
 let old=[1,2,usize::MAX];
 assert_eq!(quest::selected_route(&old,0,0,3),None);
 assert_eq!(quest::selected_route(&old,1,1,3),None);
 assert_eq!(quest::selected_route(&old,3,2,3),None);
 assert_eq!(quest::selected_route(&old,4,0,3),None);
 assert_eq!(quest::selected_route(&old,1,0,3),Some(1));
}
