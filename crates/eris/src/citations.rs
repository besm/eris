//! ERIS citation pattern definitions
//!
//! Loads citation patterns from RON files in `defs/citations/`.

use once_cell::sync::Lazy;
use serde::Deserialize;

/// Citation pattern definition loaded from RON file
#[derive(Debug, Clone, Deserialize)]
pub struct CitationDef {
    pub name: String,
    pub pattern: String,
    pub template: String,
    pub description: String,
    pub slots: Vec<(String, String)>,
    pub examples: Vec<String>,
    pub constraints: Vec<String>,
}

impl CitationDef {
    /// Render citation definition as formatted ERIS text
    pub fn to_eris_text(&self) -> String {
        let mut lines = vec![
            format!("{} {}", self.pattern, self.name),
            format!("  ≡ {}", self.description),
            format!("  ⊡ {}", self.template),
        ];

        // Slots
        for (slot, desc) in &self.slots {
            lines.push(format!("  ◻ {}: {}", slot, desc));
        }

        // Examples
        for ex in &self.examples {
            lines.push(format!("  ⊛ {}", ex));
        }

        // Constraints
        for c in &self.constraints {
            lines.push(format!("  ∂ {}", c));
        }

        lines.join("\n")
    }
}

/// Parse a RON citation definition
fn parse_citation(ron_str: &str) -> CitationDef {
    ron::from_str(ron_str).expect("Failed to parse RON citation definition")
}

/// Load all citation definitions from embedded RON files
fn load_citations_inner() -> Vec<CitationDef> {
    vec![
        parse_citation(include_str!("../defs/citations/book.ron")),
        parse_citation(include_str!("../defs/citations/article.ron")),
        parse_citation(include_str!("../defs/citations/org_book.ron")),
        parse_citation(include_str!("../defs/citations/org_article.ron")),
    ]
}

/// Cached citation definitions
static CITATIONS: Lazy<Vec<CitationDef>> = Lazy::new(load_citations_inner);

/// Get all loaded citation definitions
pub fn load_citations() -> &'static Vec<CitationDef> {
    &CITATIONS
}

/// Get all citation definitions as formatted text
pub fn get_all_definitions() -> Vec<String> {
    load_citations()
        .iter()
        .map(|c| c.to_eris_text())
        .collect()
}

/// Look up a citation by pattern
pub fn lookup(pattern: &str) -> Option<String> {
    load_citations()
        .iter()
        .find(|c| c.pattern == pattern || c.name == pattern)
        .map(|c| c.to_eris_text())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_citations() {
        let citations = load_citations();
        assert_eq!(citations.len(), 4, "Expected 4 citation definitions");
    }

    #[test]
    fn test_citation_has_required_fields() {
        for c in load_citations() {
            assert!(!c.name.is_empty(), "Citation name should not be empty");
            assert!(!c.pattern.is_empty(), "Citation pattern should not be empty");
            assert!(!c.template.is_empty(), "Citation template should not be empty");
            assert!(!c.slots.is_empty(), "Citation slots should not be empty");
            assert!(!c.examples.is_empty(), "Citation examples should not be empty");
        }
    }

    #[test]
    fn test_lookup_by_pattern() {
        let book = lookup("⚘⊙⊳").expect("BookCitation should exist");
        assert!(book.contains("BookCitation"));
    }

    #[test]
    fn test_lookup_by_name() {
        let article = lookup("ArticleCitation").expect("ArticleCitation should exist");
        assert!(article.contains("⚘⊙𝄏⊳"));
    }
}
