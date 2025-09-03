use anyhow::Result;
use crate::cli::{Cli, Command};

pub mod init;
pub mod add;
pub mod done;
pub mod list;
pub mod stats;
pub mod streak;


/**
 * dispatching as terminal commands for each habit operations (initialisation, add, list, done, undo, streak, and stats)
 */

pub fn dispatch(args: Cli) -> Result<()> {
    match args.cmd {
        Command::Init => init::run(),
        Command::Add { name, goal, tags } => add::run(name, goal, tags),
        Command::List { all, archived, tag } => list::run(all, archived, tag),
        Command::Done { name, date } => done::run_done(name, date),
        Command::Undo { name, date } => done::run_undo(name, date),
        Command::Streak { name } => streak::run(name),
        Command::Stats { habit, global } => stats::run(habit, global),
    }
}