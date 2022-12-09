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

// TODO broken ン
pub(crate) async fn set_defs(
    defs: &UseState<Vec<DictEntries>>,
    db: &UseRef<Option<yomi_dict::db::DB>>,
    reasons: &Reasons,
    data: &str,
) {
    if db.read().is_none() {
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

    defs.set(entries)
}

#[inline_props]
pub(crate) fn definitions_component<'a>(cx: Scope, definitions: &'a Vec<DictEntries>) -> Element {
    cx.render(rsx!(
    aside{
        class: "container mx-auto",

        ul{
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

                                e.term.glossary.iter().map(|s| rsx!(
                                    li{
                                        span{
                                            class: "whitespace-pre-wrap",

                                            "{s}"
                                        }
                                    }
                                ))
                            })
                        }
                    }
                }
            ))
        }
    }
    ))
}
