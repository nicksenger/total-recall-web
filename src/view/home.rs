use seed::{prelude::*, *};

use crate::{
    messages::{authentication::AuthMsg, Msg},
    state::{routing::Route, Model},
};

pub fn view(model: &Model) -> Node<Msg> {
    if let Some(username) = &model.authentication.username {
        div![p![
            format!("Welcome back {}! Click ", username.as_str()),
            a![
                "here",
                attrs! { At::Href => Route::Decks(username.clone()) }
            ],
            " to view your decks. ",
            button![
                "Logout",
                ev(Ev::Click, |_| Msg::Authentication(AuthMsg::Logout))
            ]
        ]]
    } else {
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
}
