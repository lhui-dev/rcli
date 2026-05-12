use anyhow::Result;
use clap::Parser;
use rcli::{HttpSubCommand, Opts, SubCommand, process_csv, process_gen_passwd, process_http_serve};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let with_header = !opts.no_header;
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
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
    }
    Ok(())
}
