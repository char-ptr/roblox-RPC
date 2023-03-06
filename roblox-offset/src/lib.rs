use std::{fs::{self, File}, io::Write};

use poggers::external::process::ExProcess;

pub fn get_rel_offset_with_proc(proc: &ExProcess,place_id_s: u64) -> Option<usize> {
    let rblx_base = proc.get_base_module().unwrap();
    let place_id  = unsafe { rblx_base.scan_virtual_value(&place_id_s) };
    if let Some(place) = place_id {
        let rel = place - rblx_base.base_address;
        Some(rel)
    } else{
        None
    }
}


pub fn get_rel_offset(place_id: u64) -> Option<usize> {
    let rblx_proc = ExProcess::new_from_name("RobloxPlayerBeta.exe".to_string()).unwrap();
    get_rel_offset_with_proc(&rblx_proc,place_id)
}
