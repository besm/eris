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
                ("≡", "paradox ∧ contradictory coexistence"),
                ("≝", "simultaneous contradiction ∧ A∧¬A within single entity ∧ irresolvable tension"),
                ("∂", "☯⊅{⧆external opposition,⟗temporal alternation}"),
                ("◻", "◻{simultaneity,single entity locus,genuine contradiction}"),
                ("⊛", "☯⦑Paradox|Social Science of Change|Counterintuitive Policy|Pyrrhic Victory⦒"),
                ("◻", [
                    "Contradictory coexistence within single entity:",
                    "  simultaneous{A∧¬A same moment same entity}",
                    "  ∂¬external dyadic opposition(use ⧆)",
                    "  ∂¬temporal alternation(use ⟗)",
                    "  genuine irresolvable contradiction"
                ]),
                ("≟", [
                    "A∧¬A simultaneously within single?→☯",
                    "external binary opposition?→⧆",
                    "temporal alternation cycle?→⟗"
                ]),
                ("⊨", "☯⊂simultaneous contradictions ∧ single entity locus ∧ irresolvable"),
            ],
        },
    ]
}
