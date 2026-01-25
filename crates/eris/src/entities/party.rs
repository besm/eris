//! ⚐ Political Party entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type PartyDef = EntityDef;

pub fn get_entity_definitions() -> Vec<PartyDef> {
    vec![
        PartyDef {
            symbol: "⚐",
            name: "Political Party",
            description: "Electoral organization that nominates candidates for public office",
            sort_order: 9,  // after ⍚
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["political_party", "electoral_organization", "candidate_nominating_body"]),
                ("≝", "nominates candidates for public office ∧ contests or seeks to contest elections"),
                ("∂", [
                    "⚐⊂⍚ (⚐ specialized ⍚ | electoral function)",
                    "⚐⊅⌯ (⚐ institutionalized∧nominates | ⌯ diffuse∧¬nominates)",
                    "⚐⊅⎈ (⚐ contests for power | ⎈ holds∧exercises power)",
                    "⚐⊅⧈ (⚐ organization | ⧈ supporters as category)"
                ]),
                ("⊡", [
                    "⚐: Բ6Փ7Ֆ5Պ5Հ6Շ6Ի6Ղ6Ց6",
                    "major_party: Բ7Փ8Ֆ6Պ4Հ7Շ7Ի7Ղ7Ց4",
                    "minor_party: Բ5Փ5Ֆ4Պ6Հ4Շ4Ի5Ղ4Ց7"
                ]),
                ("⊡", [
                    "gradients:",
                    "⚐→⎈: via electoral_victory→governing_authority (party becomes government)",
                    "⚐→⌯: Ֆ↓Պ↑Բ↓ (abandons elections, becomes movement)",
                    "⌯→⚐: Ֆ↑Պ↓Բ↑ (institutionalizes, begins contesting)"
                ]),
                ("⊛", [
                    "us_major: ⚐⦑Democratic Party⦒ ∧ ⚐⦑Republican Party⦒",
                    "us_minor: ⚐⦑Green Party⦒ ∧ ⚐⦑Libertarian Party⦒ ∧ ⚐⦑Communist Party USA⦒",
                    "uk: ⚐⦑Labour Party⦒ ∧ ⚐⦑Conservative Party⦒ ∧ ⚐⦑Liberal Democrats⦒",
                    "european: ⚐⦑CDU⦒ ∧ ⚐⦑SPD⦒ ∧ ⚐⦑Fidesz⦒ ∧ ⚐⦑Syriza⦒ ∧ ⚐⦑Five Star Movement⦒",
                    "historical: ⚐⦑Whig Party⦒ ∧ ⚐⦑Federalist Party⦒ ∧ ⚐⦑NSDAP⦒ ∧ ⚐⦑Bolshevik Party⦒",
                    "non_governing: ⚐⦑KKE⦒ ∧ ⚐⦑PCF⦒ ∧ ⚐⦑CPGB⦒"
                ]),
                ("≟", [
                    "test{nominates_candidates?→YES:⚐|NO:continue}",
                    "test{rejects_electoralism?→YES:⌯|NO:continue}",
                    "test{governing_apparatus?→YES:⎈|NO:continue}",
                    "test{supporters_as_people?→YES:⧈⦑Democrats|Republicans⦒|NO:review}"
                ]),
                ("≟", [
                    "edge_cases:",
                    "  banned_party_seeking_restoration→⚐: ⚐⦑PKK political wing⦒ (seeks to contest)",
                    "  vanguard_rejecting_elections→⌯: ⌯⦑Maoist Movement⦒ (¬nominates)",
                    "  party_in_government→⚐∧⎈: ⚐⦑Labour Party⦒ contest ∧ ⎈⦑UK Government⦒ governs",
                    "  faction_within_party→⧈: ⧈⦑Progressive Democrats⦒ ∨ ⧈⦑Tea Party Republicans⦒"
                ]),
                ("≟", [
                    "historical_discrimination:",
                    "  party_that_became_state→⚐ historical, ⎈ when governing:",
                    "    ⚐⦑NSDAP⦒ (party) ∧ ⎈⦑Nazi Germany⦒ (state)",
                    "    ⚐⦑Bolshevik Party⦒ (party) ∧ ⎈⦑Soviet Union⦒ (state)",
                    "  single_party_state: party∧state fused, context determines tag"
                ]),
                ("⊨", "⚐ ≡ nominates_candidates ∧ contests_elections ∧ ⚐⊂⍚"),
            ],
        },
    ]
}