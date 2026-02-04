//! Nix generation for ERIS definitions
//!
//! Generates Nix attribute sets from entity and operator definitions.
//! Output structure:
//! - entity.person, entity.place, etc. (flat)
//! - operator.logical.equivalence, operator.property.functional, etc. (module-nested)

use std::collections::BTreeMap;

/// Escape a string for Nix (double quotes)
fn nix_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t")
        .replace("${", "\\${")
}

/// Convert a name to a valid Nix identifier (lowercase, no spaces)
fn to_nix_id(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace('/', "-")
        .replace('(', "")
        .replace(')', "")
}

/// Generate entities.nix content
pub fn generate_entities() -> String {
    let mut nix = String::new();
    nix.push_str("# ERIS Entity Definitions\n");
    nix.push_str("# Generated from Rust definitions - do not edit manually\n");
    nix.push_str("{\n");

    let entity_defs = get_entity_defs();

    for (symbol, name, description, category, sort_order, lines) in entity_defs {
        let id = to_nix_id(&name);
        nix.push_str(&format!("  {} = {{\n", id));
        nix.push_str(&format!("    symbol = \"{}\";\n", nix_escape(&symbol)));
        nix.push_str(&format!("    name = \"{}\";\n", nix_escape(&name)));
        nix.push_str(&format!("    description = \"{}\";\n", nix_escape(&description)));
        nix.push_str(&format!("    category = \"{}\";\n", nix_escape(&category)));
        nix.push_str(&format!("    sortOrder = {};\n", sort_order));

        // Definition lines
        nix.push_str("    lines = [\n");
        for (prefix, content) in &lines {
            nix.push_str(&format!(
                "      {{ prefix = \"{}\"; content = \"{}\"; }}\n",
                nix_escape(prefix),
                nix_escape(content)
            ));
        }
        nix.push_str("    ];\n");
        nix.push_str("  };\n");
    }

    nix.push_str("}\n");
    nix
}

/// Generate operators.nix content (nested by module)
pub fn generate_operators() -> String {
    let mut nix = String::new();
    nix.push_str("# ERIS Operator Definitions\n");
    nix.push_str("# Generated from Rust definitions - do not edit manually\n");
    nix.push_str("{\n");

    let operator_defs = get_operator_defs();

    // Group by module
    let mut by_module: BTreeMap<String, Vec<_>> = BTreeMap::new();
    for def in operator_defs {
        by_module
            .entry(def.1.clone()) // module name
            .or_default()
            .push(def);
    }

    for (module, ops) in by_module {
        let module_id = to_nix_id(&module);
        nix.push_str(&format!("  {} = {{\n", module_id));

        for (symbol, _module, name, category, lines) in ops {
            let id = to_nix_id(&name);
            nix.push_str(&format!("    {} = {{\n", id));
            nix.push_str(&format!("      symbol = \"{}\";\n", nix_escape(&symbol)));
            nix.push_str(&format!("      name = \"{}\";\n", nix_escape(&name)));
            nix.push_str(&format!("      category = \"{}\";\n", nix_escape(&category)));

            // Definition lines
            nix.push_str("      lines = [\n");
            for (prefix, content) in &lines {
                nix.push_str(&format!(
                    "        {{ prefix = \"{}\"; content = \"{}\"; }}\n",
                    nix_escape(prefix),
                    nix_escape(content)
                ));
            }
            nix.push_str("      ];\n");
            nix.push_str("    };\n");
        }

        nix.push_str("  };\n");
    }

    nix.push_str("}\n");
    nix
}

/// Generate default.nix that imports both
pub fn generate_default() -> String {
    r#"# ERIS Nix Package
# Generated from Rust definitions - do not edit manually
{
  entity = import ./entities.nix;
  operator = import ./operators.nix;
}
"#
    .to_string()
}

/// Get entity definitions with category info from RON loader
fn get_entity_defs() -> Vec<(String, String, String, String, i32, Vec<(String, String)>)> {
    crate::entities::loader::load_all_entities()
        .iter()
        .map(|e| {
            (
                e.symbol.clone(),
                e.name.clone(),
                e.description.clone(),
                e.category.clone(),
                e.sort_order,
                e.lines(),
            )
        })
        .collect()
}

/// Get operator definitions with module info
/// Returns: (symbol, module, name, category, lines)
fn get_operator_defs() -> Vec<(String, String, String, String, Vec<(String, String)>)> {
    use crate::operators::*;

    let mut results = Vec::new();

    macro_rules! process_operator {
        ($mod:ident, $defs_fn:ident, $mod_name:expr) => {
            for def in $mod::$defs_fn() {
                let lines: Vec<(String, String)> = def
                    .lines
                    .iter()
                    .map(|(p, c)| (p.to_string(), c.to_string()))
                    .collect();
                results.push((
                    def.symbol.to_string(),
                    $mod_name.to_string(),
                    def.name.to_string(),
                    format!("{:?}", def.category),
                    lines,
                ));
            }
        };
    }

    process_operator!(loader, get_property_vector_definitions, "property");
    process_operator!(chronos, get_chronos_operator_definitions, "chronos");
    process_operator!(georgian, get_georgian_operator_definitions, "georgian");
    process_operator!(logical, get_logical_operator_definitions, "logical");
    process_operator!(meta, get_meta_operator_definitions, "meta");
    process_operator!(ontology, get_ontology_operator_definitions, "ontology");
    process_operator!(semantic, get_semantic_operator_definitions, "semantic");

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_escape() {
        assert_eq!(nix_escape("hello"), "hello");
        assert_eq!(nix_escape("it's \"quoted\""), "it's \\\"quoted\\\"");
        assert_eq!(nix_escape("${var}"), "\\${var}");
        assert_eq!(nix_escape("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_to_nix_id() {
        assert_eq!(to_nix_id("Person"), "person");
        assert_eq!(to_nix_id("Dated Event"), "dated-event");
        assert_eq!(to_nix_id("Loop (Feedback)"), "loop-feedback");
    }

    #[test]
    fn test_entities_generation() {
        let entities = generate_entities();
        assert!(entities.contains("person = {"));
        assert!(entities.contains("symbol = \"⚘\""));
        assert!(entities.contains("category = \"Primary\""));
    }

    #[test]
    fn test_operators_generation() {
        let operators = generate_operators();
        assert!(operators.contains("logical = {"));
        assert!(operators.contains("equivalence = {"));
        assert!(operators.contains("symbol = \"≡\""));
    }

    #[test]
    fn test_default_generation() {
        let default = generate_default();
        assert!(default.contains("entity = import ./entities.nix"));
        assert!(default.contains("operator = import ./operators.nix"));
    }
}
