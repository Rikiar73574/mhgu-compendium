use std::{env,fs,path::PathBuf};
fn main() {
    let root=PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../build/generated");
    let out=PathBuf::from(env::var("OUT_DIR").unwrap());
    for name in ["game_ui","quest_data","armor_data","catalog","world","weapon_data","item_data","map_pixels","ui_pixels"] {
        let input=root.join(format!("{name}.rs"));
        println!("cargo:rerun-if-changed={}",input.display());
        fs::copy(&input,out.join(format!("{name}.rs")))
            .unwrap_or_else(|_| panic!("missing local generated input {name}; run tools/prepare.py first"));
    }
}
