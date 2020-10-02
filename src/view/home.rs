use seed::{prelude::*, *};
use seed_style::{pc, px, *};

use crate::{
    BASE_URI,
    components::link,
    messages::Msg,
    state::Model,
};

pub fn view(_model: &Model) -> Node<Msg> {
    div![
        header![
            attrs! { At::Class => "spectrum-CSSComponent-heading" },
            h1![
                attrs! { At::Class => "spectrum-Heading spectrum-Heading--XXXL spectrum-Heading-serif" },
                "Welcome to Total Recall"
            ],
        ],
        p![
            "Total Recall is a spaced-repetition flashcard application designed to enable efficient study of foreign language vocabulary.",
            attrs! { At::Class => "spectrum-Body spectrum-Body--M" }
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
                div![
                    s().flex_basis("100%"),
                    s().max_width("100%"),
                    s().media("@media only screen and (min-width: 768px)").flex_basis("33.33333333%"),
                    s().media("@media only screen and (min-width: 768px)").max_width("33.33333333%"),
                    s().padding_right(px(16)),
                    s().padding_left(px(16)),
                    h3![
                        attrs! { At::Class => "spectrum-Heading spectrum-Heading--S" },
                        "Efficiency"
                    ],
                    br![],
                    div![
                        p![
                            attrs! { At::Class => "spectrum-Body--M" },
                            "Total Recall uses the SuperMemo-2 algorithm to determine if and when each of your vocabulary",
                            " flashcards should be reviewed.", " Reviewing flashcards at calculated intervals means that you",
                            " will lean more, faster."
                        ],
                        br![],
                        p![
                            attrs! { At::Class => "spectrum-Body--M" },
                            link(
                                "Read more about SuperMemo",
                                "https://en.wikipedia.org/wiki/SuperMemo"
                            )
                        ],
                    ],
                    br![],
                ],
                div![
                    s().flex_basis("100%"),
                    s().max_width("100%"),
                    s().media("@media only screen and (min-width: 768px)").flex_basis("33.33333333%"),
                    s().media("@media only screen and (min-width: 768px)").max_width("33.33333333%"),
                    s().padding_right(px(16)),
                    s().padding_left(px(16)),
                    h3![
                        attrs! { At::Class => "spectrum-Heading spectrum-Heading--S" },
                        "Automation"
                    ],
                    br![],
                    div![
                        p![
                            attrs! { At::Class => "spectrum-Body--M" },
                            "Images and audio are downloaded for each and every flashcard added to Total Recall,",
                            " so you can spend less time configuring and more time studying."
                        ],
                        br![],
                        p![
                            attrs! { At::Class => "spectrum-Body--M" },
                            link(
                                "View supported languages",
                                "https://translate.google.com/about/languages/"
                            )
                        ],
                    ],
                    br![],
                ],
                div![
                    s().flex_basis("100%"),
                    s().max_width("100%"),
                    s().media("@media only screen and (min-width: 768px)").flex_basis("33.33333333%"),
                    s().media("@media only screen and (min-width: 768px)").max_width("33.33333333%"),
                    s().padding_right(px(16)),
                    s().padding_left(px(16)),
                    h3![
                        attrs! { At::Class => "spectrum-Heading spectrum-Heading--S" },
                        "Flexibility"
                    ],
                    br![],
                    div![
                        p![
                            attrs! { At::Class => "spectrum-Body--M" },
                            "Applications are available for web and Android (not iOS because they charge to target their platform)",
                            ", and a lightning-fast GraphQL API written in Rust is exposed to support custom client implementations."
                        ],
                        br![],
                        p![
                            attrs! { At::Class => "spectrum-Body--M" },
                            link(
                                "Download the Android client",
                                format!("{}/trc.apk", BASE_URI).as_str(),
                            ),
                            br![],
                            link(
                                "Explore the API",
                                format!("{}/graphiql", BASE_URI).as_str(),
                            )
                        ],
                    ],
                    br![],
                ]
            ]
        ]
    ]
}
