-- =============================================================================
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
