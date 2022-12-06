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
        crate::reader::reader_component{ doc: doc, db: db, reasons: reasons }
    })
}
