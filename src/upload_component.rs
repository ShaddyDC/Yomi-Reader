use dioxus::prelude::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Props)]
pub struct UploadProps<'a, F: FnMut(Vec<u8>) + Clone> {
    label: &'a str,
    id: &'a str,
    upload_callback: F,
}

pub fn upload_component<'a, F: FnMut(Vec<u8>) + Clone + 'static>(
    cx: Scope<'a, UploadProps<'a, F>>,
) -> Element<'a> {
    let label = cx.props.label;
    let id = cx.props.id;

    cx.render(rsx! {
        label {
            r#for: "{id}",
            "{label}"
        }
        input{
            r#type: "file",
            id: "{id}",
            name: "{id}",
            onchange: move|_| {
                let id = id.to_owned();
                let mut onupload = cx.props.upload_callback.clone();
                cx.spawn(async move{
                    let window = web_sys::window()
                        .expect("should have window")
                        .document()
                        .expect("should have a document.");
                    let element = window.get_element_by_id(&id)
                        .expect("element with id should exist")
                        .dyn_into::<HtmlInputElement>()
                        .expect("element should have correct type");
                    let files = element.files().expect("element should have files");
                    if let Some(file) = files.get(0){
                        let buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
                            .await
                            .expect("file should be loadable");
                        let array = Uint8Array::new(&buffer).to_vec();
                        onupload(array);
                    }
                });
            }
        }
    })
}
