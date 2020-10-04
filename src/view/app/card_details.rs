use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    components::*,
    messages::{
        cards::{CardsMsg, DeleteCardPayload, EditCardLinkPayload},
        routing::RoutingMsg,
        Msg,
    },
    state::{entities::Card, Model},
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

fn card_view(c: &Card) -> Node<Msg> {
    let card_id = c.id;
    let delete_modal_visible = use_state(|| false);
    let edit_link_modal_visible = use_state(|| false);
    let card_link = use_state(|| c.link.as_ref().map(|s| s.clone()).unwrap_or("".to_owned()));

    div![
        IF!(edit_link_modal_visible.get() => dialog(
            "Edit Link",
            form![
                attrs! { At::Class => "spectrum-Form" },
                text_input(
                    "text",
                    "Link",
                    "Enter a context link",
                    card_link.get().as_str(),
                    move |value| card_link.set(value),
                ),
            ],
            vec![
                button(
                    "Cancel",
                    ButtonType::Secondary,
                    move |_| {
                        edit_link_modal_visible.set(false);
                        Msg::Routing(RoutingMsg::ModalOpen(false))
                    },
                    false,
                ),
                button(
                    "Confirm",
                    ButtonType::CTA,
                    move |_| {
                        edit_link_modal_visible.set(false);
                        Msg::Cards(CardsMsg::EditCardLink(EditCardLinkPayload {
                            card_id,
                            link: card_link.get()
                        }))
                    },
                    false,
                ),
            ],
        )),
        IF!(delete_modal_visible.get() => dialog(
            format!(
                "Delete Card \"{}\" ?",
                c.front.as_str()
            )
            .as_str(),
            p!["Are you sure you want to delete this card? It will be removed from the deck and any containing sets."],
            vec![
                button(
                    "Cancel",
                    ButtonType::Secondary,
                    move |_| {
                        delete_modal_visible.set(false);
                        Msg::Routing(RoutingMsg::ModalOpen(false))
                    },
                    false,
                ),
                button(
                    "Delete",
                    ButtonType::Warning,
                    move |_| {
                        delete_modal_visible.set(false);
                        Msg::Cards(CardsMsg::DeleteCard(
                            DeleteCardPayload { card_id }
                        ))
                    },
                    false,
                ),
            ],
        )),
        card(c),
        br![],
        div![
            C!["spectrum-ButtonGroup spectrum-ButtonGroup--vertical"],
            button(
                "Edit Link",
                ButtonType::Secondary,
                move |_| {
                    edit_link_modal_visible.set(true);
                    Msg::Routing(RoutingMsg::ModalOpen(true))
                },
                false
            ),
            button(
                "Delete",
                ButtonType::Warning,
                move |_| {
                    delete_modal_visible.set(true);
                    Msg::Routing(RoutingMsg::ModalOpen(true))
                },
                false
            )
        ]
    ]
}
