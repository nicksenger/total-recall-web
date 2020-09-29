use std::collections::VecDeque;

use rand::prelude::*;
use seed::prelude::*;

use super::entities::Card;
use crate::messages::{
    session::{ScoreValue, SessionMsg},
    Msg,
};
use crate::utilities::super_memo::needs_review;

pub enum SessionStatus {
    Prompt,
    Score,
}

pub struct SessionModel {
    pub loading: bool,
    pub rate_queue: VecDeque<Card>,
    pub review_queue: VecDeque<Card>,
    pub status: SessionStatus,
}

impl SessionModel {
    pub fn new() -> Self {
        Self {
            loading: false,
            rate_queue: VecDeque::new(),
            review_queue: VecDeque::new(),
            status: SessionStatus::Prompt,
        }
    }
}

pub fn update(action: &Msg, model: &mut SessionModel) {
    match action {
        Msg::Session(SessionMsg::Study(payload)) => {
            let mut cards: Vec<Card> = payload
                .cards
                .iter()
                .cloned()
                .filter(|c| needs_review(c))
                .collect();
            cards.shuffle(&mut thread_rng());
            model.rate_queue = cards.into_iter().collect();
            model.review_queue.retain(|_| false);
            model.status = SessionStatus::Prompt;
        }

        Msg::Session(SessionMsg::RateCard(_)) => {
            model.loading = true;
        }

        Msg::Session(SessionMsg::RateCardSuccess(payload)) => {
            model.loading = false;
            model.status = SessionStatus::Prompt;
            let len = model.review_queue.len();
            if let Some(card) = model.rate_queue.pop_front() {
                match payload.rating {
                    ScoreValue::Zero | ScoreValue::One | ScoreValue::Two | ScoreValue::Three => {
                        model.review_queue.insert(
                            (unsafe { js_sys::Math::random() } * (len - len / 2) as f64) as usize
                                + len / 2,
                            card,
                        );
                    }
                    _ => {}
                }
            }
        }

        Msg::Session(SessionMsg::RateCardFailed(_)) => {
            model.loading = false;
        }

        Msg::Session(SessionMsg::RevealCard(_)) => {
            model.status = SessionStatus::Score;
        }

        Msg::Session(SessionMsg::ReviewCard(payload)) => {
            let len = model.review_queue.len();
            model.status = SessionStatus::Prompt;
            if let Some(card) = model.review_queue.pop_front() {
                match payload.rating {
                    ScoreValue::Four | ScoreValue::Five => {}
                    _ => {
                        model.review_queue.insert(
                            (unsafe { js_sys::Math::random() } * (len - len / 2) as f64) as usize
                                + len / 2,
                            card,
                        );
                    }
                }
            }
        }

        _ => {}
    }
}
