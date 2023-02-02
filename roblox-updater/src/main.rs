use std::{process::Command, fs::File, io::Write};

use gp_rbx::roblox::ApiBased;

fn main() {
    let cook = ApiBased::get_cookie_ff();
    if let Ok(mut ch) = Command::new("deno").arg("run").arg("--allow-all").arg("./roblox-starter/start.ts").arg(cook).spawn() {
        ch.wait();
        println!("launched roblox");
        std::thread::sleep(std::time::Duration::from_secs(30));
        println!("getting offset...");
        let offset = roblox_offset::get_rel_offset().unwrap();
        println!("offset: {:X}", offset);
        File::create("place_id.txt").unwrap().write_all(format!("0x{:X}", offset).as_bytes()).unwrap();
    }
}
