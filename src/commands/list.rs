use anyhow::{Ok, Result};
use crate::store::JsonStore;

pub fn run(all: bool, archived: bool, tag: Option<String>) -> Result<()> {

    let store = JsonStore::default()?;
    let data = store.read()?;

    let include_archived = archived || all;
    let tag_ref = tag.as_ref().map(|s| s.as_str());
    let items = store.list_habits(&data, include_archived, tag_ref);

    if items.is_empty() {
        println!("No habits found!!");
        return Ok(());
    }

    println!("{} Habits", if include_archived { "All" } else { "Active" });
    for h in items {
        let freq = match &h.frequency {
            crate::model::Frequency::Daily => "daily".to_string(),
            crate::model::Frequency::Weekly => "weekly".to_string(),
            crate::model::Frequency::EveryNDays(n) => format!("every:{}", n),
        };
        let tags = if h.tags.is_empty() { "-".to_string() } else { h.tags.join(",") };
        let arch = if h.archived { " (archived)" } else { "" };
        println!("• {} [{}] tags:{}{}", h.name, freq, tags, arch);
    }
    Ok(())
}