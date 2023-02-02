use std::{fs::{self, File}, io::Write};

use poggers::external::process::ExProcess;

fn main() {
    let rblx_proc = ExProcess::new_from_name("RobloxPlayerBeta.exe".to_string()).unwrap();
    let rblx_base = rblx_proc.get_base_module().unwrap();
    let place_id_static = 3199745785u64;
    let place_id  = unsafe { rblx_base.scan_virtual_value(&place_id_static) };
    if let Some(place_id) = place_id {
        let rel = place_id - rblx_base.base_address;
        println!("Place ID: {:X?}, rel = {:X}", place_id, rel);
        File::create("place_id.txt").unwrap().write_all(format!("0x{:X}", rel).as_bytes()).unwrap();
    } else {
        println!("Place ID not found");
    }
}
