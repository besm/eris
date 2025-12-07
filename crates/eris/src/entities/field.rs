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
                ("≡", "academic discipline ∧ institutionalized domain"),
                ("≝", "knowledge container ∧ institutional boundaries ∧ scholarly legitimacy"),
                ("∂", "❖⊅{⧏procedures,⧊abstractions} ∧ ◻ institutional markers"),
                ("⊛", "❖⦑Philosophy|Sociology|Quantum Mechanics|Knowledge Engineering⦒"),
                ("◻", [
                    "Descriptive field names ∂¬abbreviations:",
                    "  core: {❖⦑Physics|Sociology|Philosophy|History⦒}",
                    "  subdisciplines: {❖⦑Quantum Mechanics|Knowledge Engineering|Epistemology⦒}",
                    "  interdisciplines: {❖⦑Cognitive Science|Science and Technology Studies|Computational Linguistics⦒}",
                    "  specialized: {❖⦑Mormon History|Classical Philology|Media Theory⦒}",
                    "Institutional criterion ◻□markers:",
                    "  departments: {\"Department of X\"exists}",
                    "  degrees: {\"PhD in X\"granted}",
                    "  journals: {\"Journal of X\"published}",
                    "  conferences: {\"Conference on X\"convenes}",
                    "  one marker sufficient|multiple typical",
                    "Subdiscipline vs field both valid ❖:",
                    "  core: {❖⦑Physics⦒parent discipline}",
                    "  sub: {❖⦑Theoretical Physics|Quantum Mechanics⦒specializations}",
                    "  specialization level ∂¬type discrimination",
                    "Compound fields natural compounds valid:",
                    "  ❖⦑Philosophy of Language|Sociology of Science⦒",
                    "  ❖⦑History of Mathematics|Social Studies of Science⦒",
                    "  ∂descriptive compounds(¬format violations)",
                    "Hyphenated fields single ❖:",
                    "  ❖⦑Science and Technology Studies⦒(interdisciplinary field)",
                    "  ∂¬separate tags for components(STS=unified field)"
                ]),
                ("≟", [
                    "institutional→❖",
                    "procedural→⧏",
                    "abstract→⧊:",
                    "  test{\"Department of X\"exists?→YES:❖|NO:continue}",
                    "  test{\"Apply X method\"?→YES:⧏|NO:continue}",
                    "  test{\"X is concept/idea\"?→YES:⧊|NO:review}",
                    "  context-dependent{\"study phenomenology\"→❖, \"apply phenomenological method\"→⧏}"
                ]),
                ("⊨", "❖⊂institutionalized domains ∧ □academic legitimacy"),
            ],
        },
    ]
}
