//! ⚘ Person entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type PersonDef = EntityDef;

pub fn get_entity_definitions() -> Vec<PersonDef> {
    vec![
        PersonDef {
            symbol: "⚘",
            name: "Person",
            description: "Named individual, human agent, biographical subject",
            sort_order: 1,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["named_individual", "human_agent", "biographical_subject"]),
                ("≝", "historical attestation ∧ proper name ∧ attributed author"),
                ("∂", [
                    "⚘⊅⧈ (⚘ individual | ⧈ category of people)",
                    "⚘⊅⧊ (⚘ person | ⧊ role/concept)",
                    "⚘⊅fictional (⚘ attested | fictional→∅)"
                ]),
                ("⊛", [
                    "⚘⦑C.S. Peirce|W.E.B. Du Bois|J.L. Austin|Hannah Arendt|Simone de Beauvoir⦒",
                    "⚘⦑Karl Marx|Max Weber|Émile Durkheim|Michel Foucault|Pierre Bourdieu⦒",
                    "⚘⦑Ibn Khaldun|Frantz Fanon|Gayatri Spivak|Edward Said|Amartya Sen⦒",
                    "⚘⦑Confucius|Mozi|Zhuangzi|Xuanzang|Wang Yangming⦒",
                    "⚘⦑Pytheas of Massalia|Hypatia of Alexandria|Avicenna|Maimonides⦒"
                ]),
                ("◻", [
                    "naming:",
                    "  ✓⚘⦑J.L. Austin|W.E.B. Du Bois|C.S. Peirce⦒ — no space between initials",
                    "  ✗⚘⦑J. L. Austin|W. E. B. Du Bois|C. S. Peirce⦒ — spaces between initials",
                    "  toponym: ⚘⦑Pytheas of Massalia⦒ — complete_id ¬separate ⌖",
                    "  ✗⚘⦑Person (Disambiguation)⦒ — no parenthetical",
                    "  co-authors: ⚘⦑Author1∧Author2⦒",
                    "compound citations:",
                    "  ⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒ (book)",
                    "  ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒ (article)"
                ]),
                ("≟", [
                    "test{named individual?→YES:⚘|NO:continue}",
                    "test{'the [occupation]'?→YES:⧈|NO:continue}",
                    "test{collective/category?→YES:⧈|NO:continue}",
                    "test{role/concept?→YES:⧊|NO:continue}",
                    "test{fictional?→YES:∅|NO:review}"
                ]),
                ("⊨", "⚘ ≡ historically_attested ∧ named_individual")
            ],
        },
    ]
}