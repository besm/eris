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
            description: "Natural language: references, non-trivial foreign terms, non-English passages",
            sort_order: 14,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["natural_language", "linguistic_system", "foreign_term_marker"]),
                ("≝", "human communication system ∧ linguistic code ∧ non-English usage"),
                ("∂", [
                    "⧩⊅⧊ (⧩ specific language | ⧊ 'Language' as concept)",
                    "⧩⊅⌬ (⧩ natural | ⌬ programming language)"
                ]),
                ("⊛", [
                    "major: ⧩⦑English|French|German|Spanish|Portuguese|Russian|Arabic|Mandarin|Japanese⦒",
                    "classical: ⧩⦑Greek|Latin|Sanskrit|Classical Arabic|Classical Chinese|Hebrew⦒",
                    "varieties: ⧩⦑Katharevousa|Demotic Greek|Classical Latin|Vulgar Latin|Old English⦒",
                    "regional: ⧩⦑Yoruba|Swahili|Hindi|Bengali|Tagalog|Quechua|Nahuatl⦒"
                ]),
                ("◻", [
                    "tagging triggers:",
                    "  reference: 'written in Greek' | 'the French term' | 'from Arabic'",
                    "  non-trivial terms: Aufhebung | Weltanschauung | habitus | ressentiment | Dasein",
                    "  passages: block quotes in non-English | extended foreign text",
                    "  ✗trivial: café | résumé | et cetera — fully assimilated",
                    "discrimination:",
                    "  ⧩⦑German⦒ (language) vs ⧈⦑Germans⦒ (people)",
                    "  ⧩⦑French⦒ (language) vs ⌖⦑France⦒ (place)",
                    "  ⧩⦑Greek⦒ for 'λόγος' vs ⧊⦑Logos⦒ for concept discussion"
                ]),
                ("≟", [
                    "test{natural language reference?→YES:⧩|NO:continue}",
                    "test{non-trivial foreign term?→YES:⧩|NO:continue}",
                    "test{non-English passage?→YES:⧩|NO:continue}",
                    "test{programming language?→YES:⌬|NO:continue}",
                    "test{abstract 'Language' concept?→YES:⧊|NO:review}"
                ]),
                ("⊨", "⧩ ≡ natural_language ∧ (reference∨foreign_term∨passage)")
            ],
        },
    ]
}