extern crate web_sys;

use std::io::Cursor;

use epub::doc::EpubDoc;

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

fn save_scroll(page: i32) {
    let window = web_sys::window().expect("should have window");
    let storage = window
        .local_storage()
        .expect("should be able to get storage")
        .expect("should have storage");
    storage.set_item("scroll_top", &page.to_string()).ok();
}

impl ReaderState {
    pub(crate) fn new(mut doc: EpubDoc<Cursor<Vec<u8>>>, page: usize, scroll_top: i32) -> Self {
        let text = doc.get_current_str().ok(); // TODO Look up errors
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

    pub(crate) const fn get_page(&self) -> usize {
        self.page
    }

    pub(crate) fn get_page_count(&self) -> usize {
        self.doc.get_num_pages()
    }

    pub(crate) fn next_page(&mut self) {
        if let Ok(()) = self.doc.go_next() {
            self.page = self.doc.get_current_page();
            self.text = self.doc.get_current_str().ok();
            save_page(self.page);
            self.set_scroll(0);
            self.apply_scroll();
        }
    }

    pub(crate) fn prev_page(&mut self) {
        if let Ok(()) = self.doc.go_prev() {
            self.page = self.doc.get_current_page();
            self.text = self.doc.get_current_str().ok();
            save_page(self.page);
            self.set_scroll(0);
            self.apply_scroll();
        }
    }

    // pub(crate) fn get_scroll(&self) -> i32 {
    //     self.scroll_top
    // }

    pub(crate) fn set_scroll(&mut self, scroll_top: i32) {
        self.scroll_top = scroll_top;
        save_scroll(scroll_top);
    }

    pub(crate) fn apply_scroll(&self) {
        let window = web_sys::window().expect("should have window");
        let document = window.document().expect("should have document");

        // TODO error handling
        document.get_element_by_id("reader-scroll").map_or_else(
            || {
                log::warn!("Couldn't get element to set scroll position");
            },
            |element| {
                element.set_scroll_top(self.scroll_top);
            },
        );
    }
}
