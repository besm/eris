//! ⍓ Publisher entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type PublisherDef = EntityDef;

pub fn get_entity_definitions() -> Vec<PublisherDef> {
    vec![
        PublisherDef {
            symbol: "⍓",
            name: "Publisher",
            description: "Publishing house with content distribution as primary function",
            sort_order: 12,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "publishing house ∧ content distribution organization"),
                ("≝", "editorial function ∧ content production ∧ commercial distribution"),
                ("∂", "⍓⊂⍚(publishers⊂organizations) ∧ ◻{content distribution as primary function,editorial selection,commercial/institutional publishing}"),
                ("⊛", [
                    "⍓⦑MIT Press|Cambridge University Press|Oxford University Press⦒university presses",
                    "⍓⦑Faber|Penguin|Random House|Norton⦒commercial publishers",
                    "⍓⦑IEEE|ACM⦒professional societies as publishers"
                ]),
                ("◻", [
                    "Publisher types all valid when publishing primary:",
                    "  university presses{⍓⦑MIT Press|Cambridge University Press|Oxford University Press|Harvard University Press⦒}",
                    "  commercial publishers{⍓⦑Faber|Penguin Random House|Norton|HarperCollins⦒}",
                    "  professional society publishers{⍓⦑IEEE|ACM|American Physical Society⦒}",
                    "  independent/specialty presses{⍓⦑Verso|O'Reilly|MIT Press⦒}",
                    "  imprints valid when established brand{⍓⦑Vintage|Anchor Books|Knopf⦒}",
                    "  established names≫avoiding over-specificity",
                    "Name forms follow conventions:",
                    "  university presses{full institutional names:",
                    "    ✓⍓⦑MIT Press|Cambridge University Press|Oxford University Press⦒",
                    "    ✗⍓⦑MIT|Cambridge|Oxford⦒when referring to publisher not university}",
                    "  commercial publishers{",
                    "    ✓⍓⦑Faber|Penguin|Random House|Norton⦒established forms",
                    "    merger handling{⍓⦑Penguin Random House⦒post-merger∧⍓⦑Penguin|Random House⦒historical}",
                    "    conventional usage≫full legal corporate names}",
                    "  professional societies{",
                    "    ✓⍓⦑IEEE|ACM|APS⦒when functioning as publishers",
                    "    acronyms standard when conventional}",
                    "Discrimination from related entities:",
                    "  publisher vs parent university{⍓⦑MIT Press⦒publisher∧⍢⦑MIT⦒university",
                    "    both valid when contextually relevant:",
                    "    citation context→⍓⦑MIT Press⦒",
                    "    university affiliation→⍢⦑MIT⦒",
                    "    publishing function=discriminator}",
                    "  publisher vs professional society as membership org{",
                    "    ⍓⦑IEEE⦒publishing function∧⍚⦑IEEE⦒membership organization",
                    "    context determines classification}",
                    "  imprint vs parent publisher{both valid when established brands:",
                    "    ⍓⦑Vintage⦒imprint∧⍓⦑Random House⦒parent both legitimate}",
                    "Generic publishing categories{DELETE:",
                    "  ✗⍓⦑University Presses|Commercial Publishers|Academic Publishers⦒",
                    "  categorical plurals→DELETE}",
                    "Historical publishers valid:",
                    "  ⍓⦑Faber and Faber⦒(even if name changed/merged)",
                    "  publishing existence≫current operations",
                    "Self-publishing platforms discrimination:",
                    "  established publishers{⍓⦑MIT Press|Faber⦒editorial selection}",
                    "  platforms{⌬⦑Amazon KDP|Substack⦒technology platforms ∂publishers}",
                    "  editorial function=discriminator"
                ]),
                ("≟", [
                    "content distribution primary function?→⍓",
                    "parent university not publisher?→⍢",
                    "professional society not publisher?→⍚",
                    "self-publishing platform?→⌬:",
                    "  test{editorial selection and content distribution?→YES:⍓|NO:continue}",
                    "  test{university press vs university?→⍓press∧⍢university|NO:continue}",
                    "  test{professional society as publisher?→YES:⍓|NO:⍚membership org}",
                    "  test{platform without editorial function?→YES:⌬|NO:continue}",
                    "  test{non-governmental organization?→YES:⍚|NO:review}"
                ]),
                ("⊨", "⍓⊂⍚ ∧ content distribution≫generic coordination"),
            ],
        },
    ]
}
