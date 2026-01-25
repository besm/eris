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
                ("≡", ["tripartite_entrainment", "mutual_influence", "structural_coupling"]),
                ("≝", "agent∧category∧behavior ∧ elements≥3 ∧ mutual influence"),
                ("∂", [
                    "⇋⊅⧆ (⇋ tripartite≥3 | ⧆ dyadic binary)",
                    "⇋⊅⟗ (⇋ structural coupling | ⟗ iterating feedback)"
                ]),
                ("⊛", [
                    "⇋⦑Classification-Person-Behavior|Diagnosis-Patient-Symptom|Label-Group-Action⦒",
                    "⇋⦑Market-Investor-Price|Institution-Role-Practice|Norm-Agent-Conduct⦒"
                ]),
                ("◻", [
                    "structure □mandatory:",
                    "  agent (human/organizational) ∧ category (classificatory) ∧ behavior (action/response)",
                    "  elements≥3 ∧ mutual influence evident",
                    "discrimination:",
                    "  ⇋⦑Diagnosis-Patient-Symptom⦒ (tripartite) vs ⧆⦑Mind vs Body⦒ (dyadic)",
                    "  elements=2→⧆ | elements≥3→⇋"
                ]),
                ("≟", [
                    "test{elements≥3∧mutual influence?→YES:⇋|NO:continue}",
                    "test{dyadic binary?→YES:⧆|NO:review}"
                ]),
                ("⊨", "⇋ ≡ tripartite_entrainment ∧ □elements≥3")
            ],
        },
    ]
}