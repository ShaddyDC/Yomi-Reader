extern crate web_sys;

use dioxus::prelude::*;

use crate::read_state::ReaderState;

#[derive(Props)]
pub(crate) struct ViewProps<'a> {
    read_state: &'a UseRef<Option<ReaderState>>,
    onselect: EventHandler<'a, String>,
}

fn clicked(onselect: &EventHandler<String>) {
    // TODO Breaks on double click
    let sel = web_sys::window().unwrap().get_selection().unwrap().unwrap();
    let n = sel.anchor_node().unwrap();
    let s: String = n
        .text_content()
        .unwrap()
        .chars()
        .skip(sel.anchor_offset().try_into().unwrap())
        .take(16)
        .collect();

    log::info!("Clicked: {}", s);

    onselect.call(s);
}

pub(crate) fn view_component<'a>(cx: Scope<'a, ViewProps<'a>>) -> Element<'a> {
    let text = cx
        .props
        .read_state
        .write()
        .as_mut()
        .and_then(|state| state.get_text());

    let onselect = &cx.props.onselect;

    if let Some(text) = text {
        cx.render(rsx! {
            div {
                // TODO: Properly sandbox / iframe
                dangerous_inner_html: "{text}",
                onclick: |_| clicked(onselect)
            }
        })
    } else {
        cx.render(rsx! {p{"No document"}})
    }
}
