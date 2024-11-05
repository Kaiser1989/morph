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
    // start game loop
    Box::new(Game::new(app, GameManager::default())).with_logging(log::LevelFilter::Info).init();
}

// declared as pub to avoid dead_code warnings from cdylib target build
#[cfg(not(target_os = "android"))]
pub fn main() {
    // start game loop
    Box::new(Game::new(GameManager::default())).with_logging(log::LevelFilter::Info).init();
}
