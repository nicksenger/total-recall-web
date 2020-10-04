use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    components::{button, button_link, dialog, ButtonType},
    messages::{
        decks::{DecksMsg, DeleteDeckPayload},
        routing::RoutingMsg,
        Msg,
    },
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, deck_id: usize, username: &str) -> Node<Msg> {
    let deck = model.entities.decks.get(&deck_id);
    let modal_visible = use_state(|| false);

    div![
        header![
            attrs! { At::Class => "spectrum-CSSComponent-heading" },
            h1![
                attrs! { At::Class => "spectrum-Heading spectrum-Heading--L spectrum-Heading-serif" },
                format!(
                    "{}",
                    deck.map(|d| d.name.as_str()).unwrap_or("Unknown deck")
                )
            ],
        ],
        if model.ui.deck_details_screen.loading {
            vec![vec![p!["loading..."]]]
        } else {
            vec![
                if modal_visible.get() {
                    vec![
                        dialog(
                            format!(
                                "Delete Deck \"{}\" ?",
                                deck.map(|d| d.name.as_str()).unwrap_or("Unknown deck")
                            )
                            .as_str(),
                            p!["Are you sure you want to delete this deck? All cards and sets for this deck will also be deleted."],
                            vec![
                                button(
                                    "Cancel",
                                    ButtonType::Secondary,
                                    move |_| modal_visible.set(false),
                                    false,
                                ),
                                button(
                                    "Delete",
                                    ButtonType::Warning,
                                    move |_| {
                                        modal_visible.set(false);
                                        Msg::Decks(DecksMsg::DeleteDeck(DeleteDeckPayload { deck_id }))
                                    },
                                    false,
                                ),
                            ],
                        ),
                    ]
                } else {
                    vec![]
                },
                vec![
                    p![
                        "Language: ",
                        strong![deck.map(|d| d.language.as_str()).unwrap_or("")],
                        attrs! { At::Class => "spectrum-Body spectrum-Body--M" }
                    ],
                    br![],
                    br![],
                    div![
                        attrs! { At::Class => "spectrum-ButtonGroup spectrum-ButtonGroup--vertical" },
                        button_link(
                            "View Cards",
                            ButtonType::Primary,
                            format!("{}", Route::DeckCards(username.to_owned(), deck_id)).as_str(),
                        ),
                        button_link(
                            "View Sets",
                            ButtonType::Primary,
                            format!("{}", Route::DeckSets(username.to_owned(), deck_id)).as_str()
                        ),
                        button(
                            "Delete Deck",
                            ButtonType::Warning,
                            move |_| {
                                modal_visible.set(true);
                                Msg::Routing(RoutingMsg::ModalOpen(true))
                            },
                            model
                                .authentication
                                .username
                                .as_ref()
                                .map(|s| s.as_str())
                                .unwrap_or("")
                                != username
                        )
                    ],
                ],
            ]
        }
    ]
}
