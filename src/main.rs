#![allow(clippy::future_not_send)]

mod definitions;
mod info_state;
mod nav;
mod read_state;
mod reader;
mod upload_component;
mod view;

extern crate web_sys;

use std::io::Cursor;

use dioxus::prelude::*;
use info_state::InfoState;
use read_state::ReaderState;
use yomi_dict::DB;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app);
}

async fn load_db(db: &UseRef<Option<yomi_dict::IndexedDB>>, info_state: &UseRef<InfoState>) {
    if db.read().is_none() {
        // TODO get rid of all unwraps
        let new_db = yomi_dict::IndexedDB::new("data").await.unwrap();
        db.with_mut(|db| db.replace(new_db));
        info_state.with_mut(|s| {
            if *s == InfoState::LoadDB {
                *s = InfoState::Idle;
            }
        });
        log::info!("Database loaded");
    }
}

async fn load_doc(read_state: &UseRef<Option<ReaderState>>) {
    log::info!("Loading doc state");

    match ReaderState::from_storage().await {
        Ok(state) => {
            read_state.set(state);
            log::info!("Loaded doc state!");
        }
        Err(e) => log::error!("Failed to load doc state with {e}"),
    }
}

// This shouldn't be an issue since we only mutate the db on creation with load_db
// https://github.com/rust-lang/rust-clippy/issues/6671
#[allow(clippy::await_holding_refcell_ref)]
async fn import_dict(
    db: &UseRef<Option<yomi_dict::IndexedDB>>,
    info_state: &UseRef<InfoState>,
    data: Vec<u8>,
) {
    log::info!("Loading dictionary");

    // TODO Only allow when idle
    // TODO Reset InfoState on failure
    info_state.with_mut(|s| *s = InfoState::LoadDict(info_state::LoadDictState::ParsingDict));
    if db.read().is_none() {
        log::error!("Cannot load dictionary as no database is loaded.");
        return;
    }
    let res = yomi_dict::Dict::new(Cursor::new(&data));

    if let Err(err) = &res {
        log::error!("Failed to read dictionary with error {:?}", err);
        return;
    }
    if let Ok(valid_dict) = res {
        log::info!("Dictionary read. Attempting to save to storage");
        info_state
            .with_mut(|s| *s = InfoState::LoadDict(info_state::LoadDictState::AddingDictIndex));

        let db = db.read();
        let db = db.as_ref().unwrap();

        let steps = match db.add_dict_stepwise(valid_dict).await {
            Err(err) => {
                log::error!("Failed to save dictionary dictionary with error {:?}", err);
                return;
            }
            Ok(steps) => steps,
        };

        let mut progress = 0;
        for step in steps.steps {
            info_state.with_mut(|s| {
                *s = InfoState::LoadDict(info_state::LoadDictState::AddingDictContent(
                    progress,
                    steps.total_count,
                ));
            });
            log::info!("DB progress {progress}/{}", steps.total_count);

            progress += match step.await {
                Ok(p) => p,
                Err(err) => {
                    log::error!("Failed to save dictionary dictionary with error {:?}", err);
                    return;
                }
            };
        }

        info_state.with_mut(|s| *s = InfoState::Idle);
        log::info!("Loaded dictionary");
    }
}

async fn import_doc(data: Vec<u8>, read_state: &UseRef<Option<ReaderState>>) {
    log::info!("Loading document");

    match ReaderState::from_bytes(data).await {
        Ok(doc) => {
            read_state.set(Some(doc));
            log::info!("Loaded document!");
        }
        Err(e) => log::error!("Failed to load document with {e}"),
    }
}

fn app(cx: Scope) -> Element {
    let reasons = use_state(cx, yomi_dict::inflection_reasons);

    let dict_db = use_ref(cx, || None);
    let info_state = use_ref(cx, || InfoState::LoadDB);

    let read_state = use_ref(cx, || None);
    let read_state_tomove = read_state.clone();

    // Cannot use async init for use_ref directly, so load database at next opportunity
    let loading = use_future(cx, (), |()| {
        let db_tomove = dict_db.clone();
        let info_state_tomove = info_state.clone();
        async move {
            load_db(&db_tomove, &info_state_tomove).await;
            load_doc(&read_state_tomove).await;
        }
    });

    let db_tomove = dict_db.clone();
    let read_state_tomove = read_state.clone();
    let info_state_tomove = info_state.clone();

    let tile = read_state
        .with(|s| {
            s.as_ref().map(|s| {
                format!(
                    "Yomi-Reader – {} – Chapter {}/{}",
                    s.get_title(),
                    s.get_page(),
                    s.get_page_count()
                )
            })
        })
        .unwrap_or_else(|| "Yomi-Reader".to_string());

    let body = loading.value().map_or_else(|| rsx!{
        div{
            class: "flex flex-col h-screen pb-4",

            div{
                class: "flex-1 grow max-h-full overflow-y-hidden",

                "Loading ..."
            }
        }}, |_|
        rsx!{
            div{
                class: "flex flex-col h-screen pb-4",
    
                header{
                    class: "flex-none",
    
                    ul{
                        class: "flex",
    
                        li{
                            class: "mx-auto",
    
                            upload_component::upload_component{
                                id: "dict_id",
                                label: "Upload Dict",
                                upload_callback: move |data|{
                                    let db_tomove = db_tomove.clone();
                                    let info_state_tomove = info_state_tomove.clone();
                                    wasm_bindgen_futures::spawn_local(async move{
                                        import_dict(&db_tomove, &info_state_tomove, data).await;
                                    });
                                }
                            }
                        },
    
                        li{
                            class: "mx-auto",
    
                            upload_component::upload_component{
                                id: "book_id",
                                label: "Upload book",
                                upload_callback: move |data| {
                                    let read_state_tomove = read_state_tomove.clone();
                                    wasm_bindgen_futures::spawn_local(async move{
                                        import_doc(data, &read_state_tomove).await;
                                    });
                                }
                            }
                        }
                    }
                }
                div{
                    class: "flex-1 grow max-h-full overflow-y-hidden",
    
                    onscroll: |_| log::info!("scroll"),
    
                    crate::reader::reader_component{ read_state: read_state, db: dict_db, reasons: reasons, info_state: info_state }
                }
            }
        }
    );

    cx.render(rsx! {
        title{
            "{tile}"
        }
        body
    })
}
