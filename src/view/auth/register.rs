use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    messages::{
        authentication::{AuthMsg, RegisterPayload},
        Msg,
    },
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model) -> Node<Msg> {
    let username = use_state(String::new);
    let password = use_state(String::new);

    if model.ui.register_screen.loading {
        return p!["loading..."];
    }

    div![
        h3!["Register:"],
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
            ev(Ev::Click, move |_| Msg::Authentication(AuthMsg::Register(
                RegisterPayload {
                    username: username.get(),
                    password: password.get()
                }
            )))
        ],
        br![],
        br![],
        p![
            "Already have an account? ",
            a!["Login", attrs! { At::Href => Route::Login }]
        ]
    ]
}
