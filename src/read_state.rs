extern crate web_sys;

use std::io::Cursor;

use epub::doc::EpubDoc;

pub(crate) struct ReaderState {
    doc: EpubDoc<Cursor<Vec<u8>>>,
    page: usize,
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

impl ReaderState {
    pub(crate) fn new(doc: EpubDoc<Cursor<Vec<u8>>>, page: usize) -> ReaderState {
        ReaderState { doc, page }
    }

    pub(crate) fn get_text(&mut self) -> Option<String> {
        self.doc.get_current_str().ok() // TODO Look up errors
    }

    pub(crate) fn get_page(&self) -> usize {
        self.page
    }

    pub(crate) fn get_page_count(&self) -> usize {
        self.doc.get_num_pages()
    }

    pub(crate) fn next_page(&mut self) {
        if let Ok(()) = self.doc.go_next() {
            self.page = self.doc.get_current_page();
            save_page(self.page);
        }
    }

    pub(crate) fn prev_page(&mut self) {
        if let Ok(()) = self.doc.go_prev() {
            self.page = self.doc.get_current_page();
            save_page(self.page);
        }
    }
}
