//! ⧃ Transformation entity type

define_entity_module! {
    Entity {
        Complex => "Complex/process entity types",
    }
}

type TransformationDef = EntityDef;

pub fn get_entity_definitions() -> Vec<TransformationDef> {
    vec![
        TransformationDef {
            symbol: "⧃",
            name: "Transformation",
            description: "Unidirectional temporal transformation with sequential developmental stages",
            sort_order: 40,
            category: EntityCategory::Complex,
            lines: lines![
                ("≡", "transformation ∧ directional change"),
                ("≝", "directional transformation A→B→C ∧ temporal progression ∧ developmental stages ∧ unidirectional sequence"),
                ("∂", "⧃⊅{☯static paradox,⟗bidirectional feedback,⧆dyadic opposition,⬢simultaneous}"),
                ("∂", "⧃⊅⧊_transformation_concepts (analyzing change ≠ describing specific transformation)"),
                ("◻", "◻{temporal sequence,progression direction,irreversibility}"),
                ("⊛", "⧃⦑Magic→Religion→Science|Primitive→Modern Schema|Medicalization|Secularization|Slang Evolution⦒"),
                ("⧠", [
                    "⧃⦑Medicalization⦒: condition→medical frame (unidirectional, no return, no feedback)",
                    "⧃⦑Secularization⦒: sacred→secular shift (directional historical process)",
                    "⬢⦑Making Up People⦒: classification constitutes classified (simultaneity, no sequence)",
                    "⟗⦑Multiple Personality Formation Loop⦒: diagnosis↔behavior↔criteria (bidirectional, returns)"
                ]),
                ("⊡", "typical: Շ7Գ6Բ8"),
                ("⊡", "⧃⦑Medicalization⦒: Շ8Գ7Բ9"),
                ("◻", [
                    "Directional arrows→for sequences:",
                    "  ⧃⦑A→B→C⦒",
                    "  descriptive transformation names",
                    "  temporal requirement{must describe change over time¬simultaneous states}",
                    "  directionality{unidirectional→¬bidirectional↔}",
                    "  composition{⧃∧⧆ valid,⧃∧⇋ valid}",
                    "3-test algorithm:",
                    "  test1 temporal{stages separated t₀<t₁<t₂?→continue|simultaneous t₀=t₀?→⬢}",
                    "  test2 directionality{A→B unidirectional?→continue|A↔B bidirectional?→⟗}",
                    "  test3 progression{irreversible transformation?→⧃|cyclical return?→⟗|static opposition?→⧆}"
                ]),
                ("≟", [
                    "○₁ discourse check: describing specific transformation?→continue | analyzing change concept?→⧊",
                    "○₂ \"A becomes B over time\"→⧃",
                    "○₃ \"A cycles back to A\"→⟗",
                    "○₄ \"Classified as A makes one A\"→⬢"
                ]),
                ("⊨", "⧃⊂temporal transformations ∧ directional progression ∧ sequential stages ∧ unidirectional ∧ irreversible"),
            ],
        },
    ]
}