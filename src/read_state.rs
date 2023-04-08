extern crate web_sys;

use epub::doc::EpubDoc;
use rexie::Rexie;
use std::{
    io::Cursor,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub struct ReaderState {
    doc: EpubDoc<Cursor<Vec<u8>>>,
    page: usize,
    scroll_top: i32,
    text: Option<String>,
}

// TODO error checking
fn save_page(page: usize) {
    let window = web_sys::window().expect("should have window");
    let storage = window
        .local_storage()
        .expect("should be able to get storage")
        .expect("should have storage");
    storage.set_item("page", &page.to_string()).ok();
}

fn save_scroll(scroll_top: i32) {
    let window = web_sys::window().expect("should have window");
    let storage = window
        .local_storage()
        .expect("should be able to get storage")
        .expect("should have storage");
    storage.set_item("scroll_top", &scroll_top.to_string()).ok();
}

fn apply_scroll(scroll_top: i32) {
    let window = web_sys::window().expect("should have window");
    let document = window.document().expect("should have document");

    // TODO error handling
    document.get_element_by_id("reader-scroll").map_or_else(
        || {
            log::warn!("Couldn't get element to set scroll position");
        },
        |element| {
            element.set_scroll_top(scroll_top);
        },
    );
}

async fn get_doc_db() -> rexie::Result<Rexie> {
    Rexie::builder("EpubDatabase")
        .version(1)
        .add_object_store(
            rexie::ObjectStore::new("books")
                .key_path("id")
                .auto_increment(true),
        )
        .build()
        .await
}

impl ReaderState {
    fn new(mut doc: EpubDoc<Cursor<Vec<u8>>>, page: usize, scroll_top: i32) -> Self {
        let text = doc.get_current_str().map(|(s, _)| s);

        save_page(page);
        save_scroll(scroll_top);
        apply_scroll(scroll_top);

        Self {
            doc,
            page,
            scroll_top,
            text,
        }
    }

    pub(crate) fn get_title(&self) -> String {
        self.doc
            .mdata("title")
            .unwrap_or_else(|| "<Document has no title>".to_string())
    }

    pub(crate) fn get_text(&self) -> Option<String> {
        self.text.clone()
    }

    pub(crate) fn get_current_path(&self) -> Option<PathBuf> {
        self.doc.get_current_path()
    }

    pub(crate) fn get_resource_by_path(&mut self, path: &Path) -> Option<Vec<u8>> {
        self.doc.get_resource_by_path(path)
    }

    pub(crate) const fn get_page(&self) -> usize {
        self.page
    }

    pub(crate) fn get_page_count(&self) -> usize {
        self.doc.get_num_pages()
    }

    pub(crate) fn next_page(&mut self) {
        if self.doc.go_next() {
            self.page = self.doc.get_current_page();
            self.text = self.doc.get_current_str().map(|(s, _)| s);
            save_page(self.page);
            self.set_scroll(0);
            self.apply_scroll();
        }
    }

    pub(crate) fn prev_page(&mut self) {
        if self.doc.go_prev() {
            self.page = self.doc.get_current_page();
            self.text = self.doc.get_current_str().map(|(s, _)| s);
            save_page(self.page);
            self.set_scroll(0);
            self.apply_scroll();
        }
    }

    pub(crate) async fn from_storage() -> Result<Option<Self>, ReadStateError> {
        let db = get_doc_db().await?;

        let transaction = db.transaction(&["books"], rexie::TransactionMode::ReadOnly)?;
        let books = transaction.store("books")?;

        let Some(data) = books
        .get_all(None, Some(1), None, None)
        .await
        .unwrap()
        .first()
        .map(|(_, v)| v.clone()) else{
            return Ok(None);
        };

        let data: Vec<u8> = serde_wasm_bindgen::from_value(data)?;

        let mut doc = EpubDoc::from_reader(Cursor::new(data))?;

        let window = web_sys::window().expect("should have window");
        let storage = window
            .local_storage()
            .expect("should be able to get storage")
            .expect("should have storage");

        let page = storage
            .get_item("page")
            .expect("Should be able to access storage")
            .map(|s| s.parse().map_err(|_| ReadStateError::ParseError(s)))
            .unwrap_or(Ok(0))?;

        let scroll_top = storage
            .get_item("scroll_top")
            .expect("Should be able to access storage")
            .map(|s| s.parse().map_err(|_| ReadStateError::ParseError(s)))
            .unwrap_or(Ok(0))?;

        doc.set_current_page(page);

        Ok(Some(ReaderState::new(doc, page, scroll_top)))
    }

    pub(crate) async fn from_bytes(data: Vec<u8>) -> Result<Self, ReadStateError> {
        let doc = EpubDoc::from_reader(Cursor::new(data.clone()))?;

        log::info!("document read. Attempting to save to storage");

        let db = get_doc_db().await.expect("should be able to get doc db");
        let transaction = db.transaction(&["books"], rexie::TransactionMode::ReadWrite)?;
        let books = transaction.store("books")?;

        let epub = serde_wasm_bindgen::to_value(&data)?;

        books.clear().await?;
        books.add(&epub, None).await?;

        transaction.done().await?;

        Ok(ReaderState::new(doc, 0, 0))
    }

    // pub(crate) fn get_scroll(&self) -> i32 {
    //     self.scroll_top
    // }

    pub(crate) fn set_scroll(&mut self, scroll_top: i32) {
        self.scroll_top = scroll_top;
        save_scroll(scroll_top);
    }

    pub(crate) fn apply_scroll(&self) {
        apply_scroll(self.scroll_top)
    }
}

#[derive(Error, Debug)]
pub enum ReadStateError {
    #[error("An error occured with the rexie IndexedDB backend: `{0}`")]
    RexieError(#[from] rexie::Error),
    #[error("An error occured parsing `{0}`")]
    ParseError(String),
    #[error("Error parsing JSObject: `{0}`")]
    JsobjError(#[from] serde_wasm_bindgen::Error),
    #[error("Error parsing EPUB: `{0}`")]
    EpubError(#[from] epub::doc::DocError),
}
