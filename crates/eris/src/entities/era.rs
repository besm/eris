//! ⧖ Era entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type EraDef = EntityDef;

pub fn get_entity_definitions() -> Vec<EraDef> {
    vec![
        EraDef {
            symbol: "⧖",
            name: "Era",
            description: "Named temporal span with historiographic legitimacy",
            sort_order: 3,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "named temporal span ∧ historiographic period"),
                ("≝", "bounded duration ∧ scholarly legitimacy ∧ temporal marker"),
                ("∂", "⧖⊅{⊙,⌁} ∧ ◻ 4 criteria{named,bounded,geographic,citable}"),
                ("⊛", "⧖⦑Era|Century|Decade⦒"),
                ("◻", [
                    "4 criteria ◻□∀⧖:",
                    "  named: {historiographic designation ∃scholars reference}",
                    "  bounded: {temporal limits start∧end}",
                    "  geographic: {spatial context implicit∨explicit}",
                    "  citable: {scholarly legitimacy academic∨conventional}",
                    "Century:",
                    "  Spell out ∂¬abbreviations",
                    "  ✓⧖⦑Eighteenth Century|Nineteenth Century|Early Twentieth Century⦒",
                    "  ∂✗⧖⦑18th Century|19th C.⦒",
                    "  Modifiers: {Early, Late, Mid-}",
                    "Decade:",
                    "  Numeric primary|written valid",
                    "  ⧖⦑1970s|1980s|1920s⦒",
                    "  ⧖⦑The Sixties|The Twenties⦒",
                    "Era suffixes:",
                    "  Era consistent|conventional→variant",
                    "  ⧖⦑Victorian Era|Nazi Era|Industrial Era⦒",
                    "  ⧖⦑Pre-Socratic Period|Interwar Period⦒",
                    "  ⧖⦑Archaic Greece|Renaissance⦒",
                    "War:",
                    "  Δt>1yr∧sustained→⧖",
                    "  specific battles→⌁",
                    "  ⧖⦑World War I|World War II⦒",
                    "Geographic specificity:",
                    "  global→implicit|regional→explicit",
                    "  ⧖⦑Renaissance⦒(European implied)",
                    "  ⧖⦑Italian Renaissance|Northern Renaissance⦒",
                    "Composite:",
                    "  single ⧖ ∂¬separate events",
                    "  ⧖⦑Post-World War II Era⦒",
                    "  ∂¬⌁⦑World War II⦒(unless event discussed)"
                ]),
                ("≟", [
                    "Δt<1yr∧discrete→⌁",
                    "Δt>1yr∧sustained→⧖",
                    "point→⊙",
                    "unbounded→∅"
                ]),
                ("⊨", "⧖⊂historiographic legitimacy ∧ □4 criteria"),
            ],
        },
    ]
}
