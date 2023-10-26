#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    Started(usize, String),
    Succeeded(usize),
    Failed(usize, String),
}
