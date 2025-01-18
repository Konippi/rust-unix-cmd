use std::ops::Range;

use clap::Args;
use csv::StringRecord;

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct ExtractArgs {
    #[arg(short, long, value_name = "FIELDS", help = "Selected fields")]
    pub fields: Option<String>,
    #[arg(short, long, value_name = "BYTES", help = "Selected bytes")]
    pub bytes: Option<String>,
    #[arg(short, long, value_name = "CHARS", help = "Selected characters")]
    pub chars: Option<String>,
}

pub type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

pub fn extract_fields<'a>(record: &'a StringRecord, field_pos: &[Range<usize>]) -> Vec<&'a str> {
    field_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| record.get(i)))
        .collect()
}

pub fn extract_bytes(line: &str, byte_pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = byte_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

pub fn extract_chars(line: &str, char_pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    char_pos
        .iter()
        .cloned()
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        .collect()
}
