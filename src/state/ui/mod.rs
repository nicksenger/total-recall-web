use seed::prelude::Orders;

use crate::messages::Msg;

mod add_card_screen;
mod add_deck_screen;
mod add_set_screen;
mod card_details_screen;
mod card_link_screen;
mod cards_screen;
mod deck_details_screen;
mod decks_screen;
mod edit_card_link_screen;
mod register_screen;
mod set_details_screen;
mod sets_screen;

pub struct UIModel {
    pub add_card_screen: add_card_screen::AddCardScreenModel,
    pub add_deck_screen: add_deck_screen::AddDeckScreenModel,
    pub add_set_screen: add_set_screen::AddSetScreenModel,
    pub card_details_screen: card_details_screen::CardDetailsScreenModel,
    pub card_link_screen: card_link_screen::CardLinkScreenModel,
    pub cards_screen: cards_screen::CardsScreenModel,
    pub deck_details_screen: deck_details_screen::DeckDetailsScreenModel,
    pub decks_screen: decks_screen::DecksScreenModel,
    pub edit_card_link_screen: edit_card_link_screen::EditCardLinkScreenModel,
    pub register_screen: register_screen::RegisterScreenModel,
    pub sets_screen: sets_screen::SetsScreenModel,
    pub set_details_screen: set_details_screen::SetDetailsScreenModel,
}

impl UIModel {
    pub fn new() -> Self {
      Self {
        add_card_screen: add_card_screen::AddCardScreenModel::new(),
        add_deck_screen: add_deck_screen::AddDeckScreenModel::new(),
        add_set_screen: add_set_screen::AddSetScreenModel::new(),
        card_details_screen: card_details_screen::CardDetailsScreenModel::new(),
        card_link_screen: card_link_screen::CardLinkScreenModel::new(),
        cards_screen: cards_screen::CardsScreenModel::new(),
        deck_details_screen: deck_details_screen::DeckDetailsScreenModel::new(),
        decks_screen: decks_screen::DecksScreenModel::new(),
        edit_card_link_screen: edit_card_link_screen::EditCardLinkScreenModel::new(),
        register_screen: register_screen::RegisterScreenModel::new(),
        sets_screen: sets_screen::SetsScreenModel::new(),
        set_details_screen: set_details_screen::SetDetailsScreenModel::new(),
      }
    }
  }

pub fn update(action: &Msg, model: &mut UIModel, orders: &mut impl Orders<Msg>) {
    add_card_screen::update(action, &mut model.add_card_screen, orders);
    add_deck_screen::update(action, &mut model.add_deck_screen, orders);
    add_set_screen::update(action, &mut model.add_set_screen, orders);
    card_details_screen::update(action, &mut model.card_details_screen, orders);
    card_link_screen::update(action, &mut model.card_link_screen, orders);
    cards_screen::update(action, &mut model.cards_screen, orders);
    deck_details_screen::update(action, &mut model.deck_details_screen, orders);
    decks_screen::update(action, &mut model.decks_screen, orders);
    edit_card_link_screen::update(action, &mut model.edit_card_link_screen, orders);
    register_screen::update(action, &mut model.register_screen, orders);
    sets_screen::update(action, &mut model.sets_screen, orders);
    set_details_screen::update(action, &mut model.set_details_screen, orders);
}
