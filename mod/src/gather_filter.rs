//! Allocation-free drop distributions and independent category/item selections.
//! Rates are tenths of a percent: 1000 means 100%. Runtime supplies categories.

pub const CAPACITY: usize = 512;
const MAX_ROWS: usize = 64;
fn valid_item(item: u16) -> bool { (1..=2990).contains(&item) }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropChoice { pub item: u16, pub weight: u16 }
impl DropChoice { const EMPTY: Self = Self { item: 0, weight: 0 }; }

pub struct DropTable { choices: [DropChoice; MAX_ROWS], len: usize, total: u16 }
impl DropTable {
    /// Decode a complete native table. A malformed positive-weight row rejects
    /// the entire distribution, so invalid rows cannot inflate displayed odds.
    pub fn parse(raw: &[u8]) -> Option<Self> {
        let mut table=Self { choices: [DropChoice::EMPTY; MAX_ROWS], len: 0, total: 0 };
        for row in raw.chunks_exact(4).take(MAX_ROWS) {
            let weight=row[0] as u16;
            if weight==255 { return if table.total>0 { Some(table) } else { None }; }
            if weight==0 { continue; }
            let item=u16::from_le_bytes([row[2],row[3]]);
            if !valid_item(item) || !(1..=99).contains(&row[1]) { return None; }
            if let Some(choice)=table.choices[..table.len].iter_mut().find(|c|c.item==item) {
                choice.weight+=weight;
            } else {
                table.choices[table.len]=DropChoice { item, weight };
                table.len+=1;
            }
            table.total+=weight;
        }
        None
    }
    /// Fallback only when no actual table is available. A cPop forced item may
    /// cache one outcome; it must never override a present full distribution.
    pub fn from_forced(item: u16) -> Option<Self> {
        if !valid_item(item) { return None; }
        let mut table=Self { choices: [DropChoice::EMPTY; MAX_ROWS], len: 1, total: 1 };
        table.choices[0]=DropChoice { item, weight: 1 };
        Some(table)
    }
    pub fn entries(&self) -> &[DropChoice] { &self.choices[..self.len] }
    pub fn rate(&self, item: u16) -> Option<u16> {
        self.entries().iter().find(|c|c.item==item).map(|c|
            ((c.weight as u32*1000+self.total as u32/2)/self.total as u32) as u16)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemChoice { pub item: u16, pub category: u8, pub min_rate: u16, pub max_rate: u16 }
impl ItemChoice { const EMPTY: Self = Self { item: 0, category: 0, min_rate: 0, max_rate: 0 }; }
pub struct Catalogue { choices: [ItemChoice; CAPACITY], len: usize }
impl Catalogue {
    pub const fn new() -> Self { Self { choices: [ItemChoice::EMPTY; CAPACITY], len: 0 } }
    pub fn clear(&mut self) { self.len = 0; }
    pub fn entries(&self) -> &[ItemChoice] { &self.choices[..self.len] }
    /// Aggregate rates across nodes containing each item. Capacity failure is
    /// atomic: existing ranges and entries remain unchanged.
    pub fn add_table(&mut self, category: u8, table: &DropTable) -> bool {
        let missing=table.entries().iter().filter(|drop|
            !self.entries().iter().any(|c|c.category==category&&c.item==drop.item)).count();
        if self.len+missing>CAPACITY { return false; }
        for drop in table.entries() {
            let rate=table.rate(drop.item).unwrap_or(0);
            if let Some(choice)=self.choices[..self.len].iter_mut().find(|c|c.category==category&&c.item==drop.item) {
                choice.min_rate=choice.min_rate.min(rate);
                choice.max_rate=choice.max_rate.max(rate);
            } else {
                self.choices[self.len]=ItemChoice { item: drop.item, category, min_rate: rate, max_rate: rate };
                self.len+=1;
            }
        }
        true
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckState { None, Some, All }
#[derive(Clone, Copy)]
struct Key { item: u16, category: u8 }
pub struct Selection { keys: [Key; CAPACITY], len: usize }
impl Selection {
    /// No selected items means no map markers.
    pub const fn new() -> Self { Self { keys: [Key { item: 0, category: 0 }; CAPACITY], len: 0 } }
    pub fn clear(&mut self) { self.len = 0; }
    pub fn selected(&self, category: u8, item: u16) -> bool {
        self.keys[..self.len].iter().any(|k|k.category==category&&k.item==item)
    }
    /// Choices are item keys, independent of catalogue positions and refreshes.
    /// Return false on invalid item or capacity failure, without mutation.
    pub fn set(&mut self, category: u8, item: u16, selected: bool) -> bool {
        if !valid_item(item) { return false; }
        let found=self.keys[..self.len].iter().position(|k|k.category==category&&k.item==item);
        match (found,selected) {
            (Some(index),false)=>{ self.len-=1; self.keys[index]=self.keys[self.len]; }
            (None,true)=>{
                if self.len==CAPACITY { return false; }
                self.keys[self.len]=Key { item, category }; self.len+=1;
            }
            _=>{}
        }
        true
    }
    pub fn toggle(&mut self, category: u8, item: u16) -> bool { self.set(category,item,!self.selected(category,item)) }
    /// Change currently catalogued members only; retain absent/depleted choices.
    pub fn set_category(&mut self, catalogue: &Catalogue, category: u8, selected: bool) -> bool {
        if selected {
            let missing=catalogue.entries().iter().filter(|c|
                c.category==category&&!self.selected(category,c.item)).count();
            if self.len+missing>CAPACITY { return false; }
        }
        for choice in catalogue.entries().iter().filter(|c|c.category==category) {
            self.set(category,choice.item,selected);
        }
        true
    }
    pub fn category_state(&self, catalogue: &Catalogue, category: u8) -> CheckState {
        let mut total=0; let mut selected=0;
        for choice in catalogue.entries().iter().filter(|c|c.category==category) {
            total+=1;
            if self.selected(category,choice.item) { selected+=1; }
        }
        if selected==0 { CheckState::None }
        else if selected==total { CheckState::All }
        else { CheckState::Some }
    }
    pub fn matches(&self, category: u8, table: &DropTable) -> bool {
        table.entries().iter().any(|c|self.selected(category,c.item))
    }
}
