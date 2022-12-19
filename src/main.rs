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
use epub::doc::EpubDoc;
use info_state::InfoState;
use read_state::ReaderState;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

async fn load_db(db: &UseRef<Option<yomi_dict::db::DB>>, info_state: &UseRef<InfoState>) {
    if db.read().is_none() {
        // TODO get rid of all unwraps
        let new_db = yomi_dict::db::DB::new("data").await.unwrap();
        db.with_mut(|db| db.replace(new_db));
        info_state.with_mut(|s| {
            if *s == InfoState::LoadDB {
                *s = InfoState::Idle
            }
        });
        log::info!("Database loaded");
    }
}

// This shouldn't be an issue since we only mutate the db on creation with load_db
// https://github.com/rust-lang/rust-clippy/issues/6671
#[allow(clippy::await_holding_refcell_ref)]
async fn load_dict(
    db: &UseRef<Option<yomi_dict::db::DB>>,
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
                ))
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

fn load_doc(data: Vec<u8>, read_state: UseRef<Option<ReaderState>>) {
    log::info!("Loading document");

    let res = EpubDoc::from_reader(Cursor::new(data.clone()));

    if let Err(err) = &res {
        log::error!("Failed to read document with error {:?}", err);
        return;
    }
    if let Ok(doc) = res {
        log::info!("document read. Attempting to save to storage");

        let data = base64::encode(data);
        let window = web_sys::window().expect("should have window");
        let storage = window
            .local_storage()
            .expect("should be able to get storage")
            .expect("should have storage");
        storage.set_item("doc", &data).ok();
        storage.set_item("page", "0").ok();
        storage.set_item("scroll_top", "0").ok();

        read_state.set(Some(ReaderState::new(doc, 0, 0)));

        log::info!("Loaded document");
    }
}

fn load_stored_reader_state() -> Option<ReaderState> {
    let window = web_sys::window().expect("should have window");
    let storage = window
        .local_storage()
        .expect("should be able to get storage")
        .expect("should have storage");
    let doc_string = storage
        .get_item("doc")
        .expect("should be able to access storage")?;
    let doc_bytes = base64::decode(doc_string).ok()?;

    let mut doc = EpubDoc::from_reader(Cursor::new(doc_bytes)).ok()?;

    let page_string = storage
        .get_item("page")
        .expect("Should be able to access storage")?;
    let page = page_string.parse().ok()?;

    let scroll_string = storage
        .get_item("scroll_top")
        .expect("Should be able to access storage")?;
    let scroll_top = scroll_string.parse().ok()?;

    doc.set_current_page(page).ok()?;

    Some(ReaderState::new(doc, page, scroll_top))
}

fn app(cx: Scope) -> Element {
    let reasons = use_state(&cx, yomi_dict::deinflect::inflection_reasons);

    let db = use_ref(&cx, || None);
    let info_state = use_ref(&cx, || InfoState::LoadDB);

    // Cannot use async init for use_ref directly, so load database at next opportunity
    use_future(&cx, (), |()| {
        let db_tomove = db.clone();
        let info_state_tomove = info_state.clone();
        async move {
            load_db(&db_tomove, &info_state_tomove).await;
        }
    });

    let read_state = use_ref(&cx, load_stored_reader_state);

    let db_tomove = db.clone();
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

    cx.render(rsx! {
        title{
            "{tile}"
        }
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
                                wasm_bindgen_futures::spawn_local(async move{load_dict(&db_tomove, &info_state_tomove, data).await;});
                            }
                        }
                    },

                    li{
                        class: "mx-auto",

                        upload_component::upload_component{
                            id: "book_id",
                            label: "Upload book",
                            upload_callback: move |data| {
                                load_doc(data, read_state_tomove.clone());
                            }
                        }
                    }
                }
            }
            div{
                class: "flex-1 grow max-h-full overflow-y-hidden",

                onscroll: |_| log::info!("scroll"),

                crate::reader::reader_component{ read_state: read_state, db: db, reasons: reasons, info_state: info_state }
            }
        }
    })
}
