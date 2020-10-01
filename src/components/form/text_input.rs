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
        attrs! { At::Class => "spectrum-Form-item" },
        label![
            attrs! {
                At::Class => "spectrum-Form-itemLabel spectrum-FieldLabel--left",
                At::For => format!("{}{}-input", label, placeholder),
            },
            label
        ],
        div![
            attrs! { At::Class => "spectrum-Form-itemField" },
            div![
                attrs! { At::Class => "spectrum-Textfield" },
                input![
                    attrs! {
                        At::Type => input_type,
                        At::Class => "spectrum-Textfield-input",
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
