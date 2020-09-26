use std::collections::VecDeque;

use rand::prelude::*;

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
            if let Some(card) = model.rate_queue.pop_front() {
                match payload.rating {
                    ScoreValue::Zero | ScoreValue::One | ScoreValue::Two | ScoreValue::Three => {
                        model.rate_queue.insert(
                            thread_rng()
                                .gen_range(model.rate_queue.len() / 2, model.rate_queue.len()),
                            card,
                        );
                    }
                    _ => {}
                }
            }
            model.rate_queue.pop_front();
        }

        Msg::Session(SessionMsg::RateCardFailed(_)) => {
            model.loading = false;
        }

        Msg::Session(SessionMsg::RevealCard(_)) => {
            model.status = SessionStatus::Score;
        }

        Msg::Session(SessionMsg::ReviewCard(payload)) => {
            if let Some(card) = model.rate_queue.pop_front() {
                match payload.rating {
                    ScoreValue::Four | ScoreValue::Five => {
                        model.review_queue.pop_front();
                    }
                    _ => {
                        model.review_queue.insert(
                            thread_rng()
                                .gen_range(model.rate_queue.len() / 2, model.rate_queue.len()),
                            card,
                        );
                    }
                }
            }
        }

        _ => {}
    }
}
