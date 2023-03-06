use std::{fs, error::Error, path::PathBuf};

use poggers::{external::process::ExProcess, traits::Mem};
use reqwest::Url;
use rusqlite::Connection;

use crate::data::{place_details, thumb_data};

pub struct ApiBased {
    client : reqwest::Client ,
}

impl ApiBased {
    pub fn new(cookie:&str) -> ApiBased {

        let cjar = reqwest::cookie::Jar::default();
        let url = "https://roblox.com".parse::<Url>().unwrap();
        cjar.add_cookie_str(&format!(".ROBLOSECURITY={}; Domain=.roblox.com",cookie), &url);
        // println!("{:?}",cjar);

        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .cookie_provider(cjar.into())
            .build()
            .unwrap();
        ApiBased  {
            client : client,
        }
    }
    // pub fn new_with_chromium(browser_name: PathBuf) -> Self {
    //     let config_dir = directories::BaseDirs::new().unwrap();
    //     let config_dir = config_dir.data_local_dir();
    //     let profile_dir = config_dir.join(browser_name).join("User Data/Network/Cookies");
        

    // }
    pub fn new_with_firefox() -> Self {

        // println!("Cookie: {}",cookie_val);
        ApiBased::new(&Self::get_cookie_ff())
    }
    pub fn get_cookie_ff() -> String {
        let config_dir = directories::BaseDirs::new().unwrap();
        let config_dir = config_dir.config_dir();
        let profile_dir = config_dir.join("Mozilla/Firefox/Profiles");
        let mut cookie_val = String::new();
        for entry in fs::read_dir(profile_dir).unwrap() {
            if let Ok(entry) = entry {
                if let Ok(cookee)  = (|| -> Result<String, Box<dyn Error>>{
                    let con =Connection::open(entry.path().join("cookies.sqlite"))?;
                    Ok(con.query_row("SELECT value FROM moz_cookies WHERE name = '.ROBLOSECURITY'",[],|row| row.get(0))?)
                })() {
                    cookie_val = cookee;
                    // println!("got cookie from {}", entry.path().file_name().unwrap().to_string_lossy());
                    break;
                }
                else {
                    println!("Error: {}", entry.path().display());
                }
            }
        }
        if cookie_val.is_empty() {
            panic!("No cookie found");
        }
        return cookie_val
    }


    pub async fn get_data(&self,place_id: u64) -> place_details::Root2 {
        self.client.get(format!("https://games.roblox.com/v1/games/multiget-place-details?placeIds={}",place_id)).send().await.unwrap().json::<place_details::Root>().await.unwrap()[0].clone()
        // let a = self.client.get(format!("https://games.roblox.com/v1/games/multiget-place-details?placeIds={}",place_id)).send().await.unwrap().text().await;
        // println!("{:?}",a);
        // 0
    }
    pub async fn get_thumb_url(&self,universe_id: u64) -> String {
        self.client.get(format!("https://thumbnails.roblox.com/v1/games/icons?universeIds={}&returnPolicy=PlaceHolder&size=256x256&format=Png&isCircular=false",universe_id)).send().await.unwrap().json::<thumb_data::Root>().await.unwrap().data[0].image_url.clone()
    }
}

pub struct MemoryBased {
    pub attached:bool,
    pub proc: Option<ExProcess>,
    pub base_mod : usize,
}
impl MemoryBased {
    pub const PLACE_ID_OFFSET : usize = include!("../place_id_offset.txt");
    pub fn new() -> MemoryBased {
        let proc = None;
        let attached = false;
        MemoryBased {
            attached,
            proc,
            base_mod: 0,
        }
    }
    pub fn attach(&mut self) {
        if self.attached && self.proc.as_ref().unwrap().alive() {
            return;
        }
        if let Ok(proc) = ExProcess::new_from_name("RobloxPlayerBeta.exe".to_string()) {
            self.base_mod = proc.get_base_module().unwrap().base_address;
            self.proc = Some(proc);
            self.attached = true;
        } else {
            self.attached = false;
            self.proc = None;
        }
    }
    pub fn get_id(&self) -> u64 {
        if let Some(proc) = &self.proc {
            if proc.alive() {
                let base_of = self.base_mod;
                let data = unsafe { proc.read::<u64>(base_of + Self::PLACE_ID_OFFSET) } .unwrap();
                return data;
            }
        } 
        return 0;
    }


}