use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),

    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(long,value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser=verify_file, default_value = "-")]
    pub input: String,

    #[arg(long,value_parser=parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl std::str::FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format: {}", input)),
        }
    }
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse::<Base64Format>()
}

fn verify_file(input_file: &str) -> Result<String, anyhow::Error> {
    if input_file == "-" || std::path::Path::new(input_file).exists() {
        Ok(input_file.into())
    } else {
        anyhow::bail!("File '{}' does not exist", input_file);
    }
}
