//! ERIS frame system
//!
//! Operational framing for agent behavior - distinct from entity types.
//! Frames are not printed with `eris all` but accessed via dedicated flags.
//!
//! Frame types:
//!   - Role: functional classification (nav, pln, evl, crt, itg)
//!   - Context: environmental framing (ann, wfl, str, eps)
//!   - Task: objective specification (validate, tag, review, etc.)
//!
//! Frames compose: --role evl --context wfl --task validate
//!
//! Workflows: task-specific documentary workflows for LLM ingestion
//!   - `eris workflow tag` for initTagging workflow

pub mod role;
pub mod context;
pub mod task;

pub use role::{Role, get_role};
pub use context::{Context, get_context};
pub use task::{Task, get_task, get_workflow, list_workflows};

use std::fmt;

/// Composed frame specification
#[derive(Debug, Clone, Default)]
pub struct Frame {
    pub role: Option<Role>,
    pub context: Option<Context>,
    pub task: Option<Task>,
}

impl Frame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_role(mut self, role: Role) -> Self {
        self.role = Some(role);
        self
    }

    pub fn with_context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_task(mut self, task: Task) -> Self {
        self.task = Some(task);
        self
    }

    /// Check if frame has any components
    pub fn is_empty(&self) -> bool {
        self.role.is_none() && self.context.is_none() && self.task.is_none()
    }

    /// Generate unified frame specification symbol
    pub fn spec(&self) -> String {
        let mut parts = Vec::new();
        if let Some(r) = &self.role {
            parts.push(format!("Ψ.{}", r.code()));
        }
        if let Some(c) = &self.context {
            parts.push(format!("⯐.{}", c.code()));
        }
        if let Some(t) = &self.task {
            parts.push(format!("τ.{}", t.code()));
        }
        if parts.is_empty() {
            "⟜⟨∅⟩".to_string()
        } else {
            format!("⟜⟨{}⟩", parts.join("⊗"))
        }
    }

    /// Generate unified description
    pub fn description(&self) -> String {
        let mut descs = Vec::new();
        if let Some(r) = &self.role {
            descs.push(r.name().to_lowercase());
        }
        if let Some(c) = &self.context {
            descs.push(c.name().to_lowercase());
        }
        if let Some(t) = &self.task {
            descs.push(t.name().to_lowercase());
        }
        if descs.is_empty() {
            "empty frame".to_string()
        } else {
            descs.join("∧")
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Unified frame specification
        writeln!(f, "{}", self.spec())?;
        writeln!(f, "  ≡ {}", self.description())?;
        writeln!(f)?;

        // Component definitions
        if let Some(r) = &self.role {
            writeln!(f, "{r}")?;
            writeln!(f)?;
        }
        if let Some(c) = &self.context {
            writeln!(f, "{c}")?;
            writeln!(f)?;
        }
        if let Some(t) = &self.task {
            writeln!(f, "{t}")?;
        }

        Ok(())
    }
}

