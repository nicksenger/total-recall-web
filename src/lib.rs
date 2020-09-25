mod messages;
mod operations;
mod state;
mod utilities;
mod view;

use seed::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, state::update, view::root);
}

fn init(_: Url, _: &mut impl Orders<messages::Msg>) -> state::Model {
    state::Model::new()
}
