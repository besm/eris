//! ❖ Field entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type FieldDef = EntityDef;

pub fn get_entity_definitions() -> Vec<FieldDef> {
    vec![
        FieldDef {
            symbol: "❖",
            name: "Field",
            description: "Academic discipline with institutional markers (departments, degrees, journals)",
            sort_order: 6,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["academic_discipline", "institutionalized_domain", "knowledge_container"]),
                ("≝", "institutional boundaries ∧ scholarly legitimacy ∧ ∃{departments|degrees|journals|conferences}"),
                ("∂", [
                    "❖⊅⧏ (❖ field | ⧏ method within field)",
                    "❖⊅⧊ (❖ institutional | ⧊ abstract concept)"
                ]),
                ("⊡", [
                    "❖: Ֆ7Շ6Բ7",
                    "core: Շ8Բ8 (physics, sociology)",
                    "subdiscipline: Շ5Բ6 (quantum mechanics)",
                    "interdiscipline: Շ6Բ5 (cognitive science)"
                ]),
                ("⊛", [
                    "core: ❖⦑Physics|Chemistry|Biology|Sociology|Philosophy|History|Economics|Psychology⦒",
                    "subdiscipline: ❖⦑Quantum Mechanics|Epistemology|Macroeconomics|Social Psychology⦒",
                    "interdiscipline: ❖⦑Cognitive Science|Science and Technology Studies|Computational Linguistics|Bioethics⦒",
                    "compound: ❖⦑Philosophy of Language|Sociology of Science|History of Mathematics|Philosophy of Mind⦒",
                    "specialized: ❖⦑Mormon History|Classical Philology|Media Theory|Subaltern Studies|Africana Studies⦒",
                    "regional: ❖⦑Sinology|Japanology|Indology|Slavic Studies|Latin American Studies⦒"
                ]),
                ("◻", [
                    "institutional markers (one sufficient):",
                    "  'Department of X' | 'PhD in X' | 'Journal of X' | 'Conference on X'",
                    "naming:",
                    "  ✓❖⦑[Descriptive Name]⦒ | ✗abbreviations",
                    "  ✓❖⦑Science and Technology Studies⦒ — unified field ¬separate tags",
                    "  ✓❖⦑Philosophy of Language⦒ — compound valid",
                    "discrimination:",
                    "  'study phenomenology'→❖ | 'apply phenomenological method'→⧏",
                    "  'epistemology as field'→❖ | 'epistemological question'→⧊"
                ]),
                ("≟", [
                    "test{'Department of X' exists?→YES:❖|NO:continue}",
                    "test{'Apply X method'?→YES:⧏|NO:continue}",
                    "test{abstract concept?→YES:⧊|NO:review}"
                ]),
                ("⊨", "❖ ≡ institutionalized_domain ∧ □academic_legitimacy")
            ],
        },
    ]
}