use anyhow::bail;
use clap::Parser;

use crate::extract::ExtractArgs;

#[derive(Debug, Parser)]
#[clap(
    name = "cut",
    version = "0.1.0",
    about = "Description: A simple `cut` command implementation"
)]
pub struct Cli {
    #[arg(default_value = "-", value_name = "FILE", help = "Input file(s)")]
    pub files: Vec<String>,
    #[arg(
        short,
        long,
        value_name = "DELIMITER",
        default_value = "\t",
        help = "Field delimiter",
        value_parser = validate_delimiter
    )]
    pub delimiter: String,
    #[command(flatten)]
    pub extract: ExtractArgs,
}

fn validate_delimiter(val: &str) -> anyhow::Result<()> {
    let bytes = val.as_bytes();
    if bytes.len() != 1 {
        bail!(anyhow::anyhow!(
            "--delimiter \"{}\" must be a single character",
            val
        ));
    }
    Ok(())
}
