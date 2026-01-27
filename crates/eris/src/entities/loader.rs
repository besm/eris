//! RON entity definition loader
//!
//! Loads entity definitions from RON files at compile time.

use once_cell::sync::Lazy;
use serde::Deserialize;

/// Entity definition loaded from RON file
#[derive(Debug, Clone, Deserialize)]
pub struct RonEntityDef {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub sort_order: i32,
    pub category: String,
    pub lines: Vec<(String, String)>,
}

impl RonEntityDef {
    /// Render entity definition as formatted ERIS text
    ///
    /// Replicates the output format from the original macro-based system.
    pub fn to_eris_text(&self) -> String {
        let symbol = &self.symbol;
        let symbol_width = symbol.chars().count();

        if self.lines.is_empty() {
            return format!("{} {}", symbol, self.name);
        }

        let mut result = format!(
            "{} {} {}",
            symbol,
            self.lines[0].0,
            self.lines[0].1
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

    /// Convert to simplified EntityTypeDef for tag composer
    pub fn to_entity_type_def(&self) -> super::types::EntityTypeDef {
        super::types::EntityTypeDef {
            symbol: self.symbol.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            sort_order: self.sort_order,
        }
    }
}

/// Parse a RON entity definition
fn parse_entity(ron_str: &str) -> RonEntityDef {
    ron::from_str(ron_str).expect("Failed to parse RON entity definition")
}

/// Load all entity definitions from embedded RON files
fn load_all_entities_inner() -> Vec<RonEntityDef> {
    let mut entities = vec![
        // Primary entities
        parse_entity(include_str!("../../defs/entities/person.ron")),
        parse_entity(include_str!("../../defs/entities/place.ron")),
        parse_entity(include_str!("../../defs/entities/era.ron")),
        parse_entity(include_str!("../../defs/entities/date.ron")),
        parse_entity(include_str!("../../defs/entities/event.ron")),
        parse_entity(include_str!("../../defs/entities/field.ron")),
        parse_entity(include_str!("../../defs/entities/group.ron")),
        parse_entity(include_str!("../../defs/entities/organization.ron")),
        parse_entity(include_str!("../../defs/entities/agency.ron")),
        parse_entity(include_str!("../../defs/entities/party.ron")),
        parse_entity(include_str!("../../defs/entities/tech.ron")),
        parse_entity(include_str!("../../defs/entities/identifier.ron")),
        parse_entity(include_str!("../../defs/entities/publisher.ron")),
        parse_entity(include_str!("../../defs/entities/university.ron")),
        parse_entity(include_str!("../../defs/entities/language.ron")),
        // Conceptual entities
        parse_entity(include_str!("../../defs/entities/concept.ron")),
        parse_entity(include_str!("../../defs/entities/method.ron")),
        parse_entity(include_str!("../../defs/entities/movement.ron")),
        // Relational entities
        parse_entity(include_str!("../../defs/entities/relation.ron")),
        parse_entity(include_str!("../../defs/entities/tension.ron")),
        parse_entity(include_str!("../../defs/entities/loop.ron")),
        parse_entity(include_str!("../../defs/entities/paradox.ron")),
        // Process entities
        parse_entity(include_str!("../../defs/entities/evolution.ron")),
        parse_entity(include_str!("../../defs/entities/action.ron")),
        parse_entity(include_str!("../../defs/entities/effect.ron")),
        // Compound entities
        parse_entity(include_str!("../../defs/entities/work.ron")),
        parse_entity(include_str!("../../defs/entities/journal.ron")),
        // User-defined entities
        parse_entity(include_str!("../../defs/entities/meta.ron")),
        parse_entity(include_str!("../../defs/entities/question.ron")),
        parse_entity(include_str!("../../defs/entities/project.ron")),
        parse_entity(include_str!("../../defs/entities/idea.ron")),
        parse_entity(include_str!("../../defs/entities/user.ron")),
        parse_entity(include_str!("../../defs/entities/section.ron")),
    ];

    // Sort by sort_order for consistent output
    entities.sort_by_key(|e| e.sort_order);
    entities
}

/// Cached entity definitions
static ENTITIES: Lazy<Vec<RonEntityDef>> = Lazy::new(load_all_entities_inner);

/// Get all loaded entity definitions
pub fn load_all_entities() -> &'static Vec<RonEntityDef> {
    &ENTITIES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_entities() {
        let entities = load_all_entities();
        assert_eq!(entities.len(), 33, "Expected 33 entity definitions");
    }

    #[test]
    fn test_entity_has_required_fields() {
        let entities = load_all_entities();
        for entity in entities {
            assert!(!entity.symbol.is_empty(), "Entity symbol should not be empty");
            assert!(!entity.name.is_empty(), "Entity name should not be empty");
            assert!(!entity.lines.is_empty(), "Entity lines should not be empty");
        }
    }

    #[test]
    fn test_to_eris_text_format() {
        let entity = RonEntityDef {
            symbol: "⚘".to_string(),
            name: "Person".to_string(),
            description: "Test".to_string(),
            sort_order: 1,
            category: "Primary".to_string(),
            lines: vec![
                ("≡".to_string(), "named_individual".to_string()),
                ("≡".to_string(), "human_agent".to_string()),
                ("≝".to_string(), "historical attestation".to_string()),
            ],
        };

        let text = entity.to_eris_text();
        assert!(text.starts_with("⚘ ≡ named_individual"));
        assert!(text.contains("  human_agent")); // Repeated prefix becomes spaces
        assert!(text.contains("≝ historical")); // New prefix shown
    }
}
