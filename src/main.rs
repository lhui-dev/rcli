use anyhow::Result;
use clap::Parser;
use rcli::{
    Base64SubCommand, HttpSubCommand, Opts, SubCommand, process_base64_decode,
    process_base64_encode, process_csv, process_gen_passwd, process_http_serve,
};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let with_header = !opts.no_header;
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format));
            process_csv(
                &opts.input,
                with_header,
                opts.delimiter,
                output,
                opts.format,
            )?;
        }
        SubCommand::Passwd(opts) => {
            opts.validate()?;

            let uppercase = !opts.no_uppercase;
            let lowercase = !opts.no_lowercase;
            let numeric = !opts.no_numeric;
            let symbolic = !opts.no_symbolic;
            let output = opts.output.as_deref();

            process_gen_passwd(
                opts.length,
                uppercase,
                lowercase,
                numeric,
                symbolic,
                opts.batch_count,
                output,
            )?;
        }
        SubCommand::Http(sub_command) => match sub_command {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, &opts.ip, opts.port).await?;
            }
        },
        SubCommand::Base64(sub_command) => match sub_command {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_base64_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_base64_decode(&opts.input, opts.format)?;
                std::io::stdout().write_all(&decoded)?;
            }
        },
    }
    Ok(())
}
