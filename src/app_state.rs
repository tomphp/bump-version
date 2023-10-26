use crate::UpdateEvent;
use anyhow::anyhow;

pub struct AppState {
    error: Result<(), &'static str>,
}

impl AppState {
    pub const fn new() -> Self {
        Self { error: Ok(()) }
    }

    pub(crate) fn update_event(&mut self, event: &UpdateEvent) {
        if let UpdateEvent::Failed(_, _) = event {
            self.error = Err("There were errors when updating files");
        }
    }

    pub fn as_result(&self) -> Result<(), anyhow::Error> {
        self.error.map_err(|message| anyhow!(message))
    }
}

#[cfg(test)]
mod tests {
    use crate::app_state::AppState;
    use crate::UpdateEvent;

    #[test]
    fn app_state_returns_ok_when_no_events() {
        assert!(AppState::new().as_result().is_ok());
    }

    #[test]
    fn app_state_returns_error_when_failed_event_received() {
        let mut state = AppState::new();
        state.update_event(&UpdateEvent::Failed(1, "error".to_string()));
        state.update_event(&UpdateEvent::Succeeded(1));
        assert!(state.as_result().is_err());
    }
    #[test]
    fn app_state_returns_ok_when_other_events_received() {
        let mut state = AppState::new();

        state.update_event(&UpdateEvent::Started(1, "file".to_string()));
        state.update_event(&UpdateEvent::Succeeded(1));
        assert!(state.as_result().is_ok());
    }
}
