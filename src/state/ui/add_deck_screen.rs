use crate::messages::{decks::DecksMsg, Msg};

pub struct AddDeckScreenModel {
  pub loading: bool,
}

impl AddDeckScreenModel {
  pub fn new() -> Self {
      Self { loading: false }
  }
}

pub fn update(
  action: &Msg,
  model: &mut AddDeckScreenModel,
) {
  match action {
      Msg::Decks(DecksMsg::AddDeck(_))
      | Msg::Decks(DecksMsg::GetLanguages) => {
          model.loading = true;
      }

      Msg::Decks(DecksMsg::AddDeckFailed(_))
      | Msg::Decks(DecksMsg::AddDeckSuccess(_))
      | Msg::Decks(DecksMsg::GetLanguagesFailed(_))
      | Msg::Decks(DecksMsg::GetLanguagesSuccess(_)) => {
          model.loading = false;
      }

      _ => {}
  }
}
