//! 𝄏 Journal entity type

define_entity_module! {
    Entity {
        Compound => "Compound citation component types",
    }
}

type JournalDef = EntityDef;

pub fn get_entity_definitions() -> Vec<JournalDef> {
    vec![
        JournalDef {
            symbol: "𝄏",
            name: "Journal",
            description: "Periodical publication: academic journals, magazines, newspapers",
            sort_order: 51,
            category: EntityCategory::Compound,
            lines: lines![
                ("≡", "periodical publication ∧ serial venue"),
                ("≝", "academic journals ∧ magazines ∧ newspapers ∧ serial publications ∧ institutional periodicals"),
                ("∂", "𝄏⊅{⊳standalone,⍚organizations,books}"),
                ("◻", "◻{serial format,publication regularity}"),
                ("⊛", [
                    "𝄏⦑Nature|Daedalus|London Review of Books|Harper's Magazine|Popular Science Monthly⦒",
                    "𝄏∈⚘⊙𝄏⊳article citations"
                ]),
                ("◻", [
                    "Full journal names ∧ established publication titles:",
                    "  academic{𝄏⦑Nature|Science|Daedalus|Journal of Roman Studies⦒}",
                    "  magazines{𝄏⦑Harper's Magazine|Fortune|Byte Magazine⦒}",
                    "  newspapers{𝄏⦑Financial Times|Asahi Shinbun|Houston Press⦒}",
                    "  reviews{𝄏⦑London Review of Books|ETC: A Review of General Semantics⦒}",
                    "  historical{𝄏⦑Popular Science Monthly|Der Monat|Anti-Masonic Enquirer⦒}",
                    "  full names≫abbreviations(unless established)",
                    "Standalone vs compound institutional context→𝄏|specific article→⚘⊙𝄏⊳:",
                    "  standalone{𝄏⦑Nature⦒when discussing journal as institution}",
                    "  compound{⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Nature⦒⦑Article⦒for specific article}",
                    "  rationale{𝄏=periodical as entity|⚘⊙𝄏⊳=article citation}",
                    "  institutional reference vs bibliographic citation",
                    "Journal vs organization discrimination publication→𝄏|publisher→⍚:",
                    "  𝄏⦑Daedalus⦒(periodical)",
                    "  ⍚⦑American Academy of Arts and Sciences⦒(organization publishing Daedalus)",
                    "  test{serial publication format?→𝄏|institutional publisher?→⍚}",
                    "  publication vehicle≠publishing organization",
                    "Journal vs book discrimination serial→𝄏|standalone→⊳:",
                    "  𝄏⦑Encyclopaedia Metropolitana⦒(serial encyclopedia periodic fascicles)",
                    "  ⊳⦑Encyclopaedia Britannica⦒(standalone encyclopedia single work)",
                    "  test{periodic issues?→𝄏|one-time publication?→⊳}",
                    "  serialization=discriminator",
                    "Abbreviations use full name unless established:",
                    "  ✓𝄏⦑IEEE Technology and Society Magazine⦒(spell out IEEE)",
                    "  ✓𝄏⦑Byte⦒(one-word)",
                    "  ∂avoid obscure acronyms",
                    "  clarity≫brevity",
                    "Subtitles/descriptors include when part of official title:",
                    "  𝄏⦑Dialogue: A Journal of Mormon Thought⦒",
                    "  𝄏⦑ETC: A Review of General Semantics⦒",
                    "  𝄏⦑2600: The Hacker Quarterly⦒",
                    "  official title format≫simplified",
                    "News services treat as publications when functioning as periodical:",
                    "  𝄏⦑Associated Press⦒(wire service)",
                    "  𝄏⦑Axios⦒(digital news)",
                    "  publication function=discriminator",
                    "Historical publications valid regardless of current status:",
                    "  𝄏⦑Der Monat⦒(defunct Cold War magazine)",
                    "  𝄏⦑Anti-Masonic Enquirer⦒(19th century newspaper)",
                    "  𝄏⦑CoEvolution Quarterly⦒(1970s counterculture)",
                    "  historical publications legitimate",
                    "Specialty publications academic/trade/popular/underground all valid:",
                    "  academic{𝄏⦑Annals of the History of Computing⦒}",
                    "  trade{𝄏⦑Datamation|Chemical Engineering Education⦒}",
                    "  popular{𝄏⦑Byte Magazine|Fortune⦒}",
                    "  underground{𝄏⦑Computer Underground Digest|FidoNews⦒}",
                    "  publication type diversity",
                    "Compound citation structure:",
                    "  ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒",
                    "  single author{⚘⊙𝄏⊳⦑C.S. Peirce⦒⦑1878⦒⦑Popular Science Monthly⦒⦑How to Make Our Ideas Clear⦒}",
                    "  co-authors{⚘⊙𝄏⊳⦑Marion Fourcade∧Kierian Healy⦒⦑2017⦒⦑Socio-Economic Review⦒⦑Seeing Like a Market⦒}",
                    "  structure{⚘author⊙year𝄏journal⊳article}",
                    "  separator∧for co-authors"
                ]),
                ("≟", [
                    "\"serial publication\"→𝄏",
                    "\"standalone work\"→⊳",
                    "\"publisher organization\"→⍚",
                    "\"academic field\"→❖:",
                    "  test1{periodic issues?→YES:𝄏|NO:continue}",
                    "  test2{one-time publication?→YES:⊳|NO:continue}",
                    "  test3{institutional publisher?→YES:⍚|NO:continue}",
                    "  test4{academic discipline?→YES:❖|NO:review}"
                ]),
                ("⊨", "𝄏⊂serial publications ∧ periodical venues ∧ regular issues ∧ 𝄏⊅{standalone,publishers,one-time books}"),
            ],
        },
    ]
}
