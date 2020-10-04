use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::{pc, *};

use crate::{
    components::{button, ButtonType},
    messages::{routing::RoutingMsg, Msg},
    state::{routing::Route, Model},
};

#[topo::nested]
pub fn view(model: &Model, username: &str) -> Node<Msg> {
    let un = username.to_owned();
    div![
        header![
            attrs! { At::Class => "spectrum-CSSComponent-heading" },
            h1![
                attrs! { At::Class => "spectrum-Heading spectrum-Heading--L spectrum-Heading-serif" },
                format!("{}'s decks", username)
            ],
        ],
        if model.ui.decks_screen.loading {
            vec![p!["loading..."]]
        } else {
            vec![
                table![
                    s().width(pc(100)),
                    attrs! { At::Class => "spectrum-Table" },
                    thead![
                        attrs! { At::Class => "spectrum-Table-head" },
                        tr![
                            th![attrs! { At::Class => "spectrum-Table-headCell" }, "Name"],
                            th![
                                attrs! { At::Class => "spectrum-Table-headCell" },
                                "Language"
                            ],
                        ]
                    ],
                    tbody![
                        attrs! { At::Class => "spectrum-Table-body" },
                        model.entities.decks.values().map(|d| {
                            let id = d.id;
                            let un = username.to_owned();
                            tr![
                                attrs! { At::Class => "spectrum-Table-row" },
                                td![
                                    attrs! { At::Class => "spectrum-Table-cell" },
                                    d.name.as_str()
                                ],
                                td![
                                    attrs! { At::Class => "spectrum-Table-cell" },
                                    d.language.as_str()
                                ],
                                ev(Ev::Click, move |_| Msg::Routing(RoutingMsg::Push(
                                    Route::DeckDetails(un, id)
                                ))),
                            ]
                        })
                    ]
                ],
                br![],
                br![],
                button(
                    "Add deck",
                    ButtonType::Secondary,
                    |_| Msg::Routing(RoutingMsg::Push(Route::AddDeck(un))),
                    false,
                ),
            ]
        }
    ]
}
