use std::collections::VecDeque;

use rand::prelude::*;
use seed::{browser::web_storage::WebStorageError, prelude::*};
use serde::{Deserialize, Serialize};

use super::entities::Card;
use crate::messages::{
    cache::CacheMsg,
    session::{ScoreValue, SessionMsg},
    Msg,
};
use crate::utilities::super_memo::needs_review;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum SessionStatus {
    Prompt,
    Score,
}

pub struct SessionModel {
    pub rng: ThreadRng,
    pub loading: bool,
    pub rate_queue: VecDeque<Card>,
    pub review_queue: VecDeque<Card>,
    pub status: SessionStatus,
}

impl SessionModel {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
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
            cards.shuffle(&mut model.rng);
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
            if let Some(card) = model.rate_queue.pop_front() {
                match payload.rating {
                    ScoreValue::Zero | ScoreValue::One | ScoreValue::Two | ScoreValue::Three => {
                        model.review_queue.push_back(card);
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
            model.status = SessionStatus::Prompt;
            let len = model.review_queue.len();
            if let Some(card) = model.review_queue.pop_front() {
                match payload.rating {
                    ScoreValue::Four | ScoreValue::Five => {}
                    _ => {
                        model
                            .review_queue
                            .insert(model.rng.gen_range(len / 2, len), card);
                    }
                }
            }
        }

        Msg::Cache(CacheMsg::Hydrate) => {
            let rate_queue: Result<VecDeque<Card>, WebStorageError> =
                LocalStorage::get("rate_queue");
            let review_queue: Result<VecDeque<Card>, WebStorageError> =
                LocalStorage::get("review_queue");
            let status: Result<SessionStatus, WebStorageError> = LocalStorage::get("status");

            if let Ok(q) = rate_queue {
                model.rate_queue = q;
            }

            if let Ok(q) = review_queue {
                model.review_queue = q;
            }

            if let Ok(status) = status {
                model.status = status;
            }
        }

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::messages::{session::*, ErrorPayload};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn get_current_time() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
    }

    fn get_test_card(id: usize, needs_review: bool) -> Card {
        Card {
            id,
            audio: "http://foo.com/bar.mp3".to_owned(),
            image: "http://foo.com/bar.jpg".to_owned(),
            back: "hello".to_owned(),
            front: "hola".to_owned(),
            created: get_current_time().as_millis(),
            last_seen: get_current_time().as_millis(),
            link: None,
            score: if needs_review {
                vec![]
            } else {
                vec![ScoreValue::Five, ScoreValue::Five, ScoreValue::Five]
            },
        }
    }

    #[test]
    fn study() {
        let mut model = SessionModel::new();
        model.review_queue.push_back(get_test_card(123, false));
        model.review_queue.push_back(get_test_card(456, false));

        update(
            &Msg::Session(SessionMsg::Study(StudyPayload {
                cards: vec![get_test_card(123, false), get_test_card(456, true)],
            })),
            &mut model,
        );

        assert_eq!(model.review_queue.is_empty(), true);
        assert_eq!(model.rate_queue.is_empty(), false);
        assert_eq!(model.rate_queue.len(), 1);
        assert_eq!(model.status, SessionStatus::Prompt);
    }

    #[test]
    fn rate_card() {
        let mut model = SessionModel::new();
        update(
            &Msg::Session(SessionMsg::RateCard(RateCardPayload {
                card_id: 123,
                rating: ScoreValue::Five,
            })),
            &mut model,
        );

        assert_eq!(model.loading, true);
    }

    #[test]
    fn rate_card_success() {
        let mut model = SessionModel::new();
        model.rate_queue.push_back(get_test_card(123, true));
        model.rate_queue.push_back(get_test_card(456, true));

        update(
            &Msg::Session(SessionMsg::RateCardSuccess(RateCardSuccessPayload {
                card_id: 123,
                rating: ScoreValue::Two,
            })),
            &mut model,
        );

        assert_eq!(model.rate_queue.len(), 1);
        assert_eq!(model.review_queue.len(), 1);

        update(
            &Msg::Session(SessionMsg::RateCardSuccess(RateCardSuccessPayload {
                card_id: 456,
                rating: ScoreValue::Five,
            })),
            &mut model,
        );

        assert_eq!(model.rate_queue.len(), 0);
        assert_eq!(model.review_queue.len(), 1);
    }

    #[test]
    fn rate_card_failed() {
        let mut model = SessionModel::new();
        model.loading = true;

        update(
            &Msg::Session(SessionMsg::RateCardFailed(ErrorPayload {
                content: "Failed!".to_owned(),
            })),
            &mut model,
        );

        assert_eq!(model.loading, false);
    }

    #[test]
    fn review_card() {
        let mut model = SessionModel::new();
        model.review_queue.push_back(get_test_card(123, true));
        model.review_queue.push_back(get_test_card(456, true));

        update(
            &Msg::Session(SessionMsg::ReviewCard(ReviewCardPayload {
                rating: ScoreValue::Zero,
            })),
            &mut model,
        );

        assert_eq!(model.review_queue.len(), 2);

        update(
            &Msg::Session(SessionMsg::ReviewCard(ReviewCardPayload {
                rating: ScoreValue::Five,
            })),
            &mut model,
        );

        assert_eq!(model.review_queue.len(), 1);

        update(
            &Msg::Session(SessionMsg::ReviewCard(ReviewCardPayload {
                rating: ScoreValue::Five,
            })),
            &mut model,
        );

        assert_eq!(model.review_queue.len(), 0);
    }
}
