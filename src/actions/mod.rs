pub mod authentication;
pub mod cache;
pub mod cards;
pub mod decks;
pub mod session;
pub mod sets;

pub struct ErrorPayload {
    content: String,
}

pub enum GlobalAction {
    Authentication(authentication::AuthAction),
    Cache(cache::CacheAction),
    Cards(cards::CardsAction),
    Decks(decks::DecksAction),
    Session(session::SessionAction),
    Sets(sets::SetsAction),
}
