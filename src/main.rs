mod definitions;
mod nav;
mod read_state;
mod reader;
mod upload_component;
mod view;

extern crate web_sys;

use std::io::Cursor;

use dioxus::prelude::*;
use epub::doc::EpubDoc;
use read_state::ReaderState;
use wasm_bindgen::{prelude::Closure, JsCast};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}

async fn load_db(db: &UseRef<Option<yomi_dict::db::DB>>) {
    if db.read().is_none() {
        db.write()
            .replace(yomi_dict::db::DB::new("data").await.unwrap());
        log::info!("Database loaded");
    }
}

async fn load_dict(dbc: &UseRef<Option<yomi_dict::db::DB>>, data: Vec<u8>) {
    log::info!("Loading dictionary");

    if dbc.read().is_none() {
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

        let res = dbc.write().as_mut().unwrap().add_dict(valid_dict).await;

        if let Err(err) = res {
            log::error!("Failed to save dictionary dictionary with error {:?}", err);
            return;
        }
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

        read_state.set(Some(ReaderState::new(doc, 0, 0.0)));

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

async fn manage_scroll(read_state: UseRef<Option<ReaderState>>) {
    let window = web_sys::window().expect("should have window");
    let document = window.document().expect("should have document");

    if let Some(read_state) = read_state.read().as_ref() {
        read_state.apply_scroll();
    }

    let scroll_callback = Closure::<dyn Fn()>::new(move || {
        if let Some(offset) = window.page_y_offset().ok() {
            read_state
                .write()
                .as_mut()
                .map(|state| state.set_scroll(offset));
        } else {
            log::error!("Couldn't get document offset to get scroll position");
        }
    });
    document.set_onscroll(Some(scroll_callback.as_ref().unchecked_ref()));

    scroll_callback.forget();
}

fn app(cx: Scope) -> Element {
    let reasons = use_state(&cx, || yomi_dict::deinflect::inflection_reasons());

    let db = use_ref(&cx, || None);

    // Cannot use async init for use_ref directly, so load database at next opportunity
    use_future(&cx, (), |()| {
        let db_tomove = db.clone();
        async move {
            load_db(&db_tomove).await;
        }
    });

    let read_state = use_ref(&cx, load_stored_reader_state);

    // Set scroll after everything is rendered
    use_future(&cx, (), |()| {
        let read_state = read_state.clone();

        async move {
            manage_scroll(read_state).await;
        }
    });

    let db_tomove = db.clone();
    let read_state_tomove = read_state.clone();

    cx.render(rsx! {
        upload_component::upload_component{
            id: "dict_id",
            label: "Upload Dict",
            upload_callback: move |data|{
                let db_tomove = db_tomove.clone();
                wasm_bindgen_futures::spawn_local(async move{load_dict(&db_tomove, data).await;});
            }
        }
        upload_component::upload_component{
            id: "book_id",
            label: "Upload book",
            upload_callback: move |data| {
                load_doc(data, read_state_tomove.clone());
            }
        }
        crate::reader::reader_component{ read_state: read_state, db: db, reasons: reasons }
    })
}
