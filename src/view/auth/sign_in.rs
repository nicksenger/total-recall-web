use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    components::{button, link, text_input, ButtonType},
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

    div![
        header![
            attrs! { At::Class => "spectrum-CSSComponent-heading" },
            h1![
                attrs! { At::Class => "spectrum-Heading spectrum-Heading--XXXL spectrum-Heading-serif" },
                "Login"
            ],
        ],
        if model.authentication.loading {
            vec![p!["loading..."]]
        } else if let Some(username) = &model.authentication.username {
            vec![
                p![format!("Already logged in as {}. ", username)],
                br![],
                button(
                    "Logout",
                    ButtonType::Secondary,
                    move |_| Msg::Authentication(AuthMsg::Logout),
                    false,
                ),
            ]
        } else {
            vec![
                p![
                    "Don't have an account? ",
                    link("Register", format!("{}", Route::Register).as_str()),
                ],
                br![],
                form![
                    attrs! { At::Class => "spectrum-Form" },
                    text_input(
                        "text",
                        "Username",
                        "Enter your username",
                        username.get().as_str(),
                        move |value| username.set(value)
                    ),
                    text_input(
                        "password",
                        "Password",
                        "Enter your password",
                        password.get().as_str(),
                        move |value| password.set(value)
                    ),
                    button(
                        "Go!",
                        ButtonType::CTA,
                        move |_| Msg::Authentication(AuthMsg::AttemptLogin(AttemptLoginPayload {
                            username: username.get(),
                            password: password.get()
                        })),
                        username.get().len() == 0 || password.get().len() == 0
                    ),
                ],
            ]
        }
    ]
}
