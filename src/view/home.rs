use seed::{prelude::*, *};
use seed_style::{pc, px, *};

use crate::{components::link, messages::Msg, state::Model, BASE_URI};

pub fn view(_model: &Model) -> Node<Msg> {
    div![
        header![
            C!["spectrum-CSSComponent-heading"],
            h1![
                C!["spectrum-Heading spectrum-Heading--XXXL spectrum-Heading-serif"],
                "Welcome to Total Recall"
            ],
        ],
        p![
            C!["spectrum-Body spectrum-Body--M"],
            "Total Recall is a spaced-repetition flashcard application designed to enable efficient study of foreign language vocabulary.",
        ],
        br![],
        br![],
        img![
            attrs! { At::Src => "http://localhost:8000/banner.jpeg" },
            s().width(pc(100)),
            s().border_radius(px(25)),
        ],
        br![],
        br![],
        br![],
        div![
            s().margin_bottom(px(80)),
            s().position("relative"),
            s().box_sizing("border-box"),
            div![
                s().media("@media screen and (max-width: 768px)").flex_direction("column"),
                s().display("flex"),
                home_section(
                    "Efficiency",
                    vec![
                        "Total Recall uses the SuperMemo-2 algorithm to determine if and when each of your vocabulary",
                        " flashcards should be reviewed.",
                        " Reviewing flashcards at calculated intervals means that you",
                        " will lean more, faster."
                    ],
                    vec![
                        link(
                            "Read more about SuperMemo",
                            "https://en.wikipedia.org/wiki/SuperMemo"
                        )
                    ]
                ),
                home_section(
                    "Automation",
                    vec![
                        "Images and audio are downloaded for each and every flashcard added to Total Recall,",
                        " so you can spend less time configuring and more time studying."
                    ],
                    vec![
                        link(
                            "View supported languages",
                            "https://translate.google.com/about/languages/"
                        )
                    ]
                ),
                home_section(
                    "Flexibility",
                    vec![
                        "Applications are available for web and Android (not iOS because they charge to target their platform)",
                        ", and a lightning-fast GraphQL API written in Rust is exposed to support custom client implementations."
                    ],
                    vec![
                        link(
                            "Download the Android client",
                            format!("{}/trc.apk", BASE_URI).as_str(),
                        ),
                        br![],
                        link(
                            "Explore the API",
                            format!("{}/graphiql", BASE_URI).as_str(),
                        )
                    ]
                )
            ]
        ]
    ]
}

fn home_section(title: &str, description: Vec<&str>, links: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        s().flex_basis("100%"),
        s().max_width("100%"),
        s().media("@media only screen and (min-width: 768px)")
            .flex_basis("33.33333333%"),
        s().media("@media only screen and (min-width: 768px)")
            .max_width("33.33333333%"),
        s().padding_right(px(16)),
        s().padding_left(px(16)),
        h3![
            C!["spectrum-Heading spectrum-Heading--S"],
            title
        ],
        br![],
        div![
            p![C!["spectrum-Body--M"], description],
            br![],
            p![C!["spectrum-Body--M"], links],
        ],
        br![],
    ]
}
