use seed::{prelude::*, *};

use crate::{
    components::{checkmark_small, dash_small},
    Msg,
};

pub enum CheckboxStatus {
    Empty,
    Indeterminate,
    Checked,
}

pub fn checkbox<MsU: 'static>(
    status: CheckboxStatus,
    invalid: bool,
    on_toggle: impl FnOnce(web_sys::Event) -> MsU + 'static + Clone,
    label: &str,
) -> Node<Msg> {
    let (checked, indeterminate) = match status {
        CheckboxStatus::Empty => (false, false),
        CheckboxStatus::Indeterminate => (false, true),
        CheckboxStatus::Checked => (true, false),
    };

    label![
        C![
            "spectrum-Checkbox",
            IF!(indeterminate => "is-indeterminate"),
            IF!(invalid => "is-invalid")
        ],
        input![
            C!["spectrum-Checkbox-input"],
            attrs! { At::Type => "checkbox", At::Checked => checked.as_at_value() },
            ev(Ev::Change, on_toggle)
        ],
        span![
            C!["spectrum-Checkbox-box"],
            svg![
                C![
                    "spectrum-Icon",
                    "spectrum-UIIcon-CheckmarkSmall",
                    "spectrum-Checkbox-checkmark"
                ],
                checkmark_small()
            ],
            svg![
                C![
                    "spectrum-Icon",
                    "spectrum-UIIcon-DashSmall",
                    "spectrum-Checkbox-partialCheckmark"
                ],
                dash_small()
            ]
        ],
        span![C!["spectrum-Checkbox-label"], label]
    ]
}
