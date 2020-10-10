use graphql_client::Response;
use seed::prelude::fetch;
use serde::{Deserialize, Serialize};

use super::ErrorPayload;
use crate::operations::session;
use crate::state::entities::Card;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum ScoreValue {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
}

pub struct RateCardPayload {
    pub card_id: usize,
    pub rating: ScoreValue,
}

pub struct RateCardSuccessPayload {
    pub card_id: usize,
    pub rating: ScoreValue,
}

pub struct RevealCardPayload {
    pub card: Card,
}

pub struct ReviewCardPayload {
    pub rating: ScoreValue,
}

pub struct StudyPayload {
    pub cards: Vec<Card>,
}

pub struct PlayAudioPayload {
    pub uri: String,
}

pub enum SessionMsg {
    PlayAudio(PlayAudioPayload),
    RateCard(RateCardPayload),
    RateCardFetched(
        usize,
        ScoreValue,
        fetch::Result<Response<session::rate_card::ResponseData>>,
    ),
    RateCardFailed(ErrorPayload),
    RateCardSuccess(RateCardSuccessPayload),
    RevealCard(RevealCardPayload),
    ReviewCard(ReviewCardPayload),
    Study(StudyPayload),
}
