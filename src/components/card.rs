use seed::{prelude::*, *};
use seed_style::{px, *};

use crate::{
    BASE_URI,
    components::link,
    messages::session::{PlayAudioPayload, SessionMsg},
    state::entities::Card,
    Msg,
};

pub fn card(card: &Card) -> Node<Msg> {
    let audio = card.audio.clone();
    div![
        s().width(px(208)),
        div![
            C!["spectrum-Card"],
            div![
                s().background_image(format!("url({}/{})", BASE_URI, card.image).as_str()),
                C!["spectrum-Card-coverPhoto"],
            ],
            div![
                C!["spectrum-Card-body"],
                div![
                    C!["spectrum-Card-header"],
                    div![
                        C![
                            "spectrum-Card-title",
                            "spectrum-Heading",
                            "spectrum-Heading--XS"
                        ],
                        card.back.as_str()
                    ],
                ],
                div![
                    C!["spectrum-Card-content"],
                    div![
                        C![
                            "spectrum-Card-subtitle",
                            "spectrum-Detail",
                            "spectrum-Detail--S"
                        ],
                        card.front.as_str()
                    ],
                ]
            ],
            div![
                s().display("flex"),
                s().flex_direction("row-reverse"),
                s().align_items("center"),
                s().justify_content("space-between"),
                s().padding_top("var(--spectrum-card-footer-padding-top, var(--spectrum-global-dimension-size-100))"),
                s().padding_bottom("var(--spectrum-card-footer-padding-bottom, var(--spectrum-global-dimension-size-150))"),
                C!["spectrum-Card-footer"],
                div![
                    C!["spectrum-Card-actionButton"],
                    button![
                        C!["spectrum-ActionButton" "spectrum-ActionButton--quiet"],
                        img![
                            s().width(px(16)),
                            s().height(px(16)),
                            s().margin_x(px(4)),
                            attrs! { At::Src => format!("{}/audio.svg", BASE_URI) },
                        ],
                        "Audio",
                        ev(Ev::Click, |_| Msg::Session(SessionMsg::PlayAudio(
                            PlayAudioPayload { uri: audio }
                        )))
                    ]
                ],
                IF!(card.link.is_some() => link(
                    "Context",
                    card.link.as_ref().unwrap()
                ))
            ]
        ]
    ]
}
