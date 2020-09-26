use graphql_client::Response;
use seed::prelude::fetch;

use super::ErrorPayload;
use crate::operations::decks;
use crate::state::entities::{Deck, Language};

pub struct AddDeckPayload {
    pub name: String,
    pub language: usize,
    pub username: String,
}

pub struct AddDeckSuccessPayload {
    pub deck: Deck,
    pub username: String,
}

pub struct DeleteDeckPayload {
    pub deck_id: usize,
}

pub struct DeleteDeckSuccessPayload {
    pub deck_id: usize,
}

pub struct GetDecksPayload {
    pub username: String,
}

pub struct GetDecksSuccessPayload {
    pub decks: Vec<Deck>,
}

pub struct GetLanguagesSuccessPayload {
    pub languages: Vec<Language>,
}

pub struct ViewDeckDetailsPayload {
    pub deck: Deck,
}

pub struct ViewDeckItemsPayload {
    pub deck: Deck,
}

pub enum DecksMsg {
    AddDeck(AddDeckPayload),
    AddDeckFetched(
        String,
        fetch::Result<Response<decks::create_deck::ResponseData>>,
    ),
    AddDeckFailed(ErrorPayload),
    AddDeckSuccess(AddDeckSuccessPayload),
    DeleteDeck(DeleteDeckPayload),
    DeleteDeckFetched(
        usize,
        fetch::Result<Response<decks::delete_deck::ResponseData>>,
    ),
    DeleteDeckFailed(ErrorPayload),
    DeleteDeckSuccess(DeleteDeckSuccessPayload),
    GetDecks(GetDecksPayload),
    GetDecksFetched(
        String,
        fetch::Result<Response<decks::user_decks::ResponseData>>,
    ),
    GetDecksFailed(ErrorPayload),
    GetDecksSuccess(GetDecksSuccessPayload),
    GetLanguages,
    GetLanguagesFetched(fetch::Result<Response<decks::language_list::ResponseData>>),
    GetLanguagesFailed(ErrorPayload),
    GetLanguagesSuccess(GetLanguagesSuccessPayload),
    ViewDeckDetails(ViewDeckDetailsPayload),
    ViewDeckItems(ViewDeckItemsPayload),
}
