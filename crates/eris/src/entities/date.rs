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
                ("≡", "temporal point ∧ publication metadata"),
                ("≝", "date specificity ∧ citation component ∧ bibliographic temporal marker"),
                ("∂", "⊙⊅{⧖,spans} ∧ ◻ compound context(primary usage)"),
                ("⊛", "⊙⦑Year⦒ ∧ ⊙∈{⚘⊕⊙⊕⊳,⚘⊕⊙⊕𝄏⊕⊳}"),
                ("◻", [
                    "Numeric dates ∂¬written:",
                    "  year: ⊙⦑1809|1995|2024⦒",
                    "  month: ⊙⦑1848-03|2024-11⦒",
                    "  day: ⊙⦑1776-07-04|2024-03-15⦒",
                    "  ∂✗written{✗⊙⦑March 2024⦒}",
                    "  format: {YYYY|YYYY-MM|YYYY-MM-DD}",
                    "Primary usage compound citations ◻□mandatory for works:",
                    "  books: {⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒}",
                    "  articles: {⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒}",
                    "  partial: {⚘⊙⦑Author⦒⦑Year⦒}",
                    "  ◻ compound structure: ⊙ never standalone in citations",
                    "Standalone:",
                    "  publication metadata only",
                    "  when: {temporal reference needed∧¬work citation}",
                    "  rare: {most dates appear in compounds}",
                    "Precision levels context determines granularity:",
                    "  year sufficient: {publication year|general temporal reference}",
                    "  month needed: {periodical issues|temporal specificity}",
                    "  day needed: {specific publication date|event date metadata}"
                ]),
                ("≟", [
                    "point→⊙",
                    "named period→⧖",
                    "span→⧖",
                    "event→⌁(¬separate ⊙)"
                ]),
                ("⊨", "⊙⊂temporal points ∧ □citation component"),
            ],
        },
    ]
}
