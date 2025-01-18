use std::num::NonZeroUsize;

use anyhow::bail;
use regex::Regex;

use crate::extract::PositionList;

pub fn parse_position(range: String) -> anyhow::Result<PositionList> {
    let range_re = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    range
        .split(',')
        .into_iter()
        .map(|val| {
            parse_index(val).map(|index| index..index + 1).or_else(|e| {
                range_re.captures(val).ok_or(e).and_then(|cap| {
                    let start = parse_index(&cap[1])?;
                    let end = parse_index(&cap[2])?;
                    if start >= end {
                        bail!("Invalid range: \"{}\"", val);
                    }
                    Ok(start..end + 1)
                })
            })
        })
        .collect::<anyhow::Result<_, _>>()
        .map_err(From::from)
}

pub fn parse_index(input: &str) -> anyhow::Result<usize> {
    let value_error = || anyhow::anyhow!("Invalid index: \"{}\"", input);
    input
        .starts_with('+')
        .then(|| Err(value_error()))
        .unwrap_or_else(|| {
            input
                .parse::<NonZeroUsize>()
                .map(|val| usize::from(val) - 1)
                .map_err(|_| value_error())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_position() {
        assert_eq!(
            parse_position("1,2,3".to_string()).unwrap(),
            vec![0..1, 1..2, 2..3]
        );
        assert_eq!(
            parse_position("1-3,5,7-10".to_string()).unwrap(),
            vec![0..3, 4..5, 6..10]
        );
    }

    #[test]
    fn test_parse_index() {
        assert_eq!(parse_index("1").unwrap(), 0);
        assert_eq!(parse_index("10").unwrap(), 9);
        assert!(parse_index("0").is_err());
        assert!(parse_index("+1").is_err());
        assert!(parse_index("a").is_err());
    }
}
