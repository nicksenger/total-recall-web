use crate::messages::{Msg, cache::CacheMsg};

mod add_deck_screen;
mod card_details_screen;
mod cards_screen;
mod deck_details_screen;
mod decks_screen;
mod register_screen;
mod set_details_screen;
mod sets_screen;

pub struct UIModel {
    pub dark_theme: bool,
    pub add_deck_screen: add_deck_screen::AddDeckScreenModel,
    pub card_details_screen: card_details_screen::CardDetailsScreenModel,
    pub cards_screen: cards_screen::CardsScreenModel,
    pub deck_details_screen: deck_details_screen::DeckDetailsScreenModel,
    pub decks_screen: decks_screen::DecksScreenModel,
    pub register_screen: register_screen::RegisterScreenModel,
    pub sets_screen: sets_screen::SetsScreenModel,
    pub set_details_screen: set_details_screen::SetDetailsScreenModel,
}

impl UIModel {
    pub fn new() -> Self {
        Self {
            dark_theme: false,
            add_deck_screen: add_deck_screen::AddDeckScreenModel::new(),
            card_details_screen: card_details_screen::CardDetailsScreenModel::new(),
            cards_screen: cards_screen::CardsScreenModel::new(),
            deck_details_screen: deck_details_screen::DeckDetailsScreenModel::new(),
            decks_screen: decks_screen::DecksScreenModel::new(),
            register_screen: register_screen::RegisterScreenModel::new(),
            sets_screen: sets_screen::SetsScreenModel::new(),
            set_details_screen: set_details_screen::SetDetailsScreenModel::new(),
        }
    }
}

pub fn update(msg: &Msg, model: &mut UIModel) {
    add_deck_screen::update(msg, &mut model.add_deck_screen);
    card_details_screen::update(msg, &mut model.card_details_screen);
    cards_screen::update(msg, &mut model.cards_screen);
    deck_details_screen::update(msg, &mut model.deck_details_screen);
    decks_screen::update(msg, &mut model.decks_screen);
    register_screen::update(msg, &mut model.register_screen);
    sets_screen::update(msg, &mut model.sets_screen);
    set_details_screen::update(msg, &mut model.set_details_screen);

    match msg {
        Msg::Cache(CacheMsg::ToggleDarkTheme(dark)) => {
            model.dark_theme = *dark;
        }

        _ => {}
    }
}
