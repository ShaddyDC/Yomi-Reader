use dioxus::prelude::*;

use crate::read_state::ReaderState;

#[derive(Props)]
pub struct NavProps<'a> {
    read_state: &'a UseRef<Option<ReaderState>>,
}

pub fn nav_component<'a>(cx: Scope<'a, NavProps<'a>>) -> Element<'a> {
    let read_state = cx.props.read_state;

    let (current_page, page_count) = match read_state.read().as_ref() {
        Some(state) => (state.get_page(), state.get_page_count()),
        _ => return cx.render(rsx! {p{"No document"}}),
    };

    cx.render(rsx! {
        nav{
            class: "flex",

            button {
                class: "flex-1 bg-gray-100 rounded-full",

                onclick: move |_| {
                    read_state.with_mut(|state| state.as_mut().unwrap().prev_page());
                },
                "Previous"
            }
            div {
                class: "flex-1 text-center",

                "Chapter {current_page}/{page_count}"
            }
            button {
                class: "flex-1 bg-gray-100 rounded-full",

                onclick: move |_| {
                    read_state.with_mut(|state| state.as_mut().unwrap().next_page());
                },
                "Next"
            }
        }
    })
}
