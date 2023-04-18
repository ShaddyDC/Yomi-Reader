extern crate web_sys;

use dioxus::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::{
    definitions::update_defs_and_selection,
    info_state::{InfoState, LoadDictState},
    read_state::ReaderState,
};

#[derive(Props)]
pub struct ReaderProps<'a> {
    read_state: &'a UseRef<Option<ReaderState>>,
    db: &'a UseRef<Option<yomi_dict::IndexedDB>>,
    reasons: &'a UseState<yomi_dict::Reasons>,
    info_state: &'a UseRef<InfoState>,
}

fn enable_scroll_callback(read_state: UseRef<Option<ReaderState>>) {
    let window = web_sys::window().expect("should have window");
    let document = window.document().expect("should have document");

    let Some(element) = document
            .get_element_by_id("reader-scroll")
            else {
                return;
            };

    let element_moved = element.clone();

    let scroll_callback = Closure::<dyn Fn()>::new(move || {
        let offset = element_moved.scroll_top();
        if let Some(state) = read_state.write().as_mut() {
            state.set_scroll(offset);
        }
    });

    element
        .add_event_listener_with_callback("scroll", scroll_callback.as_ref().unchecked_ref())
        .unwrap();

    scroll_callback.forget();
}

pub fn reader_component<'a>(cx: Scope<'a, ReaderProps<'a>>) -> Element<'a> {
    let read_state = cx.props.read_state;
    let info_state = cx.props.info_state;
    let db = cx.props.db;
    let reasons = cx.props.reasons;

    let definitions = use_state(cx, Vec::new);

    let has_document = read_state.read().is_some();

    // Set scroll after everything is rendered
    use_future(cx, (), |()| {
        let read_state = read_state.clone();

        async move {
            enable_scroll_callback(read_state);
        }
    });

    let document = if has_document {
        rsx! {
            div{
                class: "container mx-auto",

                crate::nav::nav_component{ read_state: read_state }
                crate::view::view_component{
                    read_state: read_state,
                    onselect: move |evt: String| {
                        let reasons = reasons.clone();
                        let defs = definitions.clone();
                        let db = db.clone();
                        wasm_bindgen_futures::spawn_local(async move{
                            update_defs_and_selection(&defs, &db, reasons.get(), &evt).await;
                        });
                    }
                }
                crate::nav::nav_component{ read_state: read_state }
            }
        }
    } else {
        rsx! {
        div{
            class: "container mx-auto h-full grid place-items-center",

            p{
                class: "text-center",

                "No document"
            }
        }}
    };

    let info = info_state.with(|s| match s {
        InfoState::Idle => {
            rsx! {crate::definitions::definitions_component{ definitions: definitions.get() }}
        }
        InfoState::LoadDB => rsx! {p{"Loading DB. Please wait"}},
        InfoState::LoadDict(LoadDictState::ParsingDict) => {
            rsx! {p{"Parsing dictionary. Please wait"}}
        }
        InfoState::LoadDict(LoadDictState::AddingDictIndex) => {
            rsx! {p{"Adding dictionary to database. Please wait"}}
        }
        InfoState::LoadDict(LoadDictState::AddingDictContent(current, total)) => {
            let (current, total) = (current.to_owned(), total.to_owned());
            rsx! {p{"Loading Dictionary: {current}/{total} Please wait"}}
        }
    });

    cx.render(rsx! {
        div{
            class: "px-4 h-3/5 overflow-y-scroll",
            id: "reader-scroll",

            document
        }

        div{
            class: "px-4 h-2/5 overflow-y-scroll bg-gray-50 rounded-md border-2",

            div{
                class: "container mx-auto",

                info
            }
        }

    })
}
