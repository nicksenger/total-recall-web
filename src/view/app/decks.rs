use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::Msg,
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, username: &str) -> Node<Msg> {
    if model.ui.decks_screen.loading {
        return p!["loading..."];
    }

    div![
        h3![format!("{}'s decks:", username)],
        ul![model.entities.decks.values().map(|d| li![a![
            d.name.clone(),
            attrs! { At::Href => Route::DeckCards(username.to_owned(), d.id) }
        ]])]
    ]
}
