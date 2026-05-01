use anyhow::Result;
use clap::Parser;
use rcli::{Opts, SubCommand, process_csv};
fn main() -> Result<()> {
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
    }
    Ok(())
}
