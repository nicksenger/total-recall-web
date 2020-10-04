use seed::{prelude::*, *};

use crate::Msg;

pub fn dialog(title: &str, content: Node<Msg>, buttons: Vec<Node<Msg>>) -> Node<Msg> {
    div![
        C!["spectrum-Modal-wrapper", "is-open"],
        div![
            C!["spectrum-Modal is-open"],
            section![
                C!["spectrum-Dialog spectrum-Dialog--medium spectrum-Dialog--confirmation"],
                div![
                    C!["spectrum-Dialog-grid"],
                    h1![
                        C!["spectrum-Dialog-heading spectrum-Dialog-heading--noHeader"],
                        title
                    ],
                    hr![C![
                        "spectrum-Divider",
                        "spectrum-Divider--medium",
                        "spectrum-Divider--horizontal",
                        "spectrum-Dialog-divider"
                    ]],
                    section![C!["spectrum-Dialog-content"], content],
                    div![
                      C!["spectrum-ButtonGroup", "spectrum-Dialog-buttonGroup", "spectrum-Dialog-buttonGroup--noFooter"],
                        buttons
                    ]
                ]
            ]
        ]
    ]
}
