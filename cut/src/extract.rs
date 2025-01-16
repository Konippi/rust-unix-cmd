use std::ops::Range;

use clap::Args;

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct ExtractArgs {
    #[arg(short, long, value_name = "FIELDS", help = "Selected fields")]
    fields: Option<String>,
    #[arg(short, long, value_name = "BYTES", help = "Selected bytes")]
    bytes: Option<String>,
    #[arg(short, long, value_name = "CHARS", help = "Selected characters")]
    chars: Option<String>,
}

pub type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}
