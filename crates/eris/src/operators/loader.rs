//! RON operator definition loader
//!
//! Loads vector property definitions from RON files at compile time.

use once_cell::sync::Lazy;
use serde::Deserialize;

/// Operator definition loaded from RON file
#[derive(Debug, Clone, Deserialize)]
pub struct RonOperatorDef {
    pub symbol: String,
    pub name: String,
    pub category: String,
    pub lines: Vec<(String, String)>,
}

impl RonOperatorDef {
    /// Render operator definition as formatted ERIS text
    pub fn to_eris_text(&self) -> String {
        let symbol = &self.symbol;
        let symbol_width = symbol.chars().count();

        if self.lines.is_empty() {
            return format!("{} {}", symbol, self.name);
        }

        let mut result = format!(
            "{} {} {}",
            symbol, self.lines[0].0, self.lines[0].1
        );

        // Indent subsequent lines to align with first line's prefix
        let indent = " ".repeat(symbol_width + 1);
        let mut prev_prefix = &self.lines[0].0;

        for line in &self.lines[1..] {
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
}

/// Parse a RON operator definition
fn parse_operator(ron_str: &str) -> RonOperatorDef {
    ron::from_str(ron_str).expect("Failed to parse RON operator definition")
}

/// Load all vector property definitions from embedded RON files
/// Order matches original armenian.rs for output equivalence
fn load_vectors_inner() -> Vec<RonOperatorDef> {
    vec![
        // Core properties (first batch)
        parse_operator(include_str!("../../defs/vectors/antisymmetric.ron")),
        parse_operator(include_str!("../../defs/vectors/boundary.ron")),
        parse_operator(include_str!("../../defs/vectors/generalizability.ron")),
        parse_operator(include_str!("../../defs/vectors/explanatory.ron")),
        parse_operator(include_str!("../../defs/vectors/intentionality.ron")),
        parse_operator(include_str!("../../defs/vectors/contextualization.ron")),
        parse_operator(include_str!("../../defs/vectors/lifespan.ron")),
        parse_operator(include_str!("../../defs/vectors/functional.ron")),
        parse_operator(include_str!("../../defs/vectors/semantic_density.ron")),
        parse_operator(include_str!("../../defs/vectors/volatility.ron")),
        // Relational properties (first batch)
        parse_operator(include_str!("../../defs/vectors/transitive.ron")),
        parse_operator(include_str!("../../defs/vectors/symmetric.ron")),
        parse_operator(include_str!("../../defs/vectors/reflexive.ron")),
        parse_operator(include_str!("../../defs/vectors/total.ron")),
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
        parse_operator(include_str!("../../defs/vectors/symbolic_stability.ron")),
        // Evolution properties
        parse_operator(include_str!("../../defs/vectors/quality_improvement.ron")),
        parse_operator(include_str!("../../defs/vectors/component_stability.ron")),
        // DesignBalance properties
        parse_operator(include_str!("../../defs/vectors/simplicity_expressiveness.ron")),
        parse_operator(include_str!("../../defs/vectors/orthogonality_integration.ron")),
        parse_operator(include_str!("../../defs/vectors/consistency_specialization.ron")),
        // SystemCoherence properties (second batch)
        parse_operator(include_str!("../../defs/vectors/formalization.ron")),
        // Relational properties (second batch)
        parse_operator(include_str!("../../defs/vectors/porosity.ron")),
        // SystemCoherence properties (third batch)
        parse_operator(include_str!("../../defs/vectors/hierarchy.ron")),
        // Core properties (second batch)
        parse_operator(include_str!("../../defs/vectors/scope.ron")),
        // SystemCoherence properties (fourth batch)
        parse_operator(include_str!("../../defs/vectors/cohesion.ron")),
        // Core properties (third batch)
        parse_operator(include_str!("../../defs/vectors/agency.ron")),
        parse_operator(include_str!("../../defs/vectors/property_vector.ron")),
    ]
}

/// Cached vector property definitions
static VECTORS: Lazy<Vec<RonOperatorDef>> = Lazy::new(load_vectors_inner);

/// Get all loaded vector property definitions
pub fn load_vectors() -> &'static Vec<RonOperatorDef> {
    &VECTORS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_vectors() {
        let vectors = load_vectors();
        assert_eq!(vectors.len(), 37, "Expected 37 vector property definitions");
    }

    #[test]
    fn test_vector_has_required_fields() {
        let vectors = load_vectors();
        for v in vectors {
            assert!(!v.symbol.is_empty(), "Vector symbol should not be empty");
            assert!(!v.name.is_empty(), "Vector name should not be empty");
            assert!(!v.lines.is_empty(), "Vector lines should not be empty");
        }
    }
}
