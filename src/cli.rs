use clap::{Parser, Subcommand};
use chrono::NaiveDate;

#[derive(Parser,Debug)]
#[command(name = "habit", version, about="Track daily habits from the terminal")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand,Debug)]
pub enum Command {

    // Initialise the config/data dirs nd empty store
    Init,

    // Add a new habit
    Add {
        name: String,
        #[arg(long, default_value="daily")]
        goal: String, // daily | weekly | every:N
        #[arg(long)]
        tags: Option<String> //Comma seperated
    },

    //List habits
    List {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        archived: bool,
        #[arg(long)]
        tag: Option<String>,
    },

    // Mark a habit done for a date (default: today)
    Done {
        name: String,
        #[arg(long)]
        date: Option<NaiveDate>,
    },

    // Undo a completion for a date (default: today)
    Undo {
        name: String,
        #[arg(long)]
        date: Option<NaiveDate>,
    },

    // Show streak for one habit
    Streak {
        name: String,
    },

    // Global or per habit stats
    Stats {
        #[arg(long)]
        habit: Option<String>,
        #[arg(long)]
        global: bool
    }
}