//! ERIS entity type system for semantic tagging

#[macro_use]
pub mod macros;

pub mod citations;
pub mod entities;
pub mod export;
pub mod frame;
pub mod notation;
pub mod operators;
pub mod nix;
pub mod parsers;
pub mod sql;
pub mod symbols;

pub use entities::types::get_entity_type_by_symbol;
pub use export::{definitions_for_prompt, definitions_reduced_for_prompt, entity_definitions_for_prompt, system_prompt_base};
pub use parsers::{parse_tag, validate_tag_structure, TagValidationError};

use std::collections::HashSet;

pub fn get_all_definitions() -> Vec<String> {
    let mut defs = operators::get_all_definitions();
    defs.extend(entities::get_all_definitions());
    defs.extend(citations::get_all_definitions());
    defs
}

pub fn get_all_definitions_reduced() -> Vec<String> {
    let mut defs = operators::get_all_definitions_reduced();
    defs.extend(entities::get_all_definitions());
    defs.extend(citations::get_all_definitions());
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

/// Get definitions for symbols and optionally their dependencies.
///
/// - depth=0: seeds only
/// - depth=1: seeds + direct deps (recommended for workflows)
/// - depth=None: full transitive closure
///
/// # Example
/// ```
/// use eris::definitions_with_deps;
///
/// // Get institutional entity definitions + direct deps
/// let defs = definitions_with_deps(&["⍚", "⍢", "⎈", "⍓"], Some(1));
/// ```
pub fn definitions_with_deps(seeds: &[&str], depth: Option<usize>) -> Vec<String> {
    let all_defined = get_all_symbols();
    let mut seen: HashSet<String> = HashSet::new();
    let mut current_level: Vec<String> = seeds.iter().map(|s| s.to_string()).collect();
    let mut current_depth = 0;

    while !current_level.is_empty() {
        let mut next_level: Vec<String> = Vec::new();

        for symbol in current_level {
            if seen.contains(&symbol) {
                continue;
            }
            seen.insert(symbol.clone());

            // Extract deps if we haven't hit depth limit
            if depth.map_or(true, |d| current_depth < d) {
                if let Some(def) = lookup_symbol(&symbol) {
                    for ch in def.chars() {
                        let s = ch.to_string();
                        if all_defined.contains(&s) && !seen.contains(&s) {
                            next_level.push(s);
                        }
                    }
                }
            }
        }

        current_level = next_level;
        current_depth += 1;
    }

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
        let defs = definitions_with_deps(&["⍚"], None);
        // Should include ⍚ definition
        assert!(defs.iter().any(|d| d.contains("organization")));
        // Should include Armenian properties referenced in ⍚ def
        assert!(defs.iter().any(|d| d.contains("agency"))); // Ղ
    }

    #[test]
    fn test_definitions_with_deps_multiple() {
        let defs = definitions_with_deps(&["⍚", "⎈"], None);
        assert!(defs.len() >= 2); // At least the two seeds
    }

    #[test]
    fn test_definitions_with_deps_empty() {
        let defs = definitions_with_deps(&[], None);
        assert!(defs.is_empty());
    }

    #[test]
    fn test_definitions_with_deps_unknown() {
        // Unknown symbols should be ignored gracefully
        let defs = definitions_with_deps(&["⍚", "xyz"], None);
        assert!(!defs.is_empty()); // Still returns ⍚ and its deps
    }

    #[test]
    fn test_depth_zero_seeds_only() {
        let defs = definitions_with_deps(&["⍚"], Some(0));
        assert_eq!(defs.len(), 1); // Just ⍚
    }

    #[test]
    fn test_depth_one_direct_deps() {
        let defs = definitions_with_deps(&["⍚"], Some(1));
        assert!(defs.len() > 1); // ⍚ + Armenian props
        assert!(defs.len() < 50); // But not everything
    }
}
