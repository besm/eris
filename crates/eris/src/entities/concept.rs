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
                ("≡", "reality-organizing abstraction ∧ theoretical construct"),
                ("≝", "cross-domain operation ∧ conceptual framework ∧ organizing principle ∧ transcendent idea"),
                ("∂", [
                    "⧊⊅{⧈occupational,diagnostic,role-based,❖institutionalized,⧏procedural,⌬implemented}",
                    "◻{domain transcendence,organizing power}"
                ]),
                ("⊛", "⧊⦑Democracy|Capitalism|Science|Language|Free Will|Engineering Mindset|Technical Rationality|Citizenship⦒(abstraction¬people)"),
                ("◻", [
                    "Abstract nouns ∧ theoretical constructs ∧ organizing principles:",
                    "  philosophical: {⧊⦑Free Will|Consciousness|Mind|Self⦒}",
                    "  political: {⧊⦑Democracy|Capitalism|Liberty|Republic⦒}",
                    "  social: {⧊⦑Class|Gender|Culture|Society⦒}",
                    "  epistemic: {⧊⦑Science|Knowledge|Rationality|Verification⦒}",
                    "  cultural: {⧊⦑Art|Poetry|Fiction|Taste⦒}",
                    "  analytical: {⧊⦑Classification|Explanation|Representation⦒}",
                    "  attitudes/frameworks: {⧊⦑Engineering Mindset|Technical Rationality|Puritanism⦒}",
                    "  abstract nouns≫descriptive phrases",
                    "Concept vs people discrimination abstraction→⧊|occupational→⧈:",
                    "  test{\"The [X]\"refers to people?→⧈|property/idea transcending people?→⧊}",
                    "  ⧊⦑Engineering Mindset⦒(attitude¬people)",
                    "  ⧊⦑Citizenship⦒(legal status abstraction)",
                    "  ∂occupational categories ALWAYS ⧈{",
                    "    ✗⧊⦑Engineers⦒→⧈⦑Engineers⦒people who engineer",
                    "    ✗⧊⦑Citizens⦒→⧈⦑Citizens⦒people with citizenship",
                    "    ✗⧊⦑Patients⦒→⧈⦑Patients⦒people receiving medical care}",
                    "  ◻CRITICAL RULE{occupational∈⧈(¬⧊)}",
                    "Concept vs field discrimination theoretical→⧊|institutional→❖:",
                    "  test{\"Department of X\"exists?→❖|cross-field abstract concept?→⧊}",
                    "  ⧊⦑Free Will⦒(philosophical concept¬academic field)",
                    "  ⧊⦑Representation⦒(cross-field concept)",
                    "  ∂subdisciplines often ❖{",
                    "    ✗⧊⦑Economic Anthropology⦒→❖anthropology subdiscipline",
                    "    ✗⧊⦑Quantum Mechanics⦒→❖physics subdiscipline}",
                    "  institutional criterion decisive",
                    "Concept vs method discrimination theoretical→⧊|procedural→⧏:",
                    "  test{\"Apply [X]\"natural?→⧏|organizing principle without steps?→⧊}",
                    "  test{replicable procedure?→⧏|abstract framework?→⧊}",
                    "  ⧊⦑Classification⦒(concept about organizing)",
                    "  ⧏⦑Classification Method⦒(specific technique)",
                    "  ∂theories/concepts WITHOUT procedures{",
                    "    ✓⧊⦑Shame Culture⦒cultural concept¬analytical method}",
                    "  procedural application requirement discriminates ⧏from ⧊",
                    "Concept vs technology discrimination abstract→⧊|implemented→⌬:",
                    "  test{material|computational realization?→⌬|purely abstract?→⧊}",
                    "  ⧊⦑Linguistic Technology⦒(language design as conceptual framework)",
                    "  ⌬⦑AI⦒(technology domain with implementations)",
                    "  operational capability criterion(⌬requires implementation)",
                    "Compound concepts valid when components interact:",
                    "  ⧊⦑Political Economy|Social Class|Cultural Memory⦒",
                    "Metaphors/analogies valid as concepts:",
                    "  ⧊⦑Attention as Economic Resource⦒(metaphor=conceptual framework)",
                    "  ⧊⦑Black Box⦒(metaphorical concept)",
                    "Domain transcendence test applies across contexts?:",
                    "  ⧊⦑Representation⦒(philosophy,art,politics,science)",
                    "  ⧊⦑Classification⦒(biology,library science,philosophy,social science)",
                    "  cross-domain applicability=reality-organizing power"
                ]),
                ("≟", [
                    "\"cross-domain abstraction\"→⧊",
                    "\"people category\"→⧈",
                    "\"academic department\"→❖",
                    "\"procedural technique\"→⧏",
                    "\"implemented system\"→⌬:",
                    "  test1{people?\"The [X]\"=people?→YES:⧈|NO:continue}",
                    "  test2{occupation?occupational role?→YES:⧈|NO:continue}",
                    "  test3{institution?\"Department of [X]\"?→YES:❖|NO:continue}",
                    "  test4{procedure?\"Apply [X]\"natural?→YES:⧏|NO:continue}",
                    "  test5{implementation?material|computational system?→YES:⌬|NO:continue}",
                    "  test6{domain?transcends single field/context?→YES:likely ⧊|NO:review}",
                    "  test7{organizing?structures reality|organizes experience?→YES:⧊|NO:review}"
                ]),
                ("⊨", "⧊⊂reality organizing abstractions ∧ cross domain operation ∧ theoretical constructs ∧ ⧊⊅{⧈,❖,⧏,⌬}"),
            ],
        },
    ]
}
