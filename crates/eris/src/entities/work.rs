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
                ("≡", "work title ∧ citation component"),
                ("≝", "book/article/work titles ∧ standalone publications ∧ bibliographic citation element"),
                ("∂", "⊳⊅{𝄏periodicals,⍚organizations,❖fields}"),
                ("◻", [
                    "◻{standalone work,bibliographic reference}",
                    "□ Title component = main title only ∧ ¬subtitle ∧ ¬publisher ∧ ¬edition info"
                ]),
                ("⊛", [
                    "⊳∈⚘⊙⊳book citations",
                    "⊳∈⚘⊙𝄏⊳article citations"
                ]),
                ("◻", [
                    "Compound citation usage ◻□mandatory:",
                    "  books{⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒}",
                    "  articles{⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒}",
                    "  partial{⚘⊙⦑Author⦒⦑Year⦒}",
                    "  structure{⚘author⊙year⊳work} or {⚘author⊙year𝄏journal⊳article}",
                    "Title format rules CRITICAL:",
                    "  main title only{⚘⊙⊳⦑Michael Zander⦒⦑1988⦒⦑A Matter of Justice⦒}",
                    "  ∂¬full title{✗⚘⊙⊳⦑Michael Zander⦒⦑1988⦒⦑A Matter of Justice: The Legal System in Ferment⦒}",
                    "  ◻ Subtitle exclusion mandatory",
                    "  ◻ Publisher/edition info excluded",
                    "Standalone vs compound:",
                    "  compound citation context{⚘⊙⊳ for specific work reference}",
                    "  standalone rare{⊳⦑Work⦒when discussing work without full citation}",
                    "Work vs journal discrimination:",
                    "  standalone publication→⊳",
                    "  serial periodical→𝄏",
                    "  test{one-time publication?→⊳|periodic issues?→𝄏}",
                    "Co-author format:",
                    "  separator∧for co-authors{⚘⊙⊳⦑Author1∧Author2⦒⦑Year⦒⦑Title⦒}"
                ]),
                ("≟", [
                    "\"standalone publication\"→⊳",
                    "\"serial periodical\"→𝄏",
                    "\"organization\"→⍚:",
                    "  test{one-time publication?→YES:⊳|NO:continue}",
                    "  test{periodic issues?→YES:𝄏|NO:continue}",
                    "  test{publisher not work?→YES:⍚|NO:review}"
                ]),
                ("⊨", "⊳⊂standalone works ∧ bibliographic citations ∧ main title only ∧ ⊳⊅{periodicals,publishers}"),
            ],
        },
    ]
}
