use seed::{prelude::*, *};
use seed_hooks::*;

use crate::{
    components::*,
    messages::{
        routing::RoutingMsg,
        sets::{DeleteSetPayload, SetsMsg},
        Msg,
    },
    state::{entities::Set, Model},
};

#[topo::nested]
pub fn view(model: &Model, set_id: usize) -> Node<Msg> {
    if model.ui.set_details_screen.loading {
        return p!["loading..."];
    }

    model
        .entities
        .sets
        .get(&set_id)
        .map(set_view)
        .unwrap_or(p!["Set not loaded."])
}

fn set_view(set: &Set) -> Node<Msg> {
    let delete_set_modal_open = use_state(|| false);
    let set_id = set.id;

    div![
        header![
            C!["spectrum-CSSComponent-heading"],
            h1![
                C!["spectrum-Heading spectrum-Heading--L spectrum-Heading-serif"],
                format!("{}", set.name)
            ],
        ],
        IF!(delete_set_modal_open.get() => dialog(
            format!(
                "Delete Set \"{}\" ?",
                set.name.as_str()
            )
            .as_str(),
            p!["Are you sure you want to delete this set? All cards for this set will be deleted preserved."],
            vec![
                button(
                    "Cancel",
                    ButtonType::Secondary,
                    move |_| {
                        delete_set_modal_open.set(false);
                        Msg::Routing(RoutingMsg::ModalOpen(false))
                    },
                    false,
                ),
                button(
                    "Delete",
                    ButtonType::Warning,
                    move |_| {
                        delete_set_modal_open.set(false);
                        Msg::Sets(SetsMsg::DeleteSet(DeleteSetPayload { set_id }))
                    },
                    false,
                ),
            ],
        )),
        button(
            "Delete Set",
            ButtonType::Warning,
            move |_| {
                delete_set_modal_open.set(true);
                Msg::Routing(RoutingMsg::ModalOpen(true))
            },
            false
        )
    ]
}
