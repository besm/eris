//! ERIS Armenian property assessment system
//!
//! Vector property definitions are loaded from RON files in `defs/vectors/`.

use super::loader::{load_vectors, RonOperatorDef};

/// Property value type (0-9 scale)
pub type PropertyValue = u8;

/// Validate that a property value is in the valid range (0-9)
pub fn validate_property_value(value: PropertyValue) -> bool {
    value <= 9
}

/// Get all Armenian operator definitions
pub fn get_armenian_operator_definitions() -> Vec<RonOperatorDef> {
    load_vectors().clone()
}

/// Get a specific Armenian operator by symbol
pub fn get_armenian_operator(symbol: &str) -> Option<RonOperatorDef> {
    load_vectors()
        .iter()
        .find(|op| op.symbol == symbol)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_property_value() {
        assert!(validate_property_value(0));
        assert!(validate_property_value(5));
        assert!(validate_property_value(9));
        assert!(!validate_property_value(10));
        assert!(!validate_property_value(255));
    }

    #[test]
    fn test_get_armenian_operator_definitions() {
        let defs = get_armenian_operator_definitions();
        assert_eq!(defs.len(), 37, "Expected 37 Armenian operator definitions");
    }

    #[test]
    fn test_get_armenian_operator() {
        // Test functional operator (Փ)
        let functional = get_armenian_operator("Փ").expect("Փ should exist");
        assert_eq!(functional.name, "functional");
        assert_eq!(functional.category, "Core");

        // Test generalizability operator (Գ)
        let gen = get_armenian_operator("Գ").expect("Գ should exist");
        assert_eq!(gen.name, "generalizability");

        // Test non-existent
        assert!(get_armenian_operator("X").is_none());
    }

    #[test]
    fn test_to_eris_text() {
        let functional = get_armenian_operator("Փ").expect("Փ should exist");
        let text = functional.to_eris_text();
        assert!(text.starts_with("Փ ⊡"));
        assert!(text.contains("functional"));
    }
}
