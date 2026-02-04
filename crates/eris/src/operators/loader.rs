//! RON operator definition loader
//!
//! Loads operator definitions from RON files at compile time.
//!
//! Supports two formats:
//! - Legacy: `lines: [("≡", "value"), ...]` tuple array
//! - Structured: semantic field names
//!
//! Two loader types:
//! - `load_vectors()`: Property vectors + chronos vectors (39 total)
//! - `load_chronos_operators()`: Non-vector chronos operators (19 total)
//!
//! Property vectors use Tifinagh script (U+2D30–U+2D7F), with legacy
//! Armenian symbols tracked via `supersedes` field for backwards compatibility.

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::entities::types::EntityTypeDef;

/// Entity type definition for RON deserialization
#[derive(Debug, Clone, Deserialize)]
pub struct RonEntityTypeDef {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub sort_order: i32,
}

impl From<RonEntityTypeDef> for EntityTypeDef {
    fn from(ron: RonEntityTypeDef) -> Self {
        EntityTypeDef {
            symbol: ron.symbol,
            name: ron.name,
            description: ron.description,
            sort_order: ron.sort_order,
        }
    }
}

/// Operator definition loaded from RON file
#[derive(Debug, Clone, Deserialize)]
pub struct RonOperatorDef {
    pub symbol: String,
    pub name: String,
    pub category: String,

    /// Optional: if this operator also functions as an entity type
    #[serde(default)]
    pub entity_type: Option<RonEntityTypeDef>,

    /// Optional: symbol this definition supersedes (for migrations)
    #[serde(default)]
    pub supersedes: Option<String>,

    /// Legacy format: explicit Vec of (prefix, content) tuples
    #[serde(default)]
    pub lines: Vec<(String, String)>,

    /// Structured format fields (maps to ERIS prefix symbols)
    #[serde(default)]
    pub equivalence: Vec<String>,
    #[serde(default)]
    pub definition: Vec<String>,
    #[serde(default)]
    pub vector: Vec<String>,
    #[serde(default)]
    pub examples: Vec<String>,
    #[serde(default)]
    pub validation: Vec<String>,
}

impl RonOperatorDef {
    /// Get definition lines as tuples, supporting both legacy and structured formats.
    pub fn lines(&self) -> Vec<(String, String)> {
        if !self.lines.is_empty() {
            return self.lines.clone();
        }

        let mut result = Vec::new();
        for v in &self.equivalence {
            result.push(("≡".to_string(), v.clone()));
        }
        for v in &self.vector {
            result.push(("⊡".to_string(), v.clone()));
        }
        for v in &self.definition {
            result.push(("≝".to_string(), v.clone()));
        }
        for v in &self.examples {
            result.push(("⊛".to_string(), v.clone()));
        }
        for v in &self.validation {
            result.push(("⊨".to_string(), v.clone()));
        }
        result
    }

    /// Render operator definition as formatted ERIS text
    pub fn to_eris_text(&self) -> String {
        let symbol = &self.symbol;
        let symbol_width = symbol.chars().count();
        let lines = self.lines();

        if lines.is_empty() {
            return format!("{} {}", symbol, self.name);
        }

        let mut result = format!(
            "{} {} {}",
            symbol, lines[0].0, lines[0].1
        );

        // Indent subsequent lines to align with first line's prefix
        let indent = " ".repeat(symbol_width + 1);
        let mut prev_prefix = &lines[0].0;

        for line in &lines[1..] {
            let prefix_display = if &line.0 == prev_prefix {
                " ".repeat(line.0.chars().count())
            } else {
                line.0.clone()
            };
            result.push_str(&format!("\n{}{} {}", indent, prefix_display, line.1));
            prev_prefix = &line.0;
        }

        result
    }

    /// Get entity type if this operator also functions as an entity
    pub fn entity_type(&self) -> Option<EntityTypeDef> {
        self.entity_type.clone().map(|e| e.into())
    }
}

/// Parse a RON operator definition
fn parse_operator(ron_str: &str) -> RonOperatorDef {
    ron::from_str(ron_str).expect("Failed to parse RON operator definition")
}

/// Load all vector property definitions from embedded RON files
fn load_vectors_inner() -> Vec<RonOperatorDef> {
    vec![
        // Persistence (4)
        parse_operator(include_str!("../../defs/vectors/persistence/volatility.ron")),
        parse_operator(include_str!("../../defs/vectors/persistence/lifespan.ron")),
        parse_operator(include_str!("../../defs/vectors/persistence/component.ron")),
        parse_operator(include_str!("../../defs/vectors/persistence/symbolic.ron")),

        // Organization (6)
        parse_operator(include_str!("../../defs/vectors/organization/boundary.ron")),
        parse_operator(include_str!("../../defs/vectors/organization/cohesion.ron")),
        parse_operator(include_str!("../../defs/vectors/organization/formalization.ron")),
        parse_operator(include_str!("../../defs/vectors/organization/hierarchy.ron")),
        parse_operator(include_str!("../../defs/vectors/organization/naming.ron")),
        parse_operator(include_str!("../../defs/vectors/organization/coherence.ron")),

        // Relational (7)
        parse_operator(include_str!("../../defs/vectors/relational/antisymmetric.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/transitive.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/symmetric.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/reflexive.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/total.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/porosity.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/recursion.ron")),

        // Capacity (3)
        parse_operator(include_str!("../../defs/vectors/capacity/functional.ron")),
        parse_operator(include_str!("../../defs/vectors/capacity/agency.ron")),
        parse_operator(include_str!("../../defs/vectors/capacity/intentionality.ron")),

        // Extension (3)
        parse_operator(include_str!("../../defs/vectors/extension/scope.ron")),
        parse_operator(include_str!("../../defs/vectors/extension/generalizability.ron")),
        parse_operator(include_str!("../../defs/vectors/extension/contextualization.ron")),

        // Expression (4)
        parse_operator(include_str!("../../defs/vectors/expression/semantic_density.ron")),
        parse_operator(include_str!("../../defs/vectors/expression/explanatory.ron")),
        parse_operator(include_str!("../../defs/vectors/expression/symbolic_economy.ron")),
        parse_operator(include_str!("../../defs/vectors/expression/reference_efficiency.ron")),

        // Standalone
        parse_operator(include_str!("../../defs/vectors/property_vector.ron")),

        // Chronos vectors - Teleological
        parse_operator(include_str!("../../defs/vectors/chronos/teleological_vector.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/intentionality_tele.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/alignment.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/execution.ron")),

        // Chronos vectors - Temporal (Physics)
        parse_operator(include_str!("../../defs/vectors/chronos/physics_vector.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/horizon.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/density.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/precision.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/sync.ron")),

        // Chronos vectors - Assessment
        parse_operator(include_str!("../../defs/vectors/chronos/consistency.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/flux.ron")),
        parse_operator(include_str!("../../defs/vectors/chronos/capacity.ron")),
    ]
}

/// Cached vector property definitions
static VECTORS: Lazy<Vec<RonOperatorDef>> = Lazy::new(load_vectors_inner);

/// Get all loaded vector property definitions
pub fn load_vectors() -> &'static Vec<RonOperatorDef> {
    &VECTORS
}

// ============================================================================
// Property Vector Utilities
// ============================================================================

/// Property vector categories (excludes chronos vectors)
const PROPERTY_CATEGORIES: &[&str] = &[
    "Persistence",
    "Organization",
    "Relational",
    "Capacity",
    "Extension",
    "Expression",
];

/// Property value type (0-9 scale)
pub type PropertyValue = u8;

/// Validate that a property value is in the valid range (0-9)
pub fn validate_property_value(value: PropertyValue) -> bool {
    value <= 9
}

/// Check if a category is a property vector category
fn is_property_category(category: &str) -> bool {
    PROPERTY_CATEGORIES.contains(&category)
}

/// Get all property vector definitions (filtered from unified vector loader)
pub fn get_property_vector_definitions() -> Vec<RonOperatorDef> {
    load_vectors()
        .iter()
        .filter(|op| is_property_category(&op.category))
        .cloned()
        .collect()
}

/// Get a specific property vector by symbol (checks both new and superseded symbols)
pub fn get_property_vector(symbol: &str) -> Option<RonOperatorDef> {
    get_property_vector_definitions()
        .into_iter()
        .find(|op| op.symbol == symbol || op.supersedes.as_deref() == Some(symbol))
}

// ============================================================================
// Chronos Operator Loader (non-vector operators)
// ============================================================================

/// Load all chronos operator definitions from embedded RON files
fn load_chronos_operators_inner() -> Vec<RonOperatorDef> {
    vec![
        // Teleological (with entity_type)
        parse_operator(include_str!("../../defs/chronos/teleological_anchor.ron")),

        // Temporal
        parse_operator(include_str!("../../defs/chronos/always.ron")),
        parse_operator(include_str!("../../defs/chronos/eventually.ron")),
        parse_operator(include_str!("../../defs/chronos/precedes.ron")),
        parse_operator(include_str!("../../defs/chronos/succeeds.ron")),
        parse_operator(include_str!("../../defs/chronos/prediction.ron")),
        parse_operator(include_str!("../../defs/chronos/repeats.ron")),
        parse_operator(include_str!("../../defs/chronos/simultaneous.ron")),
        parse_operator(include_str!("../../defs/chronos/state.ron")),
        parse_operator(include_str!("../../defs/chronos/process.ron")),
        parse_operator(include_str!("../../defs/chronos/accelerates.ron")),

        // Causal
        parse_operator(include_str!("../../defs/chronos/follows_resulting.ron")),
        parse_operator(include_str!("../../defs/chronos/feedback_loop.ron")),
        parse_operator(include_str!("../../defs/chronos/recursive_process.ron")),
        parse_operator(include_str!("../../defs/chronos/reflexive_op.ron")),
        parse_operator(include_str!("../../defs/chronos/reinforces.ron")),
        parse_operator(include_str!("../../defs/chronos/subverts.ron")),

        // Becoming
        parse_operator(include_str!("../../defs/chronos/prehension.ron")),
        parse_operator(include_str!("../../defs/chronos/concrescence.ron")),
    ]
}

/// Cached chronos operator definitions
static CHRONOS_OPERATORS: Lazy<Vec<RonOperatorDef>> = Lazy::new(load_chronos_operators_inner);

/// Get all loaded chronos operator definitions
pub fn load_chronos_operators() -> &'static Vec<RonOperatorDef> {
    &CHRONOS_OPERATORS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_vectors() {
        let vectors = load_vectors();
        assert_eq!(vectors.len(), 40, "Expected 40 vector definitions (27 property + 12 chronos + 1 standalone)");
    }

    #[test]
    fn test_vector_has_required_fields() {
        let vectors = load_vectors();
        for v in vectors {
            assert!(!v.symbol.is_empty(), "Vector symbol should not be empty");
            assert!(!v.name.is_empty(), "Vector name should not be empty");
            assert!(!v.lines().is_empty(), "Vector lines should not be empty");
        }
    }

    #[test]
    fn test_structured_format() {
        let ron_str = r#"(
            symbol: "ⵗ",
            name: "boundary",
            category: "Organization",
            equivalence: ["boundary", "interface"],
            vector: ["0≡∅∂|5≡⊨∂|9≡⊩∂"],
            definition: ["separation clarity"],
        )"#;

        let op: RonOperatorDef = ron::from_str(ron_str).unwrap();
        let lines = op.lines();

        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0].0, "≡");
        assert_eq!(lines[2].0, "⊡");
    }

    #[test]
    fn test_load_chronos_operators() {
        let ops = load_chronos_operators();
        assert_eq!(ops.len(), 19, "Expected 19 chronos operator definitions");
    }

    #[test]
    fn test_chronos_operator_has_required_fields() {
        let ops = load_chronos_operators();
        for op in ops {
            assert!(!op.symbol.is_empty(), "Operator symbol should not be empty");
            assert!(!op.name.is_empty(), "Operator name should not be empty");
            assert!(!op.lines().is_empty(), "Operator lines should not be empty");
        }
    }

    #[test]
    fn test_chronos_entity_type() {
        let ops = load_chronos_operators();
        let with_entity = ops.iter().filter(|op| op.entity_type.is_some()).count();
        assert_eq!(with_entity, 1, "Expected 1 operator with entity_type (⍜)");

        let purpose = ops.iter().find(|op| op.symbol == "⍜").unwrap();
        let entity_type = purpose.entity_type().unwrap();
        assert_eq!(entity_type.name, "Purpose");
    }

    #[test]
    fn test_validate_property_value() {
        assert!(validate_property_value(0));
        assert!(validate_property_value(5));
        assert!(validate_property_value(9));
        assert!(!validate_property_value(10));
        assert!(!validate_property_value(255));
    }

    #[test]
    fn test_get_property_vector_definitions() {
        let defs = get_property_vector_definitions();
        assert_eq!(defs.len(), 27, "Expected 27 property vector definitions");
    }

    #[test]
    fn test_get_property_vector() {
        // Test by new Tifinagh symbol
        let functional = get_property_vector("ⵟ").expect("ⵟ should exist");
        assert_eq!(functional.name, "functional");
        assert_eq!(functional.category, "Capacity");

        // Test by old Armenian symbol (via supersedes)
        let functional_old = get_property_vector("Փ").expect("Փ should exist via supersedes");
        assert_eq!(functional_old.name, "functional");

        // Test non-existent
        assert!(get_property_vector("X").is_none());
    }
}
