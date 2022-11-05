use std::io::{Cursor, Read, Seek};

use dioxus::prelude::*;
use epub::doc::EpubDoc;

fn main() {
    dioxus::web::launch(app);
}

#[derive(PartialEq, Props)]
struct ReaderProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<EpubDoc<R>>,
}

fn nav_component<'a, R: Read + Seek + 'a>(cx: Scope<'a, ReaderProps<'a, R>>) -> Element<'a> {
    let doc = cx.props.doc;

    let page = use_state(&cx, || doc.read().get_current_page());

    let count = doc.read().get_num_pages();

    cx.render(rsx! {
        div { "Page {page}/{count}" }
        button {
            onclick: move |_| {
               std::mem::drop(doc.write().go_prev());
               page.set(doc.read().get_current_page());
           },
            "Previous"
         }
         button {
             onclick: move |_| {
                std::mem::drop(doc.write().go_next());
                page.set(doc.read().get_current_page());
            },
             "Next"
          }
    })
}

fn text_component<'a, R: Read + Seek + 'a>(cx: Scope<'a, ReaderProps<'a, R>>) -> Element<'a> {
    let doc = cx.props.doc;

    let text = doc
        .write()
        .get_current_str()
        .unwrap_or_else(|_| "".to_string());

    cx.render(rsx! {
        div { class: "iframe",  dangerous_inner_html: "{text}"  } // TODO: Properly sandbox
    })
}

fn reader_component<'a, R: Read + Seek + 'a>(cx: Scope<'a, ReaderProps<'a, R>>) -> Element<'a> {
    let doc = cx.props.doc;

    cx.render(rsx! {
        crate::nav_component{ doc: doc }
        crate::text_component{ doc: doc }
        crate::nav_component{ doc: doc }
    })
}

fn app(cx: Scope) -> Element {
    let file = include_bytes!("test.epub");
    let doc = EpubDoc::from_reader(Cursor::new(file)).unwrap();

    let doc = use_ref(&cx, || doc);

    cx.render(rsx! {
        crate::reader_component{ doc: doc }
    })
}
