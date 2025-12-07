//! ERIS notation parsing system
//!
//! Four notation types with distinct bracket styles:
//! - **Entity** `⦑⦒` - simple and compound entity tags
//! - **Vector** `⟨⟩` - Armenian property vectors
//! - **Reference** `⟦⟧` - ID references
//! - **Temporal** `⊙⦑⦒` `⧖⦑⦒` - dates and eras
//!
//! All notation types support the conjunction operator `∧` for multiple values.

pub mod entity;
pub mod reference;
pub mod temporal;
pub mod vector;

pub use entity::{CompoundTag, EntityNotation};
pub use reference::{Reference, ReferenceNotation};
pub use temporal::{
    extract_dates, format_bucket, is_date_tag, is_era_tag, parse_date_tag, parse_era_tag,
    validate_date_tag, year_to_bucket, ExtractedDate, MONTHS,
};
pub use vector::{PropertyVector, VectorNotation};

use crate::symbols::CONJUNCTION;

/// Split a string by the conjunction operator `∧`, trimming whitespace.
///
/// # Examples
///
/// ```
/// use eris::notation::split_conjunction;
///
/// assert_eq!(split_conjunction("A∧B∧C"), vec!["A", "B", "C"]);
/// assert_eq!(split_conjunction("George Lakoff ∧ Mark Johnson"), vec!["George Lakoff", "Mark Johnson"]);
/// assert_eq!(split_conjunction("single"), vec!["single"]);
/// ```
pub fn split_conjunction(s: &str) -> Vec<&str> {
    s.split(CONJUNCTION).map(|part| part.trim()).collect()
}

/// Join multiple values with the conjunction operator `∧`.
///
/// # Examples
///
/// ```
/// use eris::notation::join_conjunction;
///
/// assert_eq!(join_conjunction(&["A", "B", "C"]), "A∧B∧C");
/// assert_eq!(join_conjunction(&["single"]), "single");
/// ```
pub fn join_conjunction(parts: &[&str]) -> String {
    parts.join(&CONJUNCTION.to_string())
}

/// Common trait for all notation types
pub trait Notation: Sized {
    /// The opening bracket character for this notation
    const OPEN: char;
    /// The closing bracket character for this notation
    const CLOSE: char;

    /// Parse a string into this notation type
    fn parse(input: &str) -> Option<Self>;

    /// Render this notation back to a string
    fn render(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_conjunction() {
        assert_eq!(split_conjunction("A∧B∧C"), vec!["A", "B", "C"]);
        assert_eq!(
            split_conjunction("George Lakoff ∧ Mark Johnson"),
            vec!["George Lakoff", "Mark Johnson"]
        );
        assert_eq!(split_conjunction("single"), vec!["single"]);
        assert_eq!(split_conjunction(""), vec![""]);
    }

    #[test]
    fn test_join_conjunction() {
        assert_eq!(join_conjunction(&["A", "B", "C"]), "A∧B∧C");
        assert_eq!(join_conjunction(&["single"]), "single");
    }
}
