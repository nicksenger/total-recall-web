use seed::prelude::Orders;

use crate::actions::{decks::DecksAction, GlobalAction};

pub struct AddDeckScreenModel {
  loading: bool,
}

impl AddDeckScreenModel {
  pub fn new() -> Self {
      Self { loading: false }
  }
}

pub fn update(
  action: &GlobalAction,
  model: &mut AddDeckScreenModel,
  orders: &mut impl Orders<GlobalAction>,
) {
  match action {
      GlobalAction::Decks(DecksAction::AddDeck(_))
      | GlobalAction::Decks(DecksAction::GetLanguages) => {
          model.loading = true;
      }

      GlobalAction::Decks(DecksAction::AddDeckFailed(_))
      | GlobalAction::Decks(DecksAction::AddDeckSuccess(_))
      | GlobalAction::Decks(DecksAction::GetLanguagesFailed(_))
      | GlobalAction::Decks(DecksAction::GetLanguagesSuccess(_)) => {
          model.loading = false;
      }

      _ => {}
  }
}
