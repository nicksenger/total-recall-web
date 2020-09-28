pub mod authentication;
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
    Cards(cards::CardsMsg),
    Decks(decks::DecksMsg),
    Routing(routing::RoutingMessage),
    Session(session::SessionMsg),
    Sets(sets::SetsMsg),
}
