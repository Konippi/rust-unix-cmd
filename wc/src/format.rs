pub fn format_field(val: usize, show: bool) -> String {
    if show {
        format!("{:>8}", val)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, true), "       1");
        assert_eq!(format_field(1, false), "");
    }
}
