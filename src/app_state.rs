use crate::{Formatter, UpdateEvent};
use anyhow::anyhow;

pub struct AppState<'a, T: Formatter> {
    error: Result<(), &'static str>,
    formatter: &'a mut T,
}

impl<'a, T: Formatter> AppState<'a, T> {
    pub fn new(formatter: &'a mut T) -> Self {
        Self {
            error: Ok(()),
            formatter,
        }
    }

    pub(crate) fn update_event(&mut self, event: &UpdateEvent) {
        if let UpdateEvent::Failed(_, _) = event {
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

    use crate::app_state::AppState;
    use crate::{Formatter, UpdateEvent};

    #[derive(Default)]
    struct TestFormatter {
        received_events: Vec<UpdateEvent>,
    }

    impl Formatter for TestFormatter {
        fn format_event(&mut self, event: &UpdateEvent) {
            self.received_events.push(event.clone());
        }
    }

    #[test]
    fn app_state_has_a_default_formatter() {
        let mut formatter: TestFormatter = TestFormatter::default();
        assert!(AppState::new(&mut formatter).as_result().is_ok());
    }

    #[test]
    fn app_state_will_format_events_as_they_arrive() {
        let mut formatter = TestFormatter {
            received_events: Vec::new(),
        };
        let mut state = AppState::new(&mut formatter);

        state.update_event(&UpdateEvent::Failed(1, "error".to_string()));
        state.update_event(&UpdateEvent::Succeeded(1));
        assert_eq!(
            formatter.received_events,
            vec![
                UpdateEvent::Failed(1, "error".to_string()),
                UpdateEvent::Succeeded(1),
            ]
        );
    }

    #[test]
    fn app_state_returns_ok_when_no_events() {
        let mut formatter: TestFormatter = TestFormatter::default();
        assert!(AppState::new(&mut formatter).as_result().is_ok());
    }

    #[test]
    fn app_state_returns_error_when_failed_event_received() {
        let mut formatter: TestFormatter = TestFormatter::default();
        let mut state = AppState::new(&mut formatter);
        state.update_event(&UpdateEvent::Failed(1, "error".to_string()));
        state.update_event(&UpdateEvent::Succeeded(1));
        assert!(state.as_result().is_err());
    }
    #[test]
    fn app_state_returns_ok_when_other_events_received() {
        let mut formatter: TestFormatter = TestFormatter::default();
        let mut state = AppState::new(&mut formatter);

        state.update_event(&UpdateEvent::Started(1, "file".to_string()));
        state.update_event(&UpdateEvent::Succeeded(1));
        assert!(state.as_result().is_ok());
    }
}
