//! ⌁ Event entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type EventDef = EntityDef;

pub fn get_entity_definitions() -> Vec<EventDef> {
    vec![
        EventDef {
            symbol: "⌁",
            name: "Event",
            description: "Discrete historical occurrence with bounded causality",
            sort_order: 5,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["discrete occurrence", "historical significance"]),
                ("≝", ["specific occurrence", "precise timing", "bounded causality", "unique temporal placement"]),
                ("∂", [
                    "⌁⊅{⧖ sustained spans, ⊙ bare dates}",
                    "⌁⊅⧊_event_categories (types of events ≠ discrete occurrences)",
                    "occurred→⌁ | category of occurrences→⧊"
                ]),
                ("⊡", [
                    "typical: Շ5Չ8Ֆ6",
                    "⌁⦑Soviet Atomic Bomb Test⦒: Շ8Չ9Ֆ9",
                    "⌁⦑Battle of Amorgos⦒: Շ4Չ9Ֆ5",
                    "⌁⦑French Revolution⦒: Շ7Չ6Ֆ9"
                ]),
                ("⊛", [
                    "⌁⦑World War II|French Revolution|Battle of Amorgos⦒",
                    "⌁⦑Bandung Conference|Soviet Atomic Bomb Test⦒"
                ]),
                ("⧠", [
                    "'Car Accidents'→⧊ (category of events)",
                    "'the 1955 Le Mans disaster'→⌁ (discrete occurrence)",
                    "'Mining Accidents'→⧊ (type)",
                    "'Sago Mine disaster'→⌁ (specific)",
                    "'Cold War'→⧖ (sustained competition)",
                    "'Cuban Missile Crisis'→⌁ (discrete within Cold War)"
                ]),
                ("◻", [
                    "naming: descriptive ∂¬dates in names",
                    "  ✓⌁⦑Battle of Amorgos⦒ ✗⌁⦑March 15, 1963⦒",
                    "duration: causal unity > temporal length",
                    "  ⌁⦑WWII⦒ 6yrs single causal chain = one event",
                    "  ⧖⦑Space Race⦒ sustained competition = era",
                    "vs ⧖: test{Δt<1yr∧discrete→⌁ | Δt>10yr∧sustained→⧖}",
                    "  1yr<Δt<10yr→test causal structure",
                    "  grammatical: 'X occurred'→⌁ | 'during X'→⧖",
                    "vs ⧊: test{specific occurrence?→⌁ | category of occurrences?→⧊}"
                ]),
                ("≟", [
                    "○₁ specificity: discrete occurrence?→continue | category/type?→⧊",
                    "○₂ duration: Δt<1yr∧discrete?→⌁ | sustained?→continue",
                    "○₃ structure: single causal chain?→⌁ | ongoing competition?→⧖",
                    "○₄ temporal: bare date?→⊙ | named occurrence?→⌁"
                ]),
                ("⊨", "⌁⊂discrete occurrences ∧ ∂start ∧ ∂end ∧ ∃t₀happened(event,t₀)")
            ],
        },
    ]
}
