use clap::Parser;
use std::ops::RangeInclusive;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short = 'p', long, default_value_t = 10010, help = "http server port(1..=65535)", value_parser = port_in_range)]
    pub port: u16,

    #[arg(short = 'd', long, default_value = ".")]
    pub dir: PathBuf,

    #[arg(long, default_value = "127.0.0.1", help = "http server ip")]
    pub ip: String,
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}
