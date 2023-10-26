#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UpdateEvent {
    Started(usize, String),
    Succeeded(usize),
    Failed(usize, String),
}
