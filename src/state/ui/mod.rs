use seed::prelude::Orders;

use crate::actions::GlobalAction;

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
    add_card_screen: add_card_screen::AddCardScreenModel,
    add_deck_screen: add_deck_screen::AddDeckScreenModel,
    add_set_screen: add_set_screen::AddSetScreenModel,
    card_details_screen: card_details_screen::CardDetailsScreenModel,
    card_link_screen: card_link_screen::CardLinkScreenModel,
    cards_screen: cards_screen::CardsScreenModel,
    deck_details_screen: deck_details_screen::DeckDetailsScreenModel,
    decks_screen: decks_screen::DecksScreenModel,
    edit_card_link_screen: edit_card_link_screen::EditCardLinkScreenModel,
    register_screen: register_screen::RegisterScreenModel,
    sets_screen: sets_screen::SetsScreenModel,
    set_details_screen: set_details_screen::SetDetailsScreenModel,
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

pub fn update(action: &GlobalAction, model: &mut UIModel, orders: &mut impl Orders<GlobalAction>) {
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
