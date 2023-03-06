use std::{fs::{self, File}, io::{Write, stdin}};

use poggers::external::process::ExProcess;

fn main() {

    println!("please enter the place id you want to get the offset for");

    let mut place_id = String::new();
    stdin().read_line(&mut place_id).unwrap();

    if let Some(offset) = roblox_offset::get_rel_offset(place_id.parse().expect("unable to parse place id")) {
        println!("offset: 0x{:X}", offset);
        File::create("place_id.txt").unwrap().write_all(format!("0x{:X}", offset).as_bytes()).unwrap();
    } else {
        println!("Place ID not found");
    }
}
