//! ☯ Paradox entity type

define_entity_module! {
    Entity {
        Relational => "Relational/dynamic entity types",
    }
}

type ParadoxDef = EntityDef;

pub fn get_entity_definitions() -> Vec<ParadoxDef> {
    vec![
        ParadoxDef {
            symbol: "☯",
            name: "Paradox",
            description: "Simultaneous contradiction (A∧¬A) within single entity at same moment",
            sort_order: 33,
            category: EntityCategory::Relational,
            lines: lines![
                ("≡", ["simultaneous_contradiction", "irresolvable_tension", "single_locus"]),
                ("≝", "A∧¬A within single entity ∧ same moment ∧ genuine contradiction"),
                ("∂", [
                    "☯⊅⧆ (☯ internal contradiction | ⧆ external opposition)",
                    "☯⊅⟗ (☯ simultaneous | ⟗ temporal alternation)"
                ]),
                ("⊛", [
                    "☯⦑Pyrrhic Victory|Catch-22|Double Bind|Bootstrap Paradox⦒",
                    "☯⦑Tolerance Paradox|Sovereignty Paradox|Observer's Paradox⦒",
                    "☯⦑Knowing-by-Doing|Planned Spontaneity|Organized Anarchism⦒",
                    "☯⦑Social Science of Change|Counterintuitive Policy|Unintended Consequences⦒"
                ]),
                ("◻", [
                    "criteria: A∧¬A ∧ same moment ∧ same entity ∧ irresolvable",
                    "discrimination:",
                    "  ☯⦑Double Bind⦒ (internal) vs ⧆⦑Mind vs Body⦒ (external)",
                    "  ☯⦑Pyrrhic Victory⦒ (simultaneous) vs ⟗⦑Boom-Bust Cycle⦒ (alternating)"
                ]),
                ("≟", [
                    "test{A∧¬A simultaneously within single?→YES:☯|NO:continue}",
                    "test{external binary opposition?→YES:⧆|NO:continue}",
                    "test{temporal alternation?→YES:⟗|NO:review}"
                ]),
                ("⊨", "☯ ≡ simultaneous_contradiction ∧ single_locus ∧ irresolvable")
            ],
        },
    ]
}