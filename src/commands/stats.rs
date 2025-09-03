use anyhow::Result;
use chrono::{Datelike, Local};
use crate::store::JsonStore;


pub fn run(habit: Option<String>, global: bool) -> Result<()> {
    let store = JsonStore::default()?;
    let data = store.read()?;


    if let Some(name) = habit {
        if let Some(h) = store.get_habit_by_name(&data, &name) {
            let comps = store.completions_for(&data, h.id);
            let month = Local::now().date_naive().month();
            let month_cnt = comps.iter().filter(|c| c.date.month() == month).count();
            println!("Stats for '{}': completions this month: {}", h.name, month_cnt);
        } else {
            println!("Habit '{}' not found", name);
        }
        return Ok(());
    }


    if global {
        let today = Local::now().date_naive();
        let week = today.iso_week();
        let mut week_cnt = 0usize;
        let mut month_cnt = 0usize;
        let mut active = 0usize;
        for h in &data.habits { if !h.archived { active += 1; } }
        for c in &data.completions {
            if c.date.iso_week() == week { week_cnt += 1; }
            if c.date.month() == today.month() { month_cnt += 1; }
        }
        println!("Global Stats\n- Active habits: {}\n- Completions this week: {}\n- Completions this month: {}", active, week_cnt, month_cnt);
        return Ok(());
    }

    println!("Use --global or --habit <name>");
    Ok(())
}