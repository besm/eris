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
                ("≝", "⛫_coordination ∧ membership ∧ ¬enforcement_authority ∧ ¬nominates_candidates"),
                ("∂", [
                    "⍚⊅⎈ (⍚ coordination | ⎈ enforcement)",
                    "⍚⊅⍢ (⍚ general | ⍢ educational)",
                    "⍚⊅⌯ (⍚ institutional | ⌯ diffuse)",
                    "⍚⊅⧈ (⍚ membership | ⧈ ascribed category)",
                    "⍚⊃⚐ (⚐ specialized ⍚ | nominates candidates)"
                ]),
                ("⊡", [
                    "⍚: Բ7Փ7Ֆ6Պ5Հ6Շ5Ի6Ղ7Ց5",
                    "gradients:",
                    "  →⎈: Բ↑Փ↑Ֆ↑Պ↓Հ↑Ղ↑Ց↓ | →⌯: Բ↓Ֆ↓Պ↑Հ↓Ի↓Ց↑ | →⚐: +nominates"
                ]),
                ("⊛", [
                    "corporate: ⍚⦑Apple|Toyota|Goldman Sachs|Samsung|Tata Group|Alibaba|HSBC⦒",
                    "nonprofit: ⍚⦑Red Cross|Doctors Without Borders|ACLU|Oxfam|Amnesty International⦒",
                    "research: ⍚⦑Bell Labs|RAND Corporation|Santa Fe Institute|Brookings Institution|Max Planck Society|Fraunhofer Society⦒",
                    "professional: ⍚⦑American Bar Association|IEEE|Royal Society|British Medical Association⦒",
                    "labor: ⍚⦑AFL-CIO|UAW|IG Metall|Solidarity|Congress of South African Trade Unions⦒",
                    "international: ⍚⦑United Nations|WHO|IMF|African Union⦒",
                    "historical: ⍚⦑East India Company|Hanseatic League|Académie française|Dutch East India Company⦒"
                ]),
                ("◻", [
                    "vector test: Բ≥6∧Ֆ≥5∧Պ≤6→institutional | Բ≤4∧Ֆ≤3∧Պ≥7→⌯",
                    "discrimination:",
                    "  ⍚⦑United Nations|WHO|IMF⦒ (facilitates) vs ⎈⦑WTO|ICC⦒ (compels)",
                    "  ⍚⦑JPMorgan Chase|Deutsche Bank⦒ (commercial) vs ⎈⦑Federal Reserve|ECB⦒ (regulatory)"
                ]),
                ("≟", [
                    "test{enforcement?→YES:⎈|NO:continue}",
                    "test{educational primary?→YES:⍢|NO:continue}",
                    "test{nominates candidates?→YES:⚐|NO:continue}",
                    "test{coordination∧membership?→YES:⍚|NO:continue}",
                    "test{diffuse ideological?→YES:⌯|NO:continue}",
                    "test{ascribed category?→YES:⧈|NO:review}"
                ]),
                ("⊨", "⍚ ≡ ⛫_coordination ∧ membership ∧ ¬enforcement ∧ ¬nominates")
            ],
        },
    ]
}