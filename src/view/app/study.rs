use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        session::{RateCardPayload, RevealCardPayload, ReviewCardPayload, ScoreValue, SessionMsg},
        Msg,
    },
    state::{entities::Card, session::SessionStatus, Model},
    BASE_URI,
};

#[topo::nested]
pub fn view(model: &Model) -> Node<Msg> {
    if model.authentication.token.is_none() {
        p!["You must be logged in to study."]
    } else if model.session.loading {
        p!["loading..."]
    } else if let Some(card) = model.session.rate_queue.iter().next() {
        study_card_view(card, &model.session.status, false)
    } else if let Some(card) = model.session.review_queue.iter().next() {
        study_card_view(card, &model.session.status, true)
    } else {
        p!["Nothing to study."]
    }
}

fn study_card_view(card: &Card, status: &SessionStatus, is_review: bool) -> Node<Msg> {
    match status {
        SessionStatus::Prompt => {
            let card = card.clone();
            div![
                strong![card.front.to_owned()],
                br![],
                br![],
                button![
                    "Flip",
                    ev(Ev::Click, move |_| Msg::Session(SessionMsg::RevealCard(
                        RevealCardPayload { card }
                    )))
                ]
            ]
        }
        SessionStatus::Score => {
            let card_id = card.id;
            div![
                strong![card.back.to_owned()],
                br![],
                img![attrs! { At::Src => format!("{}/{}", BASE_URI, card.image) }],
                br![],
                br![],
                [
                    ScoreValue::Zero,
                    ScoreValue::One,
                    ScoreValue::Two,
                    ScoreValue::Three,
                    ScoreValue::Four,
                    ScoreValue::Five
                ]
                .iter()
                .map(|rating| button![
                    match rating {
                        ScoreValue::Zero => "Zero",
                        ScoreValue::One => "One",
                        ScoreValue::Two => "Two",
                        ScoreValue::Three => "Three",
                        ScoreValue::Four => "Four",
                        _ => "Five",
                    },
                    ev(Ev::Click, move |_| if is_review {
                        Msg::Session(SessionMsg::ReviewCard(ReviewCardPayload {
                            rating: rating.clone(),
                        }))
                    } else {
                        Msg::Session(SessionMsg::RateCard(RateCardPayload {
                            card_id,
                            rating: rating.clone(),
                        }))
                    })
                ])
            ]
        }
    }
}
