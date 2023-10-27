use super::r#trait::Formatter;
use crate::commands::update::Event;

#[derive(Debug)]
pub struct Plain;

impl Plain {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl Formatter for Plain {
    fn format_event(&self, event: &Event) {
        match event {
            Event::Started(_, file) => {
                print!("Updating {file}...");
            }
            Event::Succeeded(_) => {
                println!("success");
            }
            Event::Failed(_, message) => {
                println!("failed");
                println!("Error: {message}");
            }
        }
    }
}
