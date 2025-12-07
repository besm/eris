//! Context specification ⟜⟨⯐⟩
//!
//! Environmental framing for agent operation.
//! Contexts define the situational parameters.

use std::fmt;

/// Context types for agent operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    /// Annotation: marking, labeling, commenting
    Ann,
    /// Workflow: process execution, step sequencing
    Wfl,
    /// Structural: architectural, organizational
    Str,
    /// Epistemic: knowledge-oriented, understanding-focused
    Eps,
}

impl Context {
    pub fn code(&self) -> &'static str {
        match self {
            Context::Ann => "ann",
            Context::Wfl => "wfl",
            Context::Str => "str",
            Context::Eps => "eps",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Context::Ann => "Annotation",
            Context::Wfl => "Workflow",
            Context::Str => "Structural",
            Context::Eps => "Epistemic",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Context::Ann => "Marking highlights, adding tags, commenting on content",
            Context::Wfl => "Executing processes, sequencing steps, managing state",
            Context::Str => "Analyzing architecture, organizing components, structural work",
            Context::Eps => "Building understanding, knowledge synthesis, conceptual work",
        }
    }

    pub fn all() -> &'static [Context] {
        &[Context::Ann, Context::Wfl, Context::Str, Context::Eps]
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟜⟨⯐⟩.{} ≡ {}\n  ≝ {}", 
            self.code(), 
            self.name(),
            self.description()
        )
    }
}

/// Get a context by code
pub fn get_context(code: &str) -> Option<Context> {
    match code {
        "ann" => Some(Context::Ann),
        "wfl" => Some(Context::Wfl),
        "str" => Some(Context::Str),
        "eps" => Some(Context::Eps),
        _ => None,
    }
}
