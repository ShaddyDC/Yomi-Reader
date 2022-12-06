use dioxus::prelude::*;

use crate::read_state::ReaderState;

#[derive(Props)]
pub(crate) struct NavProps<'a> {
    read_state: &'a UseRef<Option<ReaderState>>,
}

pub(crate) fn nav_component<'a>(cx: Scope<'a, NavProps<'a>>) -> Element<'a> {
    let read_state = cx.props.read_state;

    let (current_page, page_count) = match read_state.read().as_ref() {
        Some(state) => (state.get_page(), state.get_page_count()),
        _ => return cx.render(rsx! {p{"No document"}}),
    };

    cx.render(rsx! {
        div { "Page {current_page}/{page_count}" }
        button {
            onclick: move |_| {
               std::mem::drop(read_state.write().as_mut().unwrap().prev_page());
           },
            "Previous"
         }
         button {
             onclick: move |_| {
                std::mem::drop(read_state.write().as_mut().unwrap().next_page());
            },
             "Next"
          }
    })
}
