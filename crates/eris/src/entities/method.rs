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
                ("≡", ["analytical_technique", "systematic_procedure", "replicable_steps"]),
                ("≝", "procedural application ∧ 'Apply [X]' natural ∧ HOW-writing"),
                ("∂", [
                    "⧏⊅❖ (⧏ technique | ❖ institutional field)",
                    "⧏⊅⧊ (⧏ doing | ⧊ discussing/critiquing)",
                    "⧏⊅⌬ (⧏ analytical | ⌬ material/computational)"
                ]),
                ("⊡", [
                    "⧏: Ֆ6Շ6Գ7Բ6",
                    "lab protocol: Ֆ9Շ3Բ9 | ethnographic: Ֆ5Շ7Բ4 | philosophical: Ֆ4Շ9Գ9"
                ]),
                ("⊛", [
                    "qualitative: ⧏⦑Content Analysis|Discourse Analysis|Grounded Theory|Thematic Analysis⦒",
                    "ethnographic: ⧏⦑Participant Observation|Thick Description|Life History Method⦒",
                    "historical: ⧏⦑Genealogical Method|Comparative Method|Prosopography|Archival Method⦒",
                    "quantitative: ⧏⦑Regression Analysis|Factor Analysis|Network Analysis|Time Series⦒",
                    "philosophical: ⧏⦑Dialectics|Phenomenological Method|Hermeneutic Circle|Deconstruction⦒",
                    "scientific: ⧏⦑Western Blot|PCR|Spectroscopy|Chromatography⦒"
                ]),
                ("◻", [
                    "discrimination:",
                    "  'apply content analysis'→⧏ | 'the idea of content analysis'→⧊",
                    "  Popper critiquing 'holistic experiments'→⧊ | methods textbook teaching it→⧏",
                    "  methodological (handbooks, protocols)→⧏ | epistemological (philosophy)→⧊",
                    "tests:",
                    "  'Apply [X]' natural?→⧏ | 'Department of [X]'?→❖ | implemented system?→⌬"
                ]),
                ("≟", [
                    "test{HOW-writing?→continue | WHAT-writing?→⧊}",
                    "test{replicable steps?→continue | abstract?→⧊}",
                    "test{departments/degrees?→❖ | technique?→continue}",
                    "test{material/computational?→⌬ | analytical?→⧏}"
                ]),
                ("⊨", "⧏ ≡ systematic_procedure ∧ replicable ∧ 'Apply [X]' natural")
            ],
        },
    ]
}