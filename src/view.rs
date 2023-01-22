extern crate web_sys;

use dioxus::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::read_state::ReaderState;

#[derive(Props)]
pub(crate) struct ViewProps<'a> {
    read_state: &'a UseRef<Option<ReaderState>>,
    onselect: EventHandler<'a, String>,
}

fn clicked(onselect: &EventHandler<String>) {
    const SELECTION_LENGTH: usize = 16;

    let window = web_sys::window().expect("should have window");

    let selection = window.get_selection().expect("Should have selection");

    // Selection in eg iframe or otherwise inaccessible
    let Some(selection) = selection else { return };

    // We want to allow user range selection
    if selection.type_() != "Caret" {
        return;
    }

    if selection.modify("extend", "forward", "sentence").is_err() {
        for _ in 0..SELECTION_LENGTH {
            if selection.modify("extend", "forward", "character").is_err() {
                break;
            }
        }
    }

    if let Some(sentence_end) = selection.to_string().as_string() {
        let s = sentence_end.chars().take(SELECTION_LENGTH).collect();
        log::info!("Clicked: {}", s);

        onselect.call(s);
    }
}

async fn apply_current_scroll(read_state: UseRef<Option<ReaderState>>) {
    let window = web_sys::window().expect("should have window");

    // If we apply the scroll position immediately, it will be reset to 0
    // Instead, we push it to the end of the current event queue

    let callback = Closure::<dyn Fn()>::new(move || {
        if let Some(read_state) = read_state.read().as_ref() {
            read_state.apply_scroll();
        }
    });

    window
        .set_timeout_with_callback(callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}

pub(crate) fn view_component<'a>(cx: Scope<'a, ViewProps<'a>>) -> Element<'a> {
    let text = cx
        .props
        .read_state
        .with(|state| state.as_ref().and_then(|state| state.get_text()));

    let onselect = &cx.props.onselect;
    let read_state = cx.props.read_state;

    // Set scroll after everything is rendered
    use_future(&cx, (), |()| {
        let read_state = read_state.clone();

        async move {
            apply_current_scroll(read_state).await;
        }
    });

    if let Some(text) = text {
        cx.render(rsx! {
            main {
                // TODO: Properly sandbox / iframe
                dangerous_inner_html: "{text}",
                onclick: |_| clicked(onselect)
            }
        })
    } else {
        cx.render(rsx! {p{"No document"}})
    }
}
