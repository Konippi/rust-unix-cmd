use std::num::NonZeroUsize;

use anyhow::bail;
use regex::Regex;

use crate::extract::PositionList;

pub fn parse_position(range: &str) -> anyhow::Result<PositionList> {
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
