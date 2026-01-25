//! § Section entity type
//! User-defined section marker for project/document divisions

define_entity_module! {
    Entity {
        UserDefined => "User-defined marker types",
    }
}

type SectionDef = EntityDef;

pub fn get_entity_definitions() -> Vec<SectionDef> {
    vec![
        SectionDef {
            symbol: "§",
            name: "Section",
            description: "User-defined section marker for document divisions",
            sort_order: 104,
            category: EntityCategory::UserDefined,
            lines: lines![
                ("≡", ["section_marker", "document_division", "organizational_unit"]),
                ("≝", "bounded scope ∧ within ◈ or ⊳"),
                ("∂", [
                    "∂ user-defined (¬ERIS detection)",
                    "§⊂◈ (§ divisions within projects)",
                    "§⊂⊳ (§ divisions within works)"
                ]),
                ("⊛", [
                    "◈§⦑Binding Time⦒⦑The Archive Problem⦒",
                    "◈§⦑LLMs and Feedforward⦒⦑Richards and Anticipation⦒",
                    "◈§⦑Tainted Software⦒⦑Contagion Mechanics⦒"
                ]),
                ("◻", "notation: ◈§⦑Project⦒⦑Section⦒ | §⦑Section⦒ when ◈ implicit"),
                ("⊨", "§ ≡ user_defined ∧ subdivision ∧ bounded_scope")
            ],
        },
    ]
}