//! Export ERIS definitions for LLM consumption.

use once_cell::sync::Lazy;

/// Full ERIS definitions as a single string, suitable for LLM system prompts.
///
/// This is computed once and cached for the lifetime of the program.
/// The format is optimized for DeepSeek's prefix caching - keeping this
/// identical across requests allows cache hits at ~10% of the cost.
pub static DEFINITIONS: Lazy<String> = Lazy::new(|| {
    super::get_all_definitions().join("\n\n")
});

/// Get the full ERIS definitions for use in an LLM system prompt.
///
/// Returns a static reference to avoid allocations on repeated calls.
/// The definitions include all entity types and operators with their
/// discrimination rules, examples, and validation formulas.
pub fn definitions_for_prompt() -> &'static str {
    &DEFINITIONS
}

/// Get only entity definitions (no operators) for LLM prompts.
///
/// Use this for a smaller prompt when operator definitions aren't needed.
pub static ENTITY_DEFINITIONS: Lazy<String> = Lazy::new(|| {
    super::entities::get_all_definitions().join("\n\n")
});

/// Get entity definitions only.
pub fn entity_definitions_for_prompt() -> &'static str {
    &ENTITY_DEFINITIONS
}

/// Base system prompt with ERIS definitions for cache sharing.
///
/// All tools using DeepSeek should start their system prompt with this
/// to benefit from prefix caching. The ERIS definitions come first,
/// followed by a generic instruction that works for any task.
///
/// Tools can append task-specific instructions after this base.
pub static SYSTEM_PROMPT_BASE: Lazy<String> = Lazy::new(|| {
    format!(
        r#"You are an expert assistant for academic corpus analysis using the ERIS taxonomy.

{}

When referencing entity tags, use them exactly as shown (e.g., ⧊⦑Identity⦒ or ⚘⦑Sherry Turkle⦒).
"#,
        definitions_for_prompt()
    )
});

/// Get the base system prompt for LLM calls.
///
/// This includes full ERIS definitions and should be used as the prefix
/// for all DeepSeek calls to maximize cache hit rates.
pub fn system_prompt_base() -> &'static str {
    &SYSTEM_PROMPT_BASE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_definitions_not_empty() {
        let defs = definitions_for_prompt();
        assert!(!defs.is_empty());
        // Should contain some known entity symbols
        assert!(defs.contains("⚘"));
        assert!(defs.contains("⧊"));
    }

    #[test]
    fn test_entity_definitions_smaller() {
        let full = definitions_for_prompt();
        let entities_only = entity_definitions_for_prompt();
        assert!(entities_only.len() < full.len());
    }

    #[test]
    fn test_system_prompt_base_contains_definitions() {
        let base = system_prompt_base();
        assert!(base.contains("⚘"));
        assert!(base.contains("ERIS"));
    }
}
