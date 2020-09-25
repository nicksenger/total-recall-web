use std::collections::VecDeque;

use rand::prelude::*;
use seed::prelude::Orders;

use super::entities::Card;
use crate::actions::{
    session::{ScoreValue, SessionAction},
    GlobalAction,
};
use crate::utilities::super_memo::needs_review;

pub enum SessionStatus {
    Prompt,
    Score,
}

pub struct SessionModel {
    loading: bool,
    rate_queue: VecDeque<Card>,
    review_queue: VecDeque<Card>,
    status: SessionStatus,
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

pub fn update(
    action: &GlobalAction,
    model: &mut SessionModel,
    orders: &mut impl Orders<GlobalAction>,
) {
    match action {
        GlobalAction::Session(SessionAction::Study(payload)) => {
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

        GlobalAction::Session(SessionAction::RateCard(_)) => {
            model.loading = true;
        }

        GlobalAction::Session(SessionAction::RateCardSuccess(payload)) => {
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

        GlobalAction::Session(SessionAction::RateCardFailed(_)) => {
            model.loading = false;
        }

        GlobalAction::Session(SessionAction::RevealCard(_)) => {
            model.status = SessionStatus::Score;
        }

        GlobalAction::Session(SessionAction::ReviewCard(payload)) => {
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
