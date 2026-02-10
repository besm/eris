//! ERIS entity system
//!
//! Entity definitions are loaded from RON files in `defs/entities/`.

pub mod loader;
pub mod types;

pub use loader::RonEntityDef;

/// Entity type enum for type-safe entity operations
///
/// Variants are ordered by sort_order for consistent iteration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    Person,
    Place,
    Era,
    Date,
    Event,
    Field,
    Group,
    Organization,
    Agency,
    Party,
    Tech,
    Numinous,
    Identifier,
    Publisher,
    University,
    Language,
    Concept,
    Method,
    Movement,
    Relation,
    Tension,
    Loop,
    Paradox,
    Evolution,
    Action,
    Effect,
    Work,
    Journal,
    Meta,
    Question,
    Project,
    Idea,
    User,
    Section,
}

impl EntityType {
    /// Look up entity type from its symbol character
    pub fn from_symbol(s: char) -> Option<Self> {
        let sym = s.to_string();
        loader::load_all_entities()
            .iter()
            .position(|e| e.symbol == sym)
            .and_then(|idx| Self::ALL.get(idx).copied())
    }

    /// Get the symbol for this entity type
    pub fn symbol(&self) -> char {
        let idx = Self::ALL.iter().position(|&t| t == *self).unwrap();
        loader::load_all_entities()[idx]
            .symbol
            .chars()
            .next()
            .unwrap()
    }

    /// Get the name of this entity type
    pub fn name(&self) -> &'static str {
        // Names are static in the RON files, but we leak the String to get &'static str
        // This is acceptable since entities are loaded once and cached
        let idx = Self::ALL.iter().position(|&t| t == *self).unwrap();
        let name = &loader::load_all_entities()[idx].name;
        // SAFETY: Entities are loaded once via Lazy, so the name lives for 'static
        unsafe { std::mem::transmute::<&str, &'static str>(name.as_str()) }
    }

    /// Get all entity types
    pub fn all() -> &'static [Self] {
        Self::ALL
    }

    /// All entity types in sort order
    const ALL: &'static [Self] = &[
        Self::Person,
        Self::Place,
        Self::Era,
        Self::Date,
        Self::Event,
        Self::Field,
        Self::Group,
        Self::Organization,
        Self::Agency,
        Self::Party,
        Self::Tech,
        Self::Numinous,
        Self::Identifier,
        Self::Publisher,
        Self::University,
        Self::Language,
        Self::Concept,
        Self::Method,
        Self::Movement,
        Self::Relation,
        Self::Tension,
        Self::Loop,
        Self::Paradox,
        Self::Evolution,
        Self::Action,
        Self::Effect,
        Self::Work,
        Self::Journal,
        Self::Meta,
        Self::Question,
        Self::Project,
        Self::Idea,
        Self::User,
        Self::Section,
    ];
}

/// Get all entity definitions as formatted ERIS text
pub fn get_all_definitions() -> Vec<String> {
    loader::load_all_entities()
        .iter()
        .map(|e| e.to_eris_text())
        .collect()
}

/// Get all entity symbols
pub fn get_all_symbols() -> Vec<String> {
    loader::load_all_entities()
        .iter()
        .map(|e| e.symbol.clone())
        .collect()
}

/// Look up an entity definition by symbol
pub fn lookup(symbol: &str) -> Option<String> {
    loader::load_all_entities()
        .iter()
        .find(|e| e.symbol == symbol)
        .map(|e| e.to_eris_text())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_count_matches_loader() {
        assert_eq!(
            EntityType::ALL.len(),
            loader::load_all_entities().len(),
            "EntityType variants must match loaded entity count"
        );
    }

    #[test]
    fn test_entity_type_from_symbol() {
        assert_eq!(EntityType::from_symbol('⚘'), Some(EntityType::Person));
        assert_eq!(EntityType::from_symbol('⌖'), Some(EntityType::Place));
        assert_eq!(EntityType::from_symbol('⧊'), Some(EntityType::Concept));
        assert_eq!(EntityType::from_symbol('X'), None);
    }

    #[test]
    fn test_entity_type_symbol() {
        assert_eq!(EntityType::Person.symbol(), '⚘');
        assert_eq!(EntityType::Place.symbol(), '⌖');
        assert_eq!(EntityType::Concept.symbol(), '⧊');
    }

    #[test]
    fn test_entity_type_name() {
        assert_eq!(EntityType::Person.name(), "Person");
        assert_eq!(EntityType::Place.name(), "Place");
        assert_eq!(EntityType::Concept.name(), "Concept");
    }

    #[test]
    fn test_get_all_definitions() {
        let defs = get_all_definitions();
        assert_eq!(defs.len(), 34);
        assert!(defs[0].starts_with("⚘")); // Person is first (sort_order: 1)
    }

    #[test]
    fn test_lookup() {
        let person = lookup("⚘").expect("Person should exist");
        assert!(person.contains("Person"));
        assert!(person.contains("named_individual"));

        assert!(lookup("X").is_none());
    }
}
