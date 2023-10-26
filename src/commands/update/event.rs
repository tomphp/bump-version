use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    Started(Uuid, String),
    Succeeded(Uuid),
    Failed(Uuid, String),
}
