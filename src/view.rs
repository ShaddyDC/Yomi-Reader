extern crate web_sys;

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use dioxus::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::read_state::ReaderState;

#[derive(Props)]
pub struct ViewProps<'a> {
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

fn vec_to_blob(vec: &Vec<u8>) -> Option<web_sys::Blob> {
    let array = js_sys::Array::new();
    array.push(&js_sys::Uint8Array::from(vec.as_slice()));
    web_sys::Blob::new_with_u8_array_sequence(&array).ok()
}

fn load_to_cache(
    uncached_resources: impl IntoIterator<Item = String>,
    resource_cache: &UseRef<HashMap<String, Option<String>>>,
    read_state: &UseRef<Option<ReaderState>>,
) {
    read_state.with_mut(|state| {
        let Some(state) = state.as_mut() else { return; };

        for resource in uncached_resources {
            let file = Path::new(resource.as_str());

            let url = state
                .get_resource_by_path(file)
                .and_then(|r| vec_to_blob(&r))
                .and_then(|blob| web_sys::Url::create_object_url_with_blob(&blob).ok());

            log::info!("Resource: {resource}: {url:?}");

            resource_cache.with_mut(|cache| {
                cache.insert(resource.to_string(), url);
            });
        }
    });
}

fn process_text(
    current_path: &Path,
    text: &str,
    resource_cache: &UseRef<HashMap<String, Option<String>>>,
    read_state: &UseRef<Option<ReaderState>>,
) -> String {
    // We want to only load the body of the resulting html so it can properly be loaded

    let body_regex = regex::Regex::new(r"<body[^>]*>([\s\S]*)</body>").unwrap();

    let body = body_regex
        .captures(text)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .unwrap_or_else(|| {
            log::info!("No body tag found, using full text {text}");
            text
        });

    // We will replace all relative links to resources to links we generate

    let resource_regex = regex::Regex::new(r#"=\w*"([^"]*)""#).unwrap();
    let all_resources = resource_regex
        .captures_iter(body)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str()))
        .collect::<Vec<_>>();

    let resource_to_file = |resource: &str| {
        let mut path = current_path.parent().unwrap_or_else(|| Path::new("."));
        let mut resource = resource;
        loop {
            // PathBuf::canonicalize would be better but isn't supported on this platform
            let Some(rest) = resource.strip_prefix("../") else { break };

            resource = rest;
            path = path.parent().unwrap_or_else(|| Path::new("."));
        }

        let path = path.join(resource);

        std::string::String::from(path.to_string_lossy())
    };

    let uncached_resources = resource_cache.with(|cache| {
        all_resources
            .iter()
            .map(|s| resource_to_file(s))
            .filter(|r| !cache.contains_key(r.as_str()))
            .collect::<HashSet<_>>()
    });

    if !uncached_resources.is_empty() {
        log::info!(
            "Loading {} potential new resources!",
            uncached_resources.len()
        );
        load_to_cache(uncached_resources, resource_cache, read_state);
    }

    resource_cache.with(|links| {
        let mut body = body.to_owned();

        for resource in all_resources {
            let name = resource_to_file(resource);
            if let Some(url) = links.get(&name).unwrap_or(&None).clone() {
                log::info!("Replacing {resource} with {url}");
                body = body.replace(resource, &url);
            }
        }

        body
    })
}

fn apply_current_scroll(read_state: UseRef<Option<ReaderState>>) {
    let window = web_sys::window().expect("should have window");

    // If we apply the scroll position immediately, it will be reset to 0
    // Instead, we push it to the end of the current event queue

    let callback = Closure::<dyn Fn()>::new(move || {
        if let Some(read_state) = read_state.read().as_ref() {
            log::info!("Calling apply");
            read_state.apply_scroll();
        }
    });

    window
        .set_timeout_with_callback(callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}

pub fn view_component<'a>(cx: Scope<'a, ViewProps<'a>>) -> Element<'a> {
    let text = cx.props.read_state.with(|state| {
        state
            .as_ref()
            .and_then(crate::read_state::ReaderState::get_text)
    });

    let known_text = use_ref(cx, || text.clone());
    let processed_text = use_ref(cx, || None);
    let apply_scroll_block = use_state(cx, || false);

    let resource_cache = use_ref(cx, HashMap::<String, Option<String>>::new);

    let onselect = &cx.props.onselect;
    let read_state = cx.props.read_state;

    // Set scroll after everything is rendered
    use_future(cx, (), |()| {
        let read_state = read_state.clone();

        async move {
            apply_current_scroll(read_state);
        }
    });

    text.map_or_else(
        || cx.render(rsx! {p{"No document"}}),
        |text| {
            let path = read_state.with(|state| {
                state
                    .as_ref()
                    .and_then(crate::read_state::ReaderState::get_current_path)
            });

            // Because scroll application triggers the scroll callback
            // Potential images will only load after the processing below has passed and rendered once
            // Therefore, we need to update the scroll position afterwards,
            // and we need to avoid the callback saving the wrong position.
            // Therefore, we block the callback for both renders until things have settled.
            if *apply_scroll_block.get() {
                apply_scroll_block.set(false);
                let read_state = read_state.clone();
                let window = web_sys::window().expect("should have window");
                let callback = Closure::<dyn Fn()>::new(move || {
                    read_state.with_mut(|state| state.as_mut().map(|s| s.set_scoll_blocked(true)));
                });

                window
                    .set_timeout_with_callback(callback.as_ref().unchecked_ref())
                    .unwrap();
                callback.forget();
            }

            if processed_text.read().is_none() || known_text.read().as_ref() != Some(&text) {
                if let Some(path) = path {
                    let body = process_text(&path, &text, resource_cache, read_state);

                    known_text.set(Some(text));
                    processed_text.set(Some(body));
                    read_state.with_mut(|state| state.as_mut().map(|s| s.set_scoll_blocked(true)));
                    apply_scroll_block.set(true);
                }
            }

            let body = processed_text.read().as_ref().unwrap().clone();

            cx.render(rsx! {
                div {
                    // TODO: Properly sandbox / iframe
                    dangerous_inner_html: "{body}",
                    onclick: |_| clicked(onselect)
                }
            })

            // cx.render(rsx! {
            //     iframe {
            //         // TODO: Properly sandbox / iframe
            //         srcdoc: "{body}",
            //         onclick: |_| clicked(onselect)
            //     }
            // })
        },
    )
}
