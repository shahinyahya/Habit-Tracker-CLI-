use anyhow::{bail, Context, Result};
use chrono::{NaiveDate, Utc};
use crate::store::JsonStore;
use crate::util::today_local;
use crate::model::Completion;


pub fn run_done(name: String, date: Option<NaiveDate>) -> Result<()> {

    /**
    * * This function is for marking habits done.
    * * here we access the store value vector and filter habit getting name.
    * * The we mark the completion by taking Completion struct and adding to it.     
    **/

    let date = date.unwrap_or_else(today_local);
    if date > today_local() { bail!("cannot mark future dates"); }


    let store = JsonStore::default()?;
    let mut data = store.read()?;


    let habit = store
        .get_habit_by_name(&data, &name)
        .with_context(|| format!("habit '{}' not found", name))?;


    let c = Completion { habit_id: habit.id, date, created_at: Utc::now() };
    data = store.add_completion(data, c)?;
    store.write(&data)?;
    println!("Marked '{}' as done on {}", habit.name, date);
    Ok(())
}


pub fn run_undo(name: String, date: Option<NaiveDate>) -> Result<()> {

    /*
     * * Here we undo this fn to undo done mark.
    */

    let date = date.unwrap_or_else(today_local);


    let store = JsonStore::default()?;
    let mut data = store.read()?;


    let habit = store
        .get_habit_by_name(&data, &name)
        .with_context(|| format!("habit '{}' not found", name))?;


    data = store.remove_completion(data, habit.id, date);
    store.write(&data)?;
    println!("Removed completion for '{}' on {}", habit.name, date);
    Ok(())
}