use graphql_client::Response;
use seed::prelude::fetch;

use super::ErrorPayload;
use crate::operations::cards;
use crate::state::entities::Card;

pub struct AddCardPayload {
    pub deck_id: usize,
    pub front: String,
    pub back: String,
    pub link: Option<String>,
}

pub struct AddCardSuccessPayload {
    pub deck_id: usize,
}

pub struct DeleteCardPayload {
    pub card_id: usize,
}

pub struct DeleteCardSuccessPayload {
    pub card_id: usize,
}

pub struct EditCardLinkPayload {
    pub card_id: usize,
    pub link: String,
}

pub struct EditCardLinkSuccessPayload {
    pub card_id: usize,
    pub link: String,
}

pub struct GetCardsPayload {
    pub deck_id: usize,
}

pub struct GetCardsSuccessPayload {
    pub cards: Vec<Card>,
    pub deck_id: usize,
}

pub struct ViewCardDetailsPayload {
    pub card: Card,
}

pub struct ViewCardLinkPayload {
    pub link: String,
}

pub struct ViewEditCardLinkPayload {
    pub card: Card,
}

pub enum CardsMsg {
    AddCard(AddCardPayload),
    AddCardFailed(ErrorPayload),
    AddCardSuccess(AddCardSuccessPayload),
    DeleteCard(DeleteCardPayload),
    DeleteCardFailed(ErrorPayload),
    DeleteCardSuccess(DeleteCardSuccessPayload),
    EditCardLink(EditCardLinkPayload),
    EditCardLinkFailed(ErrorPayload),
    EditCardLinkSuccess(EditCardLinkSuccessPayload),
    GetCards(GetCardsPayload),
    GetCardsFetched((usize, fetch::Result<Response<cards::deck_cards::ResponseData>>)),
    GetCardsFailed(ErrorPayload),
    GetCardsSuccess(GetCardsSuccessPayload),
    ViewCardDetails(ViewCardDetailsPayload),
    ViewCardLink(ViewCardLinkPayload),
    ViewEditCardLink(ViewEditCardLinkPayload),
}
