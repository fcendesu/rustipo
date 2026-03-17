use serde::{Deserialize, Serialize, de::Error as DeError};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentDate(String);

impl ContentDate {
    pub fn parse(input: &str) -> Result<Self, String> {
        validate_iso_date(input)?;
        Ok(Self(input.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn month_key(&self) -> String {
        self.0[..7].to_string()
    }
}

impl fmt::Display for ContentDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for ContentDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ContentDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        ContentDate::parse(&raw).map_err(D::Error::custom)
    }
}

fn validate_iso_date(input: &str) -> Result<(), String> {
    if input.len() != 10 {
        return Err("date must use YYYY-MM-DD format".to_string());
    }
    let bytes = input.as_bytes();
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return Err("date must use YYYY-MM-DD format".to_string());
    }

    let year = parse_component(input, 0, 4, "year")?;
    let month = parse_component(input, 5, 7, "month")?;
    let day = parse_component(input, 8, 10, "day")?;

    if !(1..=12).contains(&month) {
        return Err("date month must be between 01 and 12".to_string());
    }
    if day == 0 {
        return Err("date day must be at least 01".to_string());
    }

    let max_day = max_day_for_month(year, month);
    if day > max_day {
        return Err(format!(
            "date day must be between 01 and {max_day:02} for the given month"
        ));
    }

    Ok(())
}

fn parse_component(input: &str, start: usize, end: usize, name: &str) -> Result<u32, String> {
    let segment = &input[start..end];
    if !segment.chars().all(|c| c.is_ascii_digit()) {
        return Err(format!("date {name} must be numeric"));
    }
    segment
        .parse::<u32>()
        .map_err(|_| format!("date {name} must be numeric"))
}

fn max_day_for_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

#[cfg(test)]
mod tests {
    use super::ContentDate;

    #[test]
    fn parses_valid_date() {
        let date = ContentDate::parse("2026-03-17").expect("date should parse");
        assert_eq!(date.as_str(), "2026-03-17");
        assert_eq!(date.month_key(), "2026-03");
    }

    #[test]
    fn rejects_invalid_dates() {
        for raw in ["2026-13-01", "2026-02-30", "2026-3-1", "abcd-ef-gh"] {
            assert!(ContentDate::parse(raw).is_err(), "date should be invalid: {raw}");
        }
    }
}
