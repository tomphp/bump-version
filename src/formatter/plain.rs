use super::r#trait::Formatter;
use crate::update_event::UpdateEvent;

#[derive(Debug)]
pub struct Plain;

impl Formatter for Plain {
    fn format_event(&mut self, event: &UpdateEvent) {
        match event {
            UpdateEvent::Started(_, file) => {
                print!("Updating {file}...");
            }
            UpdateEvent::Succeeded(_) => {
                println!("success");
            }
            UpdateEvent::Failed(_, message) => {
                println!("failed");
                println!("Error: {message}");
            }
        }
    }
}
