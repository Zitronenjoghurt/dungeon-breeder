pub mod data;
pub mod entry;
pub mod event;
pub mod id;

#[derive(Debug)]
pub struct Dialogue<'a> {
    pub entries: &'a [entry::DialogueEntry<'a>],
}

#[macro_export]
macro_rules! dialogue {
    ($($entry:expr),* $(,)?) => {
        {
            const ENTRIES: &[$crate::data::dialogue::entry::DialogueEntry] = &[$($entry),*];
            Dialogue {
                entries: ENTRIES,
            }
        }
    };
}
