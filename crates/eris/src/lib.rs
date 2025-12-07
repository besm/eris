//! ERIS entity type system for semantic tagging

#[macro_use]
pub mod macros;

pub mod entities;
pub mod export;
pub mod frame;
pub mod notation;
pub mod operators;
pub mod parsers;
pub mod symbols;

pub use entities::types::get_entity_type_by_symbol;
pub use export::{definitions_for_prompt, entity_definitions_for_prompt, system_prompt_base};
pub use parsers::{parse_tag, validate_tag_structure, TagValidationError};

use std::collections::HashSet;

pub fn get_all_definitions() -> Vec<String> {
    let mut defs = operators::get_all_definitions();
    defs.extend(entities::get_all_definitions());
    defs
}

pub fn get_all_symbols() -> HashSet<String> {
    let mut symbols: HashSet<String> = operators::get_all_symbols().into_iter().collect();
    symbols.extend(entities::get_all_symbols());
    symbols
}

pub fn get_operator_symbols() -> HashSet<String> {
    operators::get_all_symbols().into_iter().collect()
}

pub fn get_entity_symbols() -> HashSet<String> {
    entities::get_all_symbols().into_iter().collect()
}

pub fn lookup_symbol(symbol: &str) -> Option<String> {
    operators::lookup(symbol).or_else(|| entities::lookup(symbol))
}

pub fn lookup_operator(symbol: &str) -> Option<String> {
    operators::lookup(symbol)
}

pub fn lookup_entity(symbol: &str) -> Option<String> {
    entities::lookup(symbol)
}
