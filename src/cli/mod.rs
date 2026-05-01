mod csv;

pub use self::csv::{CsvOpts, OutputFormat};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rcli",version,author, about,long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(
        name = "csv",
        about = "Show a CSV record, or convert CSV to other formats"
    )]
    Csv(CsvOpts),
}
