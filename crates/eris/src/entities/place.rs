//! ⌖ Place entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type PlaceDef = EntityDef;

pub fn get_entity_definitions() -> Vec<PlaceDef> {
    vec![
        PlaceDef {
            symbol: "⌖",
            name: "Place",
            description: "Geographic entity, spatial extent, physical location",
            sort_order: 2,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "geographic entity ∧ spatial extent"),
                ("≝", "geographic specificity ∧ spatial boundaries ∧ physical location"),
                ("∂", "⌖⊅{metonymic concept, ⚘ name component} ∧ ◻ spatial referent"),
                ("⊛", "⌖⦑Place|Region|Feature⦒ ∧ rivers∧mountains∧seas"),
                ("◻", [
                    "Simple name ∂¬tag disambiguation:",
                    "  Single: {Rome, England, Africa}",
                    "  Comma: {St. Augustine, FL | Watts, Los Angeles}",
                    "  ∂¬country suffix: {Ottawa}",
                    "Scale continents→streets:",
                    "  Continent: {Africa}",
                    "  Region: {Midwest}",
                    "  City: {Rome}",
                    "  Street: {Hudson Street}",
                    "  Features: {Gulf Stream, Northwest Passage, Ohio River}",
                    "  ◻ historically significant ∂¬buildings",
                    "Historical contextual:",
                    "  Regime: {Nazi Germany, Soviet Union}",
                    "  Cultural: {Silicon Valley, New England}",
                    "  Archaeological: {Troy, Sodom and Gomorrah}",
                    "∂ Metonymic {¬place→concept/event/system}:",
                    "  monetary→⧖⦑Bretton Woods Era⦒",
                    "  sovereignty→⧊⦑Westphalian System⦒",
                    "  financial→⍚⦑Wall Street⦒",
                    "  \"in/at [Place]\"→likely ⌖",
                    "  \"[Place] system/model\"→⧊",
                    "  \"[Place] era\"→⧖",
                    "  \"[Place] believes\"→⍚",
                    "Toponym exclusion:",
                    "  ∂¬separate ⌖ when ∈⚘ name"
                ]),
                ("≟", [
                    "literal spatial→⌖",
                    "metonymic system→⧊",
                    "metonymic institution→⍚",
                    "temporal period→⧖",
                    "specific event→⌁"
                ]),
                ("⊨", "⌖⊂literal spatial referents ∧ physical geography"),
            ],
        },
    ]
}
