use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;
mod cli;
mod task;

use cli::{Action::*, CommandLineArgs};
use task::*;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();
    let file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("failed to read file "))?;

    match action {
        Add { task } => task::add_task(file, Task::new(task)),
        Done { position } => task::complete_task(file, position),
        List => task::list_tasks(file),
    }?;
    Ok(())
}
