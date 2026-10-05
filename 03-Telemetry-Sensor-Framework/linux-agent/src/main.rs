use anyhow::{anyhow, Result};
use sensor_core::collector::{collect_current_process_event, collect_system_observation};
use sensor_core::normalizer::{normalize_event, validate_event};

fn main() -> Result<()> {
    let events = vec![
        normalize_event(collect_current_process_event("linux-agent")),
        normalize_event(collect_system_observation("linux-agent")),
    ];

    for event in &events {
        validate_event(event).map_err(|err| anyhow!(err))?;
    }

    println!("{}", serde_json::to_string_pretty(&events)?);

    Ok(())
}
