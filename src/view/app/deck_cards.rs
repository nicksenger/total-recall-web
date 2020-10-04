use std::collections::HashSet;

use seed::virtual_dom::update_el::UpdateEl;
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::*;

use crate::{
    components::*,
    messages::{
        cards::{AddCardPayload, CardsMsg},
        routing::RoutingMsg,
        session::{SessionMsg, StudyPayload},
        sets::{AddSetPayload, SetsMsg},
        Msg,
    },
    state::{entities::Card, routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, _username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.cards_screen.loading {
        return p!["loading..."];
    }

    let create_card_modal_visible = use_state(|| false);
    let card_front = use_state(|| "".to_owned());
    let card_back = use_state(|| "".to_owned());
    let card_link = use_state(|| "".to_owned());
    let create_set_modal_visible = use_state(|| false);
    let set_name = use_state(|| "".to_owned());
    let page = use_state(|| 0usize);
    let page_size = use_state(|| 10usize);
    let selected_cards = use_state(HashSet::<usize>::new);
    let deck_cards = (&model)
        .entities
        .deck_cards
        .get(&deck_id)
        .map(|dc| {
            dc.iter()
                .map(|card_id| (&model).entities.cards.get(card_id))
                .filter(|x| x.is_some())
                .map(|c| c.unwrap().id)
                .collect::<Vec<usize>>()
        })
        .unwrap_or(vec![]);
    let page_cards = deck_cards
        .iter()
        .skip(page.get() * page_size.get())
        .take(page_size.get())
        .cloned()
        .collect::<Vec<usize>>();

    let deck_length = deck_cards.len();
    let session_cards = selected_cards
        .get()
        .iter()
        .filter(|id| model.entities.cards.contains_key(id))
        .map(|id| model.entities.cards.get(id).unwrap())
        .cloned()
        .collect::<Vec<Card>>();

    div![
        header![
            attrs! { At::Class => "spectrum-CSSComponent-heading" },
            h1![
                attrs! { At::Class => "spectrum-Heading spectrum-Heading--L spectrum-Heading-serif" },
                format!(
                    "{} cards",
                    (&model)
                        .entities
                        .decks
                        .get(&deck_id)
                        .as_ref()
                        .map(|d| d.name.as_str())
                        .unwrap_or("unknown deck"),
                )
            ],
        ],
        IF!(create_card_modal_visible.get() => dialog(
            "Add Card",
            form![
                attrs! { At::Class => "spectrum-Form" },
                text_input(
                    "text",
                    "Front",
                    "Native language word",
                    card_front.get().as_str(),
                    move |value| card_front.set(value),
                ),
                text_input(
                    "text",
                    "Back",
                    "Foreign language word",
                    card_back.get().as_str(),
                    move |value| card_back.set(value),
                ),
                text_input(
                    "text",
                    "Link",
                    "Optional context link",
                    card_link.get().as_str(),
                    move |value| card_link.set(value),
                ),
            ],
            vec![
                button(
                    "Cancel",
                    ButtonType::Secondary,
                    move |_| {
                        create_card_modal_visible.set(false);
                        Msg::Routing(RoutingMsg::ModalOpen(false))
                    },
                    false
                ),
                button(
                    "Confirm",
                    ButtonType::CTA,
                    move |_| {
                        create_card_modal_visible.set(false);
                        Msg::Cards(CardsMsg::AddCard(AddCardPayload {
                            deck_id,
                            front: card_front.get(),
                            back: card_back.get(),
                            link: if card_link.get() == "" {
                                None
                            } else {
                                Some(card_link.get())
                            }
                        }))
                    },
                    card_front.get().len() == 0 || card_back.get().len() == 0
                )
            ]
        )),
        IF!(create_set_modal_visible.get() => dialog(
            "Create Set",
            form![
                attrs! { At::Class => "spectrum-Form" },
                text_input(
                    "text",
                    "Name",
                    "Enter a name for the set",
                    set_name.get().as_str(),
                    move |value| set_name.set(value),
                ),
            ],
            vec![
                button(
                    "Cancel",
                    ButtonType::Secondary,
                    move |_| {
                        create_set_modal_visible.set(false);
                        Msg::Routing(RoutingMsg::ModalOpen(false))
                    },
                    false
                ),
                button(
                    "Confirm",
                    ButtonType::CTA,
                    move |_| {
                        create_set_modal_visible.set(false);
                        Msg::Sets(SetsMsg::AddSet(AddSetPayload {
                            deck_id,
                            card_ids: selected_cards.get().iter().cloned().collect::<Vec<usize>>(),
                            name: set_name.get()
                        }))
                    },
                    set_name.get().len() == 0
                )
            ]
        )),
        table(
            page_cards.clone(),
            vec!["Front", "Score"],
            |c: usize| {
                let card = &model.entities.cards.get(&c).unwrap();
                vec![
                    link(
                        card.front.as_str(),
                        format!("{}", Route::CardDetails(c)).as_str(),
                    ),
                    span![card.score.last().map(|d| "+").unwrap_or("-")],
                ]
            },
            Some((
                move |selected: HashSet<usize>| {
                    selected_cards.set(selected);
                },
                selected_cards.get()
            )),
        ),
        action_bar(vec![
            checkbox(
                if selected_cards.get().len() == deck_length {
                    CheckboxStatus::Checked
                } else if selected_cards.get().len() == 0 {
                    CheckboxStatus::Empty
                } else {
                    CheckboxStatus::Indeterminate
                },
                false,
                move |_| {
                    if selected_cards.get().len() == deck_length {
                        selected_cards.set(HashSet::new());
                    } else {
                        selected_cards.set(deck_cards.into_iter().collect());
                    }
                },
                format!("{} selected", selected_cards.get().len()).as_str()
            ),
            div![
                C!["spectrum-ActionGroup"],
                button(
                    "Create Set",
                    ButtonType::Action,
                    move |_| {
                        create_set_modal_visible.set(true);
                        Msg::Routing(RoutingMsg::ModalOpen(true))
                    },
                    selected_cards.get().len() == 0
                ),
                button(
                    "Study",
                    ButtonType::Action,
                    move |_| Msg::Session(SessionMsg::Study(StudyPayload {
                        cards: session_cards
                    })),
                    selected_cards.get().len() == 0
                )
            ]
        ]),
        div![
            s().display("flex"),
            s().justify_content("center"),
            pager(page.get(), deck_length / page_size.get() + 1, move |i| {
                page.set(i)
            }),
        ],
        div![
            s().display("flex"),
            s().justify_content("flex-end"),
            button(
                "Add Card",
                ButtonType::CTA,
                move |_| {
                    create_card_modal_visible.set(true);
                    Msg::Routing(RoutingMsg::ModalOpen(true))
                },
                false
            ),
        ],
    ]
}
