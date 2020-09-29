use std::collections::HashSet;

use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{Msg, sets::{SetsMsg, GotoAddSetPayload}},
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.sets_screen.loading {
        return p!["loading..."];
    }

    let selected_sets = use_state(HashSet::<usize>::new);

    div![
        h3![format!(
            "{} sets:",
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
            .deck_sets
            .get(&deck_id)
            .map(|ds| ds.iter().map(|set_id| (&model)
                .entities
                .sets
                .get(set_id)
                .map(|s| {
                    let id = s.id;
                    li![
                        input![
                            ev(Ev::Change, move |_| {
                                if selected_sets.get().contains(&id) {
                                    selected_sets.get().remove(&id);
                                } else {
                                    selected_sets.get().insert(id);
                                }
                            }),
                            attrs! { At::Type => "checkbox" },
                            if selected_sets.get().contains(&id) {
                                attrs! { At::Checked => true }
                            } else {
                                attrs! {}
                            }
                        ],
                        a![
                            s.name.to_owned(),
                            attrs! { At::Href => Route::SetDetails(s.id) }
                        ]
                    ]
                })
                .unwrap_or(div![])))
            .unwrap()],
        br![],
        br![],
        a![
            "View cards",
            attrs! { At::Href => Route::DeckCards(username.to_owned(), deck_id) }
        ]
    ]
}
