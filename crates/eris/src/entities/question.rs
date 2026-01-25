//! ⋯ Question marker entity type
//! User-defined question/probe tags for flagging items needing follow-up

define_entity_module! {
    Entity {
        UserDefined => "User-defined marker types",
    }
}

type QuestionDef = EntityDef;

pub fn get_entity_definitions() -> Vec<QuestionDef> {
    vec![
        QuestionDef {
            symbol: "⋯",
            name: "Question",
            description: "User-defined question marker for generative inquiry",
            sort_order: 101,
            category: EntityCategory::UserDefined,
            lines: lines![
                ("≡", ["generative_inquiry", "fertile_unknown", "orienting_problem"]),
                ("≝", "user's own question ∧ drives work ∧ anti-crystallization"),
                ("∂", [
                    "∂ user-defined (¬ERIS detection)",
                    "⋯⊅⊟ (⋯ fertile | ⊟ needs resolution)",
                    "⋯⊅⌾ (⋯ drives work | ⌾ seeks clarification)",
                    "⋯⊅◈ (⋯ question | ◈ bounded project)"
                ]),
                ("◻", [
                    "lifecycle: ⋯⊰◈ (questions generate projects)",
                    "accretes: ❧→⋯ (highlights gather to questions)",
                    "persistence: ⋯ holds ¬⌺ | ⋯ may persist through ◈_⌺"
                ]),
                ("⊨", "⋯ ≡ user_owned_question ∧ generative ∧ user-defined")
            ],
        },
    ]
}