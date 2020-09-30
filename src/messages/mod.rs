pub mod authentication;
pub mod cache;
pub mod cards;
pub mod decks;
pub mod routing;
pub mod session;
pub mod sets;

pub struct ErrorPayload {
    pub content: String,
}

pub enum Msg {
    Authentication(authentication::AuthMsg),
    Cache(cache::CacheMsg),
    Cards(cards::CardsMsg),
    Decks(decks::DecksMsg),
    Routing(routing::RoutingMsg),
    Session(session::SessionMsg),
    Sets(sets::SetsMsg),
}
