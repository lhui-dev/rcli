use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short = 'i', long, default_value = "input.csv", help = "input file")]
    pub input: String,

    #[clap(long, help = "File output path")]
    pub output: Option<String>,

    #[arg(
        short = 'd',
        long = "delimiter",
        default_value_t = ',',
        help = "Csv file delimiter"
    )]
    pub delimiter: char,

    #[arg(long, default_value_t = false, help = "Is Csv file without header")]
    pub no_header: bool,

    #[arg(long,value_parser=parse_format,default_value = "json", help = "Format to json/yaml")]
    pub format: OutputFormat,
}

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!(
                "Unsupported format '{}' (supported: json/yaml)",
                format
            )),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
