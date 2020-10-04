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
            C!["spectrum-CSSComponent-heading"],
            h1![
                C!["spectrum-Heading spectrum-Heading--L spectrum-Heading-serif"],
                format!("{}'s decks", username)
            ],
        ],
        if model.ui.decks_screen.loading {
            vec![p!["loading..."]]
        } else {
            vec![
                table![
                    s().width(pc(100)),
                    C!["spectrum-Table"],
                    thead![
                        C!["spectrum-Table-head"],
                        tr![
                            th![
                                C!["spectrum-Table-headCell"],
                                "Name"
                            ],
                            th![
                                C!["spectrum-Table-headCell"],
                                "Language"
                            ],
                        ]
                    ],
                    tbody![
                        C!["spectrum-Table-body"],
                        model.entities.decks.values().map(|d| {
                            let id = d.id;
                            let un = username.to_owned();
                            tr![
                                C!["spectrum-Table-row"],
                                td![
                                    C!["spectrum-Table-cell"],
                                    d.name.as_str()
                                ],
                                td![
                                    C!["spectrum-Table-cell"],
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
