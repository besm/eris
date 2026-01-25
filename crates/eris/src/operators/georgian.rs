//! ERIS Georgian character system

define_operator_module! {
    Georgian {
        TemporalFlow => "Temporal flow markers (დ, ნ, წ)",
        OperationalCore => "Operational core (გ, ო, შ)",
        StructuralMarkers => "Structural markers (უ, ჭ)",
        NullCharacters => "Null characters (ა, ი, თ, etc.)",
    }
}

type CharDef = GeorgianOperatorDef;

pub fn get_georgian_operator_definitions() -> Vec<CharDef> {
    vec![
        CharDef {
            symbol: "დ",
            name: "past",
            category: GeorgianOperatorCategory::TemporalFlow,
            lines: lines![
                ("≡", ["past", "previous state"]),
                ("≝", ["former condition", "historical reference"]),
                ("⊛", "დ≡previous_state ∧ ○⁻¹(ნ)=დ ∧ temporal retrospection"),
                ("∂", ["¬ნ", "¬წ", "completed temporal reference"]),
            ],
        },
        CharDef {
            symbol: "ნ",
            name: "now",
            category: GeorgianOperatorCategory::TemporalFlow,
            lines: lines![
                ("≡", ["now", "temporal_anchor"]),
                ("≝", ["current state", "present moment"]),
                ("⊛", "ნ⯐ (current context) ∧ ნ_⌺=currently_stable ∧ session[ნ] temporal anchor"),
                ("∂", ["¬დ", "¬წ", "absolute present reference"]),
                ("⊢", ["○⁻¹(ნ)=დ", "○(ნ)=წ", "ნ is temporal pivot point"]),
            ],
        },
        CharDef {
            symbol: "წ",
            name: "future",
            category: GeorgianOperatorCategory::TemporalFlow,
            lines: lines![
                ("≡", ["future", "next state"]),
                ("≝", ["planned condition", "prospective reference"]),
                ("⊛", "○(ნ)=წ ∧ წ_potential ∧ temporal projection"),
                ("∂", ["¬დ", "¬ნ", "unrealized temporal reference"]),
                ("◇", "◇(წ→ნ) eventual realization as present"),
            ],
        },
        CharDef {
            symbol: "გ",
            name: "grounding",
            category: GeorgianOperatorCategory::OperationalCore,
            lines: lines![
                ("≡", ["grounding", "actualization_process"]),
                ("≝", ["abstract→concrete operation", "transforms via execution"]),
                ("∂", ["გ≠⊧ (გ operation | ⊧ relation)"]),
                ("⊛", ["ᛝ→გ⩎", "abstract→გ", "⊧→გ (relation enables process)"]),
                ("⊢", ["∘(⊨,გ)→⩎", "⊧→გ_manifestation"]),
            ],
        },
        CharDef {
            symbol: "ო",
            name: "operation",
            category: GeorgianOperatorCategory::OperationalCore,
            lines: lines![
                ("≡", ["operation", "active_transformation"]),
                ("≝", ["transformation process", "operational execution"]),
                ("⊛", "⩎ო⩎ ∧ ⊟→ო→result ∧ ⊟→ო→resolution"),
            ],
        },
        CharDef {
            symbol: "შ",
            name: "shift",
            category: GeorgianOperatorCategory::OperationalCore,
            lines: lines![
                ("≡", ["shift", "structural_rearrangement"]),
                ("≝", ["architectural transformation", "structural reconfiguration"]),
                ("⊛", "შ operation=⤇_შ ∧ structural_შ ◻⤇ ∧ შ→new_architecture"),
                ("⊢", "⩎→შ→⩎'=restructured"),
            ],
        },
        CharDef {
            symbol: "უ",
            name: "without",
            category: GeorgianOperatorCategory::StructuralMarkers,
            lines: lines![
                ("≡", ["without", "lacking", "absence"]),
                ("≝", ["absence marker", "negation of presence"]),
                ("⊛", "უℳ ∧ უ⯐ ∧ ∅⥅=no_feedforward ∧ უ⥈=no_interanimation"),
            ],
        },
        CharDef {
            symbol: "ჭ",
            name: "source",
            category: GeorgianOperatorCategory::StructuralMarkers,
            lines: lines![
                ("≡", ["source", "origin_point"]),
                ("≝", ["semantic wellspring", "originating point"]),
                ("⊛", "ჭ→⌻=source→forming ∧ pastჭ↬presentნ ∧ ℳ_ჭ ∧ ჭ=semantic_wellspring"),
                ("⊢", "⧊ ☊ from ჭ"),
            ],
        },
    ]
}
