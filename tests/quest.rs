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
