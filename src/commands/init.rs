use anyhow::{Ok, Result};
use crate::store::JsonStore;

pub fn run() -> Result<()>{
    let path = JsonStore::init_files()?;
    println!("Initialized data file at {:?}", path);
    Ok(())
}