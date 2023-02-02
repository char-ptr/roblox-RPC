use std::{sync::Arc, time::{Duration, SystemTime}};

use discord_sdk::Discord;
use gp_rbx::{roblox, mngr::UserMngr};
use parking_lot as pl;


#[tokio::main(worker_threads = 5)]
async fn main() {
    let discord_app = discord_sdk::DiscordApp::PlainId(1067619218964611212);
    let (wheel, handlr) = discord_sdk::wheel::Wheel::new(Box::new(|err| {
        println!("{}", err);
    }));

    let mut user = wheel.user();

    let subs = discord_sdk::Subscriptions::ACTIVITY;

    let dsc =
        Discord::new(discord_app, subs, Box::new(handlr)).expect("unable to get dsc");
    let mngr = UserMngr::new(dsc);

    let joiner = mngr.start_activity_update();
    let ac_mngr = Arc::new(pl::RwLock::new(mngr));

    println!("now waiting for discord to respond");
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                let mut ac = ac_mngr.write();
                ac.stop();
                break;
            }
            _= user.0.changed() => {
                let _user = match &*user.0.borrow() {
                    discord_sdk::wheel::UserState::Connected(user) => {
                        let u = user.clone();
                        let ac = ac_mngr.write();

                        println!("connected to {}",u.username);

                        *ac.u.clone().write() = Some(u);
                        *ac.connected.clone().write() = true
                    },
                    discord_sdk::wheel::UserState::Disconnected(err) => {

                        let ac = ac_mngr.write();

                        if *ac.connected.clone().read() {
                            eprintln!("disconnected from user reason : {}",err);
                            *ac.u.clone().write() = None;
                            *ac.connected.clone().write() = false
                        }

                    },
                };
            }
        }
    }

    ac_mngr.write().stop();

    joiner.await.expect("unable to join status updater");

    // ac_mngr.write().await.dsc.write().await..disconnect().await;

    println!("bye")
}