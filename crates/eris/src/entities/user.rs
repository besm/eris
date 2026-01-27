//! ⋖ User in context entity type
//! Self-reference marker for the one operating the system

define_entity_module! {
    Entity {
        UserDefined => "User-defined marker types",
    }
}

type UserDef = EntityDef;

pub fn get_entity_definitions() -> Vec<UserDef> {
    vec![
        UserDef {
            symbol: "⋖",
            name: "User",
            description: "Self-reference marker for user operating the system",
            sort_order: 104,
            category: EntityCategory::UserDefined,
            lines: lines![
                ("≡", ["participant", "operating_subject"]),
                ("≝", "the one navigating the system ∧ first person in ⯐"),
                ("∂", [
                    "⋖≠⚘ (⋖ self | ⚘ named other)",
                    "⋖≠dupe (⋖ co-constitutes | dupe deceived)",
                    "⋖≠customer (⋖ navigates | customer queries)"
                ]),
                ("⊛", [
                    "⋖ ⊰ ⯐ (user shapes context)",
                    "⯐ ⊰ ⋖ (context shapes user)",
                    "⋖ ∈ encounter as participant",
                    "⋖.⥅ → anticipatory apparatus",
                    "⋖.⥅ at stake through use",
                    "⋖.properties → system-specific"
                ]),
                ("⥈", [
                    "⚘⧊⦑I.A. Richards⦒⦑Feedforward⦒",
                    "⚘⧊⦑Sherry Turkle⦒⦑Participant⦒"
                ])
            ],
        },
    ]
}
