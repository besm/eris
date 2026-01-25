//! ERIS MCP Server
//!
//! Model Context Protocol server exposing ERIS notation system tools.
//!
//! ## Tools
//!
//! - `lookup` - Get definition for a symbol
//! - `list_entities` - List all entity symbols
//! - `list_operators` - List all operator symbols (optionally by module)
//! - `search` - Search definitions by text
//! - `all` - Get all definitions
//! - `guide` - Get ERIS system documentation by topic

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;
use eris::{
    get_all_definitions, get_all_symbols, lookup_symbol,
    get_operator_symbols, get_entity_symbols,
};
use eris::frame::{get_workflow, list_workflows};

// =============================================================================
// Guide Content
// =============================================================================

const GUIDE_OVERVIEW: &str = r#"ERIS (Entity Reference and Information System) is a notation system for semantic tagging using Unicode symbols.

Components:
- 30 entity types (⚘ Person, ⧊ Concept, ⌖ Place, etc.)
- 7 operator categories (~60+ operators)
- Notation patterns for tags, citations, vectors
- Frame system for operational context

Use `guide` with topic: entities, operators, notation, patterns, prefixes, frames"#;

const GUIDE_ENTITIES: &str = r#"Entity Types (30 total):

Primary: ⚘ Person, ⌖ Place, ⧖ Era, ⊙ Date, ⌁ Event, ❖ Field, ⧈ Group, ⍚ Organization, ⎈ Agency, ⌬ Tech
Institutional: ⎚ Identifier, ⍓ Publisher, ⍢ University, ⧩ Language, 𝄏 Journal
Conceptual: ⧊ Concept, ⧏ Method, ⌯ Movement
Relational: ⇋ Relation, ⧆ Tension, ⟗ Loop, ☯ Paradox
Process: ⧃ Evolution, ⟴ Action, ⬢ Effect
Compound: ⊳ Work (used in citations)
User-defined: ⑀ Meta, ⋯ Question, ◈ Project, ⟡ Idea"#;

const GUIDE_OPERATORS: &str = r#"Operator Categories:

Armenian: Property vectors (0-9 scale) - Փ performativity, Գ generalizability, Վ validity
Chronos: Temporal/teleological - ⍜ purpose, ω teleological vector, ι initiation
Georgian: Workflow states - past/present/future contexts
Logical: Mathematical notation - ≡, ≝, →, ∧, ∨, ∀, ∃, ⊂, ◻, ⊨
Meta: Self-referential - documentation, schemas, examples
Ontology: Constitution/grounding - creation, performativity, crystallization
Semantic: Meaning dynamics - gravity wells, stability states

Use `list_operators` with `module` parameter to list specific category."#;

const GUIDE_NOTATION: &str = r#"Notation Types:

Entity:    ⦑⦒ brackets    ⚘⦑Mary Douglas⦒
Compound:  ⦑⦒ brackets    ⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒
Vector:    ⟨⟩ brackets    ⊡⟨Փ9Գ8⟩
Reference: ⟦⟧ brackets    ❧⟦12345⟧
Date:      ⊙⦑⦒           ⊙⦑1984⦒, ⊙⦑2023-05-15⦒
Era:       ⧖⦑⦒           ⧖⦑1980s⦒, ⧖⦑Nineteenth Century⦒"#;

const GUIDE_PATTERNS: &str = r#"Compound Notation Patterns:

BookCitation:       ⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒
ArticleCitation:    ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Title⦒
OrgBookCitation:    ⍚⊙⊳⦑Org⦒⦑Year⦒⦑Title⦒
OrgArticleCitation: ⍚⊙𝄏⊳⦑Org⦒⦑Year⦒⦑Journal⦒⦑Title⦒
DatedEvent:         ⌁⊙⦑Event⦒⦑Year⦒
ProjectSection:     ◈§⦑Project⦒⦑Section⦒

Multi-author: Use ∧ conjunction
  ⚘⊙⊳⦑Lakoff∧Johnson⦒⦑1980⦒⦑Metaphors We Live By⦒"#;

const GUIDE_PREFIXES: &str = r#"Definition Line Prefixes:

≡  Equivalence/name - what the symbol represents
≝  Defined as/essence - core meaning
∂  Boundary/exclusions - what it is NOT
⊛  Pattern examples - usage patterns
◻  Constraints/requirements - rules for application
≟  Discrimination rules - how to distinguish from similar
⊨  Validation/evidence - how to verify correct usage
⊡  Armenian property vector - property ratings"#;

const GUIDE_FRAMES: &str = r#"Frame System (composable operational context):

Role (Ψ):
  nav - Navigational
  pln - Planning
  evl - Evaluative
  crt - Creative
  itg - Integration

Context (⯐):
  ann - Annotation
  wfl - Workflow
  str - Structural
  eps - Epistemic

Task (τ):
  validate - Check correctness
  tag - Apply entity tags
  review - Assess and feedback
  migrate - Transform content
  query - Search/retrieve

Composed spec: ⟜⟨Ψ.nav⊗⯐.wfl⊗τ.tag⟩"#;

// =============================================================================
// JSON-RPC Types
// =============================================================================

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

impl JsonRpcResponse {
    fn success(id: Option<Value>, result: Value) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: Option<Value>, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0",
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
            }),
        }
    }
}

// =============================================================================
// MCP Protocol Types
// =============================================================================

#[derive(Debug, Serialize)]
struct ServerInfo {
    name: &'static str,
    version: &'static str,
}

#[derive(Debug, Serialize)]
struct ServerCapabilities {
    tools: ToolsCapability,
}

#[derive(Debug, Serialize)]
struct ToolsCapability {
    #[serde(rename = "listChanged")]
    list_changed: bool,
}

#[derive(Debug, Serialize)]
struct Tool {
    name: &'static str,
    description: &'static str,
    #[serde(rename = "inputSchema")]
    input_schema: Value,
}

#[derive(Debug, Serialize)]
struct ToolResult {
    content: Vec<ToolContent>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    is_error: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ToolContent {
    #[serde(rename = "type")]
    content_type: &'static str,
    text: String,
}

impl ToolResult {
    fn text(s: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent {
                content_type: "text",
                text: s.into(),
            }],
            is_error: None,
        }
    }

    fn error(s: impl Into<String>) -> Self {
        Self {
            content: vec![ToolContent {
                content_type: "text",
                text: s.into(),
            }],
            is_error: Some(true),
        }
    }
}

// =============================================================================
// Tool Definitions
// =============================================================================

fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "lookup",
            description: "Look up the definition of an ERIS symbol",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "symbol": {
                        "type": "string",
                        "description": "The ERIS symbol to look up (e.g., ⚘, ≡, ⧊)"
                    }
                },
                "required": ["symbol"]
            }),
        },
        Tool {
            name: "list_entities",
            description: "List all ERIS entity symbols with their names",
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "list_operators",
            description: "List all ERIS operator symbols. Optionally filter by module.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "module": {
                        "type": "string",
                        "description": "Optional module filter: armenian, chronos, georgian, logical, meta, ontology, semantic"
                    }
                }
            }),
        },
        Tool {
            name: "search",
            description: "Search ERIS definitions for text",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Text to search for in definitions"
                    }
                },
                "required": ["query"]
            }),
        },
        Tool {
            name: "all",
            description: "Get all ERIS definitions (entities and operators)",
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "closure",
            description: "Find undefined symbols used in ERIS definitions. Returns symbols that appear in definitions but aren't themselves defined.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "count": {
                        "type": "boolean",
                        "description": "If true, show usage counts for each undefined symbol"
                    }
                }
            }),
        },
        Tool {
            name: "workflow",
            description: "Get workflow specification for a task. Use 'list' as the name to see available workflows.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string",
                        "description": "Workflow name (tag, validate, review, migrate, query) or 'list' to show available"
                    }
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "define",
            description: "Get definitions for ERIS symbols found in a file. Reads the file and returns definitions for any ERIS symbols it contains.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "file": {
                        "type": "string",
                        "description": "Path to file to analyze for ERIS symbols"
                    },
                    "category": {
                        "type": "string",
                        "description": "Filter by category: 'ops', 'entities', or 'all' (default)"
                    }
                },
                "required": ["file"]
            }),
        },
        Tool {
            name: "guide",
            description: "Get ERIS system documentation. Returns overview by default, or specific topic.",
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": {
                        "type": "string",
                        "description": "Topic: overview, entities, operators, notation, patterns, prefixes, frames"
                    }
                }
            }),
        },
    ]
}

// =============================================================================
// Tool Handlers
// =============================================================================

fn handle_lookup(params: &Value) -> ToolResult {
    let symbol = match params.get("symbol").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return ToolResult::error("Missing required parameter: symbol"),
    };

    match lookup_symbol(symbol) {
        Some(def) => ToolResult::text(def),
        None => ToolResult::error(format!("Symbol '{}' not found", symbol)),
    }
}

fn handle_list_entities(_params: &Value) -> ToolResult {
    let symbols = get_entity_symbols();
    let mut results: Vec<String> = Vec::new();

    for symbol in &symbols {
        if let Some(def) = lookup_symbol(symbol) {
            // Extract first line (symbol + name)
            let first_line = def.lines().next().unwrap_or(&def);
            results.push(first_line.to_string());
        }
    }

    results.sort();
    ToolResult::text(results.join("\n"))
}

fn handle_list_operators(params: &Value) -> ToolResult {
    let module_filter = params.get("module").and_then(|v| v.as_str());

    let symbols = get_operator_symbols();
    let mut results: Vec<String> = Vec::new();

    for symbol in &symbols {
        if let Some(def) = lookup_symbol(symbol) {
            // If module filter specified, check if definition contains module name
            if let Some(module) = module_filter {
                // Simple heuristic: check module membership by looking at where it's defined
                // This isn't perfect but works for basic filtering
                let matches = match module.to_lowercase().as_str() {
                    "armenian" => is_armenian_symbol(symbol),
                    "chronos" => is_chronos_symbol(symbol),
                    "georgian" => is_georgian_symbol(symbol),
                    "logical" => is_logical_symbol(symbol),
                    "meta" => is_meta_symbol(symbol),
                    "ontology" => is_ontology_symbol(symbol),
                    "semantic" => is_semantic_symbol(symbol),
                    _ => true,
                };

                if !matches {
                    continue;
                }
            }

            let first_line = def.lines().next().unwrap_or(&def);
            results.push(first_line.to_string());
        }
    }

    results.sort();
    ToolResult::text(results.join("\n"))
}

fn handle_search(params: &Value) -> ToolResult {
    let query = match params.get("query").and_then(|v| v.as_str()) {
        Some(q) => q.to_lowercase(),
        None => return ToolResult::error("Missing required parameter: query"),
    };

    let defs = get_all_definitions();
    let mut results: Vec<String> = Vec::new();

    for def in defs {
        if def.to_lowercase().contains(&query) {
            results.push(def);
        }
    }

    if results.is_empty() {
        ToolResult::text(format!("No definitions found matching '{}'", query))
    } else {
        ToolResult::text(results.join("\n\n"))
    }
}

fn handle_all(_params: &Value) -> ToolResult {
    let defs = get_all_definitions();
    ToolResult::text(defs.join("\n\n"))
}

fn handle_closure(params: &Value) -> ToolResult {
    let show_count = params.get("count").and_then(|v| v.as_bool()).unwrap_or(false);

    let defined = get_all_symbols();
    let all_text = get_all_definitions().join("\n");

    let standard_punct: HashSet<&str> = [
        "'", "(", ")", ",", "-", "/", ":", "=", ">", "[", "]", "_", "|", "{", "}", "⦑",
        "⦒", "\"", ".", "?", "+", "<", "≤", "≥", "≠", "✓", "✗", "⁻",
    ]
    .into_iter()
    .collect();

    let mut symbol_counts: HashMap<String, usize> = HashMap::new();
    for ch in all_text.chars() {
        if !ch.is_ascii_alphanumeric() && !ch.is_whitespace() {
            let s = ch.to_string();
            if !standard_punct.contains(s.as_str()) && !defined.contains(&s) {
                *symbol_counts.entry(s).or_insert(0) += 1;
            }
        }
    }

    if symbol_counts.is_empty() {
        return ToolResult::text("No undefined symbols found");
    }

    if show_count {
        let mut symbol_freq: Vec<_> = symbol_counts.into_iter().collect();
        symbol_freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let lines: Vec<String> = symbol_freq
            .iter()
            .map(|(symbol, count)| format!("{:4} {}", count, symbol))
            .collect();
        ToolResult::text(lines.join("\n"))
    } else {
        let mut undefined: Vec<_> = symbol_counts.keys().cloned().collect();
        undefined.sort();
        ToolResult::text(undefined.join("\n"))
    }
}

fn handle_workflow(params: &Value) -> ToolResult {
    let name = match params.get("name").and_then(|v| v.as_str()) {
        Some(n) => n,
        None => return ToolResult::error("Missing required parameter: name"),
    };

    if name == "list" {
        let workflows = list_workflows();
        ToolResult::text(workflows.join("\n"))
    } else {
        match get_workflow(name) {
            Some(text) => ToolResult::text(text),
            None => ToolResult::error(format!(
                "Workflow '{}' not found. Use 'list' to see available workflows.",
                name
            )),
        }
    }
}

fn handle_define(params: &Value) -> ToolResult {
    let file_path = match params.get("file").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ToolResult::error("Missing required parameter: file"),
    };

    let content = match std::fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => return ToolResult::error(format!("Failed to read '{}': {}", file_path, e)),
    };

    let category = params.get("category").and_then(|v| v.as_str()).unwrap_or("all");

    // Select symbol set and lookup function based on category
    let (defined, lookup_fn): (HashSet<String>, fn(&str) -> Option<String>) = match category {
        "ops" => (get_operator_symbols(), eris::lookup_operator),
        "entities" => (get_entity_symbols(), eris::lookup_entity),
        _ => (get_all_symbols(), lookup_symbol),
    };

    // Collect symbols used in file that have definitions
    let mut used: HashSet<String> = HashSet::new();
    for ch in content.chars() {
        let s = ch.to_string();
        if defined.contains(&s) {
            used.insert(s);
        }
    }

    if used.is_empty() {
        return ToolResult::text("No ERIS symbols found in file");
    }

    // Output definitions for used symbols (sorted)
    let mut symbols: Vec<_> = used.into_iter().collect();
    symbols.sort();

    let mut results: Vec<String> = Vec::new();
    for symbol in symbols {
        if let Some(text) = lookup_fn(&symbol) {
            results.push(text);
        }
    }

    ToolResult::text(results.join("\n\n"))
}

fn handle_guide(params: &Value) -> ToolResult {
    let topic = params.get("topic").and_then(|v| v.as_str()).unwrap_or("overview");

    let text = match topic {
        "overview" => GUIDE_OVERVIEW,
        "entities" => GUIDE_ENTITIES,
        "operators" => GUIDE_OPERATORS,
        "notation" => GUIDE_NOTATION,
        "patterns" => GUIDE_PATTERNS,
        "prefixes" => GUIDE_PREFIXES,
        "frames" => GUIDE_FRAMES,
        _ => return ToolResult::error(format!(
            "Unknown topic: '{}'. Available: overview, entities, operators, notation, patterns, prefixes, frames",
            topic
        )),
    };

    ToolResult::text(text)
}

// Module detection helpers (based on symbol ranges/patterns)
fn is_armenian_symbol(s: &str) -> bool {
    s.chars().next().map(|c| ('\u{0530}'..='\u{058F}').contains(&c)).unwrap_or(false)
}

fn is_georgian_symbol(s: &str) -> bool {
    s.chars().next().map(|c| ('\u{10A0}'..='\u{10FF}').contains(&c)).unwrap_or(false)
}

fn is_chronos_symbol(s: &str) -> bool {
    // Greek letters commonly used in chronos
    let chronos_chars: HashSet<char> = "ωιαεμθδπςλϕγ□◇≺≻○⬡𝀺⏣⟟𝄃𝀃𝀷𝆌𝀖𝁤⍜⊱⟲⟖𝄎".chars().collect();
    s.chars().next().map(|c| chronos_chars.contains(&c)).unwrap_or(false)
}

fn is_logical_symbol(s: &str) -> bool {
    let logical_chars: HashSet<char> = "≡≝→∧∨¬∀∃∴↔⊥∞⊂⊃⊅⊆∈∉∩∪⋃⋂⊔∅∑∫≅≃≈≐∝∇ℤ◻⊨≫⊛≟⊟⊢⊩⊧⊿←⊸∂⊐⊏⏸⊬⊰⟷✱⩕↗◩⏈∘⊕↬⌇↭⊖↓⋄⫍⊞∮⎄⨝⎊⇌".chars().collect();
    s.chars().next().map(|c| logical_chars.contains(&c)).unwrap_or(false)
}

fn is_meta_symbol(s: &str) -> bool {
    let meta_chars: HashSet<char> = "⟓⧠⌕⋕⌹⧰▢◐◔𝟎ꕥᛝ❧".chars().collect();
    s.chars().next().map(|c| meta_chars.contains(&c)).unwrap_or(false)
}

fn is_ontology_symbol(s: &str) -> bool {
    let ontology_chars: HashSet<char> = "𝀐⛣𝁚𝀼𝀞⌂⤑⬟⥎⟒⛁⩎𝀏𝀾𝀸𝀕𝀆𝁀𝁆𝀗𝀶𝀴".chars().collect();
    s.chars().next().map(|c| ontology_chars.contains(&c)).unwrap_or(false)
}

fn is_semantic_symbol(s: &str) -> bool {
    let semantic_chars: HashSet<char> = "⌺⌻⌼⤋𝀋𝀔𝀭𝀙⥈☊ℳ◭⩍⟳⥅⤇◬┃⛫⟚⟛⋈⌾⯐⊚".chars().collect();
    s.chars().next().map(|c| semantic_chars.contains(&c)).unwrap_or(false)
}

// =============================================================================
// Request Handler
// =============================================================================

fn handle_request(request: JsonRpcRequest) -> JsonRpcResponse {
    match request.method.as_str() {
        "initialize" => {
            JsonRpcResponse::success(
                request.id,
                json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": ServerCapabilities {
                        tools: ToolsCapability { list_changed: false },
                    },
                    "serverInfo": ServerInfo {
                        name: "eris-mcp",
                        version: env!("CARGO_PKG_VERSION"),
                    },
                }),
            )
        }

        "notifications/initialized" => {
            // No response needed for notifications
            JsonRpcResponse::success(request.id, json!(null))
        }

        "tools/list" => {
            JsonRpcResponse::success(
                request.id,
                json!({
                    "tools": get_tools()
                }),
            )
        }

        "tools/call" => {
            let tool_name = request.params.get("name").and_then(|v| v.as_str());
            let arguments = request.params.get("arguments").cloned().unwrap_or(json!({}));

            let result = match tool_name {
                Some("lookup") => handle_lookup(&arguments),
                Some("list_entities") => handle_list_entities(&arguments),
                Some("list_operators") => handle_list_operators(&arguments),
                Some("search") => handle_search(&arguments),
                Some("all") => handle_all(&arguments),
                Some("closure") => handle_closure(&arguments),
                Some("workflow") => handle_workflow(&arguments),
                Some("define") => handle_define(&arguments),
                Some("guide") => handle_guide(&arguments),
                Some(name) => ToolResult::error(format!("Unknown tool: {}", name)),
                None => ToolResult::error("Missing tool name"),
            };

            JsonRpcResponse::success(request.id, serde_json::to_value(result).unwrap())
        }

        "ping" => JsonRpcResponse::success(request.id, json!({})),

        _ => JsonRpcResponse::error(
            request.id,
            -32601,
            format!("Method not found: {}", request.method),
        ),
    }
}

// =============================================================================
// Main
// =============================================================================

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Parse JSON-RPC request
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let response = JsonRpcResponse::error(None, -32700, format!("Parse error: {}", e));
                writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
                stdout.flush()?;
                continue;
            }
        };

        // Handle request
        let response = handle_request(request);

        // Send response (skip for notifications without id)
        if response.id.is_some() || response.error.is_some() {
            writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
            stdout.flush()?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        let result = handle_lookup(&json!({"symbol": "⚘"}));
        assert!(result.content[0].text.contains("Person"));
    }

    #[test]
    fn test_lookup_not_found() {
        let result = handle_lookup(&json!({"symbol": "xyz"}));
        assert!(result.is_error.unwrap_or(false));
    }

    #[test]
    fn test_list_entities() {
        let result = handle_list_entities(&json!({}));
        assert!(result.content[0].text.contains("⚘"));
    }

    #[test]
    fn test_search() {
        let result = handle_search(&json!({"query": "person"}));
        assert!(result.content[0].text.contains("Person"));
    }
}
