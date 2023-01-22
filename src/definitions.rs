use dioxus::prelude::*;
use yomi_dict::DB;

// This shouldn't be an issue since we only mutate the db on creation with load_db
// https://github.com/rust-lang/rust-clippy/issues/6671
#[allow(clippy::await_holding_refcell_ref)]
async fn get_terms(
    text: &str,
    reasons: &yomi_dict::Reasons,
    db: &UseRef<Option<yomi_dict::IndexedDB>>,
) -> Result<Vec<yomi_dict::DictEntries>, yomi_dict::YomiDictError> {
    let db_ref = db.read();
    let db = db_ref.as_ref().unwrap();
    db.find_terms(text, reasons).await
}

pub async fn update_defs_and_selection(
    defs: &UseState<Vec<yomi_dict::DictEntries>>,
    db: &UseRef<Option<yomi_dict::IndexedDB>>,
    reasons: &yomi_dict::Reasons,
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

        let len = entries
            .first()
            .map_or(0, |entry| entry.entries[0].source_len);

        // Can't just execute `len` times because line-breaks etc. count extra.
        // Note that this is inefficient and a lot of work like `chars()` is repeated.
        // It's simple though and seems fast enough.
        while selection
            .to_string()
            .as_string()
            .map_or(false, |s| s.chars().count() != len)
        {
            selection.modify("extend", "forward", "character").unwrap();
        }
    }

    defs.set(entries);
}

#[inline_props]
pub fn definitions_component<'a>(
    cx: Scope,
    definitions: &'a Vec<yomi_dict::DictEntries>,
) -> Element {
    let content = if definitions.is_empty() {
        rsx!(p{"Click the first letter of an expression to look it up!"})
    } else {
        rsx!(ul{
            class: "list-none",

            definitions.iter().map(|d| rsx!(
                li{
                    key: "{d.expression}/{d.reading}",
                    h2{
                        class: "text-xl font-medium inline-block",

                        ruby {
                            p { "{d.expression}" }
                            rt{ "{d.reading}" }
                        }
                    }
                    span{
                        d.entries[0].reasons.iter().map(|r|
                            rsx!(
                                p{
                                    class: "inline-block text-sm rounded-full m-1 p-1 px-2 bg-gray-300",
                                    "{r}"
                                }
                            )
                        )
                    }
                    div{
                        ol{
                            class: "list-decimal px-4",

                            d.entries.iter().enumerate().flat_map(|(ie, e)| {
                                let key = u64::try_from(ie).unwrap_or_default() << 48 | u64::from(e.term.dict_id ) << 32 | u64::from(e.term.sequence);

                                e.term.glossary.iter().enumerate().map(move |(i, s)| {
                                    rsx!(
                                        li{
                                            // This is obviously not great, and it may lead to issues
                                            // if the database and this list are updated mid-execution,
                                            // but I don't expect this to be much of a problem in practice.
                                            // We cannot just use the string itself because there may be duplicate
                                            // definitions in the database from merged entries, and I'm undecided how I want to handle that.
                                            key: "{key}/{i}",

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
