//! Task specification and workflows
//!
//! Objective framing for agent operation.
//! Tasks define what the agent is trying to accomplish.
//! Each task can have an associated workflow with steps and validation formulas.

pub mod tag;

use std::fmt;

/// Task types for agent operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Task {
    /// Validate: check correctness, verify quality
    Validate,
    /// Tag: apply entity tags to content
    Tag,
    /// Review: assess and provide feedback
    Review,
    /// Migrate: move/transform content between formats
    Migrate,
    /// Query: search and retrieve information
    Query,
}

impl Task {
    pub fn code(&self) -> &'static str {
        match self {
            Task::Validate => "validate",
            Task::Tag => "tag",
            Task::Review => "review",
            Task::Migrate => "migrate",
            Task::Query => "query",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Task::Validate => "Validate",
            Task::Tag => "Tag",
            Task::Review => "Review",
            Task::Migrate => "Migrate",
            Task::Query => "Query",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Task::Validate => "Check tag correctness, verify entity classification, quality validation",
            Task::Tag => "Apply ERIS entity tags to highlights, entity detection workflow",
            Task::Review => "Assess content, provide feedback, evaluate quality",
            Task::Migrate => "Transform formats, move symbols, update definitions",
            Task::Query => "Search database, retrieve highlights, find patterns",
        }
    }

    pub fn all() -> &'static [Task] {
        &[Task::Validate, Task::Tag, Task::Review, Task::Migrate, Task::Query]
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟜⟨τ⟩.{} ≡ {}\n  ≝ {}", 
            self.code(), 
            self.name(),
            self.description()
        )
    }
}

/// Get a task by code
pub fn get_task(code: &str) -> Option<Task> {
    match code {
        "validate" => Some(Task::Validate),
        "tag" => Some(Task::Tag),
        "review" => Some(Task::Review),
        "migrate" => Some(Task::Migrate),
        "query" => Some(Task::Query),
        _ => None,
    }
}

/// Get workflow by task name
pub fn get_workflow(task_name: &str) -> Option<String> {
    match task_name {
        "tag" => Some(tag::get_inittagging_workflow().to_eris_text()),
        _ => None,
    }
}

/// List all available workflow names
pub fn list_workflows() -> Vec<&'static str> {
    vec!["tag"]
}
