use std::{fs::{self, File}, io::Write};

use poggers::external::process::ExProcess;

pub fn get_rel_offset() -> Option<usize> {
    let rblx_proc = ExProcess::new_from_name("RobloxPlayerBeta.exe".to_string()).unwrap();
    let rblx_base = rblx_proc.get_base_module().unwrap();
    let place_id_static = 3199745785u64;
    let place_id  = unsafe { rblx_base.scan_virtual_value(&place_id_static) };
    if let Some(place) = place_id {
        let rel = place - rblx_base.base_address;
        Some(rel)
    } else{
        None
    }
}
