//! ⊙ Date entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type DateDef = EntityDef;

pub fn get_entity_definitions() -> Vec<DateDef> {
    vec![
        DateDef {
            symbol: "⊙",
            name: "Date",
            description: "Temporal point, publication metadata (YYYY, YYYY-MM, YYYY-MM-DD)",
            sort_order: 4,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["temporal_point", "publication_metadata", "citation_component"]),
                ("≝", "date specificity ∧ bibliographic marker ∧ numeric format"),
                ("∂", [
                    "⊙⊅⧖ (⊙ point | ⧖ named span)",
                    "⊙⊅⌁ (⊙ date | ⌁ event — ¬separate ⊙ for events)"
                ]),
                ("⊛", [
                    "year: ⊙⦑1809|1848|1945|1968|1989|2001|2024⦒",
                    "month: ⊙⦑1848-03|1968-05|2024-11⦒",
                    "day: ⊙⦑1776-07-04|1945-08-06|1989-11-09|2024-03-15⦒"
                ]),
                ("◻", [
                    "format: YYYY|YYYY-MM|YYYY-MM-DD",
                    "  ✓⊙⦑1848|1848-03|1848-03-15⦒",
                    "  ✗⊙⦑March 1848|15 March 1848⦒ — numeric only",
                    "compound citations:",
                    "  ⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒ (book)",
                    "  ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒ (article)",
                    "precision: year default | month for periodicals | day for specific dates"
                ]),
                ("≟", [
                    "test{temporal point?→YES:⊙|NO:continue}",
                    "test{named period?→YES:⧖|NO:continue}",
                    "test{event?→YES:⌁ (¬separate ⊙)|NO:review}"
                ]),
                ("⊨", "⊙ ≡ temporal_point ∧ numeric_format")
            ],
        },
    ]
}