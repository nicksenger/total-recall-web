use seed::{prelude::*, *};

use crate::Msg;

pub enum ButtonType {
    CTA,
    Primary,
    Secondary,
}

pub fn button<F: Clone + 'static>(
    text: &str,
    button_type: ButtonType,
    on_click: F,
    disabled: bool,
) -> Node<Msg>
where
    F: FnOnce() -> Msg,
{
    let btn_class = match button_type {
        ButtonType::CTA => "spectrum-Button spectrum-Button--cta",
        ButtonType::Primary => "spectrum-Button spectrum-Button--primary",
        ButtonType::Secondary => "spectrum-Button spectrum-Button--secondary",
    };
    button![
        if disabled {
            attrs! { At::Class => btn_class, At::Disabled => "true" }
        } else {
            attrs! { At::Class => btn_class }
        },
        span![attrs! { At::Class => "spectrum-Button-label" }, text],
        ev(Ev::Click, |ev| {
            ev.prevent_default();
            on_click()
        })
    ]
}
