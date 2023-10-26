use crate::update_event::UpdateEvent;

pub trait Formatter {
    fn format_event(&mut self, event: &UpdateEvent);
}
