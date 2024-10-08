//////////////////////////////////////////////////
// Using

pub mod game;

use game_gl::prelude::*;

use crate::game::GameManager;

//////////////////////////////////////////////////
// Entry point for android

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    // init logging
    let log_level = log::LevelFilter::Trace;
    android_logger::init_once(android_logger::Config::default().with_max_level(log_level));

    // start game loop
    GameLoop::start(app, GameManager::default());
}

// declared as pub to avoid dead_code warnings from cdylib target build
#[cfg(not(target_os = "android"))]
pub fn main() {
    // init logging
    let log_level = log::LevelFilter::Debug;
    env_logger::builder()
        .filter_level(log_level) // Default Log Level
        .parse_default_env()
        .init();

    // start game loop
    GameLoop::start(GameManager::default());
}
