pub mod authentication;
pub mod cache;
pub mod cards;
pub mod decks;
pub mod session;
pub mod sets;

pub struct ErrorPayload {
    content: String,
}

pub enum Msg {
    Authentication(authentication::AuthMsg),
    Cache(cache::CacheMsg),
    Cards(cards::CardsMsg),
    Decks(decks::DecksMsg),
    Session(session::SessionMsg),
    Sets(sets::SetsMsg),
}
