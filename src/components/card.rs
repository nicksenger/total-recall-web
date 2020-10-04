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
                    div![
                        C!["spectrum-Card-actionButton"],
                        button![
                            C!["spectrum-ActionButton" "spectrum-ActionButton--quiet"],
                            "Audio",
                            ev(Ev::Click, |_| Msg::Session(SessionMsg::PlayAudio(
                                PlayAudioPayload { uri: audio }
                            )))
                        ]
                    ]
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
                C!["spectrum-Card-footer"],
                IF!(card.link.is_some() => link(
                    "Context",
                    card.link.as_ref().unwrap()
                ))
            ]
        ]
    ]
}
