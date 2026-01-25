//! ◈ Project entity type
//! User-defined project/endeavor tags for organizing work

define_entity_module! {
    Entity {
        UserDefined => "User-defined marker types",
    }
}

type ProjectDef = EntityDef;

pub fn get_entity_definitions() -> Vec<ProjectDef> {
    vec![
        ProjectDef {
            symbol: "◈",
            name: "Project",
            description: "User-defined project marker for bounded endeavors",
            sort_order: 102,
            category: EntityCategory::UserDefined,
            lines: lines![
                ("≡", ["project_marker", "bounded_endeavor", "goal_oriented"]),
                ("≝", "user's own project ∧ scoped work ∧ aggregates ❧"),
                ("∂", [
                    "∂ user-defined (¬ERIS detection)",
                    "◈⊅⟡ (◈ bounded | ⟡ nascent idea)",
                    "◈⊅⋯ (◈ project | ⋯ orienting question)"
                ]),
                ("⊛", [
                    "◈⦑Binding Time|LLMs and Feedforward|Tainted Software⦒",
                    "◈⦑Ronpaulization|The Intimate Machine|Classifier Stalinism⦒",
                    "◈⦑Context Widows|Politically Unreliable|Secularism Studies Project⦒"
                ]),
                ("◻", [
                    "lifecycle: ⟡→◈ (idea matures) | ◈→⟡ (project spawns idea)",
                    "aggregates: ❧→◈ (highlights gather to projects)",
                    "orients: ⋯⊰◈ (questions generate projects)"
                ]),
                ("⊨", "◈ ≡ user_owned_project ∧ bounded ∧ user-defined")
            ],
        },
    ]
}