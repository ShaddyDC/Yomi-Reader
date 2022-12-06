mod nav;
mod reader;
mod upload_component;
mod view;

extern crate web_sys;

use std::io::Cursor;

use dioxus::prelude::*;
use epub::doc::EpubDoc;

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

fn load_book(data: Vec<u8>, doc: UseRef<Option<EpubDoc<Cursor<Vec<u8>>>>>) {
    log::info!("Loading book");

    let res = EpubDoc::from_reader(Cursor::new(data.clone()));

    if let Err(err) = &res {
        log::error!("Failed to read book with error {:?}", err);
        return;
    }
    if let Ok(book) = res {
        log::info!("Book read. Attempting to save to storage");

        let data = base64::encode(data);
        doc.set(Some(book));
        let window = web_sys::window().expect("should have window");
        let storage = window
            .local_storage()
            .expect("should be able to get storage")
            .expect("should have storage");
        storage.set_item("doc", &data).ok();

        log::info!("Loaded book");
    }
}

fn load_stored_epub() -> Option<EpubDoc<Cursor<Vec<u8>>>> {
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

    let doc = use_ref(&cx, load_stored_epub);

    let db_tomove = db.clone();
    let doc_tomove = doc.clone();

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
                load_book(data, doc_tomove.clone());
            }
        }
        crate::reader::reader_component{ doc: doc, db: db, reasons: reasons }
    })
}
