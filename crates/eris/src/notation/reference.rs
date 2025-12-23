//! Reference notation parsing for ERIS ID references
//!
//! References use `⟦⟧` brackets and point to entities by ID.
//! Supports conjunction `∧` for multiple references.
//!
//! # Format
//!
//! - `❧⟦12345⟧` - single highlight reference
//! - `❧⟦123∧456∧789⟧` - multiple highlight references

use super::{split_conjunction, Notation};
use crate::symbols::{CONJUNCTION, HIGHLIGHT_REF, REF_CLOSE, REF_OPEN};

/// Reference notation definition
#[derive(Debug, Clone)]
pub struct ReferenceNotation {
    /// Name of this reference type
    pub name: &'static str,
    /// Symbol prefix for this reference type
    pub symbol: char,
    /// Description of what this reference points to
    pub description: &'static str,
}

/// Built-in reference notation definitions
pub static REFERENCE_NOTATIONS: &[ReferenceNotation] = &[ReferenceNotation {
    name: "Highlight",
    symbol: HIGHLIGHT_REF, // ❧
    description: "Reference to a highlight by ID",
}];

/// Find reference notation by symbol
pub fn find_reference_notation(symbol: char) -> Option<&'static ReferenceNotation> {
    REFERENCE_NOTATIONS.iter().find(|n| n.symbol == symbol)
}

/// A parsed reference with symbol and IDs
///
/// # Examples
///
/// - `❧⟦12345⟧` - single highlight reference
/// - `❧⟦123∧456∧789⟧` - multiple highlight references
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// Symbol prefix (e.g., ❧ for highlights)
    pub symbol: char,
    /// Referenced IDs (supports multiple via ∧)
    pub ids: Vec<String>,
}

impl Notation for Reference {
    const OPEN: char = REF_OPEN;
    const CLOSE: char = REF_CLOSE;

    fn parse(input: &str) -> Option<Self> {
        let mut chars = input.chars().peekable();

        // Get symbol prefix
        let symbol = chars.next()?;

        // Validate it's a known reference symbol
        find_reference_notation(symbol)?;

        // Expect opening bracket
        if chars.next() != Some(REF_OPEN) {
            return None;
        }

        // Collect content until closing bracket
        let mut content = String::new();
        for c in chars.by_ref() {
            if c == REF_CLOSE {
                break;
            }
            content.push(c);
        }

        if content.is_empty() {
            return None;
        }

        // Split by conjunction if present
        let ids: Vec<String> = split_conjunction(&content)
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if ids.is_empty() {
            None
        } else {
            Some(Self { symbol, ids })
        }
    }

    fn render(&self) -> String {
        let content = self.ids.join(&CONJUNCTION.to_string());
        format!("{}{}{}{}", self.symbol, REF_OPEN, content, REF_CLOSE)
    }
}

impl Reference {
    /// Parse a reference string
    ///
    /// This is a convenience method that delegates to the `Notation` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use eris::notation::Reference;
    ///
    /// let r = Reference::parse("❧⟦12345⟧").unwrap();
    /// assert_eq!(r.ids, vec!["12345"]);
    /// ```
    pub fn parse(input: &str) -> Option<Self> {
        <Self as Notation>::parse(input)
    }

    /// Render this reference back to a string
    ///
    /// This is a convenience method that delegates to the `Notation` trait.
    pub fn render(&self) -> String {
        <Self as Notation>::render(self)
    }

    /// Create a new reference with a single ID
    pub fn single(symbol: char, id: impl Into<String>) -> Self {
        Self {
            symbol,
            ids: vec![id.into()],
        }
    }

    /// Create a new reference with multiple IDs
    pub fn multiple(symbol: char, ids: Vec<String>) -> Self {
        Self { symbol, ids }
    }

    /// Create a highlight reference with a single ID
    pub fn highlight(id: i64) -> Self {
        Self::single(HIGHLIGHT_REF, id.to_string())
    }

    /// Create a highlight reference with multiple IDs
    pub fn highlights(ids: &[i64]) -> Self {
        Self::multiple(
            HIGHLIGHT_REF,
            ids.iter().map(|id| id.to_string()).collect(),
        )
    }

    /// Check if this is a highlight reference
    pub fn is_highlight(&self) -> bool {
        self.symbol == HIGHLIGHT_REF
    }

    /// Get the notation definition for this reference type
    pub fn notation(&self) -> Option<&'static ReferenceNotation> {
        find_reference_notation(self.symbol)
    }

    /// Get IDs as i64 (for highlight references)
    ///
    /// Returns None for any ID that can't be parsed as i64
    pub fn ids_as_i64(&self) -> Option<Vec<i64>> {
        self.ids
            .iter()
            .map(|s| s.parse::<i64>().ok())
            .collect()
    }

    /// Check if this reference contains a specific ID
    pub fn contains(&self, id: &str) -> bool {
        self.ids.iter().any(|i| i == id)
    }

    /// Check if this is a single reference (one ID)
    pub fn is_single(&self) -> bool {
        self.ids.len() == 1
    }

    /// Get the first/only ID
    pub fn first(&self) -> Option<&str> {
        self.ids.first().map(|s| s.as_str())
    }

    /// Number of referenced IDs
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Check if empty (should not happen with valid parse)
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

/// Compose a single reference
///
/// # Examples
///
/// ```
/// use eris::notation::reference::compose_reference;
///
/// assert_eq!(compose_reference('❧', "12345"), "❧⟦12345⟧");
/// ```
pub fn compose_reference(symbol: char, id: &str) -> String {
    format!("{}{}{}{}", symbol, REF_OPEN, id, REF_CLOSE)
}

/// Compose a reference with multiple IDs
///
/// # Examples
///
/// ```
/// use eris::notation::reference::compose_references;
///
/// assert_eq!(compose_references('❧', &["123", "456"]), "❧⟦123∧456⟧");
/// ```
pub fn compose_references(symbol: char, ids: &[&str]) -> String {
    let content = ids.join(&CONJUNCTION.to_string());
    format!("{}{}{}{}", symbol, REF_OPEN, content, REF_CLOSE)
}

/// Compose a highlight reference
///
/// # Examples
///
/// ```
/// use eris::notation::reference::compose_highlight_ref;
///
/// assert_eq!(compose_highlight_ref(12345), "❧⟦12345⟧");
/// ```
pub fn compose_highlight_ref(id: i64) -> String {
    compose_reference(HIGHLIGHT_REF, &id.to_string())
}

/// Compose multiple highlight references
///
/// # Examples
///
/// ```
/// use eris::notation::reference::compose_highlight_refs;
///
/// assert_eq!(compose_highlight_refs(&[123, 456, 789]), "❧⟦123∧456∧789⟧");
/// ```
pub fn compose_highlight_refs(ids: &[i64]) -> String {
    let id_strs: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
    let id_refs: Vec<&str> = id_strs.iter().map(|s| s.as_str()).collect();
    compose_references(HIGHLIGHT_REF, &id_refs)
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_highlight() {
        let r = Reference::parse("❧⟦12345⟧").unwrap();
        assert_eq!(r.symbol, '❧');
        assert_eq!(r.ids, vec!["12345"]);
        assert!(r.is_highlight());
        assert!(r.is_single());
    }

    #[test]
    fn test_parse_multiple_highlights() {
        let r = Reference::parse("❧⟦123∧456∧789⟧").unwrap();
        assert_eq!(r.symbol, '❧');
        assert_eq!(r.ids, vec!["123", "456", "789"]);
        assert!(r.is_highlight());
        assert!(!r.is_single());
        assert_eq!(r.len(), 3);
    }

    #[test]
    fn test_parse_with_spaces() {
        let r = Reference::parse("❧⟦123 ∧ 456⟧").unwrap();
        assert_eq!(r.ids, vec!["123", "456"]);
    }

    #[test]
    fn test_render_single() {
        let r = Reference::highlight(12345);
        assert_eq!(r.render(), "❧⟦12345⟧");
    }

    #[test]
    fn test_render_multiple() {
        let r = Reference::highlights(&[123, 456, 789]);
        assert_eq!(r.render(), "❧⟦123∧456∧789⟧");
    }

    #[test]
    fn test_ids_as_i64() {
        let r = Reference::parse("❧⟦123∧456⟧").unwrap();
        assert_eq!(r.ids_as_i64(), Some(vec![123, 456]));
    }

    #[test]
    fn test_invalid_empty() {
        assert!(Reference::parse("❧⟦⟧").is_none());
    }

    #[test]
    fn test_invalid_no_bracket() {
        assert!(Reference::parse("❧12345").is_none());
    }

    #[test]
    fn test_invalid_unknown_symbol() {
        assert!(Reference::parse("?⟦12345⟧").is_none());
    }

    #[test]
    fn test_contains() {
        let r = Reference::parse("❧⟦123∧456∧789⟧").unwrap();
        assert!(r.contains("456"));
        assert!(!r.contains("999"));
    }

    #[test]
    fn test_compose_reference() {
        assert_eq!(compose_reference('❧', "12345"), "❧⟦12345⟧");
    }

    #[test]
    fn test_compose_references() {
        assert_eq!(compose_references('❧', &["123", "456"]), "❧⟦123∧456⟧");
    }

    #[test]
    fn test_compose_highlight_ref() {
        assert_eq!(compose_highlight_ref(12345), "❧⟦12345⟧");
    }

    #[test]
    fn test_compose_highlight_refs() {
        assert_eq!(compose_highlight_refs(&[123, 456, 789]), "❧⟦123∧456∧789⟧");
    }
}
