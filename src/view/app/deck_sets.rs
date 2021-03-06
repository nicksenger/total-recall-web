use std::collections::HashSet;

use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    components::*,
    messages::{
        session::{SessionMsg, StudyPayload, ScoreValue},
        Msg,
    },
    state::{entities::Card, routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, _username: &str, deck_id: usize) -> Node<Msg> {
    if model.ui.sets_screen.loading {
        return p!["loading..."];
    }

    let selected_sets = use_state(HashSet::<usize>::new);
    let deck_sets = (&model)
        .entities
        .deck_sets
        .get(&deck_id)
        .map(|ds| {
            ds.iter()
                .map(|set_id| (&model).entities.sets.get(set_id))
                .filter(|x| x.is_some())
                .map(|c| c.unwrap().id)
                .collect::<Vec<usize>>()
        })
        .unwrap_or(vec![]);
    let session_cards = selected_sets
        .get()
        .iter()
        .filter(|id| model.entities.sets.contains_key(id))
        .flat_map(|id| {
            model
                .entities
                .set_cards
                .get(id)
                .unwrap()
                .iter()
                .filter(|id| model.entities.cards.contains_key(id))
                .map(|id| model.entities.cards.get(id).unwrap().clone())
        })
        .collect::<Vec<Card>>();

    div![
        header![
            C!["spectrum-CSSComponent-heading"],
            h1![
                C!["spectrum-Heading spectrum-Heading--L spectrum-Heading-serif"],
                format!(
                    "{} sets",
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
            deck_sets.clone(),
            vec!["Name", "Score"],
            |id: usize| {
                let set = &model.entities.sets.get(&id).unwrap();
                vec![
                    link(
                        set.name.as_str(),
                        format!("{}", Route::SetDetails(id)).as_str(),
                    ),
                    score_meter(&match set.card_ids.iter().fold(0usize, |mut acc, cur| {
                        if let Some(card) = &model.entities.cards.get(cur) {
                            acc += match card.score.last() {
                                Some(ScoreValue::One) => 1,
                                Some(ScoreValue::Two) => 2,
                                Some(ScoreValue::Three) => 3,
                                Some(ScoreValue::Four) => 4,
                                Some(ScoreValue::Five) => 5,
                                _ => 0
                            };
                        }
                        acc
                    }) / set.card_ids.len() {
                        1 => ScoreValue::One,
                        2 => ScoreValue::Two,
                        3 => ScoreValue::Three,
                        4 => ScoreValue::Four,
                        5 => ScoreValue::Five,
                        _ => ScoreValue::Zero
                    })
                ]
            },
            Some((
                move |selected: HashSet<usize>| {
                    selected_sets.set(selected);
                },
                selected_sets.get()
            )),
        ),
        action_bar(vec![
            checkbox(
                if selected_sets.get().len() == deck_sets.len() {
                    CheckboxStatus::Checked
                } else if selected_sets.get().len() == 0 {
                    CheckboxStatus::Empty
                } else {
                    CheckboxStatus::Indeterminate
                },
                false,
                move |_| {
                    if selected_sets.get().len() == deck_sets.len() {
                        selected_sets.set(HashSet::new());
                    } else {
                        selected_sets.set(deck_sets.into_iter().collect());
                    }
                },
                format!("{} selected", selected_sets.get().len()).as_str()
            ),
            div![
                C!["spectrum-ActionGroup"],
                button(
                    "Study",
                    ButtonType::Action,
                    move |_| Msg::Session(SessionMsg::Study(StudyPayload {
                        cards: session_cards
                    })),
                    selected_sets.get().len() == 0
                )
            ]
        ])
    ]
}
