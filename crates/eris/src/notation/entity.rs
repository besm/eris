//! Entity notation parsing for ERIS tags
//!
//! Handles both simple and compound entity tags using `⦑⦒` brackets.
//!
//! # Simple tags
//! - `⚘⦑Mary Douglas⦒` - person
//! - `⧊⦑Irony⦒` - concept
//!
//! # Compound tags
//! - `⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒` - book citation
//! - `⚘⊙𝄏⊳⦑Jane Smith⦒⦑2020⦒⦑Nature⦒⦑A Discovery⦒` - article citation

use super::{split_conjunction, Notation};
use crate::entities::EntityType;
use crate::symbols::{BRACKET_CLOSE, BRACKET_OPEN, CONJUNCTION};

// ═══════════════════════════════════════════════════════════════════════════
// Notation Definitions
// ═══════════════════════════════════════════════════════════════════════════

/// Definition of a compound notation pattern
#[derive(Debug, Clone)]
pub struct EntityNotation {
    /// Name of this notation pattern
    pub name: &'static str,
    /// Required symbol sequence
    pub pattern: &'static [char],
    /// Named component positions
    pub components: &'static [(&'static str, usize)],
}

impl EntityNotation {
    /// Check if a symbol sequence matches this notation pattern
    pub fn matches(&self, symbols: &[char]) -> bool {
        self.pattern == symbols
    }

    /// Get component name by index
    pub fn component_name(&self, index: usize) -> Option<&'static str> {
        self.components
            .iter()
            .find(|(_, i)| *i == index)
            .map(|(name, _)| *name)
    }

    /// Get component index by name
    pub fn component_index(&self, name: &str) -> Option<usize> {
        self.components
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, i)| *i)
    }
}

/// Built-in notation definitions
pub static NOTATIONS: &[EntityNotation] = &[
    EntityNotation {
        name: "BookCitation",
        pattern: &['⚘', '⊙', '⊳'],
        components: &[("author", 0), ("year", 1), ("title", 2)],
    },
    EntityNotation {
        name: "ArticleCitation",
        pattern: &['⚘', '⊙', '𝄏', '⊳'],
        components: &[("author", 0), ("year", 1), ("journal", 2), ("title", 3)],
    },
    EntityNotation {
        name: "OrgBookCitation",
        pattern: &['⍚', '⊙', '⊳'],
        components: &[("author", 0), ("year", 1), ("title", 2)],
    },
    EntityNotation {
        name: "OrgArticleCitation",
        pattern: &['⍚', '⊙', '𝄏', '⊳'],
        components: &[("author", 0), ("year", 1), ("journal", 2), ("title", 3)],
    },
    EntityNotation {
        name: "DatedEvent",
        pattern: &['⌁', '⊙'],
        components: &[("event", 0), ("year", 1)],
    },
];

/// Find notation definition matching a symbol sequence
pub fn find_notation(symbols: &[char]) -> Option<&'static EntityNotation> {
    NOTATIONS.iter().find(|n| n.matches(symbols))
}

// ═══════════════════════════════════════════════════════════════════════════
// Compound Tag Parser
// ═══════════════════════════════════════════════════════════════════════════

/// A parsed entity tag with symbols and components
///
/// ERIS tags use a structured format: `symbol* ⦑value⦒+`
///
/// # Examples
///
/// - `⚘⦑Albert Einstein⦒` - person tag
/// - `⊙⦑1905⦒` - date tag
/// - `⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒` - citation tag
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundTag {
    /// Prefix symbols (e.g., ⚘, ⊙, 𝄏, ⊳)
    pub symbols: Vec<char>,
    /// Component values in order
    pub components: Vec<String>,
}

impl Notation for CompoundTag {
    const OPEN: char = BRACKET_OPEN;
    const CLOSE: char = BRACKET_CLOSE;

    fn parse(tag: &str) -> Option<Self> {
        let mut chars = tag.chars().peekable();
        let mut symbols = Vec::new();
        let mut components = Vec::new();

        // Collect prefix symbols (everything before first ⦑)
        while let Some(&c) = chars.peek() {
            if c == BRACKET_OPEN {
                break;
            }
            symbols.push(c);
            chars.next();
        }

        // Parse ⦑value⦒ components - must have matched brackets
        while chars.peek().is_some() {
            // Expect opening bracket
            if chars.next() != Some(BRACKET_OPEN) {
                break;
            }

            // Collect content until closing bracket
            let mut content = String::new();
            let mut found_close = false;
            for c in chars.by_ref() {
                if c == BRACKET_CLOSE {
                    found_close = true;
                    break;
                }
                content.push(c);
            }

            // Only add component if we found the closing bracket
            if found_close && !content.is_empty() {
                components.push(content);
            } else if !found_close {
                // Unclosed bracket - stop parsing (don't add incomplete component)
                break;
            }
        }

        if components.is_empty() {
            None
        } else {
            Some(Self {
                symbols,
                components,
            })
        }
    }

    fn render(&self) -> String {
        let symbols: String = self.symbols.iter().collect();
        let components: String = self
            .components
            .iter()
            .map(|c| format!("{}{}{}", BRACKET_OPEN, c, BRACKET_CLOSE))
            .collect();
        format!("{}{}", symbols, components)
    }
}

impl CompoundTag {
    /// Parse a tag string into symbols and components
    ///
    /// This is a convenience method that delegates to the `Notation` trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use eris::notation::CompoundTag;
    ///
    /// let tag = CompoundTag::parse("⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒").unwrap();
    /// assert_eq!(tag.symbols, vec!['⚘', '⊙', '⊳']);
    /// assert_eq!(tag.components, vec!["George Orwell", "1949", "Nineteen Eighty-Four"]);
    /// ```
    pub fn parse(tag: &str) -> Option<Self> {
        <Self as Notation>::parse(tag)
    }

    /// Render this tag back to a string
    ///
    /// This is a convenience method that delegates to the `Notation` trait.
    pub fn render(&self) -> String {
        <Self as Notation>::render(self)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Basic Accessors
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if this tag has a specific symbol
    pub fn has_symbol(&self, sym: char) -> bool {
        self.symbols.contains(&sym)
    }

    /// Get component by index
    pub fn get(&self, index: usize) -> Option<&str> {
        self.components.get(index).map(|s| s.as_str())
    }

    /// Get component by name using notation definition
    pub fn get_named(&self, name: &str) -> Option<&str> {
        find_notation(&self.symbols)
            .and_then(|n| n.component_index(name))
            .and_then(|i| self.get(i))
    }

    /// Check if this is a simple tag (single symbol, single component)
    pub fn is_simple(&self) -> bool {
        self.symbols.len() == 1 && self.components.len() == 1
    }

    /// Get the notation definition if one matches
    pub fn notation(&self) -> Option<&'static EntityNotation> {
        find_notation(&self.symbols)
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Type Checks
    // ═══════════════════════════════════════════════════════════════════════

    /// Check if this is a citation tag (starts with ⚘⊙ or ⍚⊙)
    pub fn is_citation(&self) -> bool {
        self.symbols.len() >= 2
            && (self.symbols[0] == '⚘' || self.symbols[0] == '⍚')
            && self.symbols[1] == '⊙'
    }

    /// Check if this is a simple person tag (just ⚘⦑Name⦒)
    pub fn is_person(&self) -> bool {
        self.symbols == vec!['⚘'] && self.components.len() == 1
    }

    /// Check if this is a date tag (starts with ⊙)
    pub fn is_date(&self) -> bool {
        self.symbols.first() == Some(&'⊙')
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Component Accessors (using notation definitions where available)
    // ═══════════════════════════════════════════════════════════════════════

    /// For citation tags, get the author (first component)
    pub fn author(&self) -> Option<&str> {
        if self.is_citation() {
            self.get_named("author").or_else(|| self.get(0))
        } else {
            None
        }
    }

    /// For citation tags, get the year (second component)
    pub fn year(&self) -> Option<&str> {
        if self.is_citation() {
            self.get_named("year").or_else(|| self.get(1))
        } else {
            None
        }
    }

    /// For person tags (⚘⦑Name⦒), get the name
    pub fn person(&self) -> Option<&str> {
        if self.is_person() {
            self.get(0)
        } else {
            None
        }
    }

    /// For date tags (⊙⦑...⦒), get the date value
    pub fn date(&self) -> Option<&str> {
        if self.is_date() {
            self.get(0)
        } else {
            None
        }
    }

    /// Get the work/title component for citation tags
    ///
    /// Uses notation definition to find the title position.
    pub fn title(&self) -> Option<&str> {
        if self.is_citation() {
            self.get_named("title").or_else(|| {
                // Fallback: for article citations (has 𝄏), title is last component
                if self.has_symbol('𝄏') {
                    self.components.last().map(|s| s.as_str())
                } else {
                    // For book citations, title is third component
                    self.get(2)
                }
            })
        } else {
            None
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Conjunction Support
    // ═══════════════════════════════════════════════════════════════════════

    /// Get authors as a list (splits by ∧)
    pub fn authors(&self) -> Vec<&str> {
        self.author()
            .map(split_conjunction)
            .unwrap_or_default()
    }

    /// Check if this tag contains a person name (either as person tag or citation author)
    pub fn contains_person(&self, name: &str) -> bool {
        if let Some(person) = self.person() {
            return person == name;
        }
        if let Some(author_str) = self.author() {
            // Handle multi-author with ∧
            if split_conjunction(author_str).contains(&name) {
                return true;
            }
        }
        // Also check if the person appears in the title (books about a person)
        if let Some(title) = self.title() {
            if title == name {
                return true;
            }
        }
        false
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Entity Type Integration
    // ═══════════════════════════════════════════════════════════════════════

    /// Get all entity types present in this tag's symbols
    pub fn entity_types(&self) -> Vec<EntityType> {
        self.symbols
            .iter()
            .filter_map(|&s| EntityType::from_symbol(s))
            .collect()
    }

    /// Check if this tag contains a specific entity type
    pub fn has_entity_type(&self, entity_type: EntityType) -> bool {
        self.symbols
            .iter()
            .any(|&s| EntityType::from_symbol(s) == Some(entity_type))
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Implied Tags
    // ═══════════════════════════════════════════════════════════════════════

    /// Generate implied simple tag names from a compound tag.
    ///
    /// For citation tags (⚘⊙...), extracts:
    /// - `⚘⦑Author⦒` - person tag for each author
    /// - `⊙⦑Year⦒` - date tag for the year
    ///
    /// Handles multi-author citations with ∧ separator.
    ///
    /// # Examples
    ///
    /// ```
    /// use eris::notation::CompoundTag;
    /// use eris::notation::Notation;
    ///
    /// let tag = CompoundTag::parse("⚘⊙⊳⦑Mary Douglas⦒⦑1966⦒⦑Purity and Danger⦒").unwrap();
    /// let implied = tag.implied_tag_names();
    /// assert!(implied.contains(&"⚘⦑Mary Douglas⦒".to_string()));
    /// assert!(implied.contains(&"⊙⦑1966⦒".to_string()));
    ///
    /// // Multi-author
    /// let tag2 = CompoundTag::parse("⚘⊙⊳⦑George Lakoff∧Mark Johnson⦒⦑1980⦒⦑Metaphors We Live By⦒").unwrap();
    /// let implied2 = tag2.implied_tag_names();
    /// assert!(implied2.contains(&"⚘⦑George Lakoff⦒".to_string()));
    /// assert!(implied2.contains(&"⚘⦑Mark Johnson⦒".to_string()));
    /// ```
    pub fn implied_tag_names(&self) -> Vec<String> {
        let mut implied = Vec::new();

        if self.is_citation() {
            // Determine the author symbol (⚘ for person, ⍚ for organization)
            let author_symbol = self.symbols.first().copied().unwrap_or('⚘');

            // Extract author(s)
            if let Some(author_str) = self.author() {
                // Handle multi-author with ∧ separator (only for person authors)
                if author_symbol == '⚘' && author_str.contains(CONJUNCTION) {
                    for author in split_conjunction(author_str) {
                        if !author.is_empty() {
                            implied.push(format!(
                                "{}{}{}{}",
                                author_symbol, BRACKET_OPEN, author, BRACKET_CLOSE
                            ));
                        }
                    }
                } else {
                    implied.push(format!(
                        "{}{}{}{}",
                        author_symbol, BRACKET_OPEN, author_str, BRACKET_CLOSE
                    ));
                }
            }

            // Extract year
            if let Some(year) = self.year() {
                implied.push(format!("⊙{}{}{}", BRACKET_OPEN, year, BRACKET_CLOSE));
            }

            // Extract journal/venue for article citations (those with 𝄏 symbol)
            if self.has_symbol('𝄏') {
                if let Some(journal) = self.get_named("journal") {
                    implied.push(format!("𝄏{}{}{}", BRACKET_OPEN, journal, BRACKET_CLOSE));
                }
            }
        }

        implied
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Utility Functions
// ═══════════════════════════════════════════════════════════════════════════

/// Extract display name from ERIS tag (first component content)
///
/// # Examples
///
/// - `⚘⦑Mary Douglas⦒` → `Mary Douglas`
/// - `⧊⦑Knowledge⦒` → `Knowledge`
/// - `⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Title⦒` → `George Orwell`
pub fn extract_display_name(tag: &str) -> String {
    if let Some(start) = tag.find(BRACKET_OPEN) {
        if let Some(end) = tag[start..].find(BRACKET_CLOSE) {
            return tag[start + BRACKET_OPEN.len_utf8()..start + end].to_string();
        }
    }
    tag.to_string()
}

/// Get entity type name from tag symbol
///
/// Returns human-readable type like "Person", "Concept", "Citation", etc.
pub fn get_entity_type_name(tag: &str) -> String {
    if let Some(parsed) = CompoundTag::parse(tag) {
        if parsed.is_citation() {
            return "Citation".to_string();
        }
        if let Some(&first) = parsed.symbols.first() {
            if let Some(entity_type) = EntityType::from_symbol(first) {
                return entity_type.name().to_string();
            }
        }
    }
    "Other".to_string()
}

/// Compose a simple ERIS tag from symbol and content.
///
/// # Examples
///
/// ```
/// use eris::notation::entity::compose_simple;
///
/// assert_eq!(compose_simple("⧊", "Irony"), Some("⧊⦑Irony⦒".to_string()));
/// assert_eq!(compose_simple("⚘", "Mary Douglas"), Some("⚘⦑Mary Douglas⦒".to_string()));
/// assert_eq!(compose_simple("⧊", "  "), None); // Empty content
/// ```
pub fn compose_simple(symbol: &str, content: &str) -> Option<String> {
    let content = content.trim();
    if content.is_empty() {
        None
    } else {
        Some(format!(
            "{}{}{}{}",
            symbol, BRACKET_OPEN, content, BRACKET_CLOSE
        ))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_person_tag() {
        let tag = CompoundTag::parse("⚘⦑Albert Einstein⦒").unwrap();
        assert_eq!(tag.symbols, vec!['⚘']);
        assert_eq!(tag.components, vec!["Albert Einstein"]);
        assert!(tag.is_person());
        assert!(tag.is_simple());
        assert_eq!(tag.person(), Some("Albert Einstein"));
    }

    #[test]
    fn test_parse_citation_tag() {
        let tag = CompoundTag::parse("⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒").unwrap();
        assert_eq!(tag.symbols, vec!['⚘', '⊙', '⊳']);
        assert_eq!(
            tag.components,
            vec!["George Orwell", "1949", "Nineteen Eighty-Four"]
        );
        assert!(tag.is_citation());
        assert!(!tag.is_simple());
        assert_eq!(tag.author(), Some("George Orwell"));
        assert_eq!(tag.year(), Some("1949"));
        assert_eq!(tag.title(), Some("Nineteen Eighty-Four"));
    }

    #[test]
    fn test_notation_lookup() {
        let tag = CompoundTag::parse("⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒").unwrap();
        let notation = tag.notation().unwrap();
        assert_eq!(notation.name, "BookCitation");
        assert_eq!(tag.get_named("author"), Some("George Orwell"));
        assert_eq!(tag.get_named("year"), Some("1949"));
        assert_eq!(tag.get_named("title"), Some("Nineteen Eighty-Four"));
    }

    #[test]
    fn test_parse_article_citation() {
        let tag =
            CompoundTag::parse("⚘⊙𝄏⊳⦑Jane Smith⦒⦑2020⦒⦑Nature⦒⦑A Great Discovery⦒").unwrap();
        assert!(tag.is_citation());
        assert!(tag.has_symbol('𝄏'));
        assert_eq!(tag.author(), Some("Jane Smith"));
        assert_eq!(tag.title(), Some("A Great Discovery"));

        let notation = tag.notation().unwrap();
        assert_eq!(notation.name, "ArticleCitation");
        assert_eq!(tag.get_named("journal"), Some("Nature"));
    }

    #[test]
    fn test_parse_date_tag() {
        let tag = CompoundTag::parse("⊙⦑1905-03⦒").unwrap();
        assert!(tag.is_date());
        assert!(tag.is_simple());
        assert_eq!(tag.date(), Some("1905-03"));
    }

    #[test]
    fn test_multi_author() {
        let tag = CompoundTag::parse("⚘⊙⊳⦑George Lakoff∧Mark Johnson⦒⦑1980⦒⦑Metaphors We Live By⦒").unwrap();
        assert_eq!(tag.authors(), vec!["George Lakoff", "Mark Johnson"]);
        assert!(tag.contains_person("George Lakoff"));
        assert!(tag.contains_person("Mark Johnson"));
        assert!(!tag.contains_person("Lakoff"));
    }

    #[test]
    fn test_contains_person() {
        let citation =
            CompoundTag::parse("⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒").unwrap();
        assert!(citation.contains_person("George Orwell"));
        assert!(!citation.contains_person("Orwell"));

        let person = CompoundTag::parse("⚘⦑George Orwell⦒").unwrap();
        assert!(person.contains_person("George Orwell"));
    }

    #[test]
    fn test_invalid_tag() {
        assert!(CompoundTag::parse("plain text").is_none());
        assert!(CompoundTag::parse("⚘").is_none());
    }

    #[test]
    fn test_entity_types() {
        let tag = CompoundTag::parse("⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒").unwrap();
        let types = tag.entity_types();
        assert!(types.contains(&EntityType::Person));
        assert!(types.contains(&EntityType::Date));
        assert!(types.contains(&EntityType::Work));
        assert!(!types.contains(&EntityType::Concept));
    }

    #[test]
    fn test_has_entity_type() {
        let tag = CompoundTag::parse("⧊⦑Epistemology⦒").unwrap();
        assert!(tag.has_entity_type(EntityType::Concept));
        assert!(!tag.has_entity_type(EntityType::Person));
    }

    #[test]
    fn test_extract_display_name() {
        assert_eq!(extract_display_name("⚘⦑Mary Douglas⦒"), "Mary Douglas");
        assert_eq!(extract_display_name("⧊⦑Knowledge⦒"), "Knowledge");
        assert_eq!(extract_display_name("plain"), "plain");
    }

    #[test]
    fn test_get_entity_type_name() {
        assert_eq!(get_entity_type_name("⚘⦑Name⦒"), "Person");
        assert_eq!(get_entity_type_name("⧊⦑Concept⦒"), "Concept");
        assert_eq!(
            get_entity_type_name("⚘⊙⊳⦑A⦒⦑B⦒⦑C⦒"),
            "Citation"
        );
        assert_eq!(get_entity_type_name("plain"), "Other");
    }

    #[test]
    fn test_render() {
        let tag = CompoundTag::parse("⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒").unwrap();
        assert_eq!(
            tag.render(),
            "⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒"
        );
    }

    #[test]
    fn test_compose_simple() {
        assert_eq!(
            compose_simple("⧊", "Irony"),
            Some("⧊⦑Irony⦒".to_string())
        );
        assert_eq!(
            compose_simple("⚘", "Mary Douglas"),
            Some("⚘⦑Mary Douglas⦒".to_string())
        );
        assert_eq!(compose_simple("⧊", "  "), None);
    }

    #[test]
    fn test_implied_tag_names() {
        let tag =
            CompoundTag::parse("⚘⊙⊳⦑Mary Douglas⦒⦑1966⦒⦑Purity and Danger⦒").unwrap();
        let implied = tag.implied_tag_names();
        assert!(implied.contains(&"⚘⦑Mary Douglas⦒".to_string()));
        assert!(implied.contains(&"⊙⦑1966⦒".to_string()));

        // Multi-author
        let tag2 = CompoundTag::parse(
            "⚘⊙⊳⦑George Lakoff∧Mark Johnson⦒⦑1980⦒⦑Metaphors We Live By⦒",
        )
        .unwrap();
        let implied2 = tag2.implied_tag_names();
        assert!(implied2.contains(&"⚘⦑George Lakoff⦒".to_string()));
        assert!(implied2.contains(&"⚘⦑Mark Johnson⦒".to_string()));

        // Article citation with journal venue
        let tag3 = CompoundTag::parse(
            "⚘⊙𝄏⊳⦑Larry Frohman⦒⦑2020⦒⦑German History⦒⦑Network Euphoria⦒",
        )
        .unwrap();
        let implied3 = tag3.implied_tag_names();
        assert!(implied3.contains(&"⚘⦑Larry Frohman⦒".to_string()));
        assert!(implied3.contains(&"⊙⦑2020⦒".to_string()));
        assert!(implied3.contains(&"𝄏⦑German History⦒".to_string()));

        // Organization author citation
        let tag4 = CompoundTag::parse(
            "⍚⊙⊳⦑The Church of Jesus Christ of Latter-day Saints⦒⦑2020⦒⦑General Handbook⦒",
        )
        .unwrap();
        let implied4 = tag4.implied_tag_names();
        assert!(implied4.contains(&"⍚⦑The Church of Jesus Christ of Latter-day Saints⦒".to_string()));
        assert!(implied4.contains(&"⊙⦑2020⦒".to_string()));
        assert_eq!(implied4.len(), 2);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // Additional Parsing Tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_parse_all_entity_types() {
        // Test parsing for each entity type symbol
        let test_cases = vec![
            ("⚘⦑Person Name⦒", '⚘', "Person Name"),
            ("⌖⦑New York⦒", '⌖', "New York"),
            ("⧖⦑Victorian Era⦒", '⧖', "Victorian Era"),
            ("⊙⦑1984⦒", '⊙', "1984"),
            ("⌁⦑French Revolution⦒", '⌁', "French Revolution"),
            ("❖⦑Sociology⦒", '❖', "Sociology"),
            ("⧈⦑Intellectuals⦒", '⧈', "Intellectuals"),
            ("⍚⦑UNESCO⦒", '⍚', "UNESCO"),
            ("⎈⦑Collective Action⦒", '⎈', "Collective Action"),
            ("⌬⦑Printing Press⦒", '⌬', "Printing Press"),
            ("⧊⦑Reflexivity⦒", '⧊', "Reflexivity"),
            ("⧏⦑Ethnography⦒", '⧏', "Ethnography"),
            ("⌯⦑Structuralism⦒", '⌯', "Structuralism"),
            ("⇋⦑Power-Knowledge⦒", '⇋', "Power-Knowledge"),
            ("⧆⦑Agency-Structure⦒", '⧆', "Agency-Structure"),
            ("⟗⦑Feedback Loop⦒", '⟗', "Feedback Loop"),
            ("☯⦑Observer Paradox⦒", '☯', "Observer Paradox"),
            ("⧃⦑Medicalization⦒", '⧃', "Medicalization"),
            ("⟴⦑Naming⦒", '⟴', "Naming"),
            ("⬢⦑Normalization⦒", '⬢', "Normalization"),
            ("⑀⦑To Review⦒", '⑀', "To Review"),
            ("⋯⦑What is knowledge?⦒", '⋯', "What is knowledge?"),
            ("◈⦑Dissertation⦒", '◈', "Dissertation"),
            ("⟡⦑New Theory⦒", '⟡', "New Theory"),
        ];

        for (input, expected_symbol, expected_content) in test_cases {
            let tag = CompoundTag::parse(input)
                .unwrap_or_else(|| panic!("Failed to parse: {}", input));
            assert_eq!(
                tag.symbols,
                vec![expected_symbol],
                "Symbol mismatch for {}",
                input
            );
            assert_eq!(
                tag.components,
                vec![expected_content],
                "Content mismatch for {}",
                input
            );
            assert!(tag.is_simple(), "Should be simple tag: {}", input);
        }
    }

    #[test]
    fn test_parse_with_special_characters() {
        // Test content with special characters
        let tag = CompoundTag::parse("⧊⦑Self-Reference & Recursion⦒").unwrap();
        assert_eq!(tag.components[0], "Self-Reference & Recursion");

        let tag2 = CompoundTag::parse("⚘⦑José García-López⦒").unwrap();
        assert_eq!(tag2.components[0], "José García-López");

        let tag3 = CompoundTag::parse("⧊⦑\"Quoted\" Concept⦒").unwrap();
        assert_eq!(tag3.components[0], "\"Quoted\" Concept");
    }

    #[test]
    fn test_parse_empty_cases() {
        // Empty string
        assert!(CompoundTag::parse("").is_none());

        // Just symbol, no brackets
        assert!(CompoundTag::parse("⚘").is_none());

        // Empty brackets
        assert!(CompoundTag::parse("⚘⦑⦒").is_none());

        // Just brackets, no symbol
        assert!(CompoundTag::parse("⦑Content⦒").is_some()); // This is valid, symbol is empty but content exists
    }

    #[test]
    fn test_parse_malformed_brackets() {
        // Missing close bracket
        assert!(CompoundTag::parse("⚘⦑No Close").is_none());

        // Multiple unclosed brackets - parses what it can
        let tag = CompoundTag::parse("⚘⦑First⦒⦑Second");
        assert!(tag.is_some());
        let t = tag.unwrap();
        assert_eq!(t.components, vec!["First"]); // Only parses complete brackets
    }

    #[test]
    fn test_parse_whitespace_handling() {
        // Content is preserved as-is (no trimming inside brackets)
        let tag = CompoundTag::parse("⚘⦑ Name with spaces ⦒").unwrap();
        assert_eq!(tag.components[0], " Name with spaces ");

        // Multiple spaces in content
        let tag2 = CompoundTag::parse("⧊⦑Two  Words⦒").unwrap();
        assert_eq!(tag2.components[0], "Two  Words");
    }

    #[test]
    fn test_parse_unicode_content() {
        // Chinese characters
        let tag = CompoundTag::parse("⚘⦑孔子⦒").unwrap();
        assert_eq!(tag.components[0], "孔子");

        // Arabic
        let tag2 = CompoundTag::parse("⌖⦑القاهرة⦒").unwrap();
        assert_eq!(tag2.components[0], "القاهرة");

        // Mixed scripts
        let tag3 = CompoundTag::parse("⧊⦑概念 Concept⦒").unwrap();
        assert_eq!(tag3.components[0], "概念 Concept");
    }

    #[test]
    fn test_parse_multi_component_variations() {
        // Two components
        let tag = CompoundTag::parse("⚘⊙⦑Author⦒⦑2000⦒").unwrap();
        assert_eq!(tag.symbols, vec!['⚘', '⊙']);
        assert_eq!(tag.components, vec!["Author", "2000"]);

        // Four components (article citation)
        let tag2 = CompoundTag::parse("⚘⊙𝄏⊳⦑Smith⦒⦑2020⦒⦑Nature⦒⦑Title⦒").unwrap();
        assert_eq!(tag2.components.len(), 4);
    }

    #[test]
    fn test_render_roundtrip() {
        let inputs = vec![
            "⚘⦑Mary Douglas⦒",
            "⧊⦑Reflexivity⦒",
            "⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒",
            "⚘⊙𝄏⊳⦑Jane Smith⦒⦑2020⦒⦑Nature⦒⦑Discovery⦒",
        ];

        for input in inputs {
            let tag = CompoundTag::parse(input).unwrap();
            assert_eq!(tag.render(), input, "Roundtrip failed for: {}", input);
        }
    }

    #[test]
    fn test_conjunction_in_components() {
        // Multi-author with spaces around conjunction
        let tag =
            CompoundTag::parse("⚘⊙⊳⦑Alice Smith ∧ Bob Jones⦒⦑2020⦒⦑Paper⦒").unwrap();
        let authors = tag.authors();
        assert_eq!(authors, vec!["Alice Smith", "Bob Jones"]);

        // Three authors
        let tag2 =
            CompoundTag::parse("⚘⊙⊳⦑A∧B∧C⦒⦑2020⦒⦑Paper⦒").unwrap();
        let authors2 = tag2.authors();
        assert_eq!(authors2, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_get_by_index() {
        let tag = CompoundTag::parse("⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒").unwrap();
        assert_eq!(tag.get(0), Some("Author"));
        assert_eq!(tag.get(1), Some("Year"));
        assert_eq!(tag.get(2), Some("Title"));
        assert_eq!(tag.get(3), None);
        assert_eq!(tag.get(100), None);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DatedEvent Compound Type Tests
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_parse_dated_event() {
        // ⌁⊙ = DatedEvent: ⌁(Event) then ⊙(Year)
        let tag = CompoundTag::parse("⌁⊙⦑Attack on Pearl Harbor⦒⦑1941⦒").unwrap();
        assert_eq!(tag.symbols, vec!['⌁', '⊙']);
        assert_eq!(tag.components, vec!["Attack on Pearl Harbor", "1941"]);

        // Verify notation lookup works
        let notation = tag.notation().unwrap();
        assert_eq!(notation.name, "DatedEvent");

        // Verify named component access
        assert_eq!(tag.get_named("event"), Some("Attack on Pearl Harbor"));
        assert_eq!(tag.get_named("year"), Some("1941"));
    }

    #[test]
    fn test_dated_event_variations() {
        // Discrete historical events: ⌁⊙⦑event⦒⦑year⦒
        let cases = vec![
            ("⌁⊙⦑Apollo 11 Moon Landing⦒⦑1969⦒", "Apollo 11 Moon Landing", "1969"),
            ("⌁⊙⦑Signing of the Magna Carta⦒⦑1215⦒", "Signing of the Magna Carta", "1215"),
            ("⌁⊙⦑Wright Brothers First Flight⦒⦑1903⦒", "Wright Brothers First Flight", "1903"),
            ("⌁⊙⦑Assassination of Julius Caesar⦒⦑44 BCE⦒", "Assassination of Julius Caesar", "44 BCE"),
        ];

        for (input, expected_event, expected_year) in cases {
            let tag = CompoundTag::parse(input).unwrap();
            assert_eq!(tag.get_named("event"), Some(expected_event), "Failed for: {}", input);
            assert_eq!(tag.get_named("year"), Some(expected_year), "Failed for: {}", input);
        }
    }

    #[test]
    fn test_dated_event_roundtrip() {
        let input = "⌁⊙⦑Coronation of Charlemagne⦒⦑800⦒";
        let tag = CompoundTag::parse(input).unwrap();
        assert_eq!(tag.render(), input);
    }

    #[test]
    fn test_dated_event_is_not_citation() {
        // DatedEvent should not be confused with citation
        let tag = CompoundTag::parse("⌁⊙⦑Gutenberg Bible Printed⦒⦑1455⦒").unwrap();
        assert!(!tag.is_citation());
        assert!(!tag.is_person());
    }
}
