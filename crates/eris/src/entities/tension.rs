//! ⧆ Tension entity type

define_entity_module! {
    Entity {
        Relational => "Relational/dynamic entity types",
    }
}

type TensionDef = EntityDef;

pub fn get_entity_definitions() -> Vec<TensionDef> {
    vec![
        TensionDef {
            symbol: "⧆",
            name: "Tension",
            description: "Binary productive opposition between competing frameworks (dyadic only)",
            sort_order: 31,
            category: EntityCategory::Relational,
            lines: lines![
                ("≡", "tension ∧ productive opposition"),
                ("≝", "binary competing frameworks ∧ dyadic contrast ∧ generative friction"),
                ("∂", "⧆⊅{⇋tripartite≥3,☯simultaneous contradiction}"),
                ("◻", "◻{binary structure,competing frameworks,productive tension}"),
                ("⊛", "⧆⦑Individual vs Collective|Theory vs Practice|Quantitative vs Qualitative⦒"),
                ("◻", [
                    "\"vs\"format dyadic opposition:",
                    "  ⧆⦑A vs B⦒",
                    "  binary competing frameworks",
                    "  productive opposition¬simple contradiction",
                    "Dyadic strict binary only¬tripartite:",
                    "  2 elements exactly",
                    "  ∂¬3+ elements(use ⇋)",
                    "  dyadic boundary enforcement"
                ]),
                ("≟", [
                    "binary competing?→⧆",
                    "tripartite≥3?→⇋",
                    "simultaneous within single?→☯"
                ]),
                ("⊨", "⧆⊂dyadic productive oppositions ∧ binary structure"),
            ],
        },
    ]
}
