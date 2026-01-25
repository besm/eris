//! ⊳ Work entity type

define_entity_module! {
    Entity {
        Compound => "Compound citation component types",
    }
}

type WorkDef = EntityDef;

pub fn get_entity_definitions() -> Vec<WorkDef> {
    vec![
        WorkDef {
            symbol: "⊳",
            name: "Work",
            description: "Work title in compound citations (main title only, no subtitles)",
            sort_order: 50,
            category: EntityCategory::Compound,
            lines: lines![
                ("≡", ["work_title", "citation_component", "compound_only"]),
                ("≝", "bibliographic reference ∧ main title only ∧ □compound form"),
                ("∂", [
                    "⊳⊅𝄏 (⊳ standalone work | 𝄏 serial periodical)",
                    "⊳⊅⍚ (⊳ work | ⍚ publisher organization)",
                    "✗⊳⦑Title⦒ standalone — □ requires ⚘⊙ prefix"
                ]),
                ("⊛", [
                    "⚘⊙⊳⦑Thomas S. Kuhn⦒⦑1962⦒⦑The Structure of Scientific Revolutions⦒",
                    "⚘⊙⊳⦑Michel Foucault⦒⦑1975⦒⦑Discipline and Punish⦒",
                    "⚘⊙⊳⦑Edward Said⦒⦑1978⦒⦑Orientalism⦒",
                    "⚘⊙⊳⦑Kenneth Burke⦒⦑1945⦒⦑A Grammar of Motives⦒",
                    "⚘⊙𝄏⊳⦑C.S. Peirce⦒⦑1878⦒⦑Popular Science Monthly⦒⦑How to Make Our Ideas Clear⦒"
                ]),
                ("◻", [
                    "□ compound only: ✓⚘⊙⊳ | ✓⚘⊙𝄏⊳ | ✗⊳ standalone",
                    "CRITICAL — main title only:",
                    "  ✓⊳⦑A Matter of Justice⦒",
                    "  ✗⊳⦑A Matter of Justice: The Legal System in Ferment⦒",
                    "co-authors: ⚘⊙⊳⦑Author1∧Author2⦒⦑Year⦒⦑Title⦒"
                ]),
                ("≟", [
                    "test{compound form ⚘⊙?→YES:continue|NO:reject}",
                    "test{one-time publication?→YES:⊳|NO:continue}",
                    "test{periodic issues?→YES:𝄏|NO:review}"
                ]),
                ("⊨", "⊳ ≡ citation_component ∧ □compound_form ∧ main_title_only")
            ],
        },
    ]
}