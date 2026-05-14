mod cli;
mod process;

pub use cli::{
    Base64Format, Base64SubCommand, CsvOpts, HttpSubCommand, Opts, OutputFormat, SubCommand,
};
pub use process::{
    process_base64_decode, process_base64_encode, process_csv, process_gen_passwd,
    process_http_serve,
};
