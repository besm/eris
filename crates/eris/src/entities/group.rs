//! ⧈ Group entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type GroupDef = EntityDef;

pub fn get_entity_definitions() -> Vec<GroupDef> {
    vec![
        GroupDef {
            symbol: "⧈",
            name: "Group",
            description: "Human classification (occupational, legal, diagnostic, social categories)",
            sort_order: 7,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["human_classification", "social_category", "collective_label"]),
                ("≝", "classificatory system ∧ group identity ∧ people-as-category"),
                ("∂", [
                    "⧈⊅⧊ (⧈ people | ⧊ abstract property)",
                    "⧈⊅⚘ (⧈ category | ⚘ individual)",
                    "⧈⊅⍚ (⧈ category | ⍚ organization)",
                    "⧈⊅⚐ (⧈ supporters | ⚐ party)",
                    "⧈⊅⌯ (⧈ adherents | ⌯ movement)"
                ]),
                ("⊡", [
                    "⧈: Շ variable | Ֆ variable (folk→formal)",
                    "occupational: Ֆ6Շ7",
                    "diagnostic: Ֆ8Շ5",
                    "ethnic: Ֆ3Շ6"
                ]),
                ("⊛", [
                    "occupational: ⧈⦑Engineers|Priests|Bureaucrats|Physicians|Programmers|Clerks⦒",
                    "legal/political: ⧈⦑Citizens|Metics|Slaves|Subjects|Refugees|Felons⦒",
                    "diagnostic: ⧈⦑Patients|Schizophrenics|Autistics|Diabetics⦒",
                    "social: ⧈⦑Early Adopters|Elites|Radicals|Intellectuals|Youth⦒",
                    "ethnic/cultural: ⧈⦑Hebrews|Bushmen|Puritans|Creoles|Mestizos⦒",
                    "partisan: ⧈⦑Democrats|Republicans|Tories|Labourites|Peronistas|Sandinistas⦒"
                ]),
                ("◻", [
                    "emic categories ∧ actors' terms ∂ etic judgments",
                    "occupational ALWAYS ⧈:",
                    "  ✓⧈⦑Engineers⦒ | ✗⧊⦑Engineers⦒",
                    "  'the [occupation]'→⧈ people ≠ concept",
                    "partisan vs party:",
                    "  'the Democrats voted'→⧈ (people)",
                    "  'the Democratic Party nominated'→⚐ (organization)",
                    "context:",
                    "  'the engineers'→⧈ | 'engineering mindset'→⧊",
                    "  'the citizens'→⧈ | 'citizenship'→⧊"
                ]),
                ("≟", [
                    "test{'the [X]' = people?→YES:⧈|NO:continue}",
                    "test{occupational?→YES:⧈|NO:continue}",
                    "test{diagnostic?→YES:⧈|NO:continue}",
                    "test{partisan supporters?→YES:⧈|NO:continue}",
                    "test{party organization?→YES:⚐|NO:continue}",
                    "test{abstract property?→YES:⧊|NO:review}"
                ]),
                ("⊨", "⧈ ≡ human_classification ∧ people-as-category ∧ occupational⊂⧈")
            ],
        },
    ]
}