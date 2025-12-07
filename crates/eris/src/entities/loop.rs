//! ⟗ Loop entity type

define_entity_module! {
    Entity {
        Relational => "Relational/dynamic entity types",
    }
}

type LoopDef = EntityDef;

pub fn get_entity_definitions() -> Vec<LoopDef> {
    vec![
        LoopDef {
            symbol: "⟗",
            name: "Loop",
            description: "Feedback cycle with bidirectional causality and repeated iteration",
            sort_order: 32,
            category: EntityCategory::Relational,
            lines: lines![
                ("≡", "loop ∧ feedback cycle"),
                ("≝", "recursive reinforcement ∧ bidirectional causality ∧ cyclical pattern"),
                ("∂", "⟗⊅{⬢single moment,⧃unidirectional sequence}"),
                ("∂", "⟗⊅⧊_loop_concepts (analyzing feedback concept ≠ describing specific loop)"),
                ("◻", "◻{bidirectional feedback,repeated cycles,mutual reinforcement}"),
                ("◻", "textual signals: 'fed back', 'reinforced', 'in turn', 'which then', 'cycle'"),
                ("⊛", "⟗⦑Loop|Bandwagon Loop|Error-Confirmation Loop|Multiple Personality Formation Loop⦒"),
                ("⧠", [
                    "⟗⦑Multiple Personality Formation Loop⦒: diagnosis→patients exhibit more→criteria expand→therapy elicits more→'way to be a person' (1983→1991 visible iteration, 'fed back' in text)",
                    "⬢⦑Making Up People⦒: classification constitutes classified (simultaneity, no iteration visible)",
                    "⧃⦑Medicalization⦒: condition→medical frame (unidirectional transformation, no return)"
                ]),
                ("⊡", "Տ(iteration visibility)Բ(bidirectionality)Գ(cycle count)Դ(dynamics: dampen↔amplify)"),
                ("⊡", "typical: Տ7Բ8Գ4Դ6"),
                ("⊡", "⟗⦑Multiple Personality Formation Loop⦒: Տ9Բ8Գ5Դ9"),
                ("≟", [
                    "○₁ discourse check: describing loop phenomenon?→continue | analyzing loop concept?→⧊",
                    "○₂ bidirectional∧repeated?→⟗",
                    "○₃ unidirectional?→⧃",
                    "○₄ single moment constitution?→⬢"
                ]),
                ("⊨", "⟗⊂feedback cycles ∧ bidirectional reinforcement ∧ repeated iteration"),
            ],
        },
    ]
}