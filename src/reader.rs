extern crate web_sys;

use dioxus::prelude::*;
use yomi_dict::deinflect::Reasons;

use crate::{definitions::set_defs, read_state::ReaderState};

#[derive(Props)]
pub(crate) struct ReaderProps<'a> {
    read_state: &'a UseRef<Option<ReaderState>>,
    db: &'a UseRef<Option<yomi_dict::db::DB>>,
    reasons: &'a UseState<Reasons>,
}

pub(crate) fn reader_component<'a, 'b>(cx: Scope<'a, ReaderProps<'a>>) -> Element<'a> {
    let read_state = cx.props.read_state;
    let db = cx.props.db;
    let reasons = cx.props.reasons;

    let definitions = use_state(&cx, Vec::new);

    let has_document = read_state.read().is_some();

    if has_document {
        cx.render(rsx! {
            crate::nav::nav_component{ read_state: read_state }
            crate::view::view_component{
                read_state: read_state,
                onselect: move |evt: String| {
                    let reasons = reasons.clone();
                    let defs = definitions.clone();
                    let db = db.clone();
                    wasm_bindgen_futures::spawn_local(async move{
                        set_defs(&defs, &db, reasons.get(), &evt).await;
                    });
                }
            }
            crate::definitions::definitions_component{ definitions: definitions.get() }
            crate::nav::nav_component{ read_state: read_state }
        })
    } else {
        cx.render(rsx! {p{"No document"}})
    }
}
