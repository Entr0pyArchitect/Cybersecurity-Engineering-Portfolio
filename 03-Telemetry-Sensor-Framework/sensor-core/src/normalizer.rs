use crate::event::SensorEvent;

pub fn normalize_event(mut event: SensorEvent) -> SensorEvent {
    event.source = event.source.trim().to_string();
    event.host = event.host.trim().to_string();
    event.os = event.os.trim().to_lowercase();

    trim_option(&mut event.process_name);
    trim_option(&mut event.username);
    trim_option(&mut event.command_line);
    trim_option(&mut event.path);
    trim_option(&mut event.remote_address);
    trim_option(&mut event.notes);

    event.tags = event
        .tags
        .into_iter()
        .map(|tag| tag.trim().to_lowercase().replace(' ', "_"))
        .filter(|tag| !tag.is_empty())
        .collect();

    event.tags.sort();
    event.tags.dedup();

    event
}

pub fn validate_event(event: &SensorEvent) -> Result<(), String> {
    if event.event_id.trim().is_empty() {
        return Err("event_id is required".to_string());
    }

    if event.timestamp_utc.trim().is_empty() {
        return Err("timestamp_utc is required".to_string());
    }

    if event.host.trim().is_empty() {
        return Err("host is required".to_string());
    }

    if event.os.trim().is_empty() {
        return Err("os is required".to_string());
    }

    if event.source.trim().is_empty() {
        return Err("source is required".to_string());
    }

    Ok(())
}

fn trim_option(value: &mut Option<String>) {
    if let Some(current) = value {
        let trimmed = current.trim().to_string();

        if trimmed.is_empty() {
            *value = None;
        } else {
            *current = trimmed;
        }
    }
}
