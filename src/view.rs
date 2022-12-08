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
    // TODO Breaks on double click
    let sel = web_sys::window().unwrap().get_selection().unwrap().unwrap();
    let n = sel.anchor_node().unwrap();
    let s: String = n
        .text_content()
        .unwrap()
        .chars()
        .skip(sel.anchor_offset().try_into().unwrap())
        .take(16)
        .collect();

    log::info!("Clicked: {}", s);

    onselect.call(s);
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
        .write()
        .as_mut()
        .and_then(|state| state.get_text());

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
