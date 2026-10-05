use crate::event::{EventType, SensorEvent};

pub fn collect_current_process_event(source: &str) -> SensorEvent {
    let mut event = SensorEvent::new(EventType::ProcessSnapshot, source);

    event.pid = Some(std::process::id());
    event.username = current_username();
    event.command_line = Some(std::env::args().collect::<Vec<String>>().join(" "));

    if let Ok(exe_path) = std::env::current_exe() {
        event.path = Some(exe_path.display().to_string());

        if let Some(name) = exe_path.file_name() {
            event.process_name = Some(name.to_string_lossy().to_string());
        }
    }

    event.tags.push("local_userland".to_string());
    event.tags.push("safe_snapshot".to_string());
    event.notes = Some(
        "Safe userland process snapshot collected from the running agent process.".to_string(),
    );

    event
}

pub fn collect_system_observation(source: &str) -> SensorEvent {
    let mut event = SensorEvent::new(EventType::SystemObservation, source);

    event.username = current_username();
    event.notes = Some("Safe local system observation. No kernel hooks, no privileged collection, no external targeting.".to_string());
    event.tags.push("system_observation".to_string());
    event.tags.push("lab_safe".to_string());

    event
}

fn current_username() -> Option<String> {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .ok()
}
