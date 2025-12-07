//! ⚘ Person entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type PersonDef = EntityDef;

pub fn get_entity_definitions() -> Vec<PersonDef> {
    vec![
        PersonDef {
            symbol: "⚘",
            name: "Person",
            description: "Named individual, human agent, biographical subject",
            sort_order: 1,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "person ∧ named individual"),
                ("≝", "human agent ∧ biographical subject ∧ attributed author"),
                ("∂", "⚘⊅{⧈,⧊,fictional} ∧ ◻ historical attestation"),
                ("⊛", [
                    "⚘⦑Full Name|Name of Place⦒ ∧ ⚘∈{⚘⊕⊙⊕⊳,⚘⊕⊙⊕𝄏⊕⊳}",
                    "⚘→proper_name ∧ ⚘∈historical_record"
                ]),
                ("◻", [
                    "Scholarly standard ∂¬uniform:",
                    "  Initials: {C.S. Peirce, W.E.B. Du Bois, J.L. Austin}",
                    "  Full: {William James, John Stuart Mill, Karl Marx}",
                    "  Mixed: {Thomas S. Kuhn}",
                    "Toponym: ⚘⦑Pytheas of Massalia⦒≡complete_id",
                    "  ∂{¬⚘⦑Pytheas⦒, ¬separate ⌖}",
                    "  ∂{¬parentheses, ¬⚘⦑Person (Disambiguation)⦒}",
                    "Compound: {⚘⊕⊙⊕⊳, ⚘⊕⊙⊕𝄏⊕⊳}",
                    "Co-authors: →∧"
                ]),
                ("≟", [
                    "\"the [occupation]\"→⧈",
                    "collective→⧈",
                    "role→⧊",
                    "fictional→∅"
                ]),
                ("⊨", "⚘⊂historically attested individuals"),
            ],
        },
    ]
}
