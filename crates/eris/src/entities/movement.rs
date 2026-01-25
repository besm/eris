//! ⌯ Movement entity type

define_entity_module! {
    Entity {
        Conceptual => "Conceptual entity types",
    }
}

type MovementDef = EntityDef;

pub fn get_entity_definitions() -> Vec<MovementDef> {
    vec![
        MovementDef {
            symbol: "⌯",
            name: "Movement",
            description: "Social movement with sustained mobilization and collective identity",
            sort_order: 22,
            category: EntityCategory::Conceptual,
            lines: lines![
                ("≡", "social movement ∧ collective action"),
                ("≝", "sustained mobilization ∧ political activism ∧ ideological coherence"),
                ("∂", [
                    "⌯⊅{⍚formal organizations,⧈human categories,⧖eras,❖fields}",
                    "⌯⊅⚐ (⌯ ¬nominates_candidates | ⚐ nominates_candidates)"
                ]),
                ("◻", "◻{social mobilization,collective identity,political/social goals}"),
                ("⊛", "⌯⦑Civil Rights Movement|Feminism|Jansenism|Frankfurt School⦒"),
                ("◻", [
                    "Movement naming descriptive|ideological label:",
                    "  political{⌯⦑Civil Rights Movement|Labor Movement|Anti-War Movement⦒}",
                    "  intellectual{⌯⦑Frankfurt School|Jansenism|Port-Royal⦒when tradition/movement}",
                    "  religious{⌯⦑Protestantism|Jansenism⦒}",
                    "  social{⌯⦑Feminism|Environmentalism⦒}",
                    "Movement vs institution movement/tradition→⌯|actual institute→⍚:",
                    "  ⌯⦑Frankfurt School⦒(intellectual tradition¬actual institution)",
                    "  ⍚⦑Institute for Social Research⦒(actual institution)",
                    "  case-by-case{DELETE organization when concept/movement already captured:",
                    "    ⌯⦑Port-Royal⦒→⌯⦑Jansenism⦒when ⧊⦑Jansenism⦒already present}",
                    "  usage context determines classification"
                ]),
                ("≟", [
                    "sustained mobilization∧collective identity→⌯",
                    "formal organization→⍚",
                    "intellectual tradition¬institute→⌯",
                    "academic field→❖",
                    "nominates_candidates→⚐",
                    "rejects_electoralism∧'party'_name→⌯: form decoupled from function"
                ]),
                ("⊨", "⌯⊂social movements ∧ sustained mobilization"),
            ],
        },
    ]
}
