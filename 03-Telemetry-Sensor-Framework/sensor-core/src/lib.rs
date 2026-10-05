pub mod collector;
pub mod event;
pub mod normalizer;

pub use event::{EventType, SensorEvent};

#[cfg(test)]
mod tests {
    use crate::collector::{collect_current_process_event, collect_system_observation};
    use crate::normalizer::{normalize_event, validate_event};

    #[test]
    fn current_process_event_is_valid() {
        let event = collect_current_process_event("unit-test");
        let event = normalize_event(event);

        assert!(validate_event(&event).is_ok());
        assert_eq!(event.source, "unit-test");
        assert!(event.pid.is_some());
    }

    #[test]
    fn system_observation_event_is_valid() {
        let event = collect_system_observation("unit-test");
        let event = normalize_event(event);

        assert!(validate_event(&event).is_ok());
        assert_eq!(event.source, "unit-test");
    }
}
