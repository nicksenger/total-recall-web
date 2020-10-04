use std::collections::HashSet;

use seed::virtual_dom::update_el::UpdateEl;
use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::{pc, *};

use crate::{
    components::*,
    messages::{
        session::{SessionMsg, StudyPayload},
        sets::{GotoAddSetPayload, SetsMsg},
        Msg,
    },
    state::{entities::Card, routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.cards_screen.loading {
        return p!["loading..."];
    }

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
    let un = username.to_owned();
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
                    "{} cards:",
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
                    move |_| Msg::Sets(SetsMsg::GotoAddSet(GotoAddSetPayload {
                        username: un,
                        deck_id,
                        cards: selected_cards.get().iter().cloned().collect::<Vec<usize>>()
                    })),
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
            button_link(
                "Add Card",
                ButtonType::CTA,
                format!("{}", Route::AddCard(username.to_owned(), deck_id)).as_str()
            ),
        ],
    ]
}
