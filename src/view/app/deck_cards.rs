use std::collections::HashSet;

use seed::virtual_dom::update_el::UpdateEl;
use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        sets::{GotoAddSetPayload, SetsMsg},
        Msg,
    },
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.cards_screen.loading {
        return p!["loading..."];
    }

    let page = use_state(|| 0usize);
    let page_size = use_state(|| 20usize);
    let selected_cards = use_state(HashSet::<usize>::new);
    let deck_length = (&model)
        .entities
        .deck_cards
        .get(&deck_id)
        .map(|v| v.len())
        .unwrap_or(0);
    let un = username.to_owned();

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
            .map(|dc| dc
                .iter()
                .skip(page.get() * page_size.get())
                .take(page_size.get())
                .map(|card_id| (&model)
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
        p![format!(
            "Showing cards {} through {} of {}",
            deck_length.min(page_size.get() * page.get() + 1),
            deck_length.min(page_size.get() * (page.get() + 1)),
            deck_length
        )],
        button![
            "Prev",
            ev(Ev::Click, move |_| page.set(page.get() - 1)),
            if page.get() == 0 {
                attrs! { At::Disabled => true }
            } else {
                attrs! {}
            }
        ],
        button![
            "Next",
            ev(Ev::Click, move |_| page.set(page.get() + 1)),
            if page_size.get() * (page.get() + 1) >= deck_length {
                attrs! { At::Disabled => true }
            } else {
                attrs! {}
            }
        ],
        br![],
        br![],
        button![
            "Add set",
            ev(Ev::Click, move |_| Msg::Sets(SetsMsg::GotoAddSet(
                GotoAddSetPayload {
                    username: un,
                    deck_id,
                    cards: selected_cards.get().iter().cloned().collect::<Vec<usize>>()
                }
            )))
        ],
        br![],
        a![
            "Add card",
            attrs! { At::Href => Route::AddCard(username.to_owned(), deck_id) }
        ],
        br![],
        a![
            "View sets",
            attrs! { At::Href => Route::DeckSets(username.to_owned(), deck_id) }
        ]
    ]
}
