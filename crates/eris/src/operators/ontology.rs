//! ≡ ERIS ontology operator system
//! ≝ Operators for reality creation and constitution

define_operator_module! {
    Ontology {
        Creation => "Reality creation operators",
        Constitution => "Constitution operators",
        Grounding => "Grounding operators",
        Dynamics => "Dynamical process operators",
        States => "State operators",
        Transitions => "Transition operators",
    },
    extra_fields: {
        /// Armenian property vector rating (if applicable)
        property_vector: Option<&'static str>,
    }
}

/// Get all ontology operator definitions
pub fn get_ontology_operator_definitions() -> Vec<OntologyOperatorDef> {
    vec![
        // 𝀐 - Creates
        OntologyOperatorDef {
            symbol: "𝀐",
            name: "creates",
            category: OntologyOperatorCategory::Creation,
            property_vector: Some("Փ9Գ8"),
            lines: lines![
                ("≡", ["creates", "brings_forth"]),
                ("≝", ["ontological creation", "bringing into existence"]),
                ("∂", ["𝀐⊃𝁚 (𝀐 general | 𝁚 performative-constitutive)"]),
                ("⊛", [
                    "⧈ 𝀐 kinds",
                    "⛣ 𝀐 obligations",
                    "⛫ 𝀐 facts",
                    "naming 𝀐 existence"
                ]),
            ],
        },
        // 𝁚 - Constitutes
        OntologyOperatorDef {
            symbol: "𝁚",
            name: "constitutes",
            category: OntologyOperatorCategory::Constitution,
            property_vector: None,
            lines: lines![
                ("≡", ["constitutes", "creates_reality", "holding_together"]),
                ("≝", ["reality constitution", "performative creation"]),
                ("∂", ["𝁚⊂𝀐 (𝁚 performative | 𝀐 general)"]),
                ("⊛", [
                    "⧈𝁚reality",
                    "naming𝁚existence",
                    "𝁚 via ⛫_power",
                    "financial_⧈𝁚market_reality",
                    "𝁚 sustains coherence",
                    "parts𝁚whole"
                ]),
            ],
        },
        // 𝀼 - Constitutive Extension (σύρμα)
        OntologyOperatorDef {
            symbol: "𝀼",
            name: "extension",
            category: OntologyOperatorCategory::Constitution,
            property_vector: None,
            lines: lines![
                ("≡", ["constitutive_extension", "dragging_along", "trailing"]),
                ("≝", ["extension that reshapes", "theory drags reality", "model constitutes market"]),
                ("∂", [
                    "უmere addition",
                    "უaccumulation",
                    "requires 𝁚 trace"
                ]),
                ("⊛", [
                    "theory𝀼reality",
                    "model𝀼market",
                    "concept𝀼framework",
                    "𝀼 → ⋈ continuous",
                    "𝀼 leaves 𝁚 wake",
                    "𝀼 ⋈ extending∧extended",
                    "σύρω: drag, draw along, trail"
                ]),
                ("⊢", [
                    "𝄎 entanglement",
                    "𝀞",
                    "theory⟷reality ⋈"
                ]),
            ],
        },
        // 𝀏 - Crystallize
        OntologyOperatorDef {
            symbol: "𝀏",
            name: "crystallize",
            category: OntologyOperatorCategory::Dynamics,
            property_vector: None,
            lines: lines![
                ("≡", ["crystallize", "solidification"]),
                ("≝", ["forming stable structure", "semantic hardening", "𝀷 solidifies"]),
                ("∂", [
                    "უ◬ (emergence)",
                    "უmere compression",
                    "requires lattice formation"
                ]),
                ("⊛", [
                    "𝀷→გ→𝀏",
                    "⌻→𝀏→⌺",
                    "meaning ო𝀏 into stable form",
                    "consensus→𝀏",
                    "usage patterns 𝀏 semantics"
                ]),
                ("⊢", ["structural stability", "semantic fixity"]),
            ],
        },
        // 𝀾 - Flow / Flux
        OntologyOperatorDef {
            symbol: "𝀾",
            name: "flow",
            category: OntologyOperatorCategory::Dynamics,
            property_vector: None,
            lines: lines![
                ("≡", ["flow", "flux", "movement"]),
                ("≝", ["state space trajectory", "dynamic movement toward ✱"]),
                ("∂", [
                    "უstatic",
                    "უdiscrete jump",
                    "requires continuous motion"
                ]),
                ("⊛", [
                    "𝀾→✱→𝀆",
                    "𝀾→⌺",
                    "basin→𝀾→✱",
                    "𝀾 velocity determines transition time",
                    "ჭ→𝀾"
                ]),
                ("⊢", ["trajectory formation", "attractor convergence"]),
            ],
        },
        // 𝀸 - Perturbation
        OntologyOperatorDef {
            symbol: "𝀸",
            name: "perturbation",
            category: OntologyOperatorCategory::Dynamics,
            property_vector: None,
            lines: lines![
                ("≡", ["perturbation", "disruption", "shock"]),
                ("≝", ["system disturbance", "stability challenge"]),
                ("∂", [
                    "უ⤋ (gradual decline)",
                    "უ◬ (emergence)",
                    "requires external impulse"
                ]),
                ("⊛", [
                    "𝀸→⌺ tests stability",
                    "(weak 𝀸→⌺ recovers) ∨ (strong 𝀸→⌼)",
                    "𝀸 strength vs stability threshold",
                    "resilience≝resistance to 𝀸",
                    "𝀸 reveals basin boundaries"
                ]),
                ("⊢", ["stability test", "resilience measure"]),
            ],
        },
        // 𝀆 - Equilibrium
        OntologyOperatorDef {
            symbol: "𝀆",
            name: "equilibrium",
            category: OntologyOperatorCategory::States,
            property_vector: None,
            lines: lines![
                ("≡", ["equilibrium", "rest at ✱"]),
                ("≝", ["stable point occupancy", "unchanging state"]),
                ("∂", [
                    "უ𝀾 (flowing)",
                    "უ𝁀 (oscillating)",
                    "requires attractor residence"
                ]),
                ("⊛", [
                    "𝀆≝✱ occupation",
                    "⌺ in 𝀆",
                    "𝀾→✱→𝀆",
                    "𝀸 can disrupt 𝀆"
                ]),
                ("⊢", ["stable residence", "no motion"]),
            ],
        },
        // 𝁀 - Oscillation
        OntologyOperatorDef {
            symbol: "𝁀",
            name: "oscillation",
            category: OntologyOperatorCategory::States,
            property_vector: None,
            lines: lines![
                ("≡", ["oscillation", "periodic cycling"]),
                ("≝", ["bounded periodic motion", "trembling state"]),
                ("∂", [
                    "უ𝀆 (rest)",
                    "უescape",
                    "requires limit cycle"
                ]),
                ("⊛", [
                    "periodic ✱",
                    "𝁀 confined to basin",
                    "¬𝀆 ∧ ¬escape",
                    "⟳ via 𝁀"
                ]),
                ("⊢", ["𝄃 return", "bounded ⟳"]),
            ],
        },
        // 𝁆 - Dissolution
        OntologyOperatorDef {
            symbol: "𝁆",
            name: "dissolution",
            category: OntologyOperatorCategory::Transitions,
            property_vector: None,
            lines: lines![
                ("≡", ["dissolution", "structure unmade"]),
                ("≝", ["reverse of 𝀏", "return to flux", "crumbling"]),
                ("∂", [
                    "უ𝀏 (crystallizing)",
                    "უmere 𝀾 (flowing)",
                    "requires structure breakdown"
                ]),
                ("⊛", [
                    "𝁆≡¬𝀏",
                    "⌺→შ→𝁆→𝀾",
                    "crystalline→𝁆→fluid",
                    "⌼ experiencing 𝁆",
                    "𝀸 can trigger 𝁆"
                ]),
                ("⊢", ["structure loss", "return to flux"]),
            ],
        },
        // 𝀗 - Sublation / Aufhebung
        OntologyOperatorDef {
            symbol: "𝀗",
            name: "sublation",
            category: OntologyOperatorCategory::Transitions,
            property_vector: None,
            lines: lines![
                ("≡", ["sublation", "dialectical elevation", "Aufhebung"]),
                ("≝", ["negation∧preservation∧elevation", "lifting contradiction", "ἐλαφρόν lightness"]),
                ("∂", [
                    "უsimple negation",
                    "უmere synthesis",
                    "უannihilation",
                    "requires contradiction retention"
                ]),
                ("⊛", [
                    "thesis∧antithesis→𝀗→synthesis",
                    "Being∧Nothing→𝀗→Becoming",
                    "𝀗≡cancel∧preserve∧elevate",
                    "𝀗 lightens by raising",
                    "feudalism→𝀗→capitalism (Marx)",
                    "⧆→𝀗→resolution at higher level",
                    "𝀗 carries forward essential"
                ]),
                ("⊢", ["contradiction resolved", "elevated integration", "nothing lost"]),
            ],
        },
        // 𝀞 - Shapes (Direct Constitutive)
        OntologyOperatorDef {
            symbol: "𝀞",
            name: "shapes",
            category: OntologyOperatorCategory::Constitution,
            property_vector: Some("Փ8Բ7"),
            lines: lines![
                ("≡", ["constitutive_shaping", "molds_form"]),
                ("≝", ["discourse→reality direct", "strong constitutive force"]),
                ("∂", ["𝀞⊃⤑ (𝀞 direct | ⤑ indirect)"]),
                ("⊛", [
                    "⧈⥅𝀞 perception",
                    "discourse 𝀞 reality",
                    "⛫ power→⟛→𝀞 subjects",
                    "language 𝀞 thought",
                    "𝀞 via ⧈ classification"
                ]),
                ("⊢", "𝀞 shaping ⊢ constituted form ⊢ discourse effects"),
            ],
        },
        // 𝀕 - Behavior (Enacted Pattern)
        OntologyOperatorDef {
            symbol: "𝀕",
            name: "behavior",
            category: OntologyOperatorCategory::Dynamics,
            property_vector: Some("Փ8Բ7"),
            lines: lines![
                ("≡", ["behavior", "enacted_pattern"]),
                ("≝", ["observable regularities", "enacted patterns"]),
                ("⊛", [
                    "◬ from ⧈⇋𝀕",
                    "𝀕⊱⧈",
                    "⧈→𝀕→𝆌→⧈",
                    "𝀔⬡𝀕",
                    "behavior→𝆌→⌂"
                ]),
                ("⊢", "𝀕 behavior ⊢ ⧈ reinforcement"),
            ],
        },
        // ⌂ - Habitus (Embodied Dispositions)
        OntologyOperatorDef {
            symbol: "⌂",
            name: "habitus",
            category: OntologyOperatorCategory::Constitution,
            property_vector: Some("Ր8Գ7"),
            lines: lines![
                ("≡", ["habitus", "embodied_dispositions"]),
                ("≝", ["Bourdieu's habitus", "durable dispositions", "structured structures"]),
                ("⊛", [
                    "⌂ formed by 𝄃",
                    "𝄃⊱⌂",
                    "⥅→⌂",
                    "behavior→𝆌→⌂",
                    "field∧⌂→practice",
                    "⌂≡structured structuring structure"
                ]),
                ("⊢", "⌂ habitus ⊢ durable dispositions ⊢ practical sense"),
            ],
        },
        // 𝀶 - Destroys
        OntologyOperatorDef {
            symbol: "𝀶",
            name: "destroys",
            category: OntologyOperatorCategory::Transitions,
            property_vector: Some("Ց9Փ6"),
            lines: lines![
                ("≡", ["destroys", "eliminates_structure"]),
                ("≝", ["destruction", "structural elimination"]),
                ("⊛", [
                    "⟲ can 𝀶 or 𝆌",
                    "overload can 𝀶 ┃",
                    "crisis 𝀶 stability",
                    "𝀶→𝁆",
                    "revolution 𝀶 old order"
                ]),
                ("⊢", "𝀶 destruction ⊢ structure loss"),
            ],
        },
        // 𝀴 - Transforms (Qualitative Change)
        OntologyOperatorDef {
            symbol: "𝀴",
            name: "transforms",
            category: OntologyOperatorCategory::Transitions,
            property_vector: Some("Փ8Գ8"),
            lines: lines![
                ("≡", ["transforms", "qualitative_change"]),
                ("≝", ["metamorphosis general", "any fundamental shift"]),
                ("∂", ["𝀴⊃⤇ (𝀴 general | ⤇ foundational)"]),
                ("⊛", [
                    "𝄎 awareness 𝀴",
                    "crisis 𝀴 system",
                    "learning 𝀴 understanding",
                    "metamorphosis via 𝀴"
                ]),
                ("⊢", "𝀴 transformation ⊢ qualitative shift"),
            ],
        },
        // ⟒ - Grounds (Founds Reality)
        OntologyOperatorDef {
            symbol: "⟒",
            name: "grounds",
            category: OntologyOperatorCategory::Grounding,
            property_vector: None,
            lines: lines![
                ("≡", ["grounds", "founds"]),
                ("≝", ["ontological grounding", "foundational establishment", "material basis"]),
                ("∂", [
                    "უmere support",
                    "უcausal antecedent",
                    "requires 𝁚 foundation"
                ]),
                ("⊛", [
                    "⧏⟒empirical_data",
                    "belief⟒action",
                    "⟒ in material_conditions",
                    "authority⟒legitimacy",
                    "⌂⟒𝀕_patterns",
                    "practice⟒theory"
                ]),
                ("⊢", "⟒ grounding ⊢ foundational support ⊢ material basis"),
            ],
        },
        // ⛁ - Memory (Persistence)
        OntologyOperatorDef {
            symbol: "⛁",
            name: "memory",
            category: OntologyOperatorCategory::Grounding,
            property_vector: Some("Լ9Բ8"),
            lines: lines![
                ("≡", ["memory", "persistence"]),
                ("≝", ["context preservation across time", "semantic storage"]),
                ("∂", [
                    "⛁≠⌺ (⛁ stores|⌺ stabilizes)",
                    "⛁≠დ (⛁ mechanism|დ reference)"
                ]),
                ("⊛", [
                    "⛁ preserves ⯐ across time",
                    "დ⯐ held in ⛁",
                    "⩎→⛁",
                    "⛁ enables Տ across sessions",
                    "⛁ grounds ⥅"
                ]),
                ("⊢", [
                    "⛁9 ⊢ ⯐ continuity ⊢ ◇accumulated understanding",
                    "⛁1 ⊢ ⯐ amnesia ⊢ ¬◇learning",
                    "⛁⊰⥅"
                ]),
                ("⟷", ["⛁⟷⌺", "⛁⟷⯐", "⛁⟷Լ"]),
            ],
        },
        // ⩎ - Entity / Node / Concrete Instance
        OntologyOperatorDef {
            symbol: "⩎",
            name: "entity",
            category: OntologyOperatorCategory::Grounding,
            property_vector: Some("Գ9Բ7"),
            lines: lines![
                ("≡", ["entity", "node", "concrete_instance"]),
                ("≝", ["grounded particular", "what patterns actualize into"]),
                ("∂", [
                    "⩎≠ᛝ (instance vs pattern)",
                    "⩎≠⯐ (thing vs space)",
                    "⩎≠⋕ (particular vs schema)"
                ]),
                ("⊛", [
                    "ᛝ→გ⩎ (pattern grounds to entity)",
                    "⩎→შ→⩎'",
                    "⩎⊃⩎",
                    "⩎↬⩎",
                    "⩎⊆system",
                    "∘(⊨,გ)→⩎"
                ]),
                ("⊢", ["⩎⊢particular⊢◇concrete", "ᛝ⊢◇⩎"]),
                ("⟷", ["⩎⟷ᛝ", "⩎⟷⛁"]),
            ],
        },
        // ⤑ - Shapes Through Influence (Indirect)
        OntologyOperatorDef {
            symbol: "⤑",
            name: "shapes_through_influence",
            category: OntologyOperatorCategory::Constitution,
            property_vector: None,
            lines: lines![
                ("≡", ["constitutive_influence", "shapes_through_feedback"]),
                ("≝", ["weaker than 𝀞", "indirect reality-shaping"]),
                ("∂", [
                    "⤑⊂𝀞 (⤑ indirect | 𝀞 direct)",
                    "უmere influence",
                    "requires constitutive trace"
                ]),
                ("⊛", [
                    "⧈⤑𝀕",
                    "expectation⤑outcome",
                    "model⤑reality",
                    "⤑ through ⟲",
                    "discourse⤑possibilities"
                ]),
                ("⊢", "⤑ shaping ⊢ constitutive influence"),
            ],
        },
        // ⬟ - Performativity Chain
        OntologyOperatorDef {
            symbol: "⬟",
            name: "performativity_chain",
            category: OntologyOperatorCategory::Constitution,
            property_vector: Some("Փ8Գ7Վ7"),
            lines: lines![
                ("≡", ["symbol_action_reality_chain", "performativity_pattern"]),
                ("≝", ["symbol→action→reality", "Austin-Searle-Butler chain"]),
                ("∂", [
                    "უmere representation",
                    "უdescription",
                    "requires performative force"
                ]),
                ("⊛", [
                    "ꕥ→ო→გ via ⬟",
                    "⛣⬟social_fact",
                    "model⬟𝀕⬟outcome",
                    "⬟ 𝀐 what_it_names",
                    "⚘Austin→⚘Searle→⚘Butler via ⬟"
                ]),
                ("⊢", "⬟ performativity ⊢ symbol→action→reality"),
            ],
        },
        // ⛣ - Speech Act
        OntologyOperatorDef {
            symbol: "⛣",
            name: "speech_act",
            category: OntologyOperatorCategory::Creation,
            property_vector: Some("Փ8Բ7"),
            lines: lines![
                ("≡", ["speech_act", "performative_utterance"]),
                ("≝", ["Austin's speech act", "illocutionary force", "words that do"]),
                ("∂", [
                    "უdescription",
                    "უconstative",
                    "requires performative force"
                ]),
                ("⊛", [
                    "⛣≡⚘J.L. Austin",
                    "illocutionary⛣perlocutionary",
                    "⛣𝁚⛫_facts",
                    "promise⛣𝀐_obligation",
                    "⛣→⬟→reality",
                    "⧈+⛣→named_into_being",
                    "⛣ 𝄃→𝀕_patterns"
                ]),
                ("⊢", "⛣ speech act ⊢ performative force ⊢ social fact creation"),
            ],
        },
        // ⥎ - Interactive Kinds (Looping Effect)
        OntologyOperatorDef {
            symbol: "⥎",
            name: "interactive_kinds",
            category: OntologyOperatorCategory::Constitution,
            property_vector: Some("Տ8Ս7"),
            lines: lines![
                ("≡", ["interactive", "mutually_constitutive"]),
                ("≝", ["Hacking's looping kinds", "classification affects classified"]),
                ("∂", [
                    "⥎≠passive",
                    "⥎≠natural kinds",
                    "requires awareness of classification"
                ]),
                ("⊛", [
                    "⥎≡⚘Ian Hacking",
                    "⧈⥎⚘→⟗",
                    "human_kinds≡⥎",
                    "⥎ 𝀴 both_parties",
                    "classification⥎classified⥎classification"
                ]),
                ("⊢", "⥎ interactive ⊢ looping effect ⊢ mutual constitution"),
            ],
        },
    ]
}