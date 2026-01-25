//! ERIS entity type system for semantic tagging

#[macro_use]
pub mod macros;

pub mod entities;
pub mod export;
pub mod frame;
pub mod notation;
pub mod operators;
pub mod parsers;
pub mod sql;
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

/// Get definitions for symbols and all symbols they reference (transitive closure).
///
/// Given seed symbols, returns their definitions plus definitions for any
/// ERIS symbols found within those definitions. Continues recursively until
/// no new symbols are discovered.
///
/// # Example
/// ```
/// use eris::definitions_with_deps;
///
/// // Get institutional entity definitions + all referenced operators
/// let defs = definitions_with_deps(&["⍚", "⍢", "⎈", "⍓"]);
/// ```
pub fn definitions_with_deps(seeds: &[&str]) -> Vec<String> {
    let all_defined = get_all_symbols();
    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: Vec<String> = seeds.iter().map(|s| s.to_string()).collect();

    while let Some(symbol) = queue.pop() {
        if seen.contains(&symbol) {
            continue;
        }
        seen.insert(symbol.clone());

        // Get definition and extract referenced symbols
        if let Some(def) = lookup_symbol(&symbol) {
            for ch in def.chars() {
                let s = ch.to_string();
                if all_defined.contains(&s) && !seen.contains(&s) {
                    queue.push(s);
                }
            }
        }
    }

    // Return definitions in sorted order
    let mut symbols: Vec<_> = seen.into_iter().collect();
    symbols.sort();
    symbols.into_iter()
        .filter_map(|s| lookup_symbol(&s))
        .collect()
}

#[cfg(test)]
mod deps_tests {
    use super::*;

    #[test]
    fn test_definitions_with_deps_basic() {
        let defs = definitions_with_deps(&["⍚"]);
        // Should include ⍚ definition
        assert!(defs.iter().any(|d| d.contains("organization")));
        // Should include Armenian properties referenced in ⍚ def
        assert!(defs.iter().any(|d| d.contains("agency"))); // Ղ
    }

    #[test]
    fn test_definitions_with_deps_multiple() {
        let defs = definitions_with_deps(&["⍚", "⎈"]);
        assert!(defs.len() >= 2); // At least the two seeds
    }

    #[test]
    fn test_definitions_with_deps_empty() {
        let defs = definitions_with_deps(&[]);
        assert!(defs.is_empty());
    }

    #[test]
    fn test_definitions_with_deps_unknown() {
        // Unknown symbols should be ignored gracefully
        let defs = definitions_with_deps(&["⍚", "xyz"]);
        assert!(!defs.is_empty()); // Still returns ⍚ and its deps
    }
}
