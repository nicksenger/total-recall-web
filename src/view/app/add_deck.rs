use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    components::{button, combobox, text_input, ButtonType},
    messages::{
        decks::{AddDeckPayload, DecksMsg},
        Msg,
    },
    state::Model,
};

#[topo::nested]
pub fn view(model: &Model, _username: &str) -> Node<Msg> {
    let deck_name = use_state(String::new);
    let language = use_state(|| "".to_owned());
    let selected_language = model
        .entities
        .languages
        .values()
        .find(|l| l.name.as_str() == language.get())
        .map(|l| l.id);

    let username = (&model)
        .authentication
        .username
        .as_ref()
        .map(|un| un.as_str())
        .unwrap_or("")
        .to_owned();

    let mut languages: Vec<String> = model
        .entities
        .languages
        .values()
        .map(|l| l.name.clone())
        .collect();
    languages.sort_unstable();

    div![
        header![
            C!["spectrum-CSSComponent-heading"],
            h1![
                C!["spectrum-Heading spectrum-Heading--L spectrum-Heading-serif"],
                "Add deck"
            ],
        ],
        if model.ui.add_deck_screen.loading {
            p!["loading..."]
        } else if model.authentication.username.is_none()
            || model
                .authentication
                .username
                .as_ref()
                .map(|un| un.as_str())
                .unwrap()
                != username
        {
            p![
                C!["spectrum-Body--M"],
                format!(
                    "You must be logged in as {} to add decks for {}!",
                    username, username
                )
            ]
        } else {
            form![
                C!["spectrum-Form"],
                text_input(
                    "text",
                    "Name",
                    "Enter a name for the deck",
                    deck_name.get().as_str(),
                    move |value| deck_name.set(value),
                ),
                combobox("Language", language.get(), languages, move |s| language
                    .set(s)),
                button(
                    "Go!",
                    ButtonType::CTA,
                    move |_| Msg::Decks(DecksMsg::AddDeck(AddDeckPayload {
                        language: selected_language.unwrap_or(0),
                        name: deck_name.get(),
                        username
                    })),
                    deck_name.get().len() == 0 || selected_language.is_none()
                ),
            ]
        }
    ]
}
