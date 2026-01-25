//! ⧊ Concept entity type

define_entity_module! {
    Entity {
        Conceptual => "Conceptual entity types",
    }
}

type ConceptDef = EntityDef;

pub fn get_entity_definitions() -> Vec<ConceptDef> {
    vec![
        ConceptDef {
            symbol: "⧊",
            name: "Concept",
            description: "Reality-organizing abstraction with cross-domain operation and organizing power",
            sort_order: 20,
            category: EntityCategory::Conceptual,
            lines: lines![
                ("≡", ["reality_organizing_abstraction", "theoretical_construct", "cross_domain_operation"]),
                ("≝", "domain transcendence ∧ organizing power ∧ ¬{people,institution,procedure,implementation}"),
                ("∂", [
                    "⧊⊅⧈ (⧊ abstraction | ⧈ people) — occupational ALWAYS ⧈",
                    "⧊⊅❖ (⧊ cross-field | ❖ institutional)",
                    "⧊⊅⧏ (⧊ framework | ⧏ procedural)",
                    "⧊⊅⌬ (⧊ abstract | ⌬ implemented)",
                    "⧊⊅⚐ (⧊ ideology | ⚐ party)"
                ]),
                ("⊡", [
                    "⧊: Շ variable (Շ9 universal→Շ3 domain-specific)",
                    "philosophical: Շ9Գ9 | political: Շ8Գ8 | analytical: Շ7Գ7"
                ]),
                ("⊛", [
                    "philosophical: ⧊⦑Free Will|Consciousness|Mind|Self|Being|Causation⦒",
                    "political: ⧊⦑Democracy|Capitalism|Sovereignty|Liberty|Republic|Citizenship⦒",
                    "social: ⧊⦑Class|Gender|Culture|Society|Power|Identity|Habitus⦒",
                    "epistemic: ⧊⦑Science|Knowledge|Rationality|Truth|Verification|Objectivity⦒",
                    "analytical: ⧊⦑Classification|Representation|Explanation|Performativity⦒",
                    "frameworks: ⧊⦑Engineering Mindset|Technical Rationality|Neoliberalism|Orientalism⦒",
                    "metaphors: ⧊⦑Black Box|Attention as Economic Resource|Social Contract⦒",
                    "non-western: ⧊⦑Qi|Dharma|Ubuntu|Tawhid|Ren|Li⦒"
                ]),
                ("◻", [
                    "CRITICAL: occupational∈⧈ (¬⧊)",
                    "  ✗⧊⦑Engineers|Citizens|Patients⦒ → ⧈",
                    "  ✓⧊⦑Engineering Mindset|Citizenship|Patienthood⦒ — abstraction ¬people",
                    "discrimination:",
                    "  'The [X]'=people?→⧈ | abstraction?→⧊",
                    "  'Department of [X]'?→❖ | cross-field?→⧊",
                    "  'Apply [X]'?→⧏ | framework?→⧊",
                    "  implemented system?→⌬ | abstract?→⧊",
                    "domain transcendence: ⧊⦑Representation⦒ (philosophy,art,politics,science)"
                ]),
                ("≟", [
                    "test{'The [X]'=people?→YES:⧈|NO:continue}",
                    "test{occupational?→YES:⧈|NO:continue}",
                    "test{'Department of [X]'?→YES:❖|NO:continue}",
                    "test{'Apply [X]' natural?→YES:⧏|NO:continue}",
                    "test{material/computational?→YES:⌬|NO:continue}",
                    "test{cross-domain∧organizing?→YES:⧊|NO:review}"
                ]),
                ("⊨", "⧊ ≡ reality_organizing ∧ cross_domain ∧ ⧊⊅{⧈,❖,⧏,⌬}")
            ],
        },
    ]
}