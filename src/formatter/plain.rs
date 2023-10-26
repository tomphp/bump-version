use super::r#trait::Formatter;
use crate::commands::update::Event;

#[derive(Debug)]
pub struct Plain;

impl Formatter for Plain {
    fn format_event(&mut self, event: &Event) {
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
