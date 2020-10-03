use seed::{prelude::*, *};

use crate::Msg;

pub fn dialog(title: &str, content: Node<Msg>, buttons: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        attrs! {
          At::Class => "spectrum-Modal-wrapper is-open"
        },
        div![
            attrs! {
              At::Class => "spectrum-Modal is-open",
            },
            section![
                attrs! {
                  At::Class => "spectrum-Dialog spectrum-Dialog--medium spectrum-Dialog--confirmation"
                },
                div![
                    attrs! {
                      At::Class => "spectrum-Dialog-grid"
                    },
                    h1![
                        attrs! {
                          At::Class => "spectrum-Dialog-heading spectrum-Dialog-heading--noHeader"
                        },
                        title
                    ],
                    hr![attrs! {
                      At::Class => "spectrum-Divider spectrum-Divider--medium spectrum-Divider--horizontal spectrum-Dialog-divider"
                    }],
                    section![
                        attrs! {
                          At::Class => "spectrum-Dialog-content"
                        },
                        content
                    ],
                    div![
                        attrs! {
                          At::Class => "spectrum-ButtonGroup spectrum-Dialog-buttonGroup spectrum-Dialog-buttonGroup--noFooter"
                        },
                        buttons
                    ]
                ]
            ]
        ]
    ]
}
