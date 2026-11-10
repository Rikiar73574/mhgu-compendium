//! Persistent Compendium-wide preferences, separate from wishlist recipes.
#[derive(Clone,Copy,Default)]pub struct Settings{pub include_prowler:bool}
impl Settings{
 pub fn parse(text:&str)->Result<Self,()>{let mut s=Self::default();for line in text.lines(){let line=line.split([';','#']).next().unwrap_or("").trim();let Some((key,value))=line.split_once('=')else{continue;};if key.trim()=="IncludeProwlerQuests"{s.include_prowler=match value.trim(){"true"|"1"=>true,"false"|"0"=>false,_=>return Err(())};}}Ok(s)}
}
#[cfg(not(test))]mod live{
 use super::*;use core::cell::UnsafeCell;use crate::storage;
 const FILE:storage::Location=storage::Location{path:c"savedata:/compendium_settings.ini",display:"Compendium settings",defaults:"IncludeProwlerQuests = false\n"};
 struct State{value:Settings,loaded:bool,error:bool}
 struct Local(UnsafeCell<State>);unsafe impl Sync for Local{}
 static STATE:Local=Local(UnsafeCell::new(State{value:Settings{include_prowler:false},loaded:false,error:false}));
 pub fn load(){unsafe{let s=&mut*STATE.0.get();if s.loaded{return;}let mut bytes=[0;512];if let Ok(text)=FILE.read(&mut bytes){s.value=Settings::parse(text).unwrap_or_default();s.loaded=true;}}}
 pub fn include_prowler()->bool{unsafe{(*STATE.0.get()).value.include_prowler}}
 pub fn failed()->bool{unsafe{(*STATE.0.get()).error}}
 pub fn set_include_prowler(value:bool)->bool{load();unsafe{let s=&mut*STATE.0.get();s.error=FILE.save(if value{"IncludeProwlerQuests = true\n"}else{FILE.defaults}).is_err();if !s.error{s.value.include_prowler=value;s.loaded=true;}!s.error}}
}
#[cfg(not(test))]pub use live::*;
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn missing_preference_is_off(){assert!(!Settings::default().include_prowler);assert!(!Settings::parse("").unwrap().include_prowler);assert!(!Settings::parse("OtherSetting = true").unwrap().include_prowler);}
 #[test]fn explicit_opt_in_and_off(){assert!(Settings::parse("IncludeProwlerQuests = true # comment").unwrap().include_prowler);assert!(Settings::parse("IncludeProwlerQuests=1").unwrap().include_prowler);assert!(!Settings::parse("IncludeProwlerQuests = false").unwrap().include_prowler);}
 #[test]fn invalid_boolean_is_rejected(){assert!(Settings::parse("IncludeProwlerQuests = yesplease").is_err());}
}
