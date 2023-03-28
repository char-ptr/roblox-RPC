use std::{process::{Command, Stdio}, fs::File, io::{Write, Read, stdin}};

use gp_rbx::roblox::ApiBased;
use poggers::external::process::ExProcess;

fn main() {

    let place_id = if let Ok(mut pid_f) = File::open("./place_id.txt") {
        println!("place_id.txt found, using that.");
        let mut place_id = String::new();
        pid_f.read_to_string(&mut place_id).unwrap();
        place_id
    } else {
        println!("please enter the place id you want to get the offset for");
        let mut place_id = String::new();
        stdin().read_line(&mut place_id).unwrap();
        place_id
    };


    let place_id_u64 : u64 = place_id.parse().expect("unable to parse place id");

    let cook = ApiBased::get_cookie_ff();

    // install puppeteer
    Command::new("deno").arg("run").arg("-A").arg("--unstable").arg("https://deno.land/x/puppeteer@16.2.0/install.ts")
        .stdout(Stdio::piped()).env("PUPPETEER_PRODUCT", "chrome").spawn()
        .expect("unable to install pupeeteer").wait().expect("unable to install pupeteer");

    if let Ok(mut ch) = Command::new("deno").arg("run").arg("--allow-all").arg("./roblox-starter/start.ts").arg(cook).arg(place_id).spawn() {
        ch.wait();
        println!("waiting on roblox to open...");

        let mut repeats = 0i32;
        let mut roblox_proc = None;
        while repeats < 20 {
            std::thread::sleep(std::time::Duration::from_secs(3));
            if let Ok(procs) = ExProcess::get_processes() {
                if let Some(proc) = procs.iter().find(|p| p.name == "RobloxPlayerBeta.exe") {
                    repeats = -1;
                    roblox_proc = Some(proc.clone().full().expect("unable to get full roblox process"));
                    break;
                }
            }
            repeats += 1;
        }
        if repeats == -1 {
            println!("roblox opened. waiting a few seconds to load into game..");
        } else {
            println!("roblox failed to open");
            return;
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("getting offset...");
        let roblox = roblox_proc.unwrap();
        let offset = roblox_offset::get_rel_offset_with_proc(&roblox,place_id_u64).unwrap();
        println!("offset: {:X}", offset);
        File::create("place_id_offset.txt").unwrap().write_all(format!("0x{:X}", offset).as_bytes()).unwrap();
        roblox.kill();
    } else {
        println!("failure to run deno..")
    }
}