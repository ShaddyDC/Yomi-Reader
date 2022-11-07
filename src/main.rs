extern crate web_sys;

macro_rules! log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into())
        }
    }

use std::io::{Cursor, Read, Seek};

use dioxus::prelude::*;
use epub::doc::EpubDoc;
use yomi_dict::{deinflect::Reasons, translator::get_terms, Dict};

fn main() {
    let dict = include_bytes!("jmdict_english.zip");
    let dict = yomi_dict::read(std::io::Cursor::new(dict)).expect("Dictionary should be readable");
    let reasons = yomi_dict::deinflect::inflection_reasons();

    dioxus::web::launch_with_props(app, RootProps { dict, reasons }, |config| config);
}

#[derive(Props)]
struct NavProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<EpubDoc<R>>,
}

fn nav_component<'a, R: Read + Seek + 'a>(cx: Scope<'a, NavProps<'a, R>>) -> Element<'a> {
    let doc = cx.props.doc;

    let page = use_state(&cx, || doc.read().get_current_page());

    let count = doc.read().get_num_pages();

    cx.render(rsx! {
        div { "Page {page}/{count}" }
        button {
            onclick: move |_| {
               std::mem::drop(doc.write().go_prev());
               page.set(doc.read().get_current_page());
           },
            "Previous"
         }
         button {
             onclick: move |_| {
                std::mem::drop(doc.write().go_next());
                page.set(doc.read().get_current_page());
            },
             "Next"
          }
    })
}

#[derive(Props)]
struct TextProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<EpubDoc<R>>,
    onselect: EventHandler<'a, String>,
}

fn clicked(onselect: &EventHandler<String>) {
    let sel = web_sys::window().unwrap().get_selection().unwrap().unwrap();
    let n = sel.anchor_node().unwrap();
    let s: String = n
        .text_content()
        .unwrap()
        .chars()
        .skip(sel.anchor_offset().try_into().unwrap())
        .take(16)
        .collect();

    log!("Clicked: {}", s);

    onselect.call(s);
}

fn text_component<'a, R: Read + Seek + 'a>(cx: Scope<'a, TextProps<'a, R>>) -> Element<'a> {
    let doc = cx.props.doc;
    let onselect = &cx.props.onselect;

    let text = doc
        .write()
        .get_current_str()
        .unwrap_or_else(|_| "".to_string());

    cx.render(rsx! {
        div {
            // TODO: Properly sandbox / iframe
            dangerous_inner_html: "{text}",
            onclick: |_| clicked(onselect)
        }
    })
}

#[derive(Props)]
struct ReaderProps<'a, R: Read + Seek + 'a> {
    doc: &'a UseRef<EpubDoc<R>>,
    dict: &'a Dict,
    reasons: &'a Reasons,
}

struct Expression {
    expression: String,
    reading: String,
    entries: Vec<DictEntry>,
}

struct DictEntry {
    definitions: Vec<String>,
}

fn lookup<'a>(dict: &'a Dict, reasons: &'a Reasons, s: &str) -> Vec<Expression> {
    let definitions = get_terms(s, reasons, dict);

    definitions
        .iter()
        .map(|d| Expression {
            expression: d.expression.to_owned(),
            reading: d.reading.to_owned(),
            entries: {
                d.entries
                    .iter()
                    .map(|e| DictEntry {
                        definitions: e
                            .term
                            .glossary
                            .iter()
                            .map(std::borrow::ToOwned::to_owned)
                            .collect::<Vec<_>>(),
                    })
                    .collect::<Vec<_>>()
            },
        })
        .collect::<Vec<_>>()
}

fn reader_component<'a, 'b, R: Read + Seek + 'a>(cx: Scope<'a, ReaderProps<'a, R>>) -> Element<'a> {
    let doc = cx.props.doc;
    let dict = cx.props.dict;
    let reasons = cx.props.reasons;

    let defs = use_state(&cx, Vec::new);

    cx.render(rsx! {
        crate::nav_component{ doc: doc }
        crate::text_component{
            doc: doc,
            onselect: move |evt: String| defs.set(lookup(dict, reasons, &evt))
        }
        crate::definitions_component{ definitions: defs.get() }
        crate::nav_component{ doc: doc }
    })
}

#[inline_props]
fn definitions_component<'a>(cx: Scope, definitions: &'a Vec<Expression>) -> Element {
    cx.render(rsx!(ul{
        definitions.iter().map(|d| rsx!(
            li{
                div{
                    ruby {
                        p { "{d.expression}" }
                        rt{ "{d.reading}" }
                    }
                }
                div{
                    ol{
                        d.entries.iter().map(|e| rsx!(
                            li{
                                ul{
                                    e.definitions.iter().map(|s| rsx!(
                                        p{
                                            "{s}"
                                        }
                                    ))
                                }
                            }
                        ))
                    }
                }
            }
        ))
    }))
}

#[derive(Props)]
struct RootProps {
    dict: Dict,
    reasons: Reasons,
}

// TODO: This is obviously bad
impl PartialEq for RootProps {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

fn app(cx: Scope<RootProps>) -> Element {
    let dict = &cx.props.dict;
    let reasons = &cx.props.reasons;

    let file = include_bytes!("test.epub"); // TODO Prevent reloads
    let doc = EpubDoc::from_reader(Cursor::new(file)).unwrap();

    let doc = use_ref(&cx, || doc);

    cx.render(rsx! {
        crate::reader_component{ doc: doc, dict: dict, reasons: reasons }
    })
}
