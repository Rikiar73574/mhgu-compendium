//! Pure, bounded decoding of a copied native MHGU cPop record.
pub const CAPACITY: usize = 128;
pub struct Point { pub pos: [f32;4], pub area: u8, pub kind: u32, pub forced: u16 }

fn word(raw: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(raw[offset..offset+4].try_into().unwrap())
}
fn half(raw: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(raw[offset..offset+2].try_into().unwrap())
}
pub fn point(raw: &[u8], vtable: u32) -> Option<Point> {
    if raw.len()!=0x80 || word(raw,0)!=vtable || raw[0x70]&7!=3
        || word(raw,0x5c)!=0 || half(raw,0x4e)==0 || raw[0x55]>12 {
        return None;
    }
    let pos=[f32::from_bits(word(raw,4)),f32::from_bits(word(raw,8)),
        f32::from_bits(word(raw,12)),1.];
    if pos[..3].iter().any(|v| !v.is_finite() || v.abs()>1_000_000.) { return None; }
    Some(Point { pos,area:raw[0x55],kind:word(raw,0x68),forced:half(raw,0x52) })
}

// The owned English item-name table contains 2991 native slots, IDs 0..2990.
fn valid_item(item: u16) -> bool { (1..=2990).contains(&item) }
pub fn icon(point: &Point, drops: Option<&[u8]>) -> Option<u16> {
    let known=match point.kind {
        11|12|30=>Some(271),9|10=>Some(318),21..=28=>Some(385),
        1|2=>Some(98),3|4=>Some(272),5|6=>Some(292),7|8=>Some(410),_=>None,
    };
    if known.is_some() { return known; }
    if valid_item(point.forced) { return Some(point.forced); }
    let mut best=None;
    let mut weight=0;
    // A complete sentinel row must occur within the bounded copy.
    for row in drops?.chunks_exact(4).take(64) {
        if row[0]==255 { return best; }
        let item=u16::from_le_bytes([row[2],row[3]]);
        if row[0]>weight && (1..=99).contains(&row[1]) && valid_item(item) {
            best=Some(item);weight=row[0];
        }
    }
    None
}
