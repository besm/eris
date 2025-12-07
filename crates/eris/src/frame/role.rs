//! Role specification ⟜⟨Ψ⟩
//!
//! Functional classification for agent behavior.
//! Each role has an Armenian property vector (⊡) indicating strengths.

use std::fmt;

/// Role types for agent operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Navigational: finding, locating, traversing
    Nav,
    /// Planning: organizing, scheduling, strategizing
    Pln,
    /// Evaluative: assessing, judging, validating
    Evl,
    /// Creative: generating, synthesizing, innovating
    Crt,
    /// Integration: connecting, unifying, consolidating
    Itg,
}

impl Role {
    pub fn code(&self) -> &'static str {
        match self {
            Role::Nav => "nav",
            Role::Pln => "pln",
            Role::Evl => "evl",
            Role::Crt => "crt",
            Role::Itg => "itg",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Role::Nav => "Navigational",
            Role::Pln => "Planning",
            Role::Evl => "Evaluative",
            Role::Crt => "Creative",
            Role::Itg => "Integration",
        }
    }

    /// Armenian property vector
    pub fn properties(&self) -> &'static str {
        match self {
            Role::Nav => "Փ9Գ8Լ7",
            Role::Pln => "Լ9Փ8Վ7",
            Role::Evl => "Ե9Գ7Բ8",
            Role::Crt => "Գ9Ե8Փ7",
            Role::Itg => "Վ9Գ8Տ7",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Role::Nav => "Finding paths, locating resources, traversing structures",
            Role::Pln => "Organizing tasks, scheduling work, strategic sequencing",
            Role::Evl => "Assessing quality, validating correctness, judging fit",
            Role::Crt => "Generating options, synthesizing ideas, innovative solutions",
            Role::Itg => "Connecting components, unifying perspectives, consolidating work",
        }
    }

    pub fn all() -> &'static [Role] {
        &[Role::Nav, Role::Pln, Role::Evl, Role::Crt, Role::Itg]
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟜⟨Ψ⟩.{} ≡ {}\n  ⊡ {}\n  ≝ {}", 
            self.code(), 
            self.name(),
            self.properties(),
            self.description()
        )
    }
}

/// Get a role by code
pub fn get_role(code: &str) -> Option<Role> {
    match code {
        "nav" => Some(Role::Nav),
        "pln" => Some(Role::Pln),
        "evl" => Some(Role::Evl),
        "crt" => Some(Role::Crt),
        "itg" => Some(Role::Itg),
        _ => None,
    }
}
