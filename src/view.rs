extern crate web_sys;

use std::io::{Read, Seek};

use dioxus::prelude::*;
use epub::doc::EpubDoc;

#[derive(Props)]
pub(crate) struct ViewProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<Option<EpubDoc<R>>>,
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

pub(crate) fn view_component<'a, R: Read + Seek + 'a>(
    cx: Scope<'a, ViewProps<'a, R>>,
) -> Element<'a> {
    let mut doc = cx.props.doc.write();
    let onselect = &cx.props.onselect;

    if let Some(doc) = &mut *doc {
        let text = doc.get_current_str().unwrap_or_else(|_| "".to_string());

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
