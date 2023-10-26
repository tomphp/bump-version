use crate::commands::update::Event;

pub trait Formatter {
    fn format_event(&mut self, event: &Event);
}
