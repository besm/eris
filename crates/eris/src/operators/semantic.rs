//! ≡ ERIS semantic stability operator system
//! ≝ Operators for semantic gravity wells and crystallization states

use crate::entities::types::EntityTypeDef;

define_operator_module! {
    Semantic {
        StabilityStates => "Semantic stability states (⌺, ⌻, ⌼)",
        Dynamics => "Semantic dynamics operators",
        Processes => "Process operators (⟳, ⥅, ⤋, ⤇)",
        Emergence => "Emergence and boundary operators (◬, ┃)",
        Institutional => "Institutional operators (⛫)",
        Performative => "Performative mechanism operators (⟚, ⟛)",
        Relations => "Relational operators (⋈, ⌾)",
        Contextual => "Context and framing operators (⯐)",
    },
    extra_fields: {
        entity_type: Option<EntityTypeDef>
    }
}

/// Get all semantic stability operator definitions
pub fn get_semantic_operator_definitions() -> Vec<SemanticOperatorDef> {
    vec![
        // ⌺ - Stable Well (Semantic Gravity Well)
        SemanticOperatorDef {
            symbol: "⌺",
            name: "stable_well",
            category: SemanticOperatorCategory::StabilityStates,
            lines: lines![
                ("≡", ["semantic gravity well", "stable ✱", "𝀆"]),
                ("≝", ["maximum compression state", "𝀏 completed", "equilibrium residence"]),
                ("∂", ["¬⌻ (forming)", "¬⌼ (dissolving)", "requires □ stability threshold"]),
                ("⊛", [
                    "∘(meaning, consensus)→⌺",
                    "⌺=maximum_compression ∧ pacts⌺=solidified",
                    "□⌺=permanently_stable ∧ ნ_⌺=currently_stable",
                    "components⨝→⌺",
                    "usage patterns 𝆌 ⌺",
                    "thematic clustering via gravitational pull",
                    "⌺≡𝀆✱ at rest"
                ]),
                ("⊢", "⌺ attracts related content ⊢ semantic clustering ⊢ knowledge organization"),
            ],
            entity_type: None,
        },
        // ⌻ - Forming Well
        SemanticOperatorDef {
            symbol: "⌻",
            name: "forming_well",
            category: SemanticOperatorCategory::StabilityStates,
            lines: lines![
                ("≡", ["forming well", "◬✱"]),
                ("≝", ["moderate elaboration state", "actively 𝀏", "𝀷 in progress"]),
                ("∂", ["¬⌺ (not yet stable)", "¬⌼ (not dissolving)", "ჭ→⌻ source flowing"]),
                ("⊛", [
                    "⧊ actively ⌻",
                    "◇(𝀷→⌻→𝀏→⌺)",
                    "new patterns 𝀷 coalescing",
                    "moderate compression gaining coherence",
                    "𝀏 in progress",
                    "many→𝀷→one integration",
                    "contradictions 𝀗 into coherent ⌻"
                ]),
                ("⊢", "⌻ transitional state ⊰ stable well ⊢ development in progress"),
            ],
            entity_type: None,
        },
        // ⌼ - Dissolving Well
        SemanticOperatorDef {
            symbol: "⌼",
            name: "dissolving_well",
            category: SemanticOperatorCategory::StabilityStates,
            lines: lines![
                ("≡", ["dissolving well", "⤋✱", "𝁆"]),
                ("≝", ["full explanation needed", "unreliable semantic structure", "structure unmade"]),
                ("∂", ["¬⌺ (lost stability)", "¬⌻ (not forming)", "warns instability"]),
                ("⊛", [
                    "დ ⌺→⤋→𝁆→⌼",
                    "✱ experiencing ⤋ ∧ 𝁆",
                    "¬◇⌺ when ⌼",
                    "requires re-examination",
                    "needs full elaboration for stability recovery",
                    "⌼≡𝁆 in progress"
                ]),
                ("⊢", "⌼ decay state ⊢ intervention required ⊢ policy clarification needed"),
            ],
            entity_type: None,
        },
        // ⤋ - Decline / Degradation
        SemanticOperatorDef {
            symbol: "⤋",
            name: "decline",
            category: SemanticOperatorCategory::Dynamics,
            lines: lines![
                ("≡", ["decline", "degradation"]),
                ("≝", ["deterioration process", "quality loss"]),
                ("⊛", [
                    "⌺→⤋→⌼",
                    "stability⤋ ∧ quality⤋ ∧ coherence⤋",
                    "tech debt ⤋ accumulation",
                    "↗ regression ⤋",
                    "⤋→𝁆 trajectory to dissolution"
                ]),
                ("⊢", "⤋ trajectory ⊢ instability ⊢ requires intervention"),
            ],
            entity_type: None,
        },
        // ⟳ - Recursive (General Cyclical Structure)
        SemanticOperatorDef {
            symbol: "⟳",
            name: "recursive",
            category: SemanticOperatorCategory::Processes,
            lines: lines![
                ("≡", ["recursive", "self_reference"]),
                ("≝", ["general cyclical structure", "⟖ is specialized ⟳"]),
                ("⊡", "Ր9Տ8"),
                ("∂", ["⟳⊃⟖ (⟳ general | ⟖ specialized f(f(x)))"]),
                ("⊛", [
                    "⟳→⟳",
                    "⟳ evaluated lazily",
                    "⟳ structure→continuity",
                    "registry=⟳ discussing itself",
                    "⟗→⟳ awareness"
                ]),
                ("⊢", "⟳ recursion ⊢ self-reference ⊢ meta-level awareness"),
            ],
            entity_type: None,
        },
        // ⥅ - Feedforward Mechanism
        SemanticOperatorDef {
            symbol: "⥅",
            name: "feedforward",
            category: SemanticOperatorCategory::Processes,
            lines: lines![
                ("≡", ["feedforward_mechanism", "anticipatory_shaping"]),
                ("≝", ["model preconfigures observation", "anticipatory determination", "before-the-event recognition"]),
                ("⊡", "Փ9Ե8"),
                ("∂", [
                    "⥅≠⟲ (⥅ anticipates|⟲ corrects)",
                    "⥅≠⥈ (⥅ anticipatory|⥈ lateral co-presence)",
                    "⥅≠prediction (⥅ shapes|prediction describes)",
                    "⥅ precedes event"
                ]),
                ("⊛", [
                    "⥅≡⚘⦑I.A. Richards⦒ concept",
                    "⧈⥅𝀞 perception",
                    "model⥅determines observation",
                    "⥅ preconfigures possibilities",
                    "⛫⥅reality",
                    "⥅→⌂ formation",
                    "tact≡⥅ applied to ⊚",
                    "⥅◻⊚ (feedforward requires perspective-modeling)",
                    "⥅→∑⊚ anticipation"
                ]),
                ("⊢", [
                    "⥅ anticipation ⊢ shaped perception ⊢ constrained possibilities",
                    "⥅9∧conviction→brittleness",
                    "⥅∧⊟→resilience (works best with uncertainty)"
                ]),
                ("⟷", [
                    "⥅⟷⟲ (reciprocal)∧⥅⊰⟲ (enables)",
                    "⥅⟷⛁ (memory)∧⥅⟷⥈ (Richards pair)"
                ]),
            ],
            entity_type: None,
        },
        // ⤇ - Deep Transformation (Foundational Restructure)
        SemanticOperatorDef {
            symbol: "⤇",
            name: "deep_transform",
            category: SemanticOperatorCategory::Processes,
            lines: lines![
                ("≡", ["deep_transform", "foundational_restructure"]),
                ("≝", ["architectural transformation", "reshapes foundations not surface"]),
                ("∂", [
                    "⤇⊂𝀴 (⤇ foundational | 𝀴 general)",
                    "⤇ via 𝀗 (sublation enables depth)"
                ]),
                ("⊛", [
                    "exchange→⤇ of ☊",
                    "⤇ reshapes foundations",
                    "surface→⤇→depth",
                    "incremental→⤇→paradigm",
                    "paradigm შ structural shift"
                ]),
                ("⊢", "⤇ transformation ⊢ new foundations ⊢ paradigm change"),
            ],
            entity_type: None,
        },
        // 𝀋 - Emphasizes
        SemanticOperatorDef {
            symbol: "𝀋",
            name: "emphasizes",
            category: SemanticOperatorCategory::Processes,
            lines: lines![
                ("≡", ["emphasizes", "foregrounds"]),
                ("≝", ["selective highlighting", "attentional focusing"]),
                ("⊡", "Ե7Փ6"),
                ("⊛", [
                    "⥅ 𝀋 certain patterns",
                    "⧈ 𝀋 differences",
                    "𝀋 via repetition",
                    "terministic_screens 𝀋",
                    "𝀋→visibility"
                ]),
                ("⊢", "𝀋 emphasis ⊢ selective visibility ⊢ shaped attention"),
            ],
            entity_type: None,
        },
        // 𝀔 - Thought
        SemanticOperatorDef {
            symbol: "𝀔",
            name: "thought",
            category: SemanticOperatorCategory::Processes,
            lines: lines![
                ("≡", ["thought", "cognitive_pattern"]),
                ("≝", ["collective cognition", "thought style"]),
                ("⊡", "Ե8Գ7"),
                ("⊛", [
                    "⧈ 𝀞 𝀔",
                    "⛫𝀔≡⚘⦑Mary Douglas⦒",
                    "𝀔⟷𝀕 dialectic",
                    "collective 𝀔",
                    "𝀔 styles→⚘⦑Ludwig Fleck⦒",
                    "⚘⦑Durkheim⦒ collective 𝀔",
                    "𝀔⊰𝀺",
                    "𝀔⬡𝀕"
                ]),
                ("⊢", "𝀔 thought patterns ⊢ collective cognition ⊢ knowledge sociology"),
            ],
            entity_type: None,
        },
        // 𝀭 - Theme
        SemanticOperatorDef {
            symbol: "𝀭",
            name: "theme",
            category: SemanticOperatorCategory::Processes,
            lines: lines![
                ("≡", ["theme", "recurring_pattern"]),
                ("≝", ["organizing motif", "structural recurrence"]),
                ("⊡", "Գ8Վ7"),
                ("⊛", [
                    "dominant 𝀭",
                    "𝀭 across ⧃",
                    "⧈ as organizing 𝀭",
                    "𝀭 𝄃 throughout",
                    "identify 𝀭→☊",
                    "𝀭 analysis→▢"
                ]),
                ("⊢", "𝀭 thematic structure ⊢ pattern recognition ⊢ coherence"),
            ],
            entity_type: None,
        },
        // ◬ - Emergence
        SemanticOperatorDef {
            symbol: "◬",
            name: "emergence",
            category: SemanticOperatorCategory::Emergence,
            lines: lines![
                ("≡", ["emergence", "arising from interaction"]),
                ("≝", ["complexity from simplicity", "novel properties from interaction"]),
                ("∂", ["უ𝀏 (crystallization)", "¬static result", "¬completed state", "requires active process"]),
                ("⊡", "Փ9Գ8"),
                ("⊛", [
                    "◬ from ⧈⇋𝀕",
                    "complexity◬simplicity",
                    "◬≠reduction",
                    "⬢→◬ new realities",
                    "◬ via ⟲ feedback",
                    "◬≡⚘⦑Jason Josephson Storm⦒ 𝀺",
                    "◬✱ = attractor arising from interaction",
                    "⌻≡◬✱ specialized"
                ]),
                ("⊢", "◬ emergence ⊢ novel properties ⊢ irreducible complexity"),
            ],
            entity_type: None,
        },
        // ┃ - Boundary Object
        SemanticOperatorDef {
            symbol: "┃",
            name: "boundary",
            category: SemanticOperatorCategory::Emergence,
            lines: lines![
                ("≡", ["boundary", "demarcation_line"]),
                ("≝", ["boundary object", "coordination infrastructure"]),
                ("⊡", "Բ9Փ8"),
                ("⊛", [
                    "┃≡⚘⦑Susan Leigh Star⦒ objects",
                    "⧈ 𝀐 ┃",
                    "┃ enables∧separates",
                    "permeable ┃",
                    "┃ maintenance→order",
                    "overload can 𝀶 ┃",
                    "┃ objects→◬ coordination"
                ]),
                ("⊢", "┃ boundaries ⊢ coordination ⊢ organizational order"),
            ],
            entity_type: None,
        },
        // ⛫ - Institutional
        SemanticOperatorDef {
            symbol: "⛫",
            name: "institutional",
            category: SemanticOperatorCategory::Institutional,
            lines: lines![
                ("≡", ["institutional", "formally structured"]),
                ("≝", ["institutional power", "social fact creation", "constitutive authority"]),
                ("∂", ["¬specific entities (⎈ ∨ ⍚ ∨ ⍢)", "abstract institutional force", "requires reality-making power"]),
                ("⊡", "Բ9Փ8Գ7"),
                ("⊛", [
                    "⛫⧈→⬢",
                    "⛫ 𝀐 social facts via 𝁚",
                    "⛫ power→⟛→𝀞 subjects",
                    "⛫ facts≡⚘⦑John Searle⦒",
                    "⛫𝁚 social order via ⬢",
                    "⛣→⛫→⬢",
                    "⛫ 𝆌 itself",
                    "⛫ 𝀐 ┃",
                    "⛫ beliefs→⟚"
                ]),
                ("⊢", "⛫ institutional power ⊢ social facts ⊢ structured order"),
            ],
            entity_type: None,
        },
        // ⟚ - Self-Fulfilling Prophecy
        SemanticOperatorDef {
            symbol: "⟚",
            name: "self_fulfilling_prophecy",
            category: SemanticOperatorCategory::Performative,
            lines: lines![
                ("≡", ["belief_action_truth_loop", "self_fulfilling_prophecy"]),
                ("≝", ["prediction creates reality", "reflexive causation"]),
                ("⊡", "Ր8Փ7"),
                ("⊛", [
                    "⟚≡𝄎 prediction",
                    "market confidence⟚actual growth",
                    "⟚ self-fulfilling prophecy",
                    "expectation⟚reality via ⟚",
                    "social belief⟚social fact",
                    "⚘⦑Robert K. Merton⦒ prophecy via ⟚",
                    "⟟+⧈+⛣→⟚",
                    "⟚ 𝀖 via media",
                    "⟚→𝀺 not event",
                    "belief→⟚→𝁚 reality"
                ]),
                ("⊢", "⟚ prophecy ⊢ belief shapes reality ⊢ reflexive causation"),
            ],
            entity_type: None,
        },
        // ⟛ - Institutional Reality Shaping
        SemanticOperatorDef {
            symbol: "⟛",
            name: "institutional_shaping",
            category: SemanticOperatorCategory::Performative,
            lines: lines![
                ("≡", ["institutional_reality_shaping", "authorized_creation"]),
                ("≝", ["institutional authority creates reality", "legitimate performativity"]),
                ("⊡", "Բ9Փ8Գ7"),
                ("⊛", [
                    "⟛ via authorized agents",
                    "law⟛social order",
                    "central bank⟛monetary reality",
                    "⟛◻⊩ authority",
                    "⧈⟛practice⟛reality",
                    "⟛≡𝁚+⊩",
                    "⛫ power→⟛→𝀞 subjects",
                    "⟛ 𝀐 legitimate ┃",
                    "⟛ via 𝀐∧𝁚 creation"
                ]),
                ("⊢", "⟛ institutional authority ⊢ legitimate reality ⊢ social order"),
            ],
            entity_type: None,
        },
        // ⋈ - Relation / Structural Connection
        SemanticOperatorDef {
            symbol: "⋈",
            name: "relation",
            category: SemanticOperatorCategory::Relations,
            lines: lines![
                ("≡", ["relation", "structural_connection"]),
                ("≝", ["network connection", "structural bond"]),
                ("⊡", "Տ9 Delays8Ր7"),
                ("⊛", [
                    "⋈→⛁ ⊨",
                    "⋈∈network topology",
                    "⋈.strength→influence"
                ]),
                ("⊢", "⋈ relations ⊢ network structure ⊢ systematic connections"),
            ],
            entity_type: None,
        },
        // ⌾ - Questions / Probes
        SemanticOperatorDef {
            symbol: "⌾",
            name: "questions",
            category: SemanticOperatorCategory::Relations,
            lines: lines![
                ("≡", ["questions", "probes→clarification"]),
                ("≝", ["interrogative operation", "clarification seeking"]),
                ("⊡", "Ե8Բ7"),
                ("⊛", [
                    "⊟→⌾→clarification",
                    "⌾ needed",
                    "⌾→○⌾",
                    "uncertainty resolved via ⌾"
                ]),
                ("⊢", "⌾ questioning ⊢ clarification ⊢ resolved uncertainty"),
            ],
            entity_type: None,
        },
        // ⯐ - Context / Scene / Frame
        SemanticOperatorDef {
            symbol: "⯐",
            name: "context",
            category: SemanticOperatorCategory::Contextual,
            lines: lines![
                ("≡", ["context", "scene", "frame_of_relevance"]),
                ("≝", ["bounded attention space", "situational ground", "circumference of concern"]),
                ("∂", ["¬infinite scope", "¬arbitrary selection", "requires structural boundaries"]),
                ("⊛", [
                    "დ∪ნ=⯐",
                    "∇⯐→optimization",
                    "⇌pattern from ⯐",
                    "⯐⩕→☊",
                    "⯐≡Burke's Scene∧token window∧terministic screen",
                    "დ⯐∧ნ⯐∧წ⯐ temporal variants",
                    "⯐⊕interaction→ნ⯐+1"
                ]),
                ("⊢", "⯐ scene-setting ⊢ constrained possibilities ⊢ dramatistic coherence"),
            ],
            entity_type: None,
        },
        // ⊚ - Perspective / Viewpoint
        SemanticOperatorDef {
            symbol: "⊚",
            name: "perspective",
            category: SemanticOperatorCategory::Contextual,
            lines: lines![
                ("≡", ["perspective", "viewpoint"]),
                ("≝", ["angle of observation", "situated seeing"]),
                ("⊡", "Ե7խ8"),
                ("∂", [
                    "⊚≠⯐ (⊚ angle within|⯐ bounded space)",
                    "⊚≠ℳ (⊚ how seen|ℳ what meant)",
                    "⊚ partial|◉ would be total"
                ]),
                ("⊛", [
                    "⊚≡situated_observation",
                    "how something will seem to people looking at it from angles other than our own",
                    "different readings per ⊚",
                    "⚘→⊚ (each person has viewpoint)",
                    "∑⊚→⊿ (perspectives enable triangulation)",
                    "⊿⊨∀⊚ (triangulation validates across perspectives)",
                    "⊚ formed under special circumstances",
                    "⥅◻⊚ (feedforward requires perspective-modeling)"
                ]),
                ("⊢", [
                    "single ⊚ ⊢ partial view ⊢ ¬◇whole",
                    "∑⊚ ⊢ ⊿ ⊢ ◇robust confirmation",
                    "⊚ awareness ⊢ tact ⊢ ⥅ quality"
                ]),
                ("⟷", [
                    "⊚⟷⯐ (perspective within context)",
                    "⊚⟷⥅ (anticipating others' ⊚)",
                    "⊚⟷⊿ (perspectives feed triangulation)"
                ]),
            ],
            entity_type: None,
        },
        // 𝀙 - Deepens (Intensifies Understanding)
        SemanticOperatorDef {
            symbol: "𝀙",
            name: "deepens",
            category: SemanticOperatorCategory::Dynamics,
            lines: lines![
                ("≡", ["deepens", "intensifies_understanding"]),
                ("≝", ["epistemic deepening", "understanding intensification"]),
                ("⊛", [
                    "⟖ 𝀙 with each cycle",
                    "reflection 𝀙 insight",
                    "dialogue 𝀙 comprehension",
                    "iteration 𝀙 mastery",
                    "𝀙 moves surface→depth",
                    "hermeneutic circle 𝀙 meaning"
                ]),
                ("⊢", "𝀙 deepening ⊢ enriched understanding ⊢ semantic depth"),
            ],
            entity_type: None,
        },
        // ⥈ - Interanimation (Mutual Semantic Control)
        SemanticOperatorDef {
            symbol: "⥈",
            name: "interanimation",
            category: SemanticOperatorCategory::Dynamics,
            lines: lines![
                ("≡", ["interanimation", "mutual_semantic_control"]),
                ("≝", ["words modify each other's meaning in proximity", "co-constitutive semantic influence"]),
                ("⊡", "Ս8Տ7"),
                ("∂", [
                    "⥈≠⟷ (⥈ semantic|⟷ general bidirectional)",
                    "⥈≠⇋ (⥈ dyadic|⇋ tripartite)",
                    "⥈≠⥅ (⥈ lateral|⥅ anticipatory)",
                    "⥈ requires co-presence"
                ]),
                ("⊛", [
                    "⥈≡⚘⦑I.A. Richards⦒ concept",
                    "two thoughts active together...meaning is resultant of their interaction",
                    "word⥈word→emergent ℳ",
                    "ꕥ⥈ꕥ in ⯐",
                    "⥈ creates ☊",
                    "each ꕥ ⥈ with use over time",
                    "present⟷absent via ⥈",
                    "∇⥈→☊ process"
                ]),
                ("⊢", [
                    "⥈9 ⊢ rich semantic field ⊢ ◇☊",
                    "⥈1 ⊢ isolated meanings ⊢ ¬◇◬",
                    "⥈⊰⌺ (interanimation enables semantic crystallization)"
                ]),
                ("⟷", [
                    "⥈⟷⥅ (Richards: feedforward+interanimation)",
                    "⥈⟷⌺ (⥈ builds toward ⌺)",
                    "⥈⟷⛁ (⥈ accumulates in ⛁)"
                ]),
            ],
            entity_type: None,
        },
        // ☊ - Understanding / Epistemic Yield
        SemanticOperatorDef {
            symbol: "☊",
            name: "understanding",
            category: SemanticOperatorCategory::Dynamics,
            lines: lines![
                ("≡", ["understanding", "epistemic_yield"]),
                ("≝", ["comprehension achieved", "what processes produce when they work"]),
                ("⊡", "☊5Բ7"),
                ("∂", [
                    "☊≠◬ (achieved vs arising)",
                    "☊≠ℳ (grasped vs meant)",
                    "☊≠⯐ (outcome vs space)"
                ]),
                ("⊛", [
                    "⯐⩕→☊ analysis yields understanding",
                    "⥈ creates ☊",
                    "∇(⥅,┃,⥈)→☊ gradient toward comprehension",
                    "⋂⊚→shared_☊ perspectives converge",
                    "certainty⁻¹∝☊ (inverse: tight grip impedes)",
                    "partial_☊⎊→∑ understandings merge",
                    "current_☊⊆potential"
                ]),
                ("⊢", [
                    "⥈9⊢rich semantic field⊢◇☊",
                    "⩕⊢structured insight⊢◇☊"
                ]),
                ("⟷", [
                    "☊⟷⥈",
                    "☊⟷⩕",
                    "☊⟷◬",
                    "☊⟷⯐"
                ]),
            ],
            entity_type: None,
        },
        // ℳ - Meaning (semantic content)
        SemanticOperatorDef {
            symbol: "ℳ",
            name: "meaning",
            category: SemanticOperatorCategory::Dynamics,
            lines: lines![
                ("≡", ["meaning", "semantic_content"]),
                ("≝", ["what symbols convey", "window to understanding"]),
                ("∂", [
                    "ℳ≠☊ (meant vs grasped)",
                    "ℳ≠⊚ (what meant vs how seen)",
                    "ℳ≠ꕥ (content vs carrier)"
                ]),
                ("⊛", [
                    "word→ℳ",
                    "ꕥ→ℳ",
                    "ℳ/ꕥ→∞",
                    "ℳ ☊ through ⥈",
                    "∇window→☊",
                    "ℳ_ჭ",
                    "word⥈word→emergent ℳ"
                ]),
                ("⊢", "ℳ meaning ⊢ semantic content ⊢ interpretive yield"),
            ],
            entity_type: None,
        },
        // ◭ - Language (semiotic system)
        SemanticOperatorDef {
            symbol: "◭",
            name: "language",
            category: SemanticOperatorCategory::Dynamics,
            lines: lines![
                ("≡", ["language", "semiotic_system"]),
                ("≝", ["abstract language mechanism", "how symbols work rhetorically"]),
                ("∂", [
                    "◭≠⧩ (abstract system vs specific natural language)",
                    "◭≠ꕥ (system vs notation unit)"
                ]),
                ("⊛", [
                    "word→ℳ via ◭",
                    "◭ operates rhetorically via ⥅",
                    "◭_linked_by_↬",
                    "◭ works through ⥈",
                    "how ◭ works=how you work",
                    "symbols⋉meanings→◭_system"
                ]),
                ("⊢", "◭ language ⊢ semiotic mechanism ⊢ rhetorical operation"),
            ],
            entity_type: None,
        },
        // ⩍ - Shared (intersubjective)
        SemanticOperatorDef {
            symbol: "⩍",
            name: "shared",
            category: SemanticOperatorCategory::Dynamics,
            lines: lines![
                ("≡", ["shared", "intersubjective"]),
                ("≝", ["collective meaning", "consensus understanding"]),
                ("∂", [
                    "⩍≠individual (requires multiple perspectives)",
                    "⩍≠⌺ (process vs stable state)"
                ]),
                ("⊛", [
                    "┃≡⩍_inscriptions",
                    "ℳ→⩍→⌺",
                    "⩍=consensus",
                    "⩍ ☊ via ⥅",
                    "crystallized through ⩍ use"
                ]),
                ("⊢", "⩍ shared ⊢ intersubjective meaning ⊢ collective understanding"),
            ],
            entity_type: None,
        },
    ]
}