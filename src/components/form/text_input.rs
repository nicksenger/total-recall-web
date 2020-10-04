use seed::{prelude::*, *};

use crate::Msg;

pub fn text_input<F: Clone + 'static>(
    input_type: &str,
    label: &str,
    placeholder: &str,
    value: &str,
    on_change: F,
) -> Node<Msg>
where
    F: FnOnce(String),
{
    div![
        C!["spectrum-Form-item"],
        label![
            C!["spectrum-Form-itemLabel spectrum-FieldLabel--left"],
            attrs! {
                At::For => format!("{}{}-input", label, placeholder),
            },
            label
        ],
        div![
            C!["spectrum-Form-itemField"],
            div![
                C!["spectrum-Textfield"],
                input![
                    C!["spectrum-Textfield-input"],
                    attrs! {
                        At::Type => input_type,
                        At::Placeholder => placeholder,
                        At::Id => format!("{}{}-input", label, placeholder),
                        At::Value => value,
                    },
                    input_ev(Ev::Input, on_change)
                ]
            ],
        ]
    ]
}
