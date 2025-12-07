//! ⍚ Organization entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type OrgDef = EntityDef;

pub fn get_entity_definitions() -> Vec<OrgDef> {
    vec![
        OrgDef {
            symbol: "⍚",
            name: "Organization",
            description: "Non-governmental coordination body with formal institutional structure",
            sort_order: 8,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["organization", "coordinated_group", "structured_collective"]),
                ("≝", "⛫_coordination ∧ voluntary∨chartered membership ∧ shared purpose ∧ ¬enforcement_authority"),
                ("∂", [
                    "⍚⊅⎈ (⍚ ⛫_coordination | ⎈ ⛫_enforcement)",
                    "⍚⊅⍢ (⍚ ⛫_general | ⍢ ⛫_educational)",
                    "⍚⊅⌯ (⍚ ⛫_institutional | ⌯ ¬⛫_diffuse)",
                    "⍚⊅⧈ (⍚ membership_voluntary | ⧈ category_ascribed)"
                ]),
                ("⊡", [
                    "⍚: Բ7Փ7Ֆ6Պ5Հ6Շ5Ի6Ղ7Ց5",
                    "⎈: Բ9Փ9Ֆ9Պ3Հ8Շ7Ի7Ղ9Ց3",
                    "⍢: Բ7Փ8Ֆ7Պ6Հ7Շ5Ի7Ղ6Ց4",
                    "⌯: Բ3Փ4Ֆ2Պ8Հ2Շ6Ի4Ղ5Ց7"
                ]),
                ("⊡", [
                    "gradients:",
                    "⍚→⎈: Բ↑Փ↑Ֆ↑Պ↓Հ↑Ղ↑Ց↓ (boundary hardens, formalization increases, closes, hierarchy deepens, gains sovereignty, stabilizes)",
                    "⍚→⌯: Բ↓Ֆ↓Պ↑Հ↓Ի↓Ց↑ (boundary dissolves, informality, opens, flattens, fragments, destabilizes)",
                    "⍚→⍢: Փ↑Ֆ↑ (specializes educational, formalizes mission)"
                ]),
                ("⊛", [
                    "corporations: ⍚⦑Apple⦒ ∧ ⍚⦑Toyota⦒ ∧ ⍚⦑Goldman Sachs⦒",
                    "nonprofits: ⍚⦑Red Cross⦒ ∧ ⍚⦑Doctors Without Borders⦒ ∧ ⍚⦑ACLU⦒",
                    "research_orgs: ⍚⦑Bell Labs⦒ ∧ ⍚⦑RAND Corporation⦒ ∧ ⍚⦑Santa Fe Institute⦒",
                    "professional: ⍚⦑American Bar Association⦒ ∧ ⍚⦑IEEE⦒ ∧ ⍚⦑Royal Society⦒",
                    "trade_unions: ⍚⦑AFL-CIO⦒ ∧ ⍚⦑UAW⦒",
                    "historical: ⍚⦑East India Company⦒ ∧ ⍚⦑Hanseatic League⦒ ∧ ⍚⦑Académie française⦒"
                ]),
                ("≟", [
                    "test{⛫_enforcement?→YES:⎈|NO:continue}",
                    "test{⛫_educational_primary?→YES:⍢|NO:continue}",
                    "test{⛫_coordination∧membership?→YES:⍚|NO:continue}",
                    "test{¬⛫_diffuse_ideological?→YES:⌯|NO:continue}",
                    "test{ascribed_category?→YES:⧈|NO:review}"
                ]),
                ("≟", [
                    "vector_test:",
                    "Բ≥6∧Ֆ≥5∧Պ≤6→⍚∨⎈∨⍢ (institutional)",
                    "Բ≤4∧Ֆ≤3∧Պ≥7→⌯ (movement)",
                    "Փ9∧Ղ9→⎈ (enforcement∧sovereignty)",
                    "Փ8∧Շ5∧educational→⍢"
                ]),
                ("≟", [
                    "ancient_governing_bodies→⎈:",
                    "  ⎈⦑Athenian Assembly⦒ ∧ ⎈⦑Roman Senate⦒ ∧ ⎈⦑Spartan Council⦒",
                    "  Փ9Ղ9: ⛫_enforcement∧legislative_authority",
                    "historical_orgs→⍚:",
                    "  ⍚⦑Royal Society⦒ ∧ ⍚⦑East India Company⦒ ∧ ⍚⦑Hanseatic League⦒",
                    "  Բ7Փ7Ղ7: ⛫_coordination∧¬state_authority",
                    "historical_educational→⍢:",
                    "  ⍢⦑Academy of Athens⦒ ∧ ⍢⦑University of Bologna⦒ ∧ ⍢⦑University of Paris⦒",
                    "  Փ8Ֆ7: ⛫_educational∧degree_granting"
                ]),
                ("≟", [
                    "central_banks→⎈:",
                    "  ⎈⦑Federal Reserve⦒ ∧ ⎈⦑ECB⦒ ∧ ⎈⦑Bank of England⦒",
                    "  Փ9Ղ9Ֆ9: monetary_policy∧regulatory_authority",
                    "commercial_banks→⍚:",
                    "  ⍚⦑JPMorgan Chase⦒ ∧ ⍚⦑Deutsche Bank⦒",
                    "  Փ7Ղ7: ¬enforcement∧profit_motive"
                ]),
                ("≟", [
                    "research_institutes:",
                    "  ⛫_educational_primary→⍢: ⍢⦑Caltech⦒ ∧ ⍢⦑Georgia Tech⦒",
                    "  ⛫_coordination_research→⍚: ⍚⦑Bell Labs⦒ ∧ ⍚⦑RAND⦒ ∧ ⍚⦑Brookings⦒"
                ]),
                ("≟", [
                    "international_bodies:",
                    "  Փ9Ղ8→⎈: ⎈⦑WTO⦒ ∧ ⎈⦑ICC⦒ (can_compel)",
                    "  Փ6Ղ5→⍚: ⍚⦑United Nations⦒ ∧ ⍚⦑WHO⦒ ∧ ⍚⦑IMF⦒ (facilitates)"
                ]),
                ("⊨", "⍚ ≡ ⛫_coordination∧membership∧¬⛫_enforcement ∧ Բ7Փ7Ֆ6Պ5"),
            ],
        },
    ]
}
