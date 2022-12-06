extern crate web_sys;

use std::io::{Read, Seek};

use dioxus::prelude::*;
use epub::doc::EpubDoc;
use yomi_dict::deinflect::Reasons;

use crate::definitions::set_defs;

#[derive(Props)]
pub(crate) struct ReaderProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<Option<EpubDoc<R>>>,
    db: &'a UseRef<Option<yomi_dict::db::DB>>,
    reasons: &'a UseState<Reasons>,
}

pub(crate) fn reader_component<'a, 'b, R: Read + Seek + 'a>(
    cx: Scope<'a, ReaderProps<'a, R>>,
) -> Element<'a> {
    let doc = cx.props.doc;
    let db = cx.props.db;
    let reasons = cx.props.reasons;

    let defs = use_state(&cx, Vec::new);

    cx.render(rsx! {
        crate::nav::nav_component{ doc: doc }
        crate::view::view_component{
            doc: doc,
            onselect: move |evt: String| {
                let reasons = reasons.clone();
                let defs = defs.clone();
                let db = db.clone();
                wasm_bindgen_futures::spawn_local(async move{
                    set_defs(&defs, &db, reasons.get(), &evt).await;
                });
            }
        }
        crate::definitions::definitions_component{ definitions: defs.get() }
        crate::nav::nav_component{ doc: doc }
    })
}
