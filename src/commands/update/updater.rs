use super::event;

pub trait Updater {
    fn update(&self, version: String) -> event::Stream;
}
