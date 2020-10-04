use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::*;

use crate::{
    components::*,
    messages::{
        session::{RateCardPayload, RevealCardPayload, ReviewCardPayload, ScoreValue, SessionMsg},
        Msg,
    },
    state::{entities::Card, session::SessionStatus, Model},
};

#[topo::nested]
pub fn view(model: &Model) -> Node<Msg> {
    if model.authentication.token.is_none() {
        div![
            header![
                C!["spectrum-CSSComponent-heading"],
                h1![
                    C!["spectrum-Heading spectrum-Heading--XXXL spectrum-Heading-serif"],
                    "Unauthorized"
                ],
            ],
            p![
                "You must be logged in to study.",
                C!["spectrum-Body spectrum-Body--M"],
            ],
        ]
    } else if model.session.loading {
        p!["loading..."]
    } else if let Some(card) = model.session.rate_queue.iter().next() {
        study_card_view(card, &model.session.status, false)
    } else if let Some(card) = model.session.review_queue.iter().next() {
        study_card_view(card, &model.session.status, true)
    } else {
        div![
            header![
                C!["spectrum-CSSComponent-heading"],
                h1![
                    C!["spectrum-Heading spectrum-Heading--L spectrum-Heading-serif"],
                    "No Cards"
                ],
            ],
            p![
                C!["spectrum-Body spectrum-Body--M"],
                "There are currently no cards to study in the queue. You can add some from the cards page of any deck.",
            ],
        ]
    }
}

fn study_card_view(c: &Card, status: &SessionStatus, is_review: bool) -> Node<Msg> {
    match status {
        SessionStatus::Prompt => {
            let card = c.clone();
            div![
                s().display("flex"),
                s().flex_direction("column"),
                s().align_items("center"),
                header![
                    C!["spectrum-CSSComponent-heading"],
                    h1![
                        C![
                            "spectrum-Heading",
                            "spectrum-Heading--XL",
                            "spectrum-Heading-serif"
                        ],
                        c.front.as_str()
                    ],
                ],
                button(
                    "Flip",
                    ButtonType::Primary,
                    move |_| Msg::Session(SessionMsg::RevealCard(
                        RevealCardPayload { card }
                    )),
                    false
                ),
            ]
        }
        SessionStatus::Score => {
            let card_id = c.id;
            div![
                s().display("flex"),
                s().flex_direction("column"),
                s().align_items("center"),
                card(c),
                div![
                    C!["spectrum-ActionGroup", "spectrum-ActionGroup--compact"],
                    [
                        ScoreValue::Zero,
                        ScoreValue::One,
                        ScoreValue::Two,
                        ScoreValue::Three,
                        ScoreValue::Four,
                        ScoreValue::Five
                    ]
                    .iter()
                    .map(|rating| button!(
                        C!["spectrum-ActionButton" "spectrum-ActionGroup-item"],
                        ev(Ev::Click, move |_| if is_review {
                            Msg::Session(SessionMsg::ReviewCard(ReviewCardPayload {
                                rating: rating.clone(),
                            }))
                        } else {
                            Msg::Session(SessionMsg::RateCard(RateCardPayload {
                                card_id,
                                rating: rating.clone(),
                            }))
                        }),
                        span![
                            C!["spectrum-ActionButton-label"],
                            match rating {
                                ScoreValue::Zero => "0",
                                ScoreValue::One => "1",
                                ScoreValue::Two => "2",
                                ScoreValue::Three => "3",
                                ScoreValue::Four => "4",
                                _ => "5",
                            },
                        ]
                    ))
                ]
            ]
        }
    }
}
