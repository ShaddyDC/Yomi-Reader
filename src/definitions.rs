use dioxus::prelude::*;
use yomi_dict::{deinflect::Reasons, translator::DictEntries};

// This shouldn't be an issue since we only mutate the db on creation with load_db
// https://github.com/rust-lang/rust-clippy/issues/6671
#[allow(clippy::await_holding_refcell_ref)]
async fn get_terms(
    text: &str,
    reasons: &Reasons,
    db: &UseRef<Option<yomi_dict::db::DB>>,
) -> Result<Vec<DictEntries>, yomi_dict::YomiDictError> {
    let db_ref = db.read();
    let db = db_ref.as_ref().unwrap();
    yomi_dict::translator::get_terms(text, reasons, db).await
}

pub(crate) async fn update_defs_and_selection(
    defs: &UseState<Vec<DictEntries>>,
    db: &UseRef<Option<yomi_dict::db::DB>>,
    reasons: &Reasons,
    data: &str,
) {
    if db.read().is_none() {
        // TODO reset selection
        log::error!("Cannot update definitions since DB is not loaded yet!");
        return;
    }

    let entries = match get_terms(data, reasons, db).await {
        Ok(entries) => entries,
        Err(e) => {
            log::error!("Cannot get definitions due to error {}", e);
            return;
        }
    };

    let window = web_sys::window().expect("should have window");
    let selection = window.get_selection().expect("Should have selection");
    if let Some(selection) = selection {
        // TODO ensure we're only modifying our own selection
        selection.collapse_to_start().unwrap();
        for _ in 0..entries
            .first()
            .map_or(0, |entry| entry.entries[0].source_len)
        {
            selection.modify("extend", "forward", "character").unwrap();
        }
    }

    defs.set(entries)
}

#[inline_props]
pub(crate) fn definitions_component<'a>(cx: Scope, definitions: &'a Vec<DictEntries>) -> Element {
    let content = if definitions.is_empty() {
        rsx!(p{"Click the first letter of an expression to look it up!"})
    } else {
        rsx!(ul{
            class: "list-none",

            definitions.iter().map(|d| rsx!(
                li{
                    key: "{d.expression}/{d.reading}",
                    h2{
                        class: "text-lg font-medium",

                        ruby {
                            p { "{d.expression}" }
                            rt{ "{d.reading}" }
                        }
                    }
                    div{
                        ol{
                            class: "list-decimal px-4",

                            d.entries.iter().flat_map(|e| {
                                // let key = ((e.term.dict_id as u64) << 32) | e.term.sequence as u64;

                                e.term.glossary.iter().map(|s| {
                                    rsx!(
                                        li{
                                            key: "{s}", // TODO inefficient

                                            span{
                                                class: "whitespace-pre-wrap",

                                                "{s}"
                                            }
                                        }
                                    )
                                })
                            })
                        }
                    }
                }
            ))
        })
    };

    cx.render(rsx!(aside {
        class: "container mx-auto mt-2",

        content
    }))
}
