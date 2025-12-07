//! ⍢ University entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type UniversityDef = EntityDef;

pub fn get_entity_definitions() -> Vec<UniversityDef> {
    vec![
        UniversityDef {
            symbol: "⍢",
            name: "University",
            description: "Educational institution with degree-granting authority and teaching/research mission",
            sort_order: 13,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "educational institution ∧ teaching/research organization"),
                ("≝", "higher education ∧ degree-granting ∧ research activity ∧ academic mission"),
                ("∂", "⍢⊂⍚(universities⊂organizations) ∧ ◻{educational mission as primary function,degree-granting authority,teaching and/or research as institutional purpose}"),
                ("⊛", "⍢⦑MIT|Harvard University|University of Chicago|Oxford|Cambridge|Stanford University⦒"),
                ("◻", [
                    "Educational institution types all valid when education primary:",
                    "  universities{⍢⦑Harvard University|MIT|Stanford University|University of Chicago|Oxford|Cambridge⦒}",
                    "  colleges{⍢⦑Williams College|Amherst College|Trinity College⦒}",
                    "  schools standalone{⍢⦑London School of Economics|Juilliard School⦒}",
                    "  institutes educational{⍢⦑California Institute of Technology|Georgia Institute of Technology⦒}",
                    "  subdivisions within universities valid:",
                    "    schools/colleges{⍢⦑Harvard Business School|MIT Sloan School of Management|Yale Law School⦒}",
                    "    departments{⍢⦑MIT Department of Physics|Harvard Department of Philosophy⦒}",
                    "    centers/institutes{⍢⦑Stanford Center for AI|MIT Media Lab⦒}",
                    "    constituent colleges{⍢⦑Trinity College, Cambridge|Balliol College, Oxford⦒}",
                    "    established names≫avoiding over-specificity",
                    "Name forms follow institutional conventions:",
                    "  universities{full or established short forms:",
                    "    ✓⍢⦑Harvard University|MIT|Stanford|Oxford|Cambridge⦒",
                    "    ✓⍢⦑University of Chicago|University of California, Berkeley⦒",
                    "    ✗⍢⦑Harvard⦒when ambiguous",
                    "    conventional usage≫full legal names}",
                    "  colleges{",
                    "    ✓⍢⦑Williams College|Amherst College|Trinity College⦒",
                    "    ✓⍢⦑Trinity College, Cambridge⦒when disambiguation needed}",
                    "  schools/departments{",
                    "    ✓⍢⦑Harvard Business School|Yale Law School⦒professional schools",
                    "    ✓⍢⦑MIT Department of Physics⦒when specifically relevant",
                    "    subdivision only when contextually meaningful}",
                    "Discrimination from related entities:",
                    "  university vs university press{⍢⦑MIT⦒university∧⍓⦑MIT Press⦒publisher",
                    "    both valid when contextually relevant:",
                    "    paper discussing MIT research→⍢⦑MIT⦒",
                    "    paper citing MIT Press book→⍓⦑MIT Press⦒",
                    "    educational function=discriminator}",
                    "  university vs research lab{⍢university∧⍚lab as separate entities when distinct}",
                    "  academic system vs institution{⍢⦑UC System⦒system∧⍢⦑UC Berkeley⦒campus both valid}",
                    "Generic educational categories{DELETE:",
                    "  ✗⍢⦑Universities|Colleges|Business Schools|Law Schools|Engineering Schools⦒",
                    "  categorical plurals→DELETE}",
                    "Historical institutions valid when institutional:",
                    "  ⍢⦑Academy of Athens|University of Paris⦒(even if defunct/reorganized)",
                    "  institutional existence≫current operations",
                    "Research institutes discrimination:",
                    "  primarily educational{⍢⦑Caltech|Georgia Tech⦒degree-granting institutions}",
                    "  primarily research{⍚⦑Bell Labs|RAND Corporation⦒non-educational research orgs}",
                    "  educational mission=discriminator",
                    "International naming:",
                    "  ⍢⦑École Normale Supérieure|Tsinghua University|University of Tokyo⦒",
                    "  use conventional English forms when standard"
                ]),
                ("≟", [
                    "educational mission primary function?→⍢",
                    "degree-granting institution?→⍢",
                    "research org without educational mission?→⍚",
                    "subdivision within university?→⍢both",
                    "university system vs campus?→⍢both:",
                    "  test{teaching/research primary organizational function?→YES:⍢|NO:continue}",
                    "  test{university vs university press?→⍢university∧⍓press|NO:continue}",
                    "  test{degree-granting authority?→YES:⍢|NO:continue}",
                    "  test{research org without teaching?→YES:⍚|NO:continue}",
                    "  test{non-educational organization?→YES:⍚|NO:review}"
                ]),
                ("⊨", "⍢⊂⍚ ∧ educational mission≫generic coordination"),
            ],
        },
    ]
}
