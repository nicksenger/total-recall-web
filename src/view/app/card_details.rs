use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        cards::{CardsMsg, DeleteCardPayload},
        Msg,
    },
    state::{entities::Card, Model},
    BASE_URI,
};

#[topo::nested]
pub fn view(model: &Model, card_id: usize) -> Node<Msg> {
    if model.ui.card_details_screen.loading {
        return p!["loading..."];
    }

    model
        .entities
        .cards
        .get(&card_id)
        .map(card_view)
        .unwrap_or(p!["Card not loaded."])
}

fn card_view(card: &Card) -> Node<Msg> {
    let card_id = card.id;

    div![
        label!["Front: ", strong![card.front.to_owned()]],
        br![],
        label!["Back: ", strong![card.back.to_owned()]],
        br![],
        img![attrs! { At::Src => format!("{}/{}", BASE_URI, card.image) }],
        br![],
        label![
            "Audio: ",
            a![
                attrs! { At::Href => format!("{}/{}", BASE_URI, card.audio) },
                card.audio.as_str()
            ]
        ],
        br![],
        card.link
            .as_ref()
            .map(|link| label!["Link: ", a![attrs! { At::Href => link }, link.as_str()]])
            .unwrap_or(div![]),
        br![],
        br![],
        button![
            "Delete",
            ev(Ev::Click, move |_| Msg::Cards(CardsMsg::DeleteCard(
                DeleteCardPayload { card_id }
            )))
        ]
    ]
}
