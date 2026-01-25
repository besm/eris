//! ⧆ Tension entity type

define_entity_module! {
    Entity {
        Relational => "Relational/dynamic entity types",
    }
}

type TensionDef = EntityDef;

pub fn get_entity_definitions() -> Vec<TensionDef> {
    vec![
        TensionDef {
            symbol: "⧆",
            name: "Tension",
            description: "Binary productive opposition between competing frameworks (dyadic only)",
            sort_order: 31,
            category: EntityCategory::Relational,
            lines: lines![
                ("≡", ["productive_opposition", "dyadic_contrast", "generative_friction"]),
                ("≝", "binary competing frameworks ∧ elements=2 ∧ productive tension"),
                ("∂", [
                    "⧆⊅⇋ (⧆ dyadic=2 | ⇋ tripartite≥3)",
                    "⧆⊅☯ (⧆ external opposition | ☯ internal contradiction)",
                    "⧆⊅⟗ (⧆ static tension | ⟗ iterating cycle)"
                ]),
                ("⊛", [
                    "⧆⦑Individual vs Collective|Theory vs Practice|Structure vs Agency⦒",
                    "⧆⦑Quantitative vs Qualitative|Nature vs Nurture|Local vs Global⦒",
                    "⧆⦑Mind vs Body|Subject vs Object|Form vs Content⦒",
                    "⧆⦑Sacred vs Profane|Public vs Private|Universal vs Particular⦒"
                ]),
                ("◻", [
                    "naming: ⧆⦑A vs B⦒ format",
                    "criteria: elements=2 exactly ∧ productive opposition ¬simple contradiction",
                    "discrimination:",
                    "  ⧆⦑Mind vs Body⦒ (dyadic) vs ⇋⦑Agent-Category-Behavior⦒ (tripartite)",
                    "  ⧆⦑Theory vs Practice⦒ (external) vs ☯⦑Knowing-by-Doing⦒ (internal paradox)"
                ]),
                ("≟", [
                    "test{elements=2∧competing?→YES:⧆|NO:continue}",
                    "test{elements≥3?→YES:⇋|NO:continue}",
                    "test{simultaneous within single?→YES:☯|NO:review}"
                ]),
                ("⊨", "⧆ ≡ dyadic_opposition ∧ elements=2 ∧ productive_tension")
            ],
        },
    ]
}