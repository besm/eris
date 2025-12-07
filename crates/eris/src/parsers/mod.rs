//! ERIS tag parsing utilities
//!
//! Note: CompoundTag has moved to crate::notation::entity

pub use crate::entities::EntityType;
pub use crate::notation::entity::CompoundTag;
pub use crate::notation::entity::{compose_simple as compose_tag_new, extract_display_name, get_entity_type_name};

use super::symbols::{BRACKET_CLOSE, BRACKET_OPEN};

/// Compose an ERIS tag from symbol and content.
///
/// # Examples
///
/// ```
/// use eris::parsers::compose_tag;
///
/// assert_eq!(compose_tag("⧊", "Irony"), Some("⧊⦑Irony⦒".to_string()));
/// assert_eq!(compose_tag("⚘", "Mary Douglas"), Some("⚘⦑Mary Douglas⦒".to_string()));
/// assert_eq!(compose_tag("⧊", "  "), None); // Empty content
/// ```
pub fn compose_tag(symbol: &str, content: &str) -> Option<String> {
    let content = content.trim();
    if content.is_empty() {
        None
    } else {
        Some(format!("{}{}{}{}", symbol, BRACKET_OPEN, content, BRACKET_CLOSE))
    }
}

/// Parse an ERIS tag into its symbol and content.
///
/// Returns `Some((symbol, content))` for valid simple tags.
/// For compound tags with multiple components, use `CompoundTag::parse()`.
///
/// # Examples
///
/// ```
/// use eris::parsers::parse_tag;
///
/// assert_eq!(parse_tag("⧊⦑Irony⦒"), Some(("⧊", "Irony")));
/// assert_eq!(parse_tag("⚘⦑Mary Douglas⦒"), Some(("⚘", "Mary Douglas")));
/// assert_eq!(parse_tag("invalid"), None);
/// assert_eq!(parse_tag("⧊⦑⦒"), None); // Empty content
/// ```
pub fn parse_tag(tag: &str) -> Option<(&str, &str)> {
    let open_pos = tag.find(BRACKET_OPEN)?;
    let close_pos = tag.rfind(BRACKET_CLOSE)?;

    if close_pos <= open_pos + BRACKET_OPEN.len_utf8() {
        return None; // Empty or malformed content
    }

    let symbol = &tag[..open_pos];
    if symbol.is_empty() {
        return None;
    }

    let content_start = open_pos + BRACKET_OPEN.len_utf8();
    let content = &tag[content_start..close_pos];
    if content.is_empty() {
        return None;
    }

    Some((symbol, content))
}

/// Validation error for ERIS tags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagValidationError {
    /// Tag is empty
    Empty,
    /// Missing opening bracket ⦑
    MissingOpenBracket,
    /// Missing closing bracket ⦒
    MissingCloseBracket,
    /// No symbol before the opening bracket
    MissingSymbol,
    /// Content between brackets is empty
    EmptyContent,
    /// Symbol is not a recognized ERIS entity type
    UnknownSymbol(String),
}

impl std::fmt::Display for TagValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "tag is empty"),
            Self::MissingOpenBracket => write!(f, "missing opening bracket ⦑"),
            Self::MissingCloseBracket => write!(f, "missing closing bracket ⦒"),
            Self::MissingSymbol => write!(f, "no symbol before opening bracket"),
            Self::EmptyContent => write!(f, "content between brackets is empty"),
            Self::UnknownSymbol(s) => write!(f, "unknown symbol: {}", s),
        }
    }
}

impl std::error::Error for TagValidationError {}

/// Validate the structure of an ERIS tag.
///
/// Checks:
/// - Tag is non-empty
/// - Has opening and closing brackets
/// - Has a symbol prefix
/// - Has non-empty content
/// - Symbol is a recognized entity type (optional, controlled by `check_symbol`)
///
/// # Examples
///
/// ```
/// use eris::parsers::validate_tag_structure;
///
/// assert!(validate_tag_structure("⧊⦑Irony⦒", false).is_ok());
/// assert!(validate_tag_structure("⚘⦑Mary Douglas⦒", false).is_ok());
/// assert!(validate_tag_structure("invalid", false).is_err());
/// assert!(validate_tag_structure("⧊⦑⦒", false).is_err()); // Empty content
/// ```
pub fn validate_tag_structure(tag: &str, check_symbol: bool) -> Result<(), TagValidationError> {
    if tag.is_empty() {
        return Err(TagValidationError::Empty);
    }

    let open_pos = tag.find(BRACKET_OPEN).ok_or(TagValidationError::MissingOpenBracket)?;
    let close_pos = tag.rfind(BRACKET_CLOSE).ok_or(TagValidationError::MissingCloseBracket)?;

    let symbol = &tag[..open_pos];
    if symbol.is_empty() {
        return Err(TagValidationError::MissingSymbol);
    }

    let content_start = open_pos + BRACKET_OPEN.len_utf8();
    if close_pos <= content_start {
        return Err(TagValidationError::EmptyContent);
    }

    let content = &tag[content_start..close_pos];
    if content.trim().is_empty() {
        return Err(TagValidationError::EmptyContent);
    }

    if check_symbol {
        use crate::entities::types::get_entity_type_by_symbol;
        // Check first character of symbol (for compound tags like ⚘⊙⊳)
        let first_symbol = symbol.chars().next().map(|c| c.to_string()).unwrap_or_default();
        if get_entity_type_by_symbol(&first_symbol).is_none() {
            return Err(TagValidationError::UnknownSymbol(first_symbol));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag_simple() {
        assert_eq!(parse_tag("⧊⦑Irony⦒"), Some(("⧊", "Irony")));
        assert_eq!(parse_tag("⚘⦑Mary Douglas⦒"), Some(("⚘", "Mary Douglas")));
    }

    #[test]
    fn test_parse_tag_compound_symbol() {
        // Compound symbols work, returns full symbol prefix
        assert_eq!(parse_tag("⚘⊙⊳⦑Douglas⦒⦑1966⦒⦑Purity⦒"), Some(("⚘⊙⊳", "Douglas⦒⦑1966⦒⦑Purity")));
    }

    #[test]
    fn test_parse_tag_invalid() {
        assert_eq!(parse_tag("invalid"), None);
        assert_eq!(parse_tag("⧊⦑⦒"), None); // Empty content
        assert_eq!(parse_tag("⦑content⦒"), None); // No symbol
        assert_eq!(parse_tag(""), None);
    }

    #[test]
    fn test_validate_structure() {
        assert!(validate_tag_structure("⧊⦑Irony⦒", false).is_ok());
        assert!(validate_tag_structure("⚘⦑Name⦒", false).is_ok());

        assert!(matches!(
            validate_tag_structure("", false),
            Err(TagValidationError::Empty)
        ));
        assert!(matches!(
            validate_tag_structure("no brackets", false),
            Err(TagValidationError::MissingOpenBracket)
        ));
        assert!(matches!(
            validate_tag_structure("⧊⦑no close", false),
            Err(TagValidationError::MissingCloseBracket)
        ));
        assert!(matches!(
            validate_tag_structure("⦑no symbol⦒", false),
            Err(TagValidationError::MissingSymbol)
        ));
        assert!(matches!(
            validate_tag_structure("⧊⦑⦒", false),
            Err(TagValidationError::EmptyContent)
        ));
    }

    #[test]
    fn test_roundtrip() {
        let original = compose_tag("⧊", "Test Concept").unwrap();
        let (symbol, content) = parse_tag(&original).unwrap();
        assert_eq!(symbol, "⧊");
        assert_eq!(content, "Test Concept");
    }
}
