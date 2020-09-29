use seed::{prelude::*, *};

use crate::{
    messages::Msg,
    state::{routing::Route, Model},
};

pub fn view(_model: &Model) -> Node<Msg> {
    div![
    p![
      "Total Recall is a spaced-repetition flashcard application designed to enable efficient study of foreign language vocabulary. ",
      "If you already have an account, ",
      a![attrs! { At::Href => Route::Login }, "login"],
      " to view your flashcard decks. Otherwise, you may ",
      a![attrs! { At::Href => Route::Register }, "register"],
      " an account here.",
    ],
    p!["To learn more about Total Recall, please see the ",
    a!["user manual"], "."]
  ]
}
