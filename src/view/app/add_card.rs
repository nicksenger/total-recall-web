use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        cards::{AddCardPayload, CardsMsg},
        Msg,
    },
    state::Model,
};

#[topo::nested]
pub fn view(model: &Model, username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.add_card_screen.loading {
        return p!["loading..."];
    }

    if model.authentication.username.is_none()
        || model
            .authentication
            .username
            .as_ref()
            .map(|un| un.as_str())
            .unwrap()
            != username
    {
        return p![format!(
            "You must be logged in as {} to add cards for {}!",
            username, username
        )];
    }

    let front = use_state(String::new);
    let back = use_state(String::new);
    let link = use_state(|| None);

    div![
        h3!["Add card:"],
        "Front:",
        input![
            attrs! { At::Value => front },
            input_ev(Ev::Input, move |value| front.set(value))
        ],
        br![],
        br![],
        "Back:",
        input![
            attrs! { At::Value => back },
            input_ev(Ev::Input, move |value| back.set(value))
        ],
        br![],
        br![],
        "Link (optional):",
        input![
            attrs! { At::Value => link.get().unwrap_or("".to_owned()) },
            input_ev(Ev::Input, move |value| link.set(Some(value)))
        ],
        br![],
        br![],
        button![
            "Add",
            ev(Ev::Click, move |_| Msg::Cards(CardsMsg::AddCard(
                AddCardPayload {
                    deck_id,
                    front: front.get(),
                    back: back.get(),
                    link: link.get(),
                }
            )))
        ]
    ]
}
