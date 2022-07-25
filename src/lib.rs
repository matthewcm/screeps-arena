use screeps_arena::game;
use wasm_bindgen::prelude::*;

mod global;
mod logging;
mod tutorials;

fn setup() {
    logging::setup_logging(logging::Info);
}

#[wasm_bindgen (js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

    if tick == 1 {
        setup()
    }

    tutorials::first_attack::run();

}
