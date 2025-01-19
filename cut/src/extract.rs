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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_fields() {
        let record = StringRecord::from(vec!["á", "b", "c"]);
        assert_eq!(extract_fields(&record, &[0..1]), &["á"]);
        assert_eq!(extract_fields(&record, &[1..2]), &["b"]);
        assert_eq!(extract_fields(&record, &[2..3]), &["c"]);
        assert_eq!(extract_fields(&record, &[0..1, 2..3]), &["á", "c"]);
    }

    #[test]
    fn test_extract_bytes() {
        assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
        assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
        assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
        assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
        assert_eq!(extract_bytes("ábc", &[0..1, 1..2]), "á".to_string());
    }

    #[test]
    fn test_extract_chars() {
        assert_eq!(extract_bytes("", &[0..1]), "".to_string());
        assert_eq!(extract_chars("ábc", &[0..1]), "á");
        assert_eq!(extract_chars("ábc", &[1..2]), "b");
        assert_eq!(extract_chars("ábc", &[2..3]), "c");
        assert_eq!(extract_chars("ábc", &[0..1, 2..3]), "ác");
    }
}
