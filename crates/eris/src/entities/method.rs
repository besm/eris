//! ⧏ Method entity type

define_entity_module! {
    Entity {
        Conceptual => "Conceptual entity types",
    }
}

type MethodDef = EntityDef;

pub fn get_entity_definitions() -> Vec<MethodDef> {
    vec![
        MethodDef {
            symbol: "⧏",
            name: "Method",
            description: "Analytical technique with procedural application and replicable steps",
            sort_order: 21,
            category: EntityCategory::Conceptual,
            lines: lines![
                ("≡", ["analytical technique", "systematic procedure"]),
                ("≝", ["replicable steps", "procedural application", "methodological approach"]),
                ("∂", [
                    "⧏⊅{❖,⧊,⌬}",
                    "⧏⊅⧊_meta (method-terms in epistemological discourse)",
                    "doing→⧏ | discussing→⧊"
                ]),
                ("⊡", [
                    "typical: Ֆ6Շ6Գ7Բ6",
                    "⧏⦑Western Blot⦒: Ֆ9Շ3Բ9",
                    "⧏⦑Participant Observation⦒: Ֆ5Շ7Բ4",
                    "⧏⦑Dialectics⦒: Ֆ4Շ9Գ9"
                ]),
                ("⊛", [
                    "⧏⦑Content Analysis|Discourse Analysis|Participant Observation⦒",
                    "⧏⦑Genealogical Method|Comparative Method|Historical Method⦒"
                ]),
                ("⧠", [
                    "Popper on 'holistic experiments'→⧊ (critiquing concept)",
                    "methods textbook on 'holistic experiments'→⧏ (teaching procedure)",
                    "'apply content analysis to corpus'→⧏",
                    "'the idea of content analysis'→⧊"
                ]),
                ("◻", [
                    "procedural: test{\"Apply [X]\" natural?→YES:⧏|NO:review}",
                    "discourse: test{HOW-writing?→⧏|WHAT-writing?→⧊}",
                    "  methodological (handbooks, protocols)→⧏",
                    "  epistemological (philosophy of science)→⧊",
                    "vs ❖: test{departments exist?→❖|technique within field?→⧏}",
                    "vs ⌬: test{material/computational?→⌬|analytical?→⧏}"
                ]),
                ("≟", [
                    "○₁ discourse: HOW?→continue | WHAT?→⧊",
                    "○₂ procedural: replicable steps?→continue | abstract?→⧊",
                    "○₃ institutional: departments/degrees?→❖ | technique?→continue",
                    "○₄ technical: implemented system?→⌬ | analytical?→⧏"
                ]),
                ("⊨", "⧏⊂systematic procedures ∧ procedural application")
            ],
        },
    ]
}