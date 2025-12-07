//! Temporal notation parsing
//!
//! Parsing for time-related entity tags:
//!
//! ## Date `⊙⦑⦒`
//! - `⊙⦑YYYY⦒` - year only
//! - `⊙⦑YYYY-MM⦒` - year and month
//! - `⊙⦑YYYY-MM-DD⦒` - full date
//! - `⊙⦑YYY BC⦒` - BC dates
//!
//! ## Era `⧖⦑⦒`
//! - `⧖⦑1980s⦒` - decades
//! - `⧖⦑1790-1820⦒` - year ranges
//! - `⧖⦑Nineteenth Century⦒` - named centuries

use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

use crate::symbols::{BRACKET_CLOSE, BRACKET_OPEN, DATE, ERA};

// ═══════════════════════════════════════════════════════════════════════════
// Date Constants
// ═══════════════════════════════════════════════════════════════════════════

/// Month name to number mapping
pub const MONTHS: &[(&str, &str)] = &[
    ("January", "01"),
    ("February", "02"),
    ("March", "03"),
    ("April", "04"),
    ("May", "05"),
    ("June", "06"),
    ("July", "07"),
    ("August", "08"),
    ("September", "09"),
    ("October", "10"),
    ("November", "11"),
    ("December", "12"),
];

// Pre-compiled regexes for date extraction
static DAY_YEAR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(January|February|March|April|May|June|July|August|September|October|November|December)\s+(\d{1,2}),?\s+(\d{4})\b").unwrap()
});

static MONTH_YEAR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(January|February|March|April|May|June|July|August|September|October|November|December)\s+(\d{4})\b").unwrap()
});

// ═══════════════════════════════════════════════════════════════════════════
// Extracted Date Type
// ═══════════════════════════════════════════════════════════════════════════

/// A date extracted from text
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ExtractedDate {
    pub year: String,
    pub month: String,
    pub day: Option<String>,
}

impl ExtractedDate {
    /// Create a new extracted date
    pub fn new(year: impl Into<String>, month: impl Into<String>, day: Option<String>) -> Self {
        Self {
            year: year.into(),
            month: month.into(),
            day,
        }
    }

    /// Format as ERIS date tag: ⊙⦑YYYY-MM⦒ or ⊙⦑YYYY-MM-DD⦒
    pub fn to_tag(&self) -> String {
        match &self.day {
            Some(d) => format!(
                "{}{}{}{}{}",
                DATE,
                BRACKET_OPEN,
                format_args!("{}-{}-{}", self.year, self.month, d),
                BRACKET_CLOSE,
                ""
            ),
            None => format!(
                "{}{}{}{}",
                DATE,
                BRACKET_OPEN,
                format_args!("{}-{}", self.year, self.month),
                BRACKET_CLOSE
            ),
        }
    }

    /// Get year as i32
    pub fn year_i32(&self) -> Option<i32> {
        self.year.parse().ok()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Date Parsing
// ═══════════════════════════════════════════════════════════════════════════

/// Parse a date tag into a year
///
/// Handles: ⊙⦑YYYY⦒, ⊙⦑YYYY-MM⦒, ⊙⦑YYYY-MM-DD⦒, ⊙⦑YYY BC⦒
pub fn parse_date_tag(tag: &str) -> Option<i32> {
    let start = tag.find(BRACKET_OPEN)?;
    let end = tag.find(BRACKET_CLOSE)?;
    let content = &tag[start + BRACKET_OPEN.len_utf8()..end];
    parse_date_content(content)
}

/// Parse the content of a date tag (without brackets)
pub fn parse_date_content(content: &str) -> Option<i32> {
    // Check for BC dates
    if content.ends_with(" BC") || content.ends_with(" BCE") {
        let suffix_len = if content.ends_with(" BCE") { 4 } else { 3 };
        let year_str = &content[..content.len() - suffix_len];
        let year: i32 = year_str.parse().ok()?;
        return Some(-year);
    }

    // Handle YYYY, YYYY-MM, YYYY-MM-DD
    let year_str = content.split('-').next()?;
    year_str.parse().ok()
}

/// Check if a tag is a date tag (⊙⦑...⦒)
pub fn is_date_tag(tag: &str) -> bool {
    tag.starts_with(&format!("{}{}", DATE, BRACKET_OPEN)) && tag.ends_with(BRACKET_CLOSE)
}

/// Validate date tag format
pub fn validate_date_tag(tag: &str) -> bool {
    if !is_date_tag(tag) {
        return false;
    }

    let start = match tag.find(BRACKET_OPEN) {
        Some(s) => s,
        None => return false,
    };
    let end = match tag.find(BRACKET_CLOSE) {
        Some(e) => e,
        None => return false,
    };
    let content = &tag[start + BRACKET_OPEN.len_utf8()..end];

    validate_date_content(content)
}

/// Validate date content (without brackets)
pub fn validate_date_content(content: &str) -> bool {
    // BC/BCE dates
    if content.ends_with(" BC") || content.ends_with(" BCE") {
        let suffix_len = if content.ends_with(" BCE") { 4 } else { 3 };
        let year_str = &content[..content.len() - suffix_len];
        return year_str.parse::<i32>().is_ok();
    }

    let parts: Vec<&str> = content.split('-').collect();
    match parts.len() {
        1 => {
            let year: i32 = match parts[0].parse() {
                Ok(y) => y,
                Err(_) => return false,
            };
            (1..=9999).contains(&year)
        }
        2 => {
            let year: i32 = match parts[0].parse() {
                Ok(y) => y,
                Err(_) => return false,
            };
            let month: u32 = match parts[1].parse() {
                Ok(m) => m,
                Err(_) => return false,
            };
            (1..=9999).contains(&year) && (1..=12).contains(&month)
        }
        3 => {
            let year: i32 = match parts[0].parse() {
                Ok(y) => y,
                Err(_) => return false,
            };
            let month: u32 = match parts[1].parse() {
                Ok(m) => m,
                Err(_) => return false,
            };
            let day: u32 = match parts[2].parse() {
                Ok(d) => d,
                Err(_) => return false,
            };
            (1..=9999).contains(&year) && (1..=12).contains(&month) && (1..=31).contains(&day)
        }
        _ => false,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Date Text Extraction
// ═══════════════════════════════════════════════════════════════════════════

/// Extract dates from text
///
/// Finds patterns like:
/// - "January 1987" → ⊙⦑1987-01⦒
/// - "July 7, 2020" → ⊙⦑2020-07-07⦒
///
/// Returns unique dates within the year range [min_year, max_year].
pub fn extract_dates(text: &str, min_year: i32, max_year: i32) -> HashSet<ExtractedDate> {
    let mut dates = HashSet::new();

    // First find "Month Day, Year" patterns (more specific)
    for cap in DAY_YEAR_RE.captures_iter(text) {
        let month_name = cap.get(1).unwrap().as_str();
        let day: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let year: i32 = cap.get(3).unwrap().as_str().parse().unwrap();

        if year >= min_year && year <= max_year {
            if let Some((_, month_num)) = MONTHS.iter().find(|(name, _)| *name == month_name) {
                dates.insert(ExtractedDate {
                    year: year.to_string(),
                    month: month_num.to_string(),
                    day: Some(format!("{day:02}")),
                });
            }
        }
    }

    // Then find "Month Year" patterns (skip if part of day-year match)
    for cap in MONTH_YEAR_RE.captures_iter(text) {
        let match_start = cap.get(0).unwrap().start();

        let prefix: String = text[..match_start]
            .chars()
            .rev()
            .take(5)
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        if prefix.chars().last().is_some_and(|c| c.is_ascii_digit()) {
            continue;
        }

        let month_name = cap.get(1).unwrap().as_str();
        let year: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

        if year >= min_year && year <= max_year {
            if let Some((_, month_num)) = MONTHS.iter().find(|(name, _)| *name == month_name) {
                dates.insert(ExtractedDate {
                    year: year.to_string(),
                    month: month_num.to_string(),
                    day: None,
                });
            }
        }
    }

    dates
}

// ═══════════════════════════════════════════════════════════════════════════
// Date Bucket Utilities
// ═══════════════════════════════════════════════════════════════════════════

/// Round year to appropriate bucket (century before 1500, decade after)
pub fn year_to_bucket(year: i32) -> i32 {
    if year >= 1500 {
        (year / 10) * 10
    } else if year >= 0 {
        (year / 100) * 100
    } else {
        ((year - 99) / 100) * 100
    }
}

/// Format bucket label (century or decade)
pub fn format_bucket(bucket: i32) -> String {
    if bucket < 0 {
        format!("{}s BC", -bucket)
    } else {
        format!("{bucket}s")
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Era Parsing
// ═══════════════════════════════════════════════════════════════════════════

/// Parse an era tag into a representative year
///
/// Handles:
/// - Decades: ⧖⦑1980s⦒, ⧖⦑Early 1930s⦒, ⧖⦑1960s Japan⦒
/// - Ranges: ⧖⦑1790-1820⦒ (returns midpoint)
/// - Centuries: ⧖⦑Nineteenth Century⦒, ⧖⦑Early Twentieth Century⦒
pub fn parse_era_tag(tag: &str) -> Option<i32> {
    let start = tag.find(BRACKET_OPEN)?;
    let end = tag.find(BRACKET_CLOSE)?;
    let content = &tag[start + BRACKET_OPEN.len_utf8()..end];
    parse_era_content(content)
}

/// Parse era content (without brackets)
pub fn parse_era_content(content: &str) -> Option<i32> {
    // Pattern 1: Any decade pattern like "1980s" anywhere in the tag
    let decade_anywhere_re = Regex::new(r"(\d{3,4})s").ok()?;
    if let Some(caps) = decade_anywhere_re.captures(content) {
        return caps.get(1)?.as_str().parse().ok();
    }

    // Pattern 2: Year range like "1790-1820" - use midpoint
    let year_range_re = Regex::new(r"(\d{4})-(\d{4})").ok()?;
    if let Some(caps) = year_range_re.captures(content) {
        let start: i32 = caps.get(1)?.as_str().parse().ok()?;
        let end: i32 = caps.get(2)?.as_str().parse().ok()?;
        return Some((start + end) / 2);
    }

    // Pattern 3: Century words
    let century_patterns = [
        ("First Century", 0),
        ("Second Century", 100),
        ("Third Century", 200),
        ("Fourth Century", 300),
        ("Fifth Century", 400),
        ("Sixth Century", 500),
        ("Seventh Century", 600),
        ("Eighth Century", 700),
        ("Ninth Century", 800),
        ("Tenth Century", 900),
        ("Eleventh Century", 1000),
        ("Twelfth Century", 1100),
        ("Thirteenth Century", 1200),
        ("Fourteenth Century", 1300),
        ("Fifteenth Century", 1400),
        ("Sixteenth Century", 1500),
        ("Seventeenth Century", 1600),
        ("Eighteenth Century", 1700),
        ("Nineteenth Century", 1800),
        ("Twentieth Century", 1900),
        ("Twenty-First Century", 2000),
    ];

    for (pattern, base_year) in century_patterns {
        if content.contains(pattern) {
            let is_bc = content.contains("BCE") || content.contains(" BC");
            let year = if is_bc { -base_year } else { base_year };

            let offset = if content.starts_with("Early") {
                0
            } else if content.starts_with("Mid") {
                50
            } else if content.starts_with("Late") {
                75
            } else {
                50
            };

            return Some(if is_bc { year - offset } else { year + offset });
        }
    }

    None
}

/// Check if a tag is an era tag (⧖⦑...⦒)
pub fn is_era_tag(tag: &str) -> bool {
    tag.starts_with(&format!("{}{}", ERA, BRACKET_OPEN)) && tag.ends_with(BRACKET_CLOSE)
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // Date tests
    #[test]
    fn test_extracted_date_to_tag() {
        let date = ExtractedDate::new("2020", "07", Some("07".to_string()));
        assert_eq!(date.to_tag(), "⊙⦑2020-07-07⦒");

        let date = ExtractedDate::new("1987", "01", None);
        assert_eq!(date.to_tag(), "⊙⦑1987-01⦒");
    }

    #[test]
    fn test_parse_date_tag_basic() {
        assert_eq!(parse_date_tag("⊙⦑1984⦒"), Some(1984));
        assert_eq!(parse_date_tag("⊙⦑2023-05⦒"), Some(2023));
        assert_eq!(parse_date_tag("⊙⦑2023-05-15⦒"), Some(2023));
    }

    #[test]
    fn test_parse_date_tag_bc() {
        assert_eq!(parse_date_tag("⊙⦑500 BC⦒"), Some(-500));
        assert_eq!(parse_date_tag("⊙⦑44 BC⦒"), Some(-44));
        assert_eq!(parse_date_tag("⊙⦑323 BCE⦒"), Some(-323));
    }

    #[test]
    fn test_is_date_tag() {
        assert!(is_date_tag("⊙⦑1984⦒"));
        assert!(is_date_tag("⊙⦑2023-05-15⦒"));
        assert!(!is_date_tag("⧖⦑1980s⦒"));
        assert!(!is_date_tag("⚘⦑Person⦒"));
    }

    #[test]
    fn test_validate_date_tag() {
        assert!(validate_date_tag("⊙⦑1984⦒"));
        assert!(validate_date_tag("⊙⦑2023-05⦒"));
        assert!(validate_date_tag("⊙⦑2023-05-15⦒"));
        assert!(validate_date_tag("⊙⦑44 BC⦒"));
        assert!(validate_date_tag("⊙⦑323 BCE⦒"));

        assert!(!validate_date_tag("⊙⦑⦒"));
        assert!(!validate_date_tag("⊙⦑abc⦒"));
        assert!(!validate_date_tag("⊙⦑2023-13⦒"));
        assert!(!validate_date_tag("⊙⦑2023-05-32⦒"));
        assert!(!validate_date_tag("⧖⦑1980⦒"));
    }

    #[test]
    fn test_validate_date_content() {
        assert!(validate_date_content("1984"));
        assert!(validate_date_content("2023-05"));
        assert!(validate_date_content("2023-05-15"));
        assert!(validate_date_content("44 BC"));
        assert!(validate_date_content("323 BCE"));

        assert!(!validate_date_content(""));
        assert!(!validate_date_content("abc"));
        assert!(!validate_date_content("2023-13"));
        assert!(!validate_date_content("2023-05-32"));
    }

    #[test]
    fn test_extract_month_day_year() {
        let text = "On July 7, 2020, something happened.";
        let dates = extract_dates(text, 1000, 2025);
        assert_eq!(dates.len(), 1);
        let date = dates.iter().next().unwrap();
        assert_eq!(date.to_tag(), "⊙⦑2020-07-07⦒");
    }

    #[test]
    fn test_extract_month_year() {
        let text = "In January 1987, the event occurred.";
        let dates = extract_dates(text, 1000, 2025);
        assert_eq!(dates.len(), 1);
        let date = dates.iter().next().unwrap();
        assert_eq!(date.to_tag(), "⊙⦑1987-01⦒");
    }

    #[test]
    fn test_extract_multiple_dates() {
        let text = "From January 1987 to March 15, 1988.";
        let dates = extract_dates(text, 1000, 2025);
        assert_eq!(dates.len(), 2);
    }

    #[test]
    fn test_year_filtering() {
        let text = "In January 999 and January 2030.";
        let dates = extract_dates(text, 1000, 2025);
        assert_eq!(dates.len(), 0);
    }

    #[test]
    fn test_year_to_bucket() {
        assert_eq!(year_to_bucket(1984), 1980);
        assert_eq!(year_to_bucket(2023), 2020);
        assert_eq!(year_to_bucket(1500), 1500);
        assert_eq!(year_to_bucket(1066), 1000);
        assert_eq!(year_to_bucket(1450), 1400);
        assert_eq!(year_to_bucket(-44), -100);
        assert_eq!(year_to_bucket(-500), -500);
    }

    #[test]
    fn test_format_bucket() {
        assert_eq!(format_bucket(1980), "1980s");
        assert_eq!(format_bucket(1400), "1400s");
        assert_eq!(format_bucket(-100), "100s BC");
        assert_eq!(format_bucket(-500), "500s BC");
    }

    // Era tests
    #[test]
    fn test_parse_era_tag_decades() {
        assert_eq!(parse_era_tag("⧖⦑1980s⦒"), Some(1980));
        assert_eq!(parse_era_tag("⧖⦑Early 1930s⦒"), Some(1930));
        assert_eq!(parse_era_tag("⧖⦑1960s Japan⦒"), Some(1960));
    }

    #[test]
    fn test_parse_era_tag_range() {
        assert_eq!(parse_era_tag("⧖⦑1790-1820⦒"), Some(1805));
        assert_eq!(parse_era_tag("⧖⦑1914-1918⦒"), Some(1916));
    }

    #[test]
    fn test_parse_era_tag_century() {
        assert_eq!(parse_era_tag("⧖⦑Nineteenth Century⦒"), Some(1850));
        assert_eq!(parse_era_tag("⧖⦑Early Twentieth Century⦒"), Some(1900));
        assert_eq!(parse_era_tag("⧖⦑Late Eighteenth Century⦒"), Some(1775));
    }

    #[test]
    fn test_parse_era_content() {
        assert_eq!(parse_era_content("1980s"), Some(1980));
        assert_eq!(parse_era_content("1790-1820"), Some(1805));
        assert_eq!(parse_era_content("Nineteenth Century"), Some(1850));
    }

    #[test]
    fn test_is_era_tag() {
        assert!(is_era_tag("⧖⦑1980s⦒"));
        assert!(is_era_tag("⧖⦑Nineteenth Century⦒"));
        assert!(!is_era_tag("⊙⦑1984⦒"));
        assert!(!is_era_tag("⚘⦑Person⦒"));
    }

    #[test]
    fn test_unparseable_eras() {
        assert_eq!(parse_era_tag("⧖⦑Victorian Era⦒"), None);
        assert_eq!(parse_era_tag("⧖⦑Renaissance⦒"), None);
        assert_eq!(parse_era_tag("⧖⦑Enlightenment⦒"), None);
    }
}
