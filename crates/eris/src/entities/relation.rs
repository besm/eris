//! ⇋ Relation entity type

define_entity_module! {
    Entity {
        Relational => "Relational/dynamic entity types",
    }
}

type RelationDef = EntityDef;

pub fn get_entity_definitions() -> Vec<RelationDef> {
    vec![
        RelationDef {
            symbol: "⇋",
            name: "Relation",
            description: "Tripartite entrainment (agent-category-behavior) requiring ≥3 elements",
            sort_order: 30,
            category: EntityCategory::Relational,
            lines: lines![
                ("≡", "relation ∧ tripartite entrainment"),
                ("≝", "agent∧category∧behavior interaction ∧ mutual influence ∧ structural coupling"),
                ("∂", [
                    "◻□{tripartite:agent∧category∧behavior,elements≥3 mandatory}",
                    "∂{¬dyadic use ⧆,¬generic relationships}"
                ]),
                ("⊛", "⇋⦑Relation⦒ with tripartite structure"),
                ("◻", [
                    "Tripartite structure ◻□mandatory:",
                    "  agent{human/organizational actor}",
                    "  category{classificatory system}",
                    "  behavior{action/response pattern}",
                    "  elements≥3(¬dyadic)",
                    "  mutual influence evident",
                    "∂dyadic dyadic relations→⧆¬⇋:",
                    "  binary oppositions→⧆⦑A vs B⦒",
                    "  ∂¬⇋for 2 elements",
                    "  tripartite requirement strict"
                ]),
                ("≟", [
                    "elements≥3∧mutual influence?→⇋",
                    "dyadic binary?→⧆",
                    "general connection?→review"
                ]),
                ("⊨", "⇋⊂tripartite entrainments ∧ □3 elements"),
            ],
        },
    ]
}
