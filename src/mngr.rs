use std::{sync::Arc, time::{SystemTime, Duration}};

use discord_sdk::{user::User, Discord, activity::{ActivityBuilder, Assets, ActivityButton, Activity, ActivityArgs}};
use tokio::{sync::RwLock, task::JoinHandle};
use parking_lot as pl;

use crate::{roblox, data::thumb_data};


pub struct UserMngr {
    pub connected: Arc<pl::RwLock<bool>>,
    pub stop: Arc<pl::RwLock<bool>>,
    pub dsc: Arc<tokio::sync::RwLock<Discord>>, // tokio rwlock implements send
    pub u: Arc<pl::RwLock<Option<User>>>,
}

impl UserMngr {
    pub fn new(dsc: Discord) -> Self {
        Self {
            connected: Arc::new(Default::default()),
            stop: Arc::new(Default::default()),
            u: Arc::new(Default::default()),
            dsc: Arc::new(tokio::sync::RwLock::new(dsc)),
        }
    }

    pub fn stop(&mut self) {
        *self.stop.clone().write() = true;
    }

    pub fn start_activity_update(&self) -> JoinHandle<()> {
        let stopper = self.stop.clone();
        let con = self.connected.clone();
        let dscc = self.dsc.clone();
        tokio::task::spawn(async move {
            let mut s = 0;
            let mut last_update = SystemTime::now();
            let mut game_thumbnail = String::new();
            let mut game_name = String::new();
            let mut game_builder = String::new();
            let mut memer = roblox::MemoryBased::new();
            let mut game_place_id = 0u64;
            let apir = roblox::ApiBased::new_with_firefox();

            loop {
                if !game_name.is_empty() {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                let w = *stopper.clone().read();
                // println!("stop : {w}");
                if w {
                    break;
                }
                if s == 0 {
                    // 0 <-> 5 = 4
                    if !*con.clone().read() {
                        println!("no connect");
                        continue;
                    }
                    if !memer.attached {
                        memer.attach();
                        if !memer.attached {
                            println!("no attach");
                            Arc::clone(&dscc).write().await.clear_activity().await;
                            continue;
                        }
                    }
                    let last_id = game_place_id;
                    println!("fetch new data");
                    memer.attach();
                    let place_id = memer.get_id();
                    if place_id == 0 {
                        println!("no place id");
                        Arc::clone(&dscc).write().await.clear_activity().await;
                        continue;
                    } else if place_id != last_id {
                        println!("new place id: {}, fetching data.",place_id);
                        let univ = apir.get_data(place_id).await;
                        game_thumbnail = apir.get_thumb_url(univ.universe_id as u64).await;
                        game_name = univ.name;
                        game_builder = univ.builder;
                        last_update = SystemTime::now();
                    }
                    game_place_id = place_id;
                    
                    if !memer.proc.as_ref().unwrap().alive() {
                        memer.attach();
                        println!("no alive");
                        Arc::clone(&dscc).write().await.clear_activity().await;
                        continue;
                    }
                    println!("update status");

                    let assets = Assets::default();
                    let assets = assets.large(game_thumbnail.clone(), Some(format!("{:.^3}",game_name)));
                    let assets = assets.small("gh".to_string(), Some("github.com/pozm/roblox-RPC".to_string()));
                    let button = ActivityButton{
                        label: "Play".to_string(),
                        url: format!("https://www.roblox.com/games/{}",game_place_id),
                    }; // their crappy serialization is not working
                    let activity = ActivityBuilder::default()
                        .details(format!("Playing {}",game_name))
                        .state(format!("By {}", game_builder))
                        .assets(
                            assets,
                        )
                        .start_timestamp(last_update);
                        // .set_button_1(button);
                    {
                        // println!("activityjsn = {}",serde_json::to_string::<ActivityArgs>(&activity.into()).unwrap());
                        match Arc::clone(&dscc).write().await.update_activity(activity).await{
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("unable to update status: {}",e)
                            }
                        };
                    }
                }
                s = (s + 1) % 5;
            }
        })
    }
}