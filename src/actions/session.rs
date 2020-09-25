use super::ErrorPayload;
use crate::state::entities::Card;

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

pub enum SessionAction {
    RateCard(RateCardPayload),
    RateCardFailed(ErrorPayload),
    RateCardSuccess(RateCardSuccessPayload),
    RevealCard(RevealCardPayload),
    ReviewCard(ReviewCardPayload),
    Study(StudyPayload),
}
