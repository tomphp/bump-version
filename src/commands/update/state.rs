use anyhow::anyhow;

use crate::commands::update::Event;
use crate::formatter::Formatter;

pub struct State<'a, T: Formatter> {
    error: Result<(), &'static str>,
    formatter: &'a T,
}

impl<'a, T: Formatter> State<'a, T> {
    pub const fn new(formatter: &'a T) -> Self {
        Self {
            error: Ok(()),
            formatter,
        }
    }

    pub(crate) fn update_event(&mut self, event: &Event) {
        if let Event::Failed(_, _) = event {
            self.error = Err("There were errors when updating files");
        }

        self.formatter.format_event(event);
    }

    pub fn as_result(&self) -> Result<(), anyhow::Error> {
        self.error.map_err(|message| anyhow!(message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use uuid::Uuid;

    #[derive(Default)]
    struct TestFormatter {
        received_events: RefCell<Vec<Event>>,
    }

    impl TestFormatter {
        fn received_events(&self) -> Vec<Event> {
            self.received_events.take()
        }
    }

    impl Formatter for TestFormatter {
        fn format_event(&self, event: &Event) {
            self.received_events.borrow_mut().push(event.clone());
        }
    }

    #[test]
    fn app_state_has_a_default_formatter() {
        let formatter: TestFormatter = TestFormatter::default();
        assert!(State::new(&formatter).as_result().is_ok());
    }

    #[test]
    fn app_state_will_format_events_as_they_arrive() {
        let id = Uuid::new_v4();
        let formatter = TestFormatter::default();
        let mut state = State::new(&formatter);

        state.update_event(&Event::Failed(id, "error".to_string()));
        state.update_event(&Event::Succeeded(id));
        assert_eq!(
            formatter.received_events(),
            vec![Event::Failed(id, "error".to_string()), Event::Succeeded(id),]
        );
    }

    #[test]
    fn app_state_returns_ok_when_no_events() {
        let formatter: TestFormatter = TestFormatter::default();
        assert!(State::new(&formatter).as_result().is_ok());
    }

    #[test]
    fn app_state_returns_error_when_failed_event_received() {
        let id = Uuid::new_v4();
        let formatter: TestFormatter = TestFormatter::default();
        let mut state = State::new(&formatter);
        state.update_event(&Event::Failed(id, "error".to_string()));
        state.update_event(&Event::Succeeded(id));
        assert!(state.as_result().is_err());
    }
    #[test]
    fn app_state_returns_ok_when_other_events_received() {
        let id = Uuid::new_v4();
        let formatter: TestFormatter = TestFormatter::default();
        let mut state = State::new(&formatter);

        state.update_event(&Event::Started(id, "file".to_string()));
        state.update_event(&Event::Succeeded(id));
        assert!(state.as_result().is_ok());
    }
}
