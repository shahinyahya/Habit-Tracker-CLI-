use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::collections::HashSet;
use crate::store::JsonStore;
use crate::util::{today_local, iso_week};
use crate::model::Frequency;


pub fn run(name: String) -> Result<()> {
    let store = JsonStore::default()?;
    let data = store.read()?;


    let habit = store
        .get_habit_by_name(&data, &name)
        .with_context(|| format!("habit '{}' not found", name))?;


    let comps = store.completions_for(&data, habit.id);
    let today = today_local();


    let (current, best) = match habit.frequency {
    Frequency::Daily | Frequency::EveryNDays(_) => daily_streak(comps.iter().map(|c| c.date).collect(), today),
    Frequency::Weekly => weekly_streak(comps.iter().map(|c| c.date).collect(), today),
    };


    println!("Streak — current: {} | best: {}", current, best);
    Ok(())
}


fn daily_streak(mut days: Vec<NaiveDate>, today: NaiveDate) -> (u32, u32) {
    if days.is_empty() { return (0, 0); }
    days.sort_unstable();
    let set: HashSet<_> = days.into_iter().collect();


    // current
    let mut cur = 0;
    let mut d = today;
    while set.contains(&d) {
        cur += 1;
        d = d.pred_opt().unwrap();
    }


    // best
    let mut best = 0;
    let mut d_opt = *set.iter().min().unwrap();
    while d_opt <= today {
        if set.contains(&d_opt) {
            let mut len = 1;
            let mut n = d_opt.succ_opt().unwrap();
            while set.contains(&n) { len += 1; n = n.succ_opt().unwrap(); }
            if len > best { best = len; }
            d_opt = n; // skip this block
        } else {
            d_opt = d_opt.succ_opt().unwrap();
        }
    }
    (cur, best)
}


fn weekly_streak(days: Vec<NaiveDate>, today: NaiveDate) -> (u32, u32) {
    if days.is_empty() { return (0, 0); }
    let weeks: HashSet<(i32,u32)> = days.into_iter().map(iso_week).collect();


    // current week-based streak
    let mut cur = 0;
    let mut y_w = iso_week(today);
    while weeks.contains(&y_w) {
        cur += 1;
        // go to previous week (approx by -7 days)
        let prev = today - chrono::Days::new((7 * cur as u64));
        y_w = iso_week(prev);
    }


    // best: walk approx from min date
    let mut best = 0;
    let mut len = 0;
    // Get all unique (year,week) pairs and walk
    let mut uniq: Vec<_> = weeks.into_iter().collect();
    uniq.sort();
    for i in 0..uniq.len() {
        if i == 0 { len = 1; }
        else {
            // consecutive if next week or year rollover
            let (py, pw) = uniq[i-1];
            let (y, w) = uniq[i];
            let consecutive = (y == py && w == pw + 1) || (y == py + 1 && pw >= 52 && w == 1);
            if consecutive { len += 1; } else { len = 1; }
        }
        if len > best { best = len; }
    }
    (cur, best)
}