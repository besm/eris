//! SQL generation for ERIS definitions
//!
//! Generates SQLite-compatible SQL from entity and operator definitions.

/// Generate the DDL schema (CREATE TABLE statements)
pub fn generate_schema() -> String {
    r#"-- =============================================================================
-- ERIS SQL Schema
-- Generated from Rust definitions
-- =============================================================================

PRAGMA foreign_keys = ON;

-- =============================================================================
-- CORE TABLES
-- =============================================================================

-- Unified symbol registry
CREATE TABLE IF NOT EXISTS symbols (
    id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE,
    codepoint TEXT NOT NULL,
    symbol_type TEXT NOT NULL CHECK (symbol_type IN ('entity', 'operator', 'bracket')),
    name TEXT NOT NULL
);

-- =============================================================================
-- ENTITIES
-- =============================================================================

CREATE TABLE IF NOT EXISTS entity_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS entities (
    id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    category_id INTEGER NOT NULL REFERENCES entity_categories(id),
    sort_order INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS entity_definition_lines (
    id INTEGER PRIMARY KEY,
    entity_id INTEGER NOT NULL REFERENCES entities(id),
    prefix TEXT NOT NULL,
    content TEXT NOT NULL,
    line_order INTEGER NOT NULL
);

-- =============================================================================
-- OPERATORS
-- =============================================================================

CREATE TABLE IF NOT EXISTS operator_modules (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS operator_categories (
    id INTEGER PRIMARY KEY,
    module_id INTEGER NOT NULL REFERENCES operator_modules(id),
    name TEXT NOT NULL,
    description TEXT,
    UNIQUE(module_id, name)
);

CREATE TABLE IF NOT EXISTS operators (
    id INTEGER PRIMARY KEY,
    symbol TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    module_id INTEGER NOT NULL REFERENCES operator_modules(id),
    category_id INTEGER NOT NULL REFERENCES operator_categories(id)
);

CREATE TABLE IF NOT EXISTS operator_definition_lines (
    id INTEGER PRIMARY KEY,
    operator_id INTEGER NOT NULL REFERENCES operators(id),
    prefix TEXT NOT NULL,
    content TEXT NOT NULL,
    line_order INTEGER NOT NULL
);

-- =============================================================================
-- INDEXES
-- =============================================================================

CREATE INDEX IF NOT EXISTS idx_symbols_type ON symbols(symbol_type);
CREATE INDEX IF NOT EXISTS idx_entities_category ON entities(category_id);
CREATE INDEX IF NOT EXISTS idx_operators_module ON operators(module_id);
CREATE INDEX IF NOT EXISTS idx_operators_category ON operators(category_id);
CREATE INDEX IF NOT EXISTS idx_entity_lines_entity ON entity_definition_lines(entity_id);
CREATE INDEX IF NOT EXISTS idx_operator_lines_operator ON operator_definition_lines(operator_id);

-- =============================================================================
-- VIEWS
-- =============================================================================

CREATE VIEW IF NOT EXISTS v_entities AS
SELECT
    e.id,
    e.symbol,
    e.name,
    e.description,
    ec.name AS category,
    e.sort_order
FROM entities e
JOIN entity_categories ec ON e.category_id = ec.id
ORDER BY e.sort_order;

CREATE VIEW IF NOT EXISTS v_operators AS
SELECT
    o.id,
    o.symbol,
    o.name,
    om.name AS module,
    oc.name AS category
FROM operators o
JOIN operator_modules om ON o.module_id = om.id
JOIN operator_categories oc ON o.category_id = oc.id
ORDER BY om.name, oc.name, o.name;

CREATE VIEW IF NOT EXISTS v_all_symbols AS
SELECT symbol, 'entity' AS type, name FROM entities
UNION ALL
SELECT symbol, 'operator' AS type, name FROM operators
ORDER BY symbol;
"#
    .to_string()
}

/// Escape a string for SQL (single quotes)
fn sql_escape(s: &str) -> String {
    s.replace('\'', "''")
}

/// Get Unicode codepoint string for a symbol
fn codepoint(s: &str) -> String {
    s.chars()
        .map(|c| format!("U+{:04X}", c as u32))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate INSERT statements for all data
pub fn generate_data() -> String {
    let mut sql = String::new();

    sql.push_str("-- =============================================================================\n");
    sql.push_str("-- ERIS Data\n");
    sql.push_str("-- Generated from Rust definitions\n");
    sql.push_str("-- =============================================================================\n\n");

    sql.push_str("BEGIN TRANSACTION;\n\n");

    // Entity categories
    sql.push_str("-- Entity Categories\n");
    let entity_categories = [
        "Primary",
        "Institutional",
        "Conceptual",
        "Relational",
        "Process",
        "Compound",
        "UserDefined",
    ];
    for (i, cat) in entity_categories.iter().enumerate() {
        sql.push_str(&format!(
            "INSERT INTO entity_categories (id, name) VALUES ({}, '{}');\n",
            i + 1,
            cat
        ));
    }
    sql.push('\n');

    // Operator modules
    sql.push_str("-- Operator Modules\n");
    let operator_modules = [
        ("property", "Property vectors (0-9 scale)"),
        ("chronos", "Temporal/teleological operators"),
        ("georgian", "Workflow state operators"),
        ("logical", "Mathematical notation"),
        ("meta", "Self-referential operators"),
        ("ontology", "Constitution/grounding operators"),
        ("semantic", "Meaning dynamics operators"),
    ];
    for (i, (name, desc)) in operator_modules.iter().enumerate() {
        sql.push_str(&format!(
            "INSERT INTO operator_modules (id, name, description) VALUES ({}, '{}', '{}');\n",
            i + 1,
            name,
            sql_escape(desc)
        ));
    }
    sql.push('\n');

    // Generate entity data
    sql.push_str(&generate_entity_data());

    // Generate operator data
    sql.push_str(&generate_operator_data());

    sql.push_str("COMMIT;\n");

    sql
}

fn category_to_id(cat_name: &str) -> i32 {
    match cat_name {
        "Primary" => 1,
        "Institutional" => 2,
        "Conceptual" => 3,
        "Relational" => 4,
        "Process" => 5,
        "Compound" => 6,
        "UserDefined" => 7,
        _ => 1, // Default to Primary
    }
}

fn module_to_id(module: &str) -> i32 {
    match module {
        "property" => 1,
        "chronos" => 2,
        "georgian" => 3,
        "logical" => 4,
        "meta" => 5,
        "ontology" => 6,
        "semantic" => 7,
        _ => 4, // Default to logical
    }
}

fn generate_entity_data() -> String {
    let mut sql = String::new();
    sql.push_str("-- Entities\n");

    let entity_defs = get_entity_defs_with_categories();

    let mut entity_id = 1;
    let mut line_id = 1;

    for (symbol, name, description, category, sort_order, lines) in entity_defs {
        let cat_id = category_to_id(&category);

        sql.push_str(&format!(
            "INSERT INTO entities (id, symbol, name, description, category_id, sort_order) VALUES ({}, '{}', '{}', '{}', {}, {});\n",
            entity_id,
            sql_escape(&symbol),
            sql_escape(&name),
            sql_escape(&description),
            cat_id,
            sort_order
        ));

        // Also insert into symbols table
        sql.push_str(&format!(
            "INSERT INTO symbols (symbol, codepoint, symbol_type, name) VALUES ('{}', '{}', 'entity', '{}');\n",
            sql_escape(&symbol),
            codepoint(&symbol),
            sql_escape(&name)
        ));

        for (order, (prefix, content)) in lines.iter().enumerate() {
            sql.push_str(&format!(
                "INSERT INTO entity_definition_lines (id, entity_id, prefix, content, line_order) VALUES ({}, {}, '{}', '{}', {});\n",
                line_id,
                entity_id,
                sql_escape(prefix),
                sql_escape(content),
                order + 1
            ));
            line_id += 1;
        }

        entity_id += 1;
    }

    sql.push('\n');
    sql
}

/// Get entity definitions with category info from RON loader
fn get_entity_defs_with_categories() -> Vec<(String, String, String, String, i32, Vec<(String, String)>)>
{
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

fn generate_operator_data() -> String {
    let mut sql = String::new();
    sql.push_str("-- Operators\n");

    let operator_defs = get_operator_defs_with_modules();

    // First, collect all unique categories per module
    let mut category_id = 1;
    let mut category_map: std::collections::HashMap<(String, String), i32> =
        std::collections::HashMap::new();

    for (_, _, module, category, _) in &operator_defs {
        let key = (module.clone(), category.clone());
        if !category_map.contains_key(&key) {
            let mod_id = module_to_id(module);
            sql.push_str(&format!(
                "INSERT INTO operator_categories (id, module_id, name) VALUES ({}, {}, '{}');\n",
                category_id, mod_id, category
            ));
            category_map.insert(key, category_id);
            category_id += 1;
        }
    }
    sql.push('\n');

    let mut operator_id = 1;
    let mut line_id = 1;

    for (symbol, name, module, category, lines) in operator_defs {
        let mod_id = module_to_id(&module);
        let cat_id = *category_map
            .get(&(module.clone(), category.clone()))
            .unwrap_or(&1);

        sql.push_str(&format!(
            "INSERT INTO operators (id, symbol, name, module_id, category_id) VALUES ({}, '{}', '{}', {}, {});\n",
            operator_id,
            sql_escape(&symbol),
            sql_escape(&name),
            mod_id,
            cat_id
        ));

        // Also insert into symbols table
        sql.push_str(&format!(
            "INSERT INTO symbols (symbol, codepoint, symbol_type, name) VALUES ('{}', '{}', 'operator', '{}');\n",
            sql_escape(&symbol),
            codepoint(&symbol),
            sql_escape(&name)
        ));

        for (order, (prefix, content)) in lines.iter().enumerate() {
            sql.push_str(&format!(
                "INSERT INTO operator_definition_lines (id, operator_id, prefix, content, line_order) VALUES ({}, {}, '{}', '{}', {});\n",
                line_id,
                operator_id,
                sql_escape(prefix),
                sql_escape(content),
                order + 1
            ));
            line_id += 1;
        }

        operator_id += 1;
    }

    sql.push('\n');
    sql
}

/// Get operator definitions with module info
fn get_operator_defs_with_modules() -> Vec<(String, String, String, String, Vec<(String, String)>)>
{
    use crate::operators::*;

    let mut results = Vec::new();

    // Helper macro to process operator modules
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
                    def.name.to_string(),
                    $mod_name.to_string(),
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

/// Generate complete SQL (schema + data)
pub fn generate_full() -> String {
    let mut sql = generate_schema();
    sql.push('\n');
    sql.push_str(&generate_data());
    sql
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_generation() {
        let schema = generate_schema();
        assert!(schema.contains("CREATE TABLE IF NOT EXISTS symbols"));
        assert!(schema.contains("CREATE TABLE IF NOT EXISTS entities"));
        assert!(schema.contains("CREATE TABLE IF NOT EXISTS operators"));
    }

    #[test]
    fn test_data_generation() {
        let data = generate_data();
        assert!(data.contains("INSERT INTO entity_categories"));
        assert!(data.contains("INSERT INTO operator_modules"));
        assert!(data.contains("INSERT INTO entities"));
        assert!(data.contains("INSERT INTO operators"));
    }

    #[test]
    fn test_sql_escape() {
        assert_eq!(sql_escape("hello"), "hello");
        assert_eq!(sql_escape("it's"), "it''s");
        assert_eq!(sql_escape("a'b'c"), "a''b''c");
    }

    #[test]
    fn test_codepoint() {
        assert_eq!(codepoint("⚘"), "U+2698");
        assert_eq!(codepoint("≡"), "U+2261");
    }
}
