use anyhow::{Context, Ok, Result};
use chrono::Utc;
use crate::model::{Habit, Frequency};
use crate::store::JsonStore;

pub fn run(name: String, goal: String, tags: Option<String>) -> Result<()> {
    let freq: Frequency = goal.parse().context("use daily | week | every:N")?;
    let tags: Vec<String> = tags
        .unwrap_or_default()
        .split(',')
        .filter_map(|s| {
            let t = s.trim();
            if t.is_empty() { None } else { Some(t.to_string()) }
        })
        .collect();

    let store = JsonStore::default()?;

    let mut data = store.read()?;

    let h = Habit {
        id: uuid::Uuid::new_v4(),
        name: name.trim().to_string(),
        frequency: freq,
        tags,
        created_at: Utc::now(),
        archived: false,
    };

    data = store.upsert_habit(data, h)?;
    store.write(&data)?;
    println!("Added habit!!");
    Ok(())
}