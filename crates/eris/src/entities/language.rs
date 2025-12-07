//! ⧩ Language entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type LanguageDef = EntityDef;

pub fn get_entity_definitions() -> Vec<LanguageDef> {
    vec![
        LanguageDef {
            symbol: "⧩",
            name: "Language",
            description: "Natural language with speakers, including historical stages and varieties",
            sort_order: 14,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "natural language ∧ linguistic system"),
                ("≝", "human communication system ∧ linguistic code"),
                ("∂", "⧩⊅{⧊(concept),⌬(tech)} ∧ ◻ linguistic system with speakers"),
                ("⊛", "⧩⦑Greek|Latin|German|French|English|Sanskrit|Arabic⦒"),
                ("◻", [
                    "Natural languages≫programming languages(→⌬)",
                    "  specific language varieties{⧩⦑Katharevousa|Demotic Greek⦒}",
                    "  historical stages{⧩⦑Classical Latin|Vulgar Latin|Old English⦒}",
                    "  ∂{¬⧊⦑Language⦒abstract,¬⌬programming}"
                ]),
                ("≟", [
                    "natural language with speakers?→⧩",
                    "programming language?→⌬",
                    "abstract concept of language?→⧊"
                ]),
                ("⊨", "⧩⊂natural languages ∧ human communication systems"),
            ],
        },
    ]
}
