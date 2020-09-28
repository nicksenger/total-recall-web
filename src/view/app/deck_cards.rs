use std::collections::HashSet;

use seed::virtual_dom::update_el::UpdateEl;
use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::Msg,
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.cards_screen.loading {
        return p!["loading..."];
    }

    let selected_cards = use_state(HashSet::<usize>::new);

    div![
        h3![format!(
            "{} cards:",
            (&model)
                .entities
                .decks
                .get(&deck_id)
                .as_ref()
                .map(|d| d.name.as_str())
                .unwrap_or("unknown deck"),
        )],
        ul![(&model)
            .entities
            .deck_cards
            .get(&deck_id)
            .map(|dc| dc.iter().map(|card_id| (&model)
                .entities
                .cards
                .get(card_id)
                .map(|c| {
                    let id = c.id;
                    li![
                        input![
                            ev(Ev::Change, move |_| {
                                if selected_cards.get().contains(&id) {
                                    selected_cards.get().remove(&id);
                                } else {
                                    selected_cards.get().insert(id);
                                }
                            }),
                            attrs! { At::Type => "checkbox" },
                            if selected_cards.get().contains(&id) {
                                attrs! { At::Checked => true }
                            } else {
                                attrs! {}
                            }
                        ],
                        a![
                            c.front.to_owned(),
                            attrs! { At::Href => Route::CardDetails(c.id) }
                        ]
                    ]
                })
                .unwrap_or(div![])))
            .unwrap()],
        br![],
        br![],
        a![
            "View sets",
            attrs! { At::Href => Route::DeckSets(username.to_owned(), deck_id) }
        ]
    ]
}
