use std::pin::Pin;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Event {
    Started(Uuid, String),
    Succeeded(Uuid),
    Failed(Uuid, String),
}

pub type Stream = Pin<Box<dyn futures::Stream<Item = Event> + Send + 'static>>;
