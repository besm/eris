//! ERIS logical operator system

define_operator_module! {
    Logical {
        CoreLogical => "Core logical operators",
        Workflow => "ERIS workflow operators",
        Process => "Process flow operators",
        Composition => "Composition operators",
        Types => "Type and value domain operators",
    }
}

type LogDef = LogicalOperatorDef;

pub fn get_logical_operator_definitions() -> Vec<LogDef> {
    vec![
        LogDef {
            symbol: "◻",
            name: "necessity_relation",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["necessity relation", "requirement enforcement"]),
                ("≝", ["mandatory conditions", "prerequisite validation"]),
                ("⊛", ["◻ requirement statements (∀ contexts)", "◻ constraint expressions"]),
                ("⊢", "◻ constraints must be satisfied before proceeding"),
            ],
        },
        LogDef {
            symbol: "⊨",
            name: "validation_relation",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["validation relation", "evidence confirmation"]),
                ("≝", ["proof requirements", "empirical support"]),
                ("⊛", [
                    "validation assertions (evidence ⊢ claim) ∧ evidence statements",
                    "≟→⊨ validation chain establishes confidence"
                ]),
            ],
        },
        LogDef {
            symbol: "≫",
            name: "strong_preference",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["strong preference relation", "dominance ordering"]),
                ("≝", ["much-greater-than", "priority enforcement"]),
                ("∂", ["¬equality (=)", "¬weak preference (>)", "categorical dominance required"]),
                ("⊛", ["≫ assertions", "priority statements (precision≫pollution)"]),
                ("□", ["Design principle: clarity≫ambiguity", "resonance≫force"]),
            ],
        },
        LogDef {
            symbol: "○",
            name: "next_state",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["next state", "temporal successor"]),
                ("≝", ["state machine transitions", "workflow pipeline sequences"]),
                ("⊛", ["process steps (○₁≺○₂≺○₃)", "verification checklists"]),
                ("◇", "◇(○₄) eventual completion"),
            ],
        },
        LogDef {
            symbol: "⊛",
            name: "pattern_detection",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["pattern detection", "recognition operation"]),
                ("≝", ["emergent pattern identification", "recurring form crystallization"]),
                ("⊛", [
                    "usage indicators ∧ detection markers (∃ pattern instances) ∧ application contexts",
                    "Noise contains signal requiring ⊛ to extract"
                ]),
            ],
        },
        LogDef {
            symbol: "≟",
            name: "testing_relation",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["testing relation", "verification operation"]),
                ("≝", ["assertion questioning", "discrimination tests"]),
                ("∂", ["¬◻ (asserting necessity)", "¬⊢ (proving entailment)", "¬⊨ (validating evidence)"]),
                ("⊛", [
                    "≟ type discrimination ∧ ≟ validation queries",
                    "≟ discovers edge cases ∧ ≟→⊨ validation chain"
                ]),
            ],
        },
        LogDef {
            symbol: "⊟",
            name: "uncertain",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["uncertain", "needs_clarification"]),
                ("≝", ["explicit uncertainty marker", "requires resolution"]),
                ("⊛", [
                    "⊟→⌾→clarification workflow",
                    "⊟ marked explicitly ∧ carried through session",
                    "⊟→ო→resolution path"
                ]),
            ],
        },
        LogDef {
            symbol: "⊢",
            name: "entails",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["entails", "proves"]),
                ("≝", ["logical entailment", "proof validation"]),
                ("⊛", [
                    "⊢ chains (A⊢B⊢C) ∧ evidence requirements (premises ⊢ conclusion)",
                    "Consistency⊢reliability⊢system_stability"
                ]),
            ],
        },
        LogDef {
            symbol: "⊩",
            name: "policy_enforcement",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["policy enforcement", "governance rule"]),
                ("≝", ["mandatory compliance", "constraint enforcement"]),
                ("⊛", ["⊩ access control", "⊩ rate limiting", "⊩ resource constraints"]),
                ("□", "⊩ rules remain invariant across contexts"),
            ],
        },
        LogDef {
            symbol: "⊧",
            name: "embodies",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["embodies", "instantiation_relation"]),
                ("≝", ["concept instantiated in particular", "relation not operation"]),
                ("∂", [
                    "⊧≠გ (⊧ relation | გ operation)",
                    "¬mere mention",
                    "¬reference",
                    "requires actual instantiation"
                ]),
                ("⊛", ["concept ⊧ pattern", "∃ concrete instances", "⊧→გ (relation enables process)"]),
                ("⊢", "⊧→გ manifestation"),
            ],
        },
        LogDef {
            symbol: "⊰",
            name: "precedes_enabling",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["precedes enabling", "causal precedence"]),
                ("≝", ["prerequisite conditions", "enabling relationships"]),
                ("∂", ["¬≺ (mere sequential ordering)", "¬→ (logical implication)", "requires causal necessity"]),
                ("⊛", ["⊰ causal chains (A⊰B⊰C)", "dependency patterns"]),
                ("⊢", "Infrastructure⊰practice⊰outcomes"),
            ],
        },

        LogDef {
            symbol: "⟷",
            name: "bidirectional",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["bidirectional relation", "mutual transformation"]),
                ("≝", ["reciprocal causation", "symmetrical influence"]),
                ("⊛", [
                    "feedback loops (A⟷B) ∧ co-constitutive relationships ∧ mutual dependency",
                    "Present⟷absent dialectic ∧ exploration⟷implementation cycles"
                ]),
                ("⊢", "⟷→𝀗 dialectical sublation"),
            ],
        },
        LogDef {
            symbol: "⊕",
            name: "structured_combination",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["structured combination", "compositional merge"]),
                ("≝", ["integrative synthesis", "additive composition"]),
                ("⊛", ["⊕ entity combinations (⚘⊕⊙⊕⊳)", "⊕ multi-component systems"]),
                ("∂", ["⊕≠simple union", "requires structural integration"]),
                ("⊢", "⊕→𝀷 concrescence through structured combination"),
            ],
        },
        LogDef {
            symbol: "✱",
            name: "attractor",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["attractor", "stable_state"]),
                ("≝", ["state space convergence point", "dynamic stability"]),
                ("∂", ["¬mere equilibrium", "¬static state", "requires basin of attraction"]),
                ("⊛", [
                    "✱ = convergence target in state space",
                    "trajectory→✱",
                    "✱ types: fixed-point, limit cycle, strange, torus",
                    "⌺≡✱ specialized",
                    "basin(✱) = region of convergence",
                    "stability(✱) measured by perturbation resistance",
                    "✱→𝀏 crystallization into stable form"
                ]),
                ("⊢", "✱ presence ⊢ predictable long-term behavior ⊢ system stability"),
            ],
        },
        // ⩕ - Analyze
        LogDef {
            symbol: "⩕",
            name: "analyze",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["analyze", "systematic_examination"]),
                ("≝", ["decomposition for understanding", "structured investigation"]),
                ("⊡", "Ե9Բ8"),
                ("⊛", [
                    "⯐⩕→☊",
                    "⊳⩕→insights",
                    "⏣⩕→diagnosis",
                    "⧊⩕→decomposition"
                ]),
                ("⊢", "⩕ analysis ⊢ structured insight ⊢ informed action"),
            ],
        },
        // ↗ - Performance
        LogDef {
            symbol: "↗",
            name: "performance",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["performance", "optimization_vector"]),
                ("≝", ["system efficiency", "resource utilization"]),
                ("⊡", "Փ9Ց7"),
                ("⊛", [
                    "system↗",
                    "⩕→↗ analysis",
                    "latency↗",
                    "throughput↗",
                    "↗⤋ detection"
                ]),
                ("⊢", "↗ performance ⊢ efficiency gains ⊢ system health"),
            ],
        },
        // ◩ - Command
        LogDef {
            symbol: "◩",
            name: "command",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["command", "directive_operation"]),
                ("≝", ["imperative control", "execution trigger"]),
                ("⊡", "Փ9Բ7"),
                ("⊛", [
                    "◩→execution_trigger",
                    "◩∈imperative_control",
                    "user◩interface",
                    "◩→action_initiation",
                    "◩→state_transition",
                    "system◩response"
                ]),
                ("⊢", "◩ command ⊢ action initiation ⊢ state transition"),
            ],
        },
        // ⏈ - Management
        LogDef {
            symbol: "⏈",
            name: "management",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["management", "oversight_operation"]),
                ("≝", ["administrative control", "governance layer"]),
                ("⊡", "Բ8Ա7"),
                ("⊛", [
                    "⏈→governance_layer",
                    "⏈∈administrative_control",
                    "resource⏈allocation",
                    "⏈→policy_enforcement",
                    "system⏈monitoring",
                    "⏈→quality_assurance"
                ]),
                ("⊢", "⏈ management ⊢ resource governance ⊢ quality assurance"),
            ],
        },
        // ↬ - Connects
        LogDef {
            symbol: "↬",
            name: "connects",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["connects", "direct_linkage"]),
                ("≝", ["direct pathway", "node connection"]),
                ("⊛", [
                    "⩎↬⩎ via ↬",
                    "direct ↬ between nodes",
                    "API↬feed↬tool",
                    "↬ creates pathway"
                ]),
                ("⊢", "↬ connection ⊢ pathway creation ⊢ system linkage"),
            ],
        },
        // ⌇ - Translation
        LogDef {
            symbol: "⌇",
            name: "translation",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["translation", "representation_transform"]),
                ("≝", ["cross-representation mapping", "format conversion"]),
                ("⊛", [
                    "⯐⌇⯐=⯐ transform",
                    "⚘⌇⌬ notation",
                    "⌇ between representations",
                    "◭⌇ꕥ"
                ]),
                ("⊢", "⌇ translation ⊢ representation equivalence ⊢ semantic preservation"),
            ],
        },
        // ↭ - Reciprocal Flow
        LogDef {
            symbol: "↭",
            name: "reciprocal_flow",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["reciprocal_flow", "collaborative exchange"]),
                ("≝", ["bidirectional exchange", "mutual transfer"]),
                ("⊛", [
                    "↭ flow between ⊚",
                    "collaborative↭exchange",
                    "ნ⯐⊕↭→ნ+1⯐",
                    "give↭receive"
                ]),
                ("⊢", "↭ reciprocity ⊢ mutual exchange ⊢ collaborative flow"),
            ],
        },
        // ∞ - Infinity
        LogDef {
            symbol: "∞",
            name: "infinity",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["infinity", "unbounded"]),
                ("≝", ["limitless extent", "unrestricted potential"]),
                ("⊛", [
                    "ℳ/ꕥ→∞",
                    "compression→∞",
                    "semantic_field→∞",
                    "potential_relations→∞"
                ]),
                ("⊢", "∞ unboundedness ⊢ unlimited potential ⊢ open horizon"),
            ],
        },
        // ⊿ - Triangulates
        LogDef {
            symbol: "⊿",
            name: "triangulates",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["triangulates", "validates across viewpoints"]),
                ("≝", ["multi-perspective validation", "semiotic trichotomy"]),
                ("∂", ["¬single viewpoint", "requires ≥3 perspectives"]),
                ("⊛", [
                    "∑⊚→⊿⊢validation",
                    "⊿=∑⊚⊢",
                    "API⊿⚘∩⚘∩⚘",
                    "⊿ ensures ⊨",
                    "⊿≡⚘⦑C.S. Peirce⦒ triadic sign"
                ]),
                ("⊢", "⊿ triangulation ⊢ validated truth ⊢ robust confirmation"),
            ],
        },
        // ⊖ - Property Difference
        LogDef {
            symbol: "⊖",
            name: "property_difference",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["property_difference", "distance_measure"]),
                ("≝", ["compatibility assessment", "gap analysis"]),
                ("⊛", [
                    "⇀₁⊖⇀₂→distance_measure",
                    "compatibility_assessment",
                    "gap_analysis",
                    "|P₁[Վ] - P₂[Վ]| < τ"
                ]),
                ("⊢", "⊖ difference ⊢ compatibility measure ⊢ alignment gap"),
            ],
        },
        // ↓ - Property Projection
        LogDef {
            symbol: "↓",
            name: "property_projection",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["property_projection", "dimension_selection"]),
                ("≝", ["filtered properties", "focused analysis"]),
                ("⊛", [
                    "⇀↓[subset]→filtered_properties",
                    "dimension_selection",
                    "focused_analysis",
                    "select Հ ∈ {Բ,Գ,Վ,Ե}"
                ]),
                ("⊢", "↓ projection ⊢ dimension selection ⊢ focused view"),
            ],
        },
        // ⋄ - Property Compatibility
        LogDef {
            symbol: "⋄",
            name: "property_compatibility",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["property_compatibility", "similarity_score"]),
                ("≝", ["threshold comparison", "alignment verification"]),
                ("⊛", [
                    "⇀₁⋄⇀₂→similarity_score",
                    "threshold_comparison",
                    "alignment_verification",
                    "∀Հ ∈ (P₁ ∩ P₂): |P₁[Հ] - P₂[Հ]| < 0.3"
                ]),
                ("⊢", "⋄ compatibility ⊢ similarity assessment ⊢ alignment"),
            ],
        },
        // ⫍ - Metaphorize
        LogDef {
            symbol: "⫍",
            name: "metaphorize",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["metaphorize", "transform_to_metaphor"]),
                ("≝", ["abstract→concrete mapping", "figurative translation"]),
                ("⊛", [
                    "⫍→ᛝ",
                    "⧊⫍concrete metaphor",
                    "requirement⫍→ᛝ",
                    "abstract⫍tangible"
                ]),
                ("⊢", "⫍ metaphorization ⊢ concrete grounding ⊢ tangible understanding"),
            ],
        },
        LogDef {
            symbol: "∘",
            name: "compose",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["compose", "function composition"]),
                ("≝", ["sequential combination", "pipeline construction"]),
                ("⊛", ["∘(use,consolidation)→stability", "∘(validation,resolution)→confidence", "pipeline≡∘sequence", "∘→𝀾 composition as flow"]),
                ("⊢", "Associativity: ∘(f,∘(g,h)) ≡ ∘(∘(f,g),h)"),
                ("◇", "Complex pipelines built through ∘ reach eventual stability"),
            ],
        },

        LogDef {
            symbol: "≡",
            name: "equivalence",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["equivalence", "semantic identity"]),
                ("≝", ["definitional equality", "mutual substitutability"]),
                ("⊛", "A≡B allows substitution in all contexts"),
            ],
        },
        LogDef {
            symbol: "≝",
            name: "defined_as",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["defined as", "definitional relation"]),
                ("≝", ["formal definition", "specification"]),
                ("⊛", "term ≝ definition structure"),
            ],
        },
        LogDef {
            symbol: "→",
            name: "implies",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["implies", "logical implication"]),
                ("≝", ["if-then relation", "conditional consequence"]),
                ("⊛", ["A→B", "antecedent→consequent"]),
            ],
        },
        LogDef {
            symbol: "∧",
            name: "and",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["and", "conjunction"]),
                ("≝", ["logical conjunction", "both conditions"]),
                ("⊛", "A∧B requires both A and B"),
            ],
        },
        LogDef {
            symbol: "∨",
            name: "or",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["or", "disjunction"]),
                ("≝", ["logical disjunction", "either condition"]),
                ("⊛", "A∨B requires at least one of A or B"),
            ],
        },
        LogDef {
            symbol: "¬",
            name: "not",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["not", "negation"]),
                ("≝", ["logical negation", "complement"]),
                ("⊛", "¬A is true when A is false"),
            ],
        },
        LogDef {
            symbol: "∀",
            name: "for_all",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["for all", "universal quantification"]),
                ("≝", ["universal quantifier", "applies to all"]),
                ("⊛", "∀x P(x) means P holds for every x"),
            ],
        },
        LogDef {
            symbol: "∃",
            name: "there_exists",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["there exists", "existential quantification"]),
                ("≝", ["existential quantifier", "at least one"]),
                ("⊛", "∃x P(x) means P holds for some x"),
            ],
        },
        LogDef {
            symbol: "∴",
            name: "therefore",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["therefore", "conclusion marker"]),
                ("≝", ["logical conclusion", "inference indicator"]),
                ("⊛", "premises ∴ conclusion"),
            ],
        },
        LogDef {
            symbol: "⊂",
            name: "subset",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["subset", "proper subset"]),
                ("≝", ["set inclusion", "contained within"]),
                ("⊛", "A⊂B means every element of A is in B"),
            ],
        },
        LogDef {
            symbol: "⊃",
            name: "superset",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["superset", "proper superset"]),
                ("≝", ["set containment", "contains within"]),
                ("⊛", "A⊃B means every element of B is in A"),
                ("⊢", "A⊃B ↔ B⊂A"),
            ],
        },
        LogDef {
            symbol: "⊅",
            name: "not_subset",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["not subset", "exclusion"]),
                ("≝", ["set non-inclusion", "not contained"]),
                ("⊛", "A⊅B means A is not a subset of B"),
            ],
        },
        LogDef {
            symbol: "↔",
            name: "iff",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["if and only if", "biconditional"]),
                ("≝", ["logical equivalence", "mutual implication"]),
                ("⊛", "A↔B means A→B ∧ B→A"),
            ],
        },
        LogDef {
            symbol: "∈",
            name: "element_of",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["element of", "membership"]),
                ("≝", ["set membership", "belongs to"]),
                ("⊛", "x∈S means x is a member of set S"),
            ],
        },
        LogDef {
            symbol: "∉",
            name: "not_element_of",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["not element of", "non-membership"]),
                ("≝", ["set exclusion", "does not belong to"]),
                ("⊛", "x∉S means x is not a member of set S"),
                ("⊢", "x∉S ↔ ¬(x∈S)"),
            ],
        },
        LogDef {
            symbol: "←",
            name: "imports",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["imports", "sources from"]),
                ("≝", ["dependency declaration", "module inclusion"]),
                ("⊛", [
                    "← core logical ⊸ [≡ ≝ → ∧ ∨]",
                    "← entity primary ⊸ [⚘ ⌖ ⧖]",
                    "module ← dependency"
                ]),
                ("⊢", "← import ⊢ namespace availability ⊢ symbol access"),
            ],
        },
        LogDef {
            symbol: "∩",
            name: "intersection",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["intersection", "common elements"]),
                ("≝", ["set intersection", "shared members"]),
                ("⊛", [
                    "A∩B contains elements in both A and B",
                    "technical∩conceptual",
                    "⚘∩⚘∩⚘",
                    "⚘∩⌬=⩍_space"
                ]),
            ],
        },
        LogDef {
            symbol: "∪",
            name: "union",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["union", "combined elements"]),
                ("≝", ["set union", "all members"]),
                ("⊛", [
                    "A∪B contains all elements from A or B",
                    "დ∪ნ=⯐",
                    "local∪global=full_ℳ"
                ]),
            ],
        },
        LogDef {
            symbol: "⊆",
            name: "subset_or_equal",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["subset or equal", "non-strict subset"]),
                ("≝", ["set inclusion", "contained within or equal"]),
                ("⊛", [
                    "A⊆B means every element of A is in B or A=B",
                    "⩎⊆⛁_system",
                    "current_☊⊆potential",
                    "affordances⊆prescriptions",
                ]),
            ],
        },
        LogDef {
            symbol: "⋃",
            name: "big_union",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["big union", "union over collection"]),
                ("≝", ["generalized union", "union of multiple sets"]),
                ("⊛", [
                    "⋃{A₁, A₂, ..., Aₙ} = union of all sets",
                    "⋃{დ,ნ,წ}=full_temporal",
                ]),
            ],
        },
        LogDef {
            symbol: "⋂",
            name: "big_intersection",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["big intersection", "intersection over collection"]),
                ("≝", ["generalized intersection", "common to all sets"]),
                ("⊛", [
                    "⋂{A₁, A₂, ..., Aₙ} = intersection of all sets",
                    "⋂{⚘,⚘,⚘}→API_design",
                    "⋂⊚→shared_☊"
                ]),
            ],
        },
        LogDef {
            symbol: "∑",
            name: "summation",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["summation", "aggregation"]),
                ("≝", ["mathematical sum", "total accumulation"]),
                ("⊛", [
                    "∑ᵢ aᵢ = sum of all elements",
                    "∑⊚=complete_picture",
                    "∑⊟→consensus",
                    "∑⧊→⌺",
                    "∑elements→comprehensive"
                ]),
            ],
        },
        LogDef {
            symbol: "∫",
            name: "integration",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["integration", "continuous accumulation"]),
                ("≝", ["mathematical integral", "continuous sum"]),
                ("⊛", [
                    "∫f(x)dx = integral over domain",
                    "∫⧊→⌺",
                    "∫⥈→☊_process",
                    "∫use→mastery"
                ]),
            ],
        },
        LogDef {
            symbol: "≅",
            name: "congruent",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["congruent", "structurally equivalent"]),
                ("≝", ["structural equivalence", "same form"]),
                ("⊛", [
                    "A≅B means A and B have equivalent structure",
                    "API≅contract"
                ]),
            ],
        },
        LogDef {
            symbol: "≃",
            name: "isomorphism",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["isomorphism", "bidirectional equivalence"]),
                ("≝", ["structural isomorphism", "perfect correspondence"]),
                ("⊛", [
                    "A≃B means structure-preserving bijection exists",
                    "structure≃behavior",
                    "form≃function",
                    "API≃contract_isomorphism"
                ]),
            ],
        },
        LogDef {
            symbol: "⊔",
            name: "join",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["join", "least upper bound"]),
                ("≝", ["lattice join", "supremum"]),
                ("⊛", [
                    "A⊔B = smallest element greater than both A and B",
                    "local⊔global→∑",
                    "⊚⊔→synthesis"
                ]),
            ],
        },
        LogDef {
            symbol: "⎊",
            name: "merger",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["merger", "structural combination"]),
                ("≝", ["deep integration", "merged structure"]),
                ("⊛", [
                    "A⎊B creates unified whole",
                    "components⎊→⛁",
                    "⊚⎊→⩍",
                    "partial_☊⎊→∑"
                ]),
                ("∂", "⎊≠⨝_deep_integration"),
            ],
        },
        LogDef {
            symbol: "⇌",
            name: "extract",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["extract", "bidirectional derivation"]),
                ("≝", ["two-way transformation", "mutual derivation"]),
                ("⊛", [
                    "A⇌B means can derive either from the other",
                    "⇌pattern from ⯐",
                    "meaning⇌symbol",
                    "abstract⇌concrete"
                ]),
            ],
        },
        LogDef {
            symbol: "⊥",
            name: "bottom",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["bottom", "falsum"]),
                ("≝", ["logical contradiction", "always false"]),
                ("⊛", ["⊥ represents impossibility", "unreachable state"]),
            ],
        },
        LogDef {
            symbol: "∂",
            name: "boundary",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["boundary", "interface_edge"]),
                ("≝", ["scope delimiter", "separation point"]),
                ("⊛", [
                    "service∂service ∧ API∂client ∧ ∂ defines scope",
                    "security∂ perimeter ∧ ∂ enforcement points",
                    "microservice∂ isolation ∧ ┃≈∂ specialized"
                ]),
            ],
        },
        LogDef {
            symbol: "≈",
            name: "approximately_equals",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["approximately equals", "similar to"]),
                ("≝", ["soft equivalence", "close approximation"]),
                ("⊛", [
                    "revision≈refinement ∧ ☊≈navigation",
                    "notation≈compression ∧ current≈evolving",
                    "allows flexible semantic matching"
                ]),
            ],
        },
        LogDef {
            symbol: "≐",
            name: "density",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["density", "concentration"]),
                ("≝", ["semantic density", "compression measure"]),
                ("⊛", [
                    "≐⧊→⌺ crystallization",
                    "semantic≐ ∧ compression≐ ∧ symbol≐efficiency",
                    "measures information concentration"
                ]),
            ],
        },
        LogDef {
            symbol: "∅",
            name: "empty_set",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["empty set", "null absence"]),
                ("≝", ["empty state", "no elements"]),
                ("⊛", [
                    "∅≡empty_state ∧ undefined→∅",
                    "∅ℳ=no_meaning ∧ ∅⌺=unstable",
                    "logical_∅ represents absence"
                ]),
            ],
        },
        LogDef {
            symbol: "∝",
            name: "proportional_to",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["proportional to", "ratio relation"]),
                ("≝", ["proportional relationship", "scalar relationship"]),
                ("⊛", [
                    "A∝B means A varies proportionally with B",
                    "certainty⁻¹∝☊",
                    "density∝compression",
                    "use∝stability",
                    "⥈∝ℳ"
                ]),
            ],
        },
        LogDef {
            symbol: "∇",
            name: "gradient",
            category: LogicalOperatorCategory::CoreLogical,
            lines: lines![
                ("≡", ["gradient", "optimal direction"]),
                ("≝", ["direction of steepest ascent", "optimization vector"]),
                ("⊛", [
                    "∇f points toward maximum increase",
                    "∇(⥅,┃,⥈)→☊",
                    "∇⧆→resolution",
                    "∇⯐→optimization",
                    "∇process→⊙"
                ]),
            ],
        },
        LogDef {
            symbol: "⊞",
            name: "commit",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["commit", "definitive addition"]),
                ("≝", ["permanent state change", "finalized addition"]),
                ("⊡", "Փ8Լ9"),
                ("∂", ["¬tentative change", "requires finalization"]),
                ("⊛", [
                    "⊞→permanent_state",
                    "⊞∈commitment_process",
                    "⊞→finalized_addition",
                    "⊞≠tentative_change",
                    "⊞→obligation_creation",
                    "⊞∮→completion"
                ]),
            ],
        },
        // ∮ - Resolve
        LogDef {
            symbol: "∮",
            name: "resolve",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["resolve", "closure_operation"]),
                ("≝", ["finalization process", "stable outcome achievement"]),
                ("⊡", "Փ8Լ8"),
                ("⊛", [
                    "∮→completion_state",
                    "∮∈finalization_process",
                    "⊟→∮ resolution",
                    "conflict∮consensus",
                    "∮→stable_outcome",
                    "workflow∮termination",
                    "⊞→∮_commitment_closure"
                ]),
                ("⊢", "∮ resolution ⊢ stable outcome ⊢ workflow completion"),
            ],
        },
        // ⎄ - Interaction
        LogDef {
            symbol: "⎄",
            name: "interaction",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["interaction", "components engage"]),
                ("≝", ["mutual engagement", "system coupling"]),
                ("⊛", [
                    "components⎄engage",
                    "⩎⎄⩎",
                    "⚘⎄⌬ via ┃",
                    "systems⎄through⎄"
                ]),
                ("⊢", "⎄ interaction ⊢ component coupling ⊢ system behavior"),
            ],
        },
        // ⨝ - Integration (systematic merging)
        LogDef {
            symbol: "⨝",
            name: "integration",
            category: LogicalOperatorCategory::Composition,
            lines: lines![
                ("≡", ["integration", "systematic_merging"]),
                ("≝", ["emergent unification", "cross-domain synthesis"]),
                ("⊡", "Փ8Գ8 delays7"),
                ("∂", ["⨝≠⊕ simple addition", "requires coherence emergence"]),
                ("⊛", [
                    "⨝→unified_system",
                    "⨝∈synthesis_process",
                    "components⨝→coherent_whole",
                    "⨝ creates ◬",
                    "⨝→cross_domain_combination",
                    "symbols⨝meanings→◭_system",
                    "⨝ ≡ 𝀷_structural"
                ]),
                ("⊢", "⨝ integration ⊢ coherent whole ⊢ emergent properties"),
            ],
        },
        // ℤ - Integer (whole number domain)
        LogDef {
            symbol: "ℤ",
            name: "integer",
            category: LogicalOperatorCategory::Types,
            lines: lines![
                ("≡", ["integer", "whole_number"]),
                ("≝", ["discrete countable value", "whole number domain"]),
                ("⊡", "Բ9Փ8"),
                ("∂", ["ℤ≠ℝ (discrete not continuous)", "ℤ⊂numeric_types"]),
                ("⊛", [
                    "ℤ∈countable_domain",
                    "ℤ→ℝ conversion",
                    "⧈[ℤ] indexing",
                    "loop_bounds≡ℤ",
                    "ნ_ℤ values",
                    "ℤ ⊨ range⌾",
                    "ℤ≡⩎_field"
                ]),
                ("⊢", "ℤ integer ⊢ countable domain ⊢ discrete values"),
            ],
        },
        // ⊸ - Import (module inclusion)
        LogDef {
            symbol: "⊸",
            name: "import",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["import", "module_inclusion"]),
                ("≝", ["dependency declaration", "external resource inclusion"]),
                ("⊡", "Գ8Լ7"),
                ("∂", ["⊸≠← (⊸ brings in|← arrow direction)", "requires external source"]),
                ("⊛", [
                    "⊸→external_⩎",
                    "⊸ creates ↬_dependencies",
                    "namespace_resolution via ⊸",
                    "⊸→reuse",
                    "⊸→⧈ availability",
                    "dependency∧⊸",
                    "circular∧⊸",
                    "lazyო◻⊸",
                    "დწ_conflict∈⊸"
                ]),
                ("⊢", "⊸ import ⊢ namespace availability ⊢ external dependency"),
            ],
        },
        // ⊐ - Awaiting (ball elsewhere)
        LogDef {
            symbol: "⊐",
            name: "awaiting",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["awaiting", "ball_elsewhere"]),
                ("≝", ["open loop held by other", "blocked pending external action"]),
                ("⊡", "0≡stale|5≡active|9≡urgent"),
                ("∂", [
                    "⊐≠⊏ (⊐ they hold | ⊏ I hold)",
                    "⊐≠◇ (⊐ blocked | ◇ eventual)",
                    "⊐≠⊟ (⊐ agent known | ⊟ unclear)"
                ]),
                ("⊛", [
                    "⊐⚘⦑agent⦒⦑item⦒",
                    "⊐⍚⦑org⦒⦑item⦒",
                    "follow-up◻⊐",
                    "γ drain low",
                    "⍜ external"
                ]),
                ("⊢", ["⊐ ⊢ action unavailable ⊢ ◇follow-up"]),
                ("⟷", [
                    "⊐⟷ς (⊐ creates coordination dependency)",
                    "⊐→⊏ when responsibility transfers"
                ]),
            ],
        },
        // ⊏ - Owing (ball here)
        LogDef {
            symbol: "⊏",
            name: "owing",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["owing", "ball_here"]),
                ("≝", ["open loop held by self", "blocked pending my action"]),
                ("⊡", "0≡stale|5≡active|9≡urgent"),
                ("∂", ["⊏≠⊐ (⊏ I hold | ⊐ they hold)"]),
                ("⊛", [
                    "⊏⚘⦑agent⦒⦑item⦒",
                    "⊏⦑item⦒",
                    "action available",
                    "guilt accumulates",
                    "γ drain high",
                    "⍜ internal"
                ]),
                ("⊢", [
                    "⊏ ⊢ action available ⊢ ◇∮",
                    "⊏ accumulation ⊢ γ⤋"
                ]),
                ("⟷", [
                    "⊏⟷γ (⊏ count drains capacity)",
                    "⊏→∮ via action",
                    "⊏→⊐ via delegation"
                ]),
            ],
        },
        // ⏸ - Dormant (intentionally paused)
        LogDef {
            symbol: "⏸",
            name: "dormant",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["dormant", "intentionally_paused"]),
                ("≝", ["open loop suspended by choice", "will reactivate"]),
                ("∂", [
                    "⏸≠⊏ (⏸ paused | ⊏ actionable)",
                    "⏸≠⊐ (⏸ by choice | ⊐ by external)"
                ]),
                ("⊛", [
                    "⏸⦑item⦒",
                    "cork without abandonment",
                    "γ drain near zero"
                ]),
                ("⟷", "⏸→⊏ via reactivation"),
            ],
        },
        // ⊬ - Orphaned (holder unclear)
        LogDef {
            symbol: "⊬",
            name: "orphaned",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["orphaned", "holder_unclear"]),
                ("≝", ["open loop fallen through", "neither party owns"]),
                ("⊛", [
                    "⊬⦑item⦒",
                    "requires triage",
                    "often surfaces as surprise"
                ]),
                ("⟷", ["⊬⟷⊟ (orphaned loops need clarification)"]),
            ],
        },
        // ⊲ - Subcommand (subordinate structure)
        LogDef {
            symbol: "⊲",
            name: "subcommand",
            category: LogicalOperatorCategory::Process,
            lines: lines![
                ("≡", ["subcommand", "subordinate_command"]),
                ("≝", ["child command within root", "nested executable action"]),
                ("∂", [
                    "⊲≠◩ (⊲ child | ◩ root)",
                    "⊲⊂◩ (subcommand contained in command)"
                ]),
                ("⊛", [
                    "◩ root⊲child hierarchy",
                    "⊲ books — Query books",
                    "⊲ completions ◻--shell — Generate completions",
                    "CLI: ◩→⊲→⊲ nesting"
                ]),
                ("⊢", "⊲ subcommand ⊢ hierarchical CLI structure ⊢ action subdivision"),
            ],
        },
        // ⊝ - Optional (omittable element)
        LogDef {
            symbol: "⊝",
            name: "optional",
            category: LogicalOperatorCategory::Workflow,
            lines: lines![
                ("≡", ["optional", "omittable"]),
                ("≝", ["may be absent", "not required for validity"]),
                ("∂", [
                    "⊝≠◻ (⊝ optional | ◻ required)",
                    "⊝ absence valid"
                ]),
                ("⊛", [
                    "⊝--flag optional argument",
                    "⊝ parameter may omit",
                    "◻ required ∧ ⊝ optional spectrum",
                    "CLI: ⊲ cmd ⊝--verbose ◻--input"
                ]),
                ("⊢", "⊝ optional ⊢ valid without ⊢ graceful absence"),
            ],
        },
    ]
}
