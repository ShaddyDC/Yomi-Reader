#[derive(PartialEq)]
pub(crate) enum InfoState {
    Idle,
    LoadDB,
    LoadDict(LoadDictState),
}

#[derive(PartialEq)]
pub(crate) enum LoadDictState {
    ParsingDict,
    AddingDictIndex,
    AddingDictContent(usize, usize),
}
