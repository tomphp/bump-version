use super::event::Event;
use uuid;
use uuid::Uuid;

pub struct LifeCycle {
    id: Uuid,
    file_name: String,
}

impl LifeCycle {
    pub(crate) fn new(file_name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_name: file_name.to_string(),
        }
    }

    pub(crate) fn start(&self) -> Event {
        Event::Started(self.id, self.file_name.clone())
    }

    pub(crate) const fn succeed(&self) -> Event {
        Event::Succeeded(self.id)
    }

    pub(crate) fn fail(&self, reason: &str) -> Event {
        Event::Failed(self.id, reason.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Event;
    use super::*;

    #[test]
    fn start_returns_a_started_event() {
        let life_cycle = LifeCycle::new("file.txt");
        assert!(
            matches!(life_cycle.start(), Event::Started(_, file_name) if file_name == "file.txt")
        );
    }

    #[test]
    fn succeed_returns_a_succeed_event() {
        let life_cycle = LifeCycle::new("file.txt");
        assert!(matches!(life_cycle.succeed(), Event::Succeeded(_)));
    }

    #[test]
    fn fail_returns_a_fail_event() {
        let life_cycle = LifeCycle::new("file.txt");
        assert!(
            matches!(life_cycle.fail("error message"), Event::Failed(_, reason) if reason == "error message")
        );
    }

    #[test]
    #[allow(clippy::panic)]
    fn new_returns_instances_with_unique_ids() {
        let life_cycle1 = LifeCycle::new("file1.txt");
        let life_cycle2 = LifeCycle::new("file2.txt");
        match (life_cycle1.start(), life_cycle2.start()) {
            (Event::Started(id1, _), Event::Started(id2, _)) => {
                assert_ne!(id1, id2);
            }
            _ => panic!("Unexpected events returned from start()"),
        }
    }

    #[test]
    #[allow(clippy::panic)]
    fn ids_are_consistent_within_a_lifecycle() {
        let life_cycle = LifeCycle::new("file.txt");
        let started = life_cycle.start();
        let succeeded = life_cycle.succeed();
        let failed = life_cycle.fail("error");

        match (started, succeeded, failed) {
            (
                Event::Started(start_id, _),
                Event::Succeeded(succeed_id),
                Event::Failed(failed_id, _),
            ) => {
                assert_eq!(start_id, succeed_id);
                assert_eq!(start_id, failed_id);
            }
            _ => panic!("Unexpected events returned from start()"),
        }
    }
}
