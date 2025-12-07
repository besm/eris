//! ⌬ Tech entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type TechDef = EntityDef;

pub fn get_entity_definitions() -> Vec<TechDef> {
    vec![
        TechDef {
            symbol: "⌬",
            name: "Tech",
            description: "Technology with material/computational realization and operational capability",
            sort_order: 10,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "technology ∧ technical artifact"),
                ("≝", "implemented systems ∧ operational capability ∧ material|computational realization"),
                ("∂", "⌬⊅{⧊abstract frameworks,⧏analytical methods,⍚organizations providing tech}"),
                ("◻", [
                    "◻{material implementation∨computational realization,operational capability,technical infrastructure}"
                ]),
                ("⊛", "⌬⦑Computer|Internet|AI|Feedback Control|Facebook⦒"),
                ("◻", [
                    "Technical specification ≫ generic description:",
                    "  hardware{⌬⦑Computer|Microprocessor|TRS-80|ENIAC⦒}",
                    "  software{⌬⦑Operating System|UNIX|Multics|LISP⦒}",
                    "  networks{⌬⦑Internet|Arpanet|SAGE System|ChaosNet⦒}",
                    "  platforms{⌬⦑Facebook|Tinder|MUD|Chat Room⦒}",
                    "  technical domains{⌬⦑AI|Nanotechnology|Large Language Models⦒}",
                    "  acronyms{expand when ambiguous:⌬⦑Terminal Interface Message Processor (TIP)⦒}",
                    "Technical vs conceptual implementation→⌬|abstraction→⧊:",
                    "  ⌬⦑AI⦒(technology domain with implementations)",
                    "  ⌬⦑Feedback Control⦒(technical system with servomechanisms)",
                    "  ⌬⦑Numerical Control⦒(implemented manufacturing system)",
                    "  ∂when purely abstract framework without implementation→⧊",
                    "  operational capability criterion(must have material|computational realization)",
                    "Technology vs method system→⌬|procedure→⧏:",
                    "  ⌬⦑Collaborative Filtering⦒(implemented algorithmic system)",
                    "  ⌬⦑General Circulation Model⦒(computational modeling system)",
                    "  ∂pure analytical frameworks without technical infrastructure→⧏",
                    "  technical infrastructure discriminator",
                    "Platform vs organization technology→⌬|institution→⍚:",
                    "  ⌬⦑Facebook⦒(platform as technical system)",
                    "  ⌬⦑Netscape⦒(software product)",
                    "  ⌬⦑Tinder⦒(dating platform as technical system)",
                    "  ∂when referencing company/organization→⍚(use context)",
                    "  technical artifact emphasis"
                ]),
                ("≟", [
                    "\"implementation system\"→⌬",
                    "\"abstract framework\"→⧊",
                    "\"analytical procedure\"→⧏",
                    "\"organization providing tech\"→⍚:",
                    "  test{material implementation|computational realization?→YES:⌬|NO:continue}",
                    "  test{technical infrastructure|operational system?→YES:⌬|NO:continue}",
                    "  test{abstract concept without implementation?→YES:⧊|NO:continue}",
                    "  test{analytical method without technical system?→YES:⧏|NO:continue}",
                    "  test{organization operating platform?→YES:check context(platform→⌬|company→⍚)}"
                ]),
                ("⊨", "⌬≡technical artifacts ∧ operational capability required ∧ material|computational realization"),
            ],
        },
    ]
}
