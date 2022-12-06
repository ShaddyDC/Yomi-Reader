mod upload_component;

extern crate web_sys;

use std::io::{Cursor, Read, Seek};

use dioxus::prelude::*;
use epub::doc::EpubDoc;
use yomi_dict::{deinflect::Reasons, translator::get_terms};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

#[derive(Props)]
struct NavProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<Option<EpubDoc<R>>>,
}

fn nav_component<'a, R: Read + Seek + 'a>(cx: Scope<'a, NavProps<'a, R>>) -> Element<'a> {
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

#[derive(Props)]
struct TextProps<'a, R: Read + Seek + 'a> {
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

fn text_component<'a, R: Read + Seek + 'a>(cx: Scope<'a, TextProps<'a, R>>) -> Element<'a> {
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

#[derive(Props)]
struct ReaderProps<'a, R: Read + Seek + 'a> {
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
    log::info!("Updating defs1");
    if db.read().is_none() {
        load_db(db).await;
    }
    if db.read().is_some() {
        log::info!("Updating defs2");
        defs.set(lookup(db.write().as_mut().unwrap(), reasons, data).await)
    }
}

fn reader_component<'a, 'b, R: Read + Seek + 'a>(cx: Scope<'a, ReaderProps<'a, R>>) -> Element<'a> {
    let doc = cx.props.doc;
    let db = cx.props.db;
    let reasons = cx.props.reasons;

    let defs = use_state(&cx, Vec::new);

    cx.render(rsx! {
        crate::nav_component{ doc: doc }
        crate::text_component{
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
        crate::definitions_component{ definitions: defs.get() }
        crate::nav_component{ doc: doc }
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

async fn load_db(db: &UseRef<Option<yomi_dict::db::DB>>) {
    if db.read().is_none() {
        db.write()
            .replace(yomi_dict::db::DB::new("data").await.unwrap());
        log::info!("Database loaded");
    }
}

async fn load_dict(dbc: &UseRef<Option<yomi_dict::db::DB>>, data: Vec<u8>) {
    log::info!("Start load");

    if dbc.read().is_none() {
        load_db(dbc).await;
    }
    log::info!("Starting");
    let res = yomi_dict::Dict::new(Cursor::new(&data));
    log::info!("Read");
    if let Err(err) = &res {
        log::info!("Dict res: {:?}", err);
    }
    if let Ok(valid_dict) = res {
        log::info!("loaded");

        dbc.write()
            .as_mut()
            .unwrap()
            .add_dict(valid_dict)
            .await
            .unwrap();

        log::info!("Updated dict");
    }
}

fn app(cx: Scope) -> Element {
    let reasons = use_state(&cx, || yomi_dict::deinflect::inflection_reasons());

    let db = use_ref(&cx, || None);

    // Cannot use async init for use_ref directly
    use_future(&cx, (), |()| {
        let dbc = db.clone();
        async move {
            load_db(&dbc).await;
        }
    });

    let doc = use_ref(&cx, || {
        let window = web_sys::window().expect("should have window");
        let storage = window
            .local_storage()
            .expect("should be able to get storage")
            .expect("should have storage");
        let item = storage
            .get_item("doc")
            .expect("should be able to access storage")?;
        let s = base64::decode(item).ok()?;

        EpubDoc::from_reader(Cursor::new(s)).ok()
    });

    let dbc = db.clone();
    let docc = doc.clone();

    cx.render(rsx! {
        upload_component::upload_component{
            id: "dict_id",
            label: "Upload Dict",
            upload_callback: move |data|{
                let dbc = dbc.clone();
                log::info!("Init load");
                wasm_bindgen_futures::spawn_local(async move{load_dict(&dbc, data).await;});
            }
        }
        upload_component::upload_component{
            id: "book_id",
            label: "Upload book",
            upload_callback: move |data| {
                log::info!("Starting");
                if let Ok(valid_doc) = EpubDoc::from_reader(Cursor::new(data.clone())){
                    log::info!("got string");
                    let data = base64::encode(data);
                    log::info!("got doc");
                    docc.set(Some(valid_doc));
                    let window = web_sys::window().expect("should have window");
                    let storage = window
                        .local_storage()
                        .expect("should be able to get storage")
                        .expect("should have storage");
                    storage.set_item("doc", &data).ok();

                    log::info!("Updated doc");

                }
            }
        }
        crate::reader_component{ doc: doc, db: db, reasons: reasons }
    })
}
