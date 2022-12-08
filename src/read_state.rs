extern crate web_sys;

use std::io::Cursor;

use epub::doc::EpubDoc;

pub(crate) struct ReaderState {
    doc: EpubDoc<Cursor<Vec<u8>>>,
    page: usize,
    scroll_top: i32,
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
    pub(crate) fn new(doc: EpubDoc<Cursor<Vec<u8>>>, page: usize, scroll_top: i32) -> ReaderState {
        ReaderState {
            doc,
            page,
            scroll_top,
        }
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
            self.set_scroll(0);
            self.apply_scroll();
        }
    }

    pub(crate) fn prev_page(&mut self) {
        if let Ok(()) = self.doc.go_prev() {
            self.page = self.doc.get_current_page();
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
        if let Some(element) = document.get_element_by_id("reader-scroll") {
            element.set_scroll_top(self.scroll_top);
            let x = self.scroll_top;
            log::info!("Set scroll {x}");

            let x = &element.inner_html()[..1000];
            log::info!("E {x:?}");
        } else {
            log::warn!("Couldn't get element to set scroll position");
        }
    }
}
