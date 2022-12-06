extern crate web_sys;

use std::io::{Read, Seek};

use dioxus::prelude::*;
use epub::doc::EpubDoc;

#[derive(Props)]
pub(crate) struct NavProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<Option<EpubDoc<R>>>,
}

pub(crate) fn nav_component<'a, R: Read + Seek + 'a>(
    cx: Scope<'a, NavProps<'a, R>>,
) -> Element<'a> {
    let doc = cx.props.doc;

    if doc.read().is_none() {
        return cx.render(rsx! {p{"No document"}});
    }

    let page = use_state(&cx, || doc.read().as_ref().unwrap().get_current_page());

    let count = doc.read().as_ref().unwrap().get_num_pages();

    cx.render(rsx! {
        div { "Page {page}/{count}" }
        button {
            onclick: move |_| {
               std::mem::drop(doc.write().as_mut().unwrap().go_prev());
               page.set(doc.read().as_ref().unwrap().get_current_page());
           },
            "Previous"
         }
         button {
             onclick: move |_| {
                std::mem::drop(doc.write().as_mut().unwrap().go_next());
                page.set(doc.read().as_ref().unwrap().get_current_page());
            },
             "Next"
          }
    })
}
