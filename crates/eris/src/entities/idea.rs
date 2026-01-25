//! ⟡ Idea entity type
//! User-defined idea/seed tags for capturing conceptual seeds

define_entity_module! {
    Entity {
        UserDefined => "User-defined marker types",
    }
}

type IdeaDef = EntityDef;

pub fn get_entity_definitions() -> Vec<IdeaDef> {
    vec![
        IdeaDef {
            symbol: "⟡",
            name: "Idea",
            description: "User-defined idea marker for user's own conceptual seeds",
            sort_order: 103,
            category: EntityCategory::UserDefined,
            lines: lines![
                ("≡", ["idea_marker", "conceptual_seed", "owned_thought"]),
                ("≝", "user's own idea ∧ implementation potential"),
                ("∂", [
                    "∂ user-defined (¬ERIS detection)",
                    "⟡⊅⧊ (⟡ mine | ⧊ in the world)",
                    "⟡⊅◈ (⟡ nascent | ◈ bounded project)"
                ]),
                ("◻", "lifecycle: ⟡→◈ (idea matures) | ◈→⟡ (project spawns idea)"),
                ("⊨", "⟡ ≡ user_owned_idea ∧ user-defined")
            ],
        },
    ]
}