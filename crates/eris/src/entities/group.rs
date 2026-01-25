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
                ("≡", "human classification ∧ social category"),
                ("≝", "classificatory systems ∧ group identities ∧ collective labels"),
                ("∂", "⧈⊅{⧊abstractions,⚘individuals,⍚organizations} ∧ ◻ human reference"),
                ("⊛", [
                    "⧈⦑Engineers|Citizens|Patients|Sociologists⦒",
                    "partisan: ⧈⦑Democrats|Republicans|Tories|Labourites⦒"
                ]),
                ("◻", [
                    "emic ∧ actors' categories ∧ historical discourse terms ∂ etic judgments",                    "Collective nouns plural∨singular:",
                    "  occupational: {⧈⦑Engineers|Priests|Bureaucrats|Chemist⦒}",
                    "  legal/political: {⧈⦑Citizens|Metics|Slaves|Kings⦒}",
                    "  diagnostic: {⧈⦑Patients|Schizophrenics|Depressive Maniacs⦒}",
                    "  social roles: {⧈⦑Early Adopters|Elites|Radicals⦒}",
                    "  ethnic/cultural: {⧈⦑Hebrews|Bushmen|Puritans⦒}",
                    "  classificatory reference≫abstract property",
                    "Occupational categories ALWAYS⧈(NEVER⧊):",
                    "  ✓⧈⦑Engineers|Computer Programmers|Technical Professionals⦒",
                    "  ∂✗⧊⦑Engineers⦒",
                    "  occupational⊅⧊",
                    "  ◻CRITICAL: {\"the [occupation]\"→⧈people≠concept}",
                    "Diagnostic categories medical/psychological classifications:",
                    "  ⧈⦑Patients|Schizophrenics|Depressive Maniacs⦒",
                    "  medical categories legitimate",
                    "Context-dependent \"the[X]\":",
                    "  {\"the engineers\"→⧈people, \"engineering mindset\"→⧊concept}",
                    "  {\"the citizens\"→⧈people, \"citizenship as legal status\"→⧊abstraction}"
                ]),
                ("≟", [
                    "\"the[X]\"=people?→⧈",
                    "collective noun?→⧈",
                    "occupational category?→⧈(◻□)",
                    "diagnostic classification?→⧈",
                    "abstract property without people reference?→⧊",
                    "\"the Democrats voted\"→⧈ (people)",
                    "\"the Democratic Party nominated\"→⚐ (organization)"
                ]),
                ("⊨", "⧈⊂human classifications ∧ □occupational⊂⧈"),
            ],
        },
    ]
}
