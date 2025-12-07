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
            description: "User-defined question marker for items needing follow-up",
            sort_order: 101,
            category: EntityCategory::UserDefined,
            lines: lines![
                ("≡", ["question", "generative_inquiry"]),
                ("≝", ["◈_orienting_problem", "fertile_unknown"]),
                ("∂", [
                    "⋯⊅⊟ (⋯ fertile | ⊟ needs resolution)",
                    "⋯⊅⌾ (⋯ drives work | ⌾ seeks clarification)",
                    "∂ user-defined (¬ERIS detection)"
                ]),
                ("⊛", [
                    "⋯⦑question_text⦒",
                    "⋯ ⊰ ◈ (questions generate projects)",
                    "❧ → ⋯ (highlights accrete to questions)",
                    "⋯ holds ¬⌺ (anti-crystallization)",
                    "⋯_open | ⋯_dormant | ⋯→☊",
                    "⋯ may persist through ◈_⌺"
                ]),
                ("⊢", "⋯ question ⊢ ❧ accretes ⊢ ◈ oriented"),
            ],
        },
    ]
}
