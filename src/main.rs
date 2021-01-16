#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(clippy::mutex_atomic)] // idk how to use AtomicBools yet
#[macro_use]
extern crate lazy_static;

pub mod app;
pub mod data;
pub mod views;

use crate::app::{MarketDashboard, USER_VEC};
use std::{sync::RwLock, thread, time::Duration};

lazy_static! {
    static ref THREAD_UPDATE_SYNC: RwLock<bool> = RwLock::new(false);
}

fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let app = MarketDashboard::default();

    USER_VEC.update();

    thread::spawn(|| loop {
        #[cfg(debug_assertions)]
        println!("-----| UPDATE SYNC |-----");

        *THREAD_UPDATE_SYNC.write().unwrap() = true;
        *THREAD_UPDATE_SYNC.write().unwrap() = false;

        thread::sleep(Duration::new(30, 0));
    });

    eframe::run_native(Box::new(app));
}
