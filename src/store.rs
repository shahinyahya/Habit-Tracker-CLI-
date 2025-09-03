use crate::model::{Completion, DataFile, Habit};
use parking_lot::Mutex;
use std::{fs, path::{Path, PathBuf}};
use directories::ProjectDirs;
use anyhow::{bail, Context, Ok, Result};
use std::io::Write;
use uuid::Uuid;


pub struct JsonStore {
    file: PathBuf,
    lock: Mutex<()>,
}

impl JsonStore {

    // Default file locations
    pub fn default() -> Result<Self> {
        let file = default_data_file()?;
        Ok(Self { file, lock: Mutex::new(()) })
    }

    //? Initialises file in parent directory
    pub fn init_files() -> Result<PathBuf> {
        let data_file = default_data_file()?;
        if let Some(parent) = data_file.parent() {fs::create_dir_all(parent)?;}
        if !data_file.exists() {
            let d = DataFile::default();
            atomic_write(&data_file, &serde_json::to_vec_pretty(&d)?)?;
        }
        Ok(data_file)
    }

    // Read Files.
    pub fn read(&self) -> Result<DataFile> {
        let _g = self.lock.lock();
        let bytes = fs::read(&self.file).with_context(|| format!("reading {:?}", &self.file))?;
        let data: DataFile = serde_json::from_slice(&bytes)?;
        Ok(data)
    }

    //? Write file.
    pub fn write(&self, data: &DataFile) -> Result<()> {
        let _g = self.lock.lock();
        let bytes = serde_json::to_vec_pretty(data)?;
        atomic_write(&self.file, &bytes)
    }

    //? Insert Data to Habit Vector
    pub fn upsert_habit(&self, mut data:DataFile, habit: Habit) -> Result<DataFile>{
        if data.habits.iter().any(|h| h.name.eq_ignore_ascii_case(&habit.name)) {
            bail!("habit with this name already exists!");
        }

        let mut h = habit;
        h.name = h.name.trim().to_string();
        data.habits.push(h);
        Ok(data)
    }

    //? Filtering and finding the habit by name.
    pub fn get_habit_by_name(&self, data: &DataFile, name: &str) -> Option<Habit> {
        data.habits
            .iter()
            .find(|h| h.name.eq_ignore_ascii_case(name))
            .cloned()
    }
    
    // Setting up Archive for habits
    pub fn set_archived(&self, mut data: DataFile, id: Uuid, archived: bool) -> DataFile {
        if let Some(h) = data.habits.iter_mut().find(|h| h.id == id) {h.archived = archived;}
        data
    }

    // List Habits which are saved
    pub fn list_habits<'a>(&self, data: &'a DataFile, include_archive: bool, tag: Option<&str>) -> Vec<&'a Habit> {
        data.habits
            .iter()
            .filter(move |h| include_archive || !h.archived)
            .filter(move |h| {
                if let Some(t) = tag { h.tags.iter().any(|x| x.eq_ignore_ascii_case(t)) } else {true}
            })
            .collect()
    }

    //? Add completion for habits
    pub fn add_completion(&self, mut data:DataFile, c:Completion) -> Result<DataFile> {
        let exists = data.completions.iter().any(|x| x.habit_id == c.habit_id && x.date == c.date); 
        if !exists {data.completions.push(c);}
        Ok(data)
    }

    // Remove completion data
    pub fn remove_completion(&self, mut data: DataFile, habit_id: Uuid, date: chrono::NaiveDate) -> DataFile {
        data.completions.retain(|x| !(x.habit_id == habit_id && x.date == date));
        data
    }


    pub fn completions_for(&self, data: &DataFile, habit_id: Uuid) -> Vec<Completion> {
        let mut v: Vec<_> = data.completions.iter().filter(|c| c.habit_id == habit_id).cloned().collect();
        v.sort_by_key(|c| c.date);
        v
    }

    
}


//? here we create json file for storage. We're storing it locally as habits.json.
fn default_data_file() -> Result<PathBuf> {
    let proj = ProjectDirs::from("dev", "habit", "habit").ok_or_else(|| anyhow::anyhow!("cannot resolve data dir.."))?;
    let data_dir = proj.data_dir().join("habits");
    Ok(data_dir.join("habits.json"))
} 


//? Creating default path and renaming to path name.
fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp =path.with_extension("tmp");

    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.flush()?;
    }
    fs::rename(tmp, path)?;
    Ok(())
}