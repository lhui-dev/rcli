mod base64;
mod csv;
mod http;
mod passwd;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::{CsvOpts, OutputFormat},
    http::HttpSubCommand,
    passwd::GenPasswdOpts,
};
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

    #[command(name = "passwd", about = "Generate a passwd")]
    Passwd(GenPasswdOpts),

    #[command(subcommand)]
    Http(HttpSubCommand),

    #[command(subcommand, name = "base64", about = "Base64 encode/decode")]
    Base64(Base64SubCommand),
}
