//! 𝄏 Journal entity type

define_entity_module! {
    Entity {
        Compound => "Compound citation component types",
    }
}

type JournalDef = EntityDef;

pub fn get_entity_definitions() -> Vec<JournalDef> {
    vec![
        JournalDef {
            symbol: "𝄏",
            name: "Journal",
            description: "Periodical publication: academic journals, magazines, newspapers",
            sort_order: 51,
            category: EntityCategory::Compound,
            lines: lines![
                ("≡", ["periodical_publication", "serial_venue", "regular_issues"]),
                ("≝", "serial format ∧ publication regularity ∧ institutional|commercial periodical"),
                ("∂", [
                    "𝄏⊅⊳ (𝄏 serial | ⊳ standalone work)",
                    "𝄏⊅⍚ (𝄏 publication | ⍚ publishing organization)",
                    "𝄏⊅⍓ (𝄏 periodical | ⍓ press/publisher)"
                ]),
                ("⊛", [
                    "academic: 𝄏⦑Nature|Science|Daedalus|Mind|Annales|Past & Present⦒",
                    "magazines: 𝄏⦑Harper's Magazine|Fortune|The New Yorker|Der Spiegel|L'Express⦒",
                    "newspapers: 𝄏⦑Financial Times|Asahi Shinbun|Le Monde|Frankfurter Allgemeine⦒",
                    "reviews: 𝄏⦑London Review of Books|New York Review of Books|Times Literary Supplement⦒",
                    "historical: 𝄏⦑Popular Science Monthly|Der Monat|Edinburgh Review|Mercure de France⦒",
                    "trade/underground: 𝄏⦑Byte|Datamation|2600: The Hacker Quarterly|FidoNews⦒"
                ]),
                ("◻", [
                    "naming:",
                    "  ✓𝄏⦑[Full Name]⦒ | ✗abbreviations unless established",
                    "  ✓𝄏⦑Dialogue: A Journal of Mormon Thought⦒ — include subtitle when official",
                    "discrimination:",
                    "  𝄏⦑Daedalus⦒ (publication) vs ⍚⦑American Academy of Arts and Sciences⦒ (publisher)",
                    "  periodic issues→𝄏 | one-time→⊳",
                    "compound citation: ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒"
                ]),
                ("≟", [
                    "test{periodic issues?→YES:𝄏|NO:continue}",
                    "test{one-time publication?→YES:⊳|NO:continue}",
                    "test{publishing organization?→YES:⍚|NO:review}"
                ]),
                ("⊨", "𝄏 ≡ serial_publication ∧ periodic_issues")
            ],
        },
    ]
}