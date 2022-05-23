use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// writing a text in a journour file
    Add {
        /// task description text
        #[structopt()]
        task: String,
    },
    /// remove an entry of a journal in the file
    Done {
        #[structopt()]
        position: usize,
    },
    /// show all the tasks available onthe jornal
    List,
}
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rust Todo app",
    about = " A command line Todo - App  written in rust "
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    /// use different journal file
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
