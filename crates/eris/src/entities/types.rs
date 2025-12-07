//! ERIS entity types for tag composer
//! Single source of truth for entity type definitions

use std::collections::HashMap;

use once_cell::sync::Lazy;

use super::*;
use crate::operators::chronos;

/// Entity type definition for tag composer
#[derive(Debug, Clone)]
pub struct EntityTypeDef {
    pub symbol: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub sort_order: i32,
}

/// Lookup table for entity types by symbol.
static ENTITY_TYPE_MAP: Lazy<HashMap<&'static str, EntityTypeDef>> = Lazy::new(|| {
    get_entity_type_definitions()
        .into_iter()
        .map(|def| (def.symbol, def))
        .collect()
});

/// Get an entity type definition by its symbol.
///
/// # Examples
///
/// ```
/// use eris::entities::types::get_entity_type_by_symbol;
///
/// let person = get_entity_type_by_symbol("⚘").unwrap();
/// assert_eq!(person.name, "Person");
///
/// let concept = get_entity_type_by_symbol("⧊").unwrap();
/// assert_eq!(concept.name, "Concept");
///
/// assert!(get_entity_type_by_symbol("invalid").is_none());
/// ```
pub fn get_entity_type_by_symbol(symbol: &str) -> Option<&EntityTypeDef> {
    ENTITY_TYPE_MAP.get(symbol)
}

/// Get all ERIS entity type definitions
/// Delegates to individual entity modules
pub fn get_entity_type_definitions() -> Vec<EntityTypeDef> {
    let mut defs = Vec::new();
    
    // Primary entities
    defs.extend(person::get_entity_type_definitions());
    defs.extend(place::get_entity_type_definitions());
    defs.extend(era::get_entity_type_definitions());
    defs.extend(date::get_entity_type_definitions());
    defs.extend(event::get_entity_type_definitions());
    defs.extend(field::get_entity_type_definitions());
    defs.extend(group::get_entity_type_definitions());
    defs.extend(organization::get_entity_type_definitions());
    defs.extend(agency::get_entity_type_definitions());
    defs.extend(tech::get_entity_type_definitions());
    defs.extend(identifier::get_entity_type_definitions());
    defs.extend(publisher::get_entity_type_definitions());
    defs.extend(university::get_entity_type_definitions());
    defs.extend(language::get_entity_type_definitions());
    
    // Conceptual entities
    defs.extend(concept::get_entity_type_definitions());
    defs.extend(method::get_entity_type_definitions());
    defs.extend(movement::get_entity_type_definitions());
    
    // Relational/dynamic entities
    defs.extend(relation::get_entity_type_definitions());
    defs.extend(tension::get_entity_type_definitions());
    defs.extend(r#loop::get_entity_type_definitions());
    defs.extend(paradox::get_entity_type_definitions());
    
    // Complex/process entities
    defs.extend(evolution::get_entity_type_definitions());
    defs.extend(action::get_entity_type_definitions());
    defs.extend(effect::get_entity_type_definitions());
    
    // Compound citation components
    defs.extend(work::get_entity_type_definitions());
    defs.extend(journal::get_entity_type_definitions());
    
    // User-defined markers
    defs.extend(meta::get_entity_type_definitions());
    defs.extend(question::get_entity_type_definitions());
    defs.extend(project::get_entity_type_definitions());
    defs.extend(idea::get_entity_type_definitions());
    
    // Operators that also function as entity types
    defs.extend(chronos::get_entity_type_definitions());
    
    defs
}
