use seed::{prelude::*, *};
use seed_hooks::*;
use seed_style::{pc, px, *};

use crate::Msg;

pub fn combobox<F: Clone + 'static>(
    label: &str,
    value: String,
    options: Vec<String>,
    on_change: F,
) -> Node<Msg>
where
    F: Fn(String),
{
    let open = use_state(|| false);
    let handler = on_change.clone();
    let filtered_options = options
        .into_iter()
        .filter(|s| s.to_lowercase().find(&value.to_lowercase()).is_some())
        .take(8)
        .collect::<Vec<String>>();
    div![
        attrs! { At::Class => "spectrum-Form-item" },
        label![
            attrs! {
                At::Class => "spectrum-Form-itemLabel spectrum-FieldLabel--left",
                At::For => format!("{}-select", label),
            },
            label
        ],
        div![
            attrs! { At::Class => "spectrum-Form-itemField" },
            div![
                attrs! { At::Class => "spectrum-InputGroup" },
                div![
                    attrs! { At::Class => "spectrum-Textfield" },
                    input![
                        attrs! {
                            At::Type => "text",
                            At::Class => "spectrum-Textfield-input",
                            At::Value => value,
                        },
                        input_ev(Ev::Input, move |s| {
                            open.set(s.as_str() != "");
                            handler(s);
                        }),
                        ev(Ev::Click, move |_| {
                            open.set(!open.get());
                        }),
                        ev(Ev::Blur, move |_| {
                            open.set(false);
                        })
                    ],
                ],
                if filtered_options.len() > 0 {
                    vec![div![
                        s().position("absolute"),
                        s().width(pc(100)),
                        s().top("100%"),
                        s().left(px(0)),
                        attrs! {
                            At::Class => format!(
                                "spectrum-Popover spectrum-Popover--bottom spectrum-Picker-popover{}",
                                if open.get() { " is-open" } else {""}
                            ),
                        },
                        ul![
                            attrs! {
                                At::Class => "spectrum-Menu"
                            },
                            filtered_options.into_iter().map(|s| {
                                let handler = on_change.clone();
                                li![
                                    attrs! { At::Class => "spectrum-Menu-item" },
                                    span![
                                        attrs! { At::Class => "spectrum-Menu-itemLabel" },
                                        s.as_str()
                                    ],
                                    ev(Ev::MouseDown, move |_| handler(s)),
                                ]
                            })
                        ]
                    ]]
                } else {
                    vec![]
                }
            ],
        ]
    ]
}
