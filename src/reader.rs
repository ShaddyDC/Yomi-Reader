extern crate web_sys;

use std::io::{Read, Seek};

use dioxus::prelude::*;
use epub::doc::EpubDoc;
use yomi_dict::{deinflect::Reasons, translator::get_terms};

#[derive(Props)]
pub(crate) struct ReaderProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<Option<EpubDoc<R>>>,
    db: &'a UseRef<Option<yomi_dict::db::DB>>,
    reasons: &'a UseState<Reasons>,
}

struct Expression {
    expression: String,
    reading: String,
    entries: Vec<DictEntry>,
}

struct DictEntry {
    definitions: Vec<String>,
}

async fn lookup(db: &mut yomi_dict::db::DB, reasons: &Reasons, s: &str) -> Vec<Expression> {
    log::info!("Lookup");
    let definitions = get_terms(s, reasons, db).await.unwrap();
    log::info!("Lookup2: {:?}", definitions);

    let x = definitions
        .iter()
        .map(|d| Expression {
            expression: d.expression.to_owned(),
            reading: d.reading.to_owned(),
            entries: {
                d.entries
                    .iter()
                    .map(|e| DictEntry {
                        definitions: e
                            .term
                            .glossary
                            .iter()
                            .map(std::borrow::ToOwned::to_owned)
                            .collect::<Vec<_>>(),
                    })
                    .collect::<Vec<_>>()
            },
        })
        .collect::<Vec<_>>();

    x
}

async fn set_defs(
    defs: &UseState<Vec<Expression>>,
    db: &UseRef<Option<yomi_dict::db::DB>>,
    reasons: &Reasons,
    data: &str,
) {
    if db.read().is_none() {
        log::error!("Cannot update definitions since DB is not loaded yet!");
    } else {
        defs.set(lookup(db.write().as_mut().unwrap(), reasons, data).await)
    }
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
                let reasons = (reasons).clone();
                let defs = defs.clone();
                let db = db.clone();
                wasm_bindgen_futures::spawn_local(async move{
                    set_defs(&defs, &db, reasons.get(), &evt).await;
                });
            }
        }
        crate::reader::definitions_component{ definitions: defs.get() }
        crate::nav::nav_component{ doc: doc }
    })
}

#[inline_props]
fn definitions_component<'a>(cx: Scope, definitions: &'a Vec<Expression>) -> Element {
    cx.render(rsx!(ul{
        definitions.iter().map(|d| rsx!(
            li{
                key: "{d.expression}",
                div{
                    ruby {
                        p { "{d.expression}" }
                        rt{ "{d.reading}" }
                    }
                }
                div{
                    ol{
                        d.entries.iter().map(|e| rsx!(
                            li{
                                ul{
                                    e.definitions.iter().map(|s| rsx!(
                                        p{
                                            key: "{s}", // TODO keys
                                            "{s}"
                                        }
                                    ))
                                }
                            }
                        ))
                    }
                }
            }
        ))
    }))
}
