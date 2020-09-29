use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        authentication::{AttemptLoginPayload, AuthMsg},
        Msg,
    },
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model) -> Node<Msg> {
    let username = use_state(String::new);
    let password = use_state(String::new);

    if model.authentication.loading {
        return p!["loading..."];
    }

    if let Some(username) = &model.authentication.username {
        return p![
            format!("Logged in as {}. ", username),
            button![
                "click here",
                ev(Ev::Click, |_| Msg::Authentication(AuthMsg::Logout))
            ],
            " to log out."
        ];
    }

    div![
        h3!["Login:"],
        "Username:",
        input![
            attrs! { At::Value => username },
            input_ev(Ev::Input, move |value| username.set(value))
        ],
        br![],
        br![],
        "Password:",
        input![
            attrs! { At::Value => password, At::Type => "password" },
            input_ev(Ev::Input, move |value| password.set(value))
        ],
        br![],
        br![],
        button![
            "Go!",
            ev(Ev::Click, move |_| Msg::Authentication(
                AuthMsg::AttemptLogin(AttemptLoginPayload {
                    username: username.get(),
                    password: password.get()
                })
            ))
        ],
        br![],
        br![],
        p![
            "Don't have an account? ",
            a!["Register", attrs! { At::Href => Route::Register }]
        ]
    ]
}
