//! Vector notation parsing for ERIS Armenian property vectors
//!
//! Property vectors use `⟨⟩` brackets and encode ratings using Armenian letters
//! followed by single-digit values (0-9).
//!
//! # Format
//!
//! `⊡⟨Փ9Գ8Վ7⟩` - property vector with three ratings
//!
//! Each property is an Armenian letter + digit:
//! - `Փ9` - high performativity
//! - `Գ8` - high generalizability
//! - `Վ7` - good validity

use super::Notation;
use crate::symbols::{VECTOR_CLOSE, VECTOR_OPEN};

/// A single property rating (Armenian letter + value 0-9)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Property {
    /// Armenian letter symbol
    pub symbol: char,
    /// Value from 0-9
    pub value: u8,
}

impl Property {
    /// Create a new property if value is valid (0-9)
    pub fn new(symbol: char, value: u8) -> Option<Self> {
        if value <= 9 {
            Some(Self { symbol, value })
        } else {
            None
        }
    }

    /// Render as string (e.g., "Փ9")
    pub fn render(&self) -> String {
        format!("{}{}", self.symbol, self.value)
    }
}

/// Vector notation definition
#[derive(Debug, Clone)]
pub struct VectorNotation {
    /// Name of this vector type
    pub name: &'static str,
    /// Prefix symbol (typically ⊡)
    pub prefix: char,
    /// Valid property symbols for this vector type
    pub valid_symbols: &'static [char],
}

impl VectorNotation {
    /// Check if a symbol is valid for this vector type
    pub fn is_valid_symbol(&self, symbol: char) -> bool {
        self.valid_symbols.contains(&symbol)
    }
}

/// Built-in vector notation definitions
pub static VECTOR_NOTATIONS: &[VectorNotation] = &[VectorNotation {
    name: "PropertyVector",
    prefix: '⊡',
    // Armenian letters used in property vectors (from armenian.rs)
    valid_symbols: &[
        '\u{0531}', // Ա antisymmetric
        '\u{0532}', // Բ boundary
        '\u{0533}', // Գ generalizability
        '\u{0534}', // Դ directionality
        '\u{0535}', // Ե explanatory
        '\u{0537}', // Է specificity
        '\u{0538}', // Ը intentionality
        '\u{053C}', // Լ longevity
        '\u{0540}', // Հ coherence
        '\u{0542}', // Ղ complexity
        '\u{0544}', // Մ modularity
        '\u{0546}', // Ն normativity
        '\u{0547}', // Շ surprise
        '\u{054A}', // Պ plasticity
        '\u{0550}', // Ր recursion
        '\u{054D}', // Ս symmetry
        '\u{054F}', // Տ transitivity
        '\u{0551}', // Ց destructive
        '\u{0553}', // Փ performativity
        '\u{0554}', // Ք quotability
        '\u{054E}', // Վ validity
        '\u{056D}', // խ contextualization (lowercase)
    ],
}];

/// A parsed Armenian property vector
///
/// # Examples
///
/// - `⊡⟨Փ9Գ8Վ7⟩` - three property ratings
/// - `⊡⟨Բ9Փ8⟩` - two property ratings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyVector {
    /// Prefix symbol (usually ⊡)
    pub prefix: char,
    /// List of property ratings
    pub properties: Vec<Property>,
}

impl Notation for PropertyVector {
    const OPEN: char = VECTOR_OPEN;
    const CLOSE: char = VECTOR_CLOSE;

    fn parse(input: &str) -> Option<Self> {
        let mut chars = input.chars().peekable();

        // Get prefix symbol
        let prefix = chars.next()?;
        if prefix != '⊡' {
            return None; // Currently only support ⊡ prefix
        }

        // Expect opening bracket
        if chars.next() != Some(VECTOR_OPEN) {
            return None;
        }

        // Parse property pairs until closing bracket
        let mut properties = Vec::new();
        while let Some(c) = chars.next() {
            if c == VECTOR_CLOSE {
                break;
            }

            // c is the symbol, next should be the digit
            if let Some(digit_char) = chars.next() {
                if let Some(digit) = digit_char.to_digit(10) {
                    if let Some(prop) = Property::new(c, digit as u8) {
                        properties.push(prop);
                    } else {
                        return None; // Invalid value
                    }
                } else {
                    return None; // Not a digit
                }
            } else {
                return None; // Missing digit
            }
        }

        if properties.is_empty() {
            None
        } else {
            Some(Self { prefix, properties })
        }
    }

    fn render(&self) -> String {
        let props: String = self.properties.iter().map(|p| p.render()).collect();
        format!("{}{}{}{}", self.prefix, VECTOR_OPEN, props, VECTOR_CLOSE)
    }
}

impl PropertyVector {
    /// Parse a property vector string
    ///
    /// This is a convenience method that delegates to the `Notation` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use eris::notation::PropertyVector;
    ///
    /// let v = PropertyVector::parse("⊡⟨Փ9Գ8Վ7⟩").unwrap();
    /// assert_eq!(v.get('Փ'), Some(9));
    /// ```
    pub fn parse(input: &str) -> Option<Self> {
        <Self as Notation>::parse(input)
    }

    /// Render this vector back to a string
    ///
    /// This is a convenience method that delegates to the `Notation` trait.
    pub fn render(&self) -> String {
        <Self as Notation>::render(self)
    }

    /// Create a new property vector with the given prefix and properties
    pub fn new(prefix: char, properties: Vec<Property>) -> Self {
        Self { prefix, properties }
    }

    /// Get a property value by symbol
    pub fn get(&self, symbol: char) -> Option<u8> {
        self.properties
            .iter()
            .find(|p| p.symbol == symbol)
            .map(|p| p.value)
    }

    /// Check if vector contains a property with the given symbol
    pub fn has(&self, symbol: char) -> bool {
        self.properties.iter().any(|p| p.symbol == symbol)
    }

    /// Get all symbols in this vector
    pub fn symbols(&self) -> Vec<char> {
        self.properties.iter().map(|p| p.symbol).collect()
    }

    /// Calculate average value across all properties
    pub fn average(&self) -> f64 {
        if self.properties.is_empty() {
            0.0
        } else {
            let sum: u32 = self.properties.iter().map(|p| p.value as u32).sum();
            sum as f64 / self.properties.len() as f64
        }
    }
}

/// Compose a property vector from prefix and property pairs
///
/// # Examples
///
/// ```
/// use eris::notation::vector::compose_vector;
///
/// let v = compose_vector('⊡', &[('Փ', 9), ('Գ', 8)]).unwrap();
/// assert_eq!(v, "⊡⟨Փ9Գ8⟩");
/// ```
pub fn compose_vector(prefix: char, properties: &[(char, u8)]) -> Option<String> {
    if properties.is_empty() {
        return None;
    }

    // Validate all values are 0-9
    if properties.iter().any(|(_, v)| *v > 9) {
        return None;
    }

    let props: String = properties
        .iter()
        .map(|(s, v)| format!("{}{}", s, v))
        .collect();

    Some(format!(
        "{}{}{}{}",
        prefix, VECTOR_OPEN, props, VECTOR_CLOSE
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vector() {
        let v = PropertyVector::parse("⊡⟨Փ9Գ8Վ7⟩").unwrap();
        assert_eq!(v.prefix, '⊡');
        assert_eq!(v.properties.len(), 3);
        assert_eq!(v.get('Փ'), Some(9));
        assert_eq!(v.get('Գ'), Some(8));
        assert_eq!(v.get('Վ'), Some(7));
    }

    #[test]
    fn test_parse_two_props() {
        let v = PropertyVector::parse("⊡⟨Բ9Փ8⟩").unwrap();
        assert_eq!(v.properties.len(), 2);
        assert_eq!(v.get('Բ'), Some(9));
        assert_eq!(v.get('Փ'), Some(8));
    }

    #[test]
    fn test_render() {
        let v = PropertyVector::parse("⊡⟨Փ9Գ8Վ7⟩").unwrap();
        assert_eq!(v.render(), "⊡⟨Փ9Գ8Վ7⟩");
    }

    #[test]
    fn test_average() {
        let v = PropertyVector::parse("⊡⟨Փ9Գ6Վ3⟩").unwrap();
        assert_eq!(v.average(), 6.0);
    }

    #[test]
    fn test_invalid_no_bracket() {
        assert!(PropertyVector::parse("⊡Փ9Գ8").is_none());
    }

    #[test]
    fn test_invalid_no_prefix() {
        assert!(PropertyVector::parse("⟨Փ9Գ8⟩").is_none());
    }

    #[test]
    fn test_invalid_empty() {
        assert!(PropertyVector::parse("⊡⟨⟩").is_none());
    }

    #[test]
    fn test_compose_vector() {
        let v = compose_vector('⊡', &[('Փ', 9), ('Գ', 8)]).unwrap();
        assert_eq!(v, "⊡⟨Փ9Գ8⟩");
    }

    #[test]
    fn test_compose_invalid_value() {
        assert!(compose_vector('⊡', &[('Փ', 10)]).is_none());
    }

    #[test]
    fn test_compose_empty() {
        assert!(compose_vector('⊡', &[]).is_none());
    }

    #[test]
    fn test_has_symbol() {
        let v = PropertyVector::parse("⊡⟨Փ9Գ8⟩").unwrap();
        assert!(v.has('Փ'));
        assert!(v.has('Գ'));
        assert!(!v.has('Վ'));
    }

    #[test]
    fn test_symbols() {
        let v = PropertyVector::parse("⊡⟨Փ9Գ8Վ7⟩").unwrap();
        assert_eq!(v.symbols(), vec!['Փ', 'Գ', 'Վ']);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Additional Parsing Tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_parse_single_property() {
        let v = PropertyVector::parse("⊡⟨Փ5⟩").unwrap();
        assert_eq!(v.properties.len(), 1);
        assert_eq!(v.get('Փ'), Some(5));
    }

    #[test]
    fn test_parse_all_digit_values() {
        // Test all valid digit values 0-9
        for digit in 0..=9 {
            let input = format!("⊡⟨Փ{}⟩", digit);
            let v = PropertyVector::parse(&input)
                .unwrap_or_else(|| panic!("Failed to parse: {}", input));
            assert_eq!(v.get('Փ'), Some(digit), "Failed for digit {}", digit);
        }
    }

    #[test]
    fn test_parse_many_properties() {
        let v = PropertyVector::parse("⊡⟨Ա1Բ2Գ3Դ4Է5⟩").unwrap();
        assert_eq!(v.properties.len(), 5);
        assert_eq!(v.get('\u{0531}'), Some(1)); // Ա antisymmetric
        assert_eq!(v.get('\u{0532}'), Some(2)); // Բ boundary
        assert_eq!(v.get('\u{0533}'), Some(3)); // Գ generalizability
        assert_eq!(v.get('\u{0534}'), Some(4)); // Դ directionality
        assert_eq!(v.get('\u{0537}'), Some(5)); // Է specificity
    }

    #[test]
    fn test_parse_lowercase_armenian() {
        // Test lowercase Armenian letter (խ = U+056D contextualization)
        let v = PropertyVector::parse("⊡⟨խ7⟩").unwrap();
        assert_eq!(v.get('խ'), Some(7));
    }

    #[test]
    fn test_parse_zero_values() {
        let v = PropertyVector::parse("⊡⟨Փ0Գ0Վ0⟩").unwrap();
        assert_eq!(v.get('Փ'), Some(0));
        assert_eq!(v.get('Գ'), Some(0));
        assert_eq!(v.get('Վ'), Some(0));
        assert_eq!(v.average(), 0.0);
    }

    #[test]
    fn test_parse_max_values() {
        let v = PropertyVector::parse("⊡⟨Փ9Գ9Վ9⟩").unwrap();
        assert_eq!(v.get('Փ'), Some(9));
        assert_eq!(v.get('Գ'), Some(9));
        assert_eq!(v.get('Վ'), Some(9));
        assert_eq!(v.average(), 9.0);
    }

    #[test]
    fn test_invalid_wrong_prefix() {
        // Wrong prefix symbol
        assert!(PropertyVector::parse("∎⟨Փ9⟩").is_none());
        assert!(PropertyVector::parse("X⟨Փ9⟩").is_none());
    }

    #[test]
    fn test_invalid_wrong_brackets() {
        // Wrong bracket types
        assert!(PropertyVector::parse("⊡⦑Փ9⦒").is_none()); // Entity brackets
        assert!(PropertyVector::parse("⊡⟦Փ9⟧").is_none()); // Reference brackets
        assert!(PropertyVector::parse("⊡(Փ9)").is_none()); // Parentheses
    }

    #[test]
    fn test_invalid_non_digit() {
        // Letter instead of digit
        assert!(PropertyVector::parse("⊡⟨Փa⟩").is_none());
        assert!(PropertyVector::parse("⊡⟨ΦX⟩").is_none());
    }

    #[test]
    fn test_invalid_missing_digit() {
        // Symbol without digit
        assert!(PropertyVector::parse("⊡⟨Φ⟩").is_none());
    }

    #[test]
    fn test_render_roundtrip() {
        let inputs = vec![
            "⊡⟨Փ9⟩",
            "⊡⟨Փ9Գ8⟩",
            "⊡⟨Փ9Գ8Վ7⟩",
            "⊡⟨Ա0Բ5Գ9⟩",
        ];

        for input in inputs {
            let v = PropertyVector::parse(input).unwrap();
            assert_eq!(v.render(), input, "Roundtrip failed for: {}", input);
        }
    }

    #[test]
    fn test_property_struct() {
        // Test Property creation with Armenian letter Փ (U+0553 performativity)
        let p = Property::new('\u{0553}', 9).unwrap();
        assert_eq!(p.symbol, '\u{0553}');
        assert_eq!(p.value, 9);
        assert_eq!(p.render(), "Փ9");

        // Invalid value (> 9)
        assert!(Property::new('\u{0553}', 10).is_none());
        assert!(Property::new('\u{0553}', 255).is_none());
    }

    #[test]
    fn test_average_precision() {
        // Փ=1 (U+0553), Գ=2 (U+0533) -> average 1.5
        let v = PropertyVector::parse("⊡⟨Փ1Գ2⟩").unwrap();
        assert_eq!(v.average(), 1.5);

        // Ա=1, Բ=2, Գ=3 -> average 2.0
        let v2 = PropertyVector::parse("⊡⟨Ա1Բ2Գ3⟩").unwrap();
        assert_eq!(v2.average(), 2.0);
    }

    #[test]
    fn test_compose_roundtrip() {
        // Use Armenian letters: Փ (U+0553), Գ (U+0533), Բ (U+0532)
        let props = &[('\u{0553}', 9), ('\u{0533}', 8), ('\u{0532}', 7)];
        let composed = compose_vector('⊡', props).unwrap();
        let parsed = PropertyVector::parse(&composed).unwrap();

        assert_eq!(parsed.get('\u{0553}'), Some(9));
        assert_eq!(parsed.get('\u{0533}'), Some(8));
        assert_eq!(parsed.get('\u{0532}'), Some(7));
    }
}
