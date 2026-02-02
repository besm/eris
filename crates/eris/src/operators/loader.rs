//! RON operator definition loader
//!
//! Loads operator definitions from RON files at compile time.
//!
//! Supports two formats:
//! - Legacy: `lines: [("≡", "value"), ...]` tuple array
//! - Structured: semantic field names
//!
//! Two loader types:
//! - `load_vectors()`: Armenian + chronos property vectors (49 total)
//! - `load_chronos_operators()`: Non-vector chronos operators (19 total)

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
/// Order matches original armenian.rs for output equivalence
fn load_vectors_inner() -> Vec<RonOperatorDef> {
    vec![
        // Relational properties (antisymmetric moved here for grouping)
        parse_operator(include_str!("../../defs/vectors/relational/antisymmetric.ron")),
        parse_operator(include_str!("../../defs/vectors/boundary.ron")),
        parse_operator(include_str!("../../defs/vectors/generalizability.ron")),
        parse_operator(include_str!("../../defs/vectors/explanatory.ron")),
        parse_operator(include_str!("../../defs/vectors/intentionality.ron")),
        parse_operator(include_str!("../../defs/vectors/contextualization.ron")),
        parse_operator(include_str!("../../defs/vectors/stability/lifespan.ron")),
        parse_operator(include_str!("../../defs/vectors/functional.ron")),
        parse_operator(include_str!("../../defs/vectors/semantic_density.ron")),
        parse_operator(include_str!("../../defs/vectors/stability/volatility.ron")),
        // Relational properties (first batch)
        parse_operator(include_str!("../../defs/vectors/relational/transitive.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/symmetric.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/reflexive.ron")),
        parse_operator(include_str!("../../defs/vectors/relational/total.ron")),
        // Compression properties
        parse_operator(include_str!("../../defs/vectors/symbolic_economy.ron")),
        parse_operator(include_str!("../../defs/vectors/reference_efficiency.ron")),
        // SelfReference properties
        parse_operator(include_str!("../../defs/vectors/recursion.ron")),
        parse_operator(include_str!("../../defs/vectors/coherence.ron")),
        parse_operator(include_str!("../../defs/vectors/naming.ron")),
        parse_operator(include_str!("../../defs/vectors/metamodel.ron")),
        parse_operator(include_str!("../../defs/vectors/junction.ron")),
        // SystemCoherence properties (first batch)
        parse_operator(include_str!("../../defs/vectors/conceptual_orthogonality.ron")),
        parse_operator(include_str!("../../defs/vectors/knowledge_propagation.ron")),
        parse_operator(include_str!("../../defs/vectors/hierarchical_consistency.ron")),
        parse_operator(include_str!("../../defs/vectors/stability/symbolic.ron")),
        // Evolution properties
        parse_operator(include_str!("../../defs/vectors/quality_improvement.ron")),
        parse_operator(include_str!("../../defs/vectors/stability/component.ron")),
        // DesignBalance properties
        parse_operator(include_str!("../../defs/vectors/simplicity_expressiveness.ron")),
        parse_operator(include_str!("../../defs/vectors/orthogonality_integration.ron")),
        parse_operator(include_str!("../../defs/vectors/consistency_specialization.ron")),
        // SystemCoherence properties (second batch)
        parse_operator(include_str!("../../defs/vectors/formalization.ron")),
        // Relational properties (second batch)
        parse_operator(include_str!("../../defs/vectors/relational/porosity.ron")),
        // SystemCoherence properties (third batch)
        parse_operator(include_str!("../../defs/vectors/hierarchy.ron")),
        // Core properties (second batch)
        parse_operator(include_str!("../../defs/vectors/scope.ron")),
        // SystemCoherence properties (fourth batch)
        parse_operator(include_str!("../../defs/vectors/cohesion.ron")),
        // Core properties (third batch)
        parse_operator(include_str!("../../defs/vectors/agency.ron")),
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
        assert_eq!(vectors.len(), 49, "Expected 49 vector property definitions (37 armenian + 12 chronos)");
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
            symbol: "Բ",
            name: "boundary",
            category: "Core",
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
}
