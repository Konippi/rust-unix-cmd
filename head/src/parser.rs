pub fn parse_positive_int(val: &str) -> anyhow::Result<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(anyhow::anyhow!("{} is not a positive integer", val)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive_int() {
        assert_eq!(parse_positive_int("0").is_err(), true);
        assert_eq!(parse_positive_int("1").unwrap(), 1);
        assert_eq!(parse_positive_int("a").is_err(), true);
    }
}
