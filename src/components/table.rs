use std::collections::HashSet;
use std::hash::Hash;

use seed::{prelude::*, *};
use seed_style::{px, *};

use crate::{
    components::{checkbox, CheckboxStatus},
    Msg,
};

pub fn table<T: 'static + Copy + Eq + Hash>(
    items: Vec<T>,
    columns: Vec<&str>,
    render_row: impl Fn(T) -> Vec<Node<Msg>>,
    multi_select: Option<(impl Fn(HashSet<T>) + 'static + Clone, HashSet<T>)>,
) -> Node<Msg> {
    div![
        C!["spectrum-Table"],
        div![
            s().display("flex"),
            C!["spectrum-Table-head"],
            if let Some(ms) = multi_select.as_ref() {
                let cloned_items = items.clone();
                let (f, selected) = ms.clone();
                vec![div![
                    C!["spectrum-Table-headCell", "spectrum-Table-checkboxCell"],
                    checkbox(
                        if items.iter().all(|id| selected.contains(id)) {
                            CheckboxStatus::Checked
                        } else if items.iter().any(|id| selected.contains(id)) {
                            CheckboxStatus::Indeterminate
                        } else {
                            CheckboxStatus::Empty
                        },
                        false,
                        move |_| {
                            let mut new_selected = selected.clone();
                            if cloned_items.iter().all(|id| selected.contains(id)) {
                                cloned_items.iter().for_each(|id| {
                                    new_selected.remove(id);
                                })
                            } else {
                                new_selected.extend(cloned_items);
                            };
                            f(new_selected);
                        },
                        ""
                    )
                ]]
            } else {
                vec![]
            },
            columns
                .iter()
                .map(|x| { div![s().flex("1"), C!["spectrum-Table-headCell"], x] })
                .collect::<Vec<Node<Msg>>>()
        ],
        div![
            C!["spectrum-Table-body"],
            items.iter().map(|x| {
                div![
                    s().display("flex"),
                    s().cursor("auto"),
                    C!["spectrum-Table-row"],
                    if let Some(ms) = multi_select.as_ref() {
                        let cloned_x = x.clone();
                        let (f, selected) = ms.clone();
                        vec![div![
                            s().padding_y(px(8)),
                            C!["spectrum-Table-cell", "spectrum-Table-checkboxCell"],
                            checkbox(
                                if selected.contains(x) {
                                    CheckboxStatus::Checked
                                } else {
                                    CheckboxStatus::Empty
                                },
                                false,
                                move |_| {
                                    let mut new_set = selected.clone();
                                    if selected.contains(&cloned_x) {
                                        new_set.remove(&cloned_x);
                                    } else {
                                        new_set.insert(cloned_x);
                                    }
                                    f(new_set);
                                },
                                ""
                            )
                        ]]
                    } else {
                        vec![]
                    },
                    render_row(*x)
                        .into_iter()
                        .map(|n| {
                            div![
                                s().flex("1"),
                                s().display("flex"),
                                s().align_items("center"),
                                C!["spectrum-Table-cell"],
                                n
                            ]
                        })
                        .collect::<Vec<Node<Msg>>>(),
                ]
            })
        ],
    ]
}
