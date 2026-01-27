//! ERIS entity types for tag composer
//! Single source of truth for entity type definitions

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::operators::chronos;

/// Entity type definition for tag composer
///
/// Fields use owned `String` to support runtime-loaded definitions (RON files).
#[derive(Debug, Clone)]
pub struct EntityTypeDef {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub sort_order: i32,
}

/// Lookup table for entity types by symbol.
static ENTITY_TYPE_MAP: Lazy<HashMap<String, EntityTypeDef>> = Lazy::new(|| {
    get_entity_type_definitions()
        .into_iter()
        .map(|def| (def.symbol.clone(), def))
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
///
/// Loads entities from RON files and includes chronos operator entity types.
pub fn get_entity_type_definitions() -> Vec<EntityTypeDef> {
    let mut defs: Vec<EntityTypeDef> = super::loader::load_all_entities()
        .iter()
        .map(|e| e.to_entity_type_def())
        .collect();

    // Operators that also function as entity types
    defs.extend(chronos::get_entity_type_definitions());

    defs
}
