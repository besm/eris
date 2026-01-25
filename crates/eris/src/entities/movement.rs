//! ⌯ Movement entity type

define_entity_module! {
    Entity {
        Conceptual => "Conceptual entity types",
    }
}

type MovementDef = EntityDef;

pub fn get_entity_definitions() -> Vec<MovementDef> {
    vec![
        MovementDef {
            symbol: "⌯",
            name: "Movement",
            description: "Social movement with sustained mobilization and collective identity",
            sort_order: 22,
            category: EntityCategory::Conceptual,
            lines: lines![
                ("≡", ["social_movement", "collective_action", "sustained_mobilization"]),
                ("≝", "sustained mobilization ∧ collective identity ∧ political/social goals ∧ ¬formal_nomination"),
                ("∂", [
                    "⌯⊅⍚ (⌯ diffuse | ⍚ institutionalized)",
                    "⌯⊅⚐ (⌯ ¬nominates_candidates | ⚐ nominates_candidates)",
                    "⌯⊅⧈ (⌯ movement | ⧈ adherents as category)",
                    "⌯⊅⧖ (⌯ mobilization | ⧖ temporal period)",
                    "⌯⊅❖ (⌯ activism | ❖ academic field)"
                ]),
                ("⊡", [
                    "⌯: Բ3Փ4Ֆ2Պ8Հ2Շ6Ի4Ղ5Ց7",
                    "institutionalizing: Բ↑Ֆ↑Պ↓ (→⍚ or →⚐)",
                    "diffusing: Պ↑Ի↓Ց↑ (fragmenting)"
                ]),
                ("⊛", [
                    "⌯⦑Civil Rights Movement|Labor Movement|Anti-War Movement|Occupy|Black Lives Matter⦒",
                    "⌯⦑Feminism|Environmentalism|LGBTQ Rights Movement|Disability Rights Movement⦒",
                    "⌯⦑Solidarity (Poland)|Arab Spring|Umbrella Movement|Landless Workers' Movement⦒",
                    "⌯⦑Peronism|Gandhian Movement|Negritude|Pan-Africanism|Zionism (pre-state)⦒",
                    "⌯⦑Frankfurt School|Vienna Circle|Jansenism|Oxford Movement|Transcendentalism⦒",
                    "⌯⦑Protestantism|Pietism|Pentecostalism|Liberation Theology⦒"
                ]),
                ("◻", [
                    "movement vs institution:",
                    "  ⌯⦑Frankfurt School⦒ (tradition) vs ⍚⦑Institute for Social Research⦒ (actual institute)",
                    "  ⌯⦑Zionism (pre-state)⦒ vs ⚐⦑Likud⦒ (party that contests)",
                    "  ⌯⦑Peronism⦒ (ideology/movement) vs ⚐⦑Partido Justicialista (PJ)⦒ (party)",
                    "rejects_electoralism∧'party'_name→⌯:",
                    "  vanguard formations rejecting electoral participation→⌯ regardless of name",
                    "  form decoupled from function"
                ]),
                ("≟", [
                    "test{nominates_candidates?→YES:⚐|NO:continue}",
                    "test{sustained mobilization∧collective identity?→YES:⌯|NO:continue}",
                    "test{formal organization∧membership?→YES:⍚|NO:continue}",
                    "test{academic field?→YES:❖|NO:continue}",
                    "test{adherents as people?→YES:⧈|NO:review}"
                ]),
                ("⊨", "⌯ ≡ sustained_mobilization ∧ collective_identity ∧ ¬nominates_candidates")
            ],
        },
    ]
}