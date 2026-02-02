//! ≡ ERIS χρόνος operator system
//! ≝ Operators for time, purpose, and teleological assessment
//!
//! Vector definitions are loaded from RON files in `defs/vectors/chronos/`.
//! Non-vector operators are loaded from RON files in `defs/chronos/`.
//!
//! Cross-System Relations:
//!   ω∘μ→complete temporal assessment
//!   ω ⊂ ⍜
//!   ι,α,ε ∈ ω
//!   θ,δ,π,ς ∈ μ
//!   λ,ϕ,γ ∉ω ∧ ∉μ (diagnostics)
//!
//!   ι⊰α⊰ε (precedence chain)
//!   γ⊰ε⊰ϕ (capacity enables execution enables flow)
//!   λ⊰θ (consistency enables horizon extension)
//!
//!   δ9∧ς9∧π9→brittleness
//!   δ9∧ς9→γ⤋
//!   γ1→𝁆
//!   λ1→Ց9
//!   ϕ1→𝁆
//!   ϕ9→𝀷
//!
//!   ε⊧𝀾, λ⊧𝀏, α→✱
//!   λ⁻¹∝Ց, δ⁻¹∝ε, δ⁻¹∝ϕ

use super::loader::{load_chronos_operators, load_vectors, RonOperatorDef};
use crate::entities::types::EntityTypeDef;

/// Chronos vector categories (from vectors/chronos/)
const CHRONOS_VECTOR_CATEGORIES: &[&str] = &["Teleological", "Temporal", "Assessment"];

/// Check if a category is a chronos vector category
fn is_chronos_vector_category(category: &str) -> bool {
    CHRONOS_VECTOR_CATEGORIES.contains(&category)
}

/// Get chronos vectors from the unified vector loader
fn get_chronos_vectors() -> Vec<RonOperatorDef> {
    load_vectors()
        .iter()
        .filter(|op| is_chronos_vector_category(&op.category))
        .cloned()
        .collect()
}

/// Get all χρόνος operator definitions (vectors + operators)
pub fn get_chronos_operator_definitions() -> Vec<RonOperatorDef> {
    let mut defs = Vec::new();

    // Add vectors first (ω, ι, α, ε, μ, θ, δ, π, ς, λ, ϕ, γ)
    defs.extend(get_chronos_vectors());

    // Add non-vector operators (⍜, □, ◇, ≺, ≻, ⟟, 𝄃, ⬡, ⏣, 𝀺, 𝀖, ⊱, ⟲, ⟖, 𝄎, 𝆌, 𝁤, 𝀃, 𝀷)
    defs.extend(load_chronos_operators().clone());

    defs
}

/// Get a specific chronos operator by symbol
pub fn get_chronos_operator(symbol: &str) -> Option<RonOperatorDef> {
    get_chronos_operator_definitions()
        .into_iter()
        .find(|op| op.symbol == symbol)
}

/// Get entity type definitions from chronos operators
/// Returns entity types for operators that also function as entities (e.g., ⍜ Purpose)
pub fn get_entity_type_definitions() -> Vec<EntityTypeDef> {
    get_chronos_operator_definitions()
        .into_iter()
        .filter_map(|op| op.entity_type())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chronos_operator_definitions() {
        let defs = get_chronos_operator_definitions();
        // 12 vectors + 19 operators = 31 total
        assert_eq!(defs.len(), 31, "Expected 31 chronos definitions (12 vectors + 19 operators)");
    }

    #[test]
    fn test_get_chronos_vectors() {
        let vectors = get_chronos_vectors();
        assert_eq!(vectors.len(), 12, "Expected 12 chronos vectors");

        // Check key vectors exist
        let symbols: Vec<&str> = vectors.iter().map(|v| v.symbol.as_str()).collect();
        assert!(symbols.contains(&"ω"), "Missing teleological_vector (ω)");
        assert!(symbols.contains(&"μ"), "Missing physics_vector (μ)");
        assert!(symbols.contains(&"λ"), "Missing consistency (λ)");
    }

    #[test]
    fn test_get_chronos_operator() {
        // Test prehension operator (𝀃)
        let prehension = get_chronos_operator("𝀃").expect("𝀃 should exist");
        assert_eq!(prehension.name, "prehension");
        assert_eq!(prehension.category, "Becoming");

        // Test teleological anchor (⍜)
        let purpose = get_chronos_operator("⍜").expect("⍜ should exist");
        assert_eq!(purpose.name, "teleological_anchor");
        assert!(purpose.entity_type().is_some());

        // Test non-existent
        assert!(get_chronos_operator("X").is_none());
    }

    #[test]
    fn test_get_entity_type_definitions() {
        let entities = get_entity_type_definitions();
        assert_eq!(entities.len(), 1, "Expected 1 entity type (⍜ Purpose)");
        assert_eq!(entities[0].name, "Purpose");
        assert_eq!(entities[0].symbol, "⍜");
    }

    #[test]
    fn test_to_eris_text() {
        let prehension = get_chronos_operator("𝀃").expect("𝀃 should exist");
        let text = prehension.to_eris_text();
        assert!(text.starts_with("𝀃 ≡"));
        assert!(text.contains("prehension"));
    }
}
