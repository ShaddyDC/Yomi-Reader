#[derive(PartialEq, Eq)]
pub enum InfoState {
    Idle,
    LoadDB,
    LoadDict(LoadDictState),
}

#[derive(PartialEq, Eq)]
pub enum LoadDictState {
    ParsingDict,
    AddingDictIndex,
    AddingDictContent(usize, usize),
}
