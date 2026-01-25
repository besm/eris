//! ⑀ Meta marker entity type
//! User-defined meta markers for arbitrary classification

define_entity_module! {
    Entity {
        UserDefined => "User-defined marker types",
    }
}

type MetaDef = EntityDef;

pub fn get_entity_definitions() -> Vec<MetaDef> {
    vec![
        MetaDef {
            symbol: "⑀",
            name: "Meta",
            description: "User-defined meta marker for arbitrary classification",
            sort_order: 100,
            category: EntityCategory::UserDefined,
            lines: lines![
                ("≡", ["status_marker", "arbitrary_classification", "flexible_tag"]),
                ("≝", "user-defined semantics ∧ no fixed meaning"),
                ("∂", [
                    "∂ user-defined (¬ERIS detection)",
                    "⑀⊅{⟡,⋯,◈} (⑀ arbitrary | ⟡⋯◈ structured)"
                ]),
                ("◻", "catch-all: workflow states | review flags | temporary markers"),
                ("⊨", "⑀ ≡ user_defined ∧ arbitrary_semantics")
            ],
        },
    ]
}