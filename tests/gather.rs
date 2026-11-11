#[path = "../mod/src/gather.rs"]
mod gather;
use gather::{Point, icon, point};

const VTABLE: u32 = 0x0195_23e0;
fn raw() -> [u8; 0x80] {
    let mut raw = [0; 0x80];
    raw[..4].copy_from_slice(&VTABLE.to_le_bytes());
    for (offset, value) in [(4, 10f32), (8, 20f32), (12, -30f32)] {
        raw[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    raw[0x4e..0x50].copy_from_slice(&2i16.to_le_bytes());
    raw[0x55] = 12;
    raw[0x68..0x6c].copy_from_slice(&11u32.to_le_bytes());
    raw[0x70] = 0x53;
    raw
}
fn node(kind: u32, forced: u16) -> Point {
    Point { pos: [0., 0., 0., 1.], area: 0, kind, forced }
}
fn drops(rows: &[(u8, u8, u16)]) -> Vec<u8> {
    rows.iter().flat_map(|&(weight, qty, id)| {
        let id = id.to_le_bytes(); [weight, qty, id[0], id[1]]
    }).collect()
}

#[test]
fn native_active_node_retains_room_position_and_item() {
    assert_eq!(gather::CAPACITY, 128);
    let mut raw = raw();
    raw[0x52..0x54].copy_from_slice(&271u16.to_le_bytes());
    let decoded = point(&raw, VTABLE).expect("active honey node");
    assert_eq!(decoded.pos, [10., 20., -30., 1.]);
    assert_eq!((decoded.area, decoded.kind, decoded.forced), (12, 11, 271));
}

#[test]
fn depletion_and_disabled_nodes_are_hidden_but_unlimited_nodes_remain() {
    for uses in [-1i16, 1, 2] {
        let mut raw = raw(); raw[0x4e..0x50].copy_from_slice(&uses.to_le_bytes());
        assert!(point(&raw, VTABLE).is_some());
    }
    let mut depleted = raw(); depleted[0x4e..0x50].fill(0);
    assert!(point(&depleted, VTABLE).is_none());
    for state in [1u32, 2, u32::MAX] {
        let mut raw = raw(); raw[0x5c..0x60].copy_from_slice(&state.to_le_bytes());
        assert!(point(&raw, VTABLE).is_none());
    }
    for flags in [0, 1, 2, 7, 0xff] {
        let mut raw = raw(); raw[0x70] = flags;
        assert!(point(&raw, VTABLE).is_none());
    }
}

#[test]
fn stale_identity_bad_room_and_bad_coordinates_are_rejected() {
    let raw = raw();
    assert!(point(&raw, VTABLE + 4).is_none());
    assert!(point(&raw[..127], VTABLE).is_none());
    let mut oversized = raw.to_vec(); oversized.push(0);
    assert!(point(&oversized, VTABLE).is_none());
    let mut bad_room = raw; bad_room[0x55] = 13;
    assert!(point(&bad_room, VTABLE).is_none());
    for offset in [4, 8, 12] {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 1_000_001., -1_000_001.] {
            let mut bad = raw; bad[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
            assert!(point(&bad, VTABLE).is_none());
        }
    }
}

#[test]
fn native_gathering_kinds_have_verified_representative_icons() {
    for (kinds, item) in [
        (&[11,12,30][..],271), (&[9,10][..],318),
        (&[21,22,23,24,25,26,27,28][..],385),
        (&[1,2][..],98), (&[3,4][..],272), (&[5,6][..],292), (&[7,8][..],410),
    ] {
        for &kind in kinds { assert_eq!(icon(&node(kind, 0), None), Some(item)); }
    }
    assert_eq!(icon(&node(11, 318), None), Some(271));
    assert_eq!(icon(&node(0, 0), None), None);
    assert_eq!(icon(&node(500, 0), None), None);
}

#[test]
fn unknown_kinds_use_valid_forced_item_first() {
    for id in [1, 271, 2990] { assert_eq!(icon(&node(0, id), None), Some(id)); }
    for id in [0, 2991, u16::MAX] { assert_eq!(icon(&node(0, id), None), None); }
    let list = drops(&[(100,1,318),(255,0,0)]);
    assert_eq!(icon(&node(0,271), Some(&list)), Some(271));
    assert_eq!(icon(&node(0,2991), Some(&list)), Some(318));
}

#[test]
fn terminated_drop_list_uses_most_likely_item_with_stable_ties() {
    let list = drops(&[(20,1,271),(60,1,318),(60,1,292),(255,0,0)]);
    assert_eq!(icon(&node(0,0), Some(&list)), Some(318));
    let honey = drops(&[(80,1,271),(20,1,318),(255,0,0)]);
    assert_eq!(icon(&node(0,0), Some(&honey)), Some(271));
    let invalid_choices = drops(&[(90,1,2991),(80,0,271),(70,100,271),(50,1,318),(255,0,0)]);
    assert_eq!(icon(&node(0,0), Some(&invalid_choices)), Some(318));
}

#[test]
fn malformed_or_unterminated_drop_lists_never_publish_an_icon() {
    let n = node(0,0);
    for list in [vec![], vec![50,1,15], drops(&[(100,1,271)]),
        drops(&[(0,1,271),(255,0,0)]), drops(&[(100,0,271),(255,0,0)]),
        drops(&[(100,1,0),(255,0,0)]), drops(&[(100,1,2991),(255,0,0)])] {
        assert_eq!(icon(&n, Some(&list)), None, "{list:?}");
    }
    let mut list = drops(&[(100,1,271)]); list.push(255);
    assert_eq!(icon(&n, Some(&list)), None);
    let mut beyond_bound = drops(&vec![(1,1,318);64]);
    beyond_bound.extend_from_slice(&[255,0,0,0]);
    assert_eq!(icon(&n, Some(&beyond_bound)), None);
    let trailing = drops(&[(100,1,271),(255,0,0),(100,0,0)]);
    assert_eq!(icon(&n, Some(&trailing)), Some(271));
}

#[test]
fn zero_weight_rows_are_ignored() {
    let list = drops(&[(0,0,0),(0,1,271),(10,1,318),(255,0,0)]);
    assert_eq!(icon(&node(0,0), Some(&list)), Some(318));
}
