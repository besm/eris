//! ≡ ERIS χρόνος operator system
//! ≝ Operators for time, purpose, and teleological assessment
//!
//! Cross-System Relations:
//!   ω∘μ→complete temporal assessment
//!   ω ⊂ ⍜
//!   ι,α,ε ∈ ω
//!   θ,δ,π,ς ∈ μ
//!   λ,ϕ,γ ∉ω ∧ ∉μ (diagnostics)
//!
//!   ι⊰α⊰ε (precedence chain)
//!   γ⊰ε⊰ϕ (capacity enables execution enables flow)
//!   λ⊰θ (consistency enables horizon extension)
//!
//!   δ9∧ς9∧π9→brittleness
//!   δ9∧ς9→γ⤋
//!   γ1→𝁆
//!   λ1→Ց9
//!   ϕ1→𝁆
//!   ϕ9→𝀷
//!
//!   ε⊧𝀾, λ⊧𝀏, α→✱
//!   λ⁻¹∝Ց, δ⁻¹∝ε, δ⁻¹∝ϕ

use crate::entities::types::EntityTypeDef;

define_operator_module! {
    Chronos {
        Teleological => "Teleological and purpose operators",
        Temporal => "Time and duration operators",
        Causal => "Causal-temporal relationships",
        Assessment => "System diagnostic operators",
        Becoming => "Process of becoming operators",
    },
    extra_fields: {
        /// Optional: if this operator also functions as an entity type
        entity_type: Option<EntityTypeDef>
    }
}

/// Get all χρόνος operator definitions
pub fn get_chronos_operator_definitions() -> Vec<ChronosOperatorDef> {
    vec![
        // ⍜ - Teleological Anchor / Purpose Entity (Dimension Marker)
        ChronosOperatorDef {
            symbol: "⍜",
            name: "teleological_anchor",
            category: ChronosOperatorCategory::Teleological,
            lines: lines![
                ("≡", ["teleological_anchor", "purpose_dimension"]),
                ("≝", ["goal-oriented force", "why behind when"]),
                ("⊡", "0≡incidental|5≡directional|9≡teleological"),
                ("∂", [
                    "⍜≠Փ (capability უ direction)",
                    "⍜≠δ (density უ purpose)",
                    "⍜≠Ը (⍜ measures|Ը describes)"
                ]),
                ("⊛", [
                    "⍜ binds temporal→goal hierarchy",
                    "ω ⊂ ⍜",
                    "⍜ exerts scheduling pressure",
                    "⍜1→drift|⍜9→telos"
                ]),
                ("⊢", [
                    "⍜9 ⊢ structured allocation ⊢ ◇goal",
                    "⍜1 ⊢ reactive motion ⊢ ¬◇goal"
                ]),
                ("⟷", ["⍜⟷ε", "⍜⟷θ"]),
            ],
            entity_type: Some(EntityTypeDef {
                symbol: "⍜".to_string(),
                name: "Purpose".to_string(),
                description: "Goal, objective, teleological anchor, intentional target".to_string(),
                sort_order: 100, // Place after standard entities but before user-defined (200+)
            }),
        },

        // ω - Teleological Vector (measurement container)
        ChronosOperatorDef {
            symbol: "ω",
            name: "teleological_vector",
            category: ChronosOperatorCategory::Teleological,
            lines: lines![
                ("≡", ["teleological_vector", "purpose_measurement"]),
                ("≝", ["ω(ιαε) composite", "⍜ quantification"]),
                ("∂", [
                    "ω≠μ (purpose≠physics)",
                    "ω measures why|μ measures how",
                    "ω≠⊡"
                ]),
                ("⊛", [
                    "ω ⊂ ⍜",
                    "ι,α,ε ∈ ω",
                    "Notation: Entity→ω(ι9α8ε7)",
                    "ω∘μ→complete temporal assessment"
                ]),
                ("⊢", [
                    "ω(ι9α9ε9) ⊢ maximal purposive 𝀾",
                    "ω(ι1α1ε1) ⊢ purposeless drift"
                ]),
                ("⟷", ["ω9⟷ϕ9", "ω1⟷Ց9"]),
            ],
            entity_type: None,
        },

        // ι - Intentionality (Teleological Vector Component)
        ChronosOperatorDef {
            symbol: "ι",
            name: "intentionality",
            category: ChronosOperatorCategory::Teleological,
            lines: lines![
                ("≡", ["intentionality", "goal_orientation_strength"]),
                ("≝", ["deliberateness measure", "ω component"]),
                ("⊡", "0≡incidental|5≡purposeful|9≡strategic"),
                ("∂", [
                    "ι≠Ը (ι measures|Ը names)",
                    "ι≠⍜ (ι∈ω⊂⍜)",
                    "ι≠α (orientation≠alignment)"
                ]),
                ("⊛", [
                    "ι ∈ ω(ιαε)",
                    "ι1→happenstance∧reactive",
                    "ι9→deliberate∧goal-driven",
                    "ι→𝀐",
                    "ι⊰α",
                    "⟡∈ι (ideas embody intentionality)"
                ]),
                ("⊢", [
                    "ι9 ⊢ ◇α9",
                    "ι1 ⊢ α≤ι"
                ]),
                ("⟷", ["ι⟷Ը", "ι⟷⟟"]),
            ],
            entity_type: None,
        },

        // α - Alignment (Teleological Vector Component)
        ChronosOperatorDef {
            symbol: "α",
            name: "alignment",
            category: ChronosOperatorCategory::Teleological,
            lines: lines![
                ("≡", ["alignment", "path_criticality"]),
                ("≝", ["goal-path coherence", "ω component"]),
                ("⊡", "0≡misaligned|5≡contributing|9≡critical_path"),
                ("∂", [
                    "α≠ι (alignment≠intention)",
                    "α≠ε (aligned≠executed)",
                    "α≠θ (path≠horizon)"
                ]),
                ("⊛", [
                    "α ∈ ω(ιαε)",
                    "α1→tangential∧divergent",
                    "α9→core_path∧essential",
                    "α measures fit→✱",
                    "α1→𝀸"
                ]),
                ("⊢", [
                    "α9 ⊢ minimal waste ⊢ ◇goal",
                    "α1∧ι9 ⊢ frustrated purpose ⊢ ⧆",
                    "ι⊰α⊰ε"
                ]),
                ("⟷", ["α⟷⯐", "α⟷✱"]),
            ],
            entity_type: None,
        },

        // ε - Execution (Teleological Vector Component)
        ChronosOperatorDef {
            symbol: "ε",
            name: "execution",
            category: ChronosOperatorCategory::Teleological,
            lines: lines![
                ("≡", ["execution", "implementation_flow"]),
                ("≝", ["action integration", "ω component"]),
                ("⊡", "0≡fragmented|5≡coordinated|9≡integrated_flow"),
                ("∂", [
                    "ε≠𝀾 (ε measures|𝀾 names)",
                    "ε≠ϕ (ε∈ω purposive|ϕ diagnostic)",
                    "ε≠α (doing≠directing)"
                ]),
                ("⊛", [
                    "ε ∈ ω(ιαε)",
                    "ε1→scattered∧interrupted",
                    "ε9→smooth∧continuous",
                    "ε⊧𝀾",
                    "ε◻γ"
                ]),
                ("⊢", [
                    "ε9 ⊢ 𝀾 ⊢ ◇completion",
                    "ε1 ⊢ 𝀸 vulnerability ⊢ ¬◇completion",
                    "ι∧α∧¬ε ⊢ paralysis"
                ]),
                ("⟷", [
                    "ε⟷γ (ε depletes γ)",
                    "ε⟷ς (ε◻ς when ς9)",
                    "ε⟷𝀾"
                ]),
            ],
            entity_type: None,
        },

        // μ - Physics Vector (temporal measurement container)
        ChronosOperatorDef {
            symbol: "μ",
            name: "physics_vector",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["physics_vector", "temporal_structure_measurement"]),
                ("≝", ["μ(θδπς) composite", "time-shape quantification"]),
                ("∂", [
                    "μ≠ω (physics≠teleology)",
                    "μ measures how structured|ω measures why used",
                    "μ≠Լ"
                ]),
                ("⊛", [
                    "θ,δ,π,ς ∈ μ",
                    "Notation: Entity→μ(θ7δ3π8ς2)",
                    "μ∘ω→complete temporal assessment",
                    "μ describes container|ω describes content"
                ]),
                ("⊢", [
                    "μ(θ9δ9π9ς9) ⊢ maximal constraint ⊢ brittleness",
                    "μ(θ1δ1π1ς1) ⊢ minimal structure ⊢ drift",
                    "optimal μ ⊢ ⧆(structure∧flexibility)"
                ]),
                ("⟷", ["μ⟷⯐", "δ9∧ς9→γ⤋"]),
            ],
            entity_type: None,
        },

        // θ - Horizon (Physics Vector Component)
        ChronosOperatorDef {
            symbol: "θ",
            name: "horizon",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["horizon", "planning_distance"]),
                ("≝", ["temporal scope", "μ component"]),
                ("⊡", "0≡immediate(hours)|5≡medium(weeks)|9≡extended(months+)"),
                ("∂", [
                    "θ≠Լ (planning≠duration)",
                    "θ≠⍜ (distance≠purpose)",
                    "θ≠წ (θ⊃წ but measures reach)"
                ]),
                ("⊛", [
                    "θ ∈ μ(θδπς)",
                    "θ1→reactive∧short-term",
                    "θ9→strategic∧long-range",
                    "θ sets temporal ⯐ ∂",
                    "θ⊰⟟",
                    "◈∈θ (projects have planning horizons)"
                ]),
                ("⊢", [
                    "θ9 ⊢ ◇strategic positioning",
                    "θ1 ⊢ reactive უ strategy",
                    "θ9∧ι1 ⊢ vision უ will"
                ]),
                ("⟷", ["θ⟷⍜", "θ⟷⟟", "θ1→Ց9"]),
            ],
            entity_type: None,
        },

        // δ - Density (Physics Vector Component)
        ChronosOperatorDef {
            symbol: "δ",
            name: "density",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["density", "commitment_volume"]),
                ("≝", ["time allocation intensity", "μ component"]),
                ("⊡", "0≡sparse|5≡moderate|9≡saturated"),
                ("∂", [
                    "δ≠Վ (temporal≠semantic)",
                    "δ≠≐ (commitment≠compression)",
                    "δ measures packing|Վ measures meaning-per-symbol"
                ]),
                ("⊛", [
                    "δ ∈ μ(θδπς)",
                    "δ1→flexible∧slack",
                    "δ9→packed∧constrained",
                    "δ9→γ⤋",
                    "δ⁻¹∝ε"
                ]),
                ("⊢", [
                    "δ9 ⊢ ¬◇flexibility ⊢ 𝀸 vulnerability",
                    "δ1 ⊢ slack ⊢ ◇absorption",
                    "δ9∧ς9 ⊢ maximal constraint ⊢ brittleness"
                ]),
                ("⟷", [
                    "δ⟷γ (δ9→γ⤋)",
                    "δ⟷ε (δ⁻¹∝ε)",
                    "δ9∧𝀸→Ց9"
                ]),
            ],
            entity_type: None,
        },

        // π - Precision (Physics Vector Component)
        ChronosOperatorDef {
            symbol: "π",
            name: "precision",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["precision", "timing_rigidity"]),
                ("≝", ["scheduling granularity", "μ component"]),
                ("⊡", "0≡loose|5≡structured|9≡rigid"),
                ("∂", [
                    "π≠ς (precision უ sync)",
                    "π≠α (temporal≠goal)",
                    "π measures tolerance|ς measures coupling"
                ]),
                ("⊛", [
                    "π ∈ μ(θδπς)",
                    "π1→fluid∧adaptable",
                    "π9→fixed∧strict",
                    "π9→𝀸 vulnerability",
                    "π constrains ε"
                ]),
                ("⊢", [
                    "π9 ⊢ exact coordination ⊢ brittleness",
                    "π1 ⊢ temporal slack ⊢ ◇absorption",
                    "π9∧ς9 ⊢ maximal rigidity ⊢ cascade failure risk"
                ]),
                ("⟷", [
                    "π⟷ε",
                    "π⟷𝀸 (π9→𝀸 impact amplified)",
                    "π⟷λ"
                ]),
            ],
            entity_type: None,
        },

        // ς - Sync (Physics Vector Component)
        ChronosOperatorDef {
            symbol: "ς",
            name: "sync",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["sync", "coordination_dependency"]),
                ("≝", ["synchronization requirements", "μ component"]),
                ("⊡", "0≡independent|5≡coordinated|9≡tightly_coupled"),
                ("∂", [
                    "ς≠π (sync≠precision)",
                    "ς≠⋈ (temporal≠structural)",
                    "ς measures coupling|π measures tolerance"
                ]),
                ("⊛", [
                    "ς ∈ μ(θδπς)",
                    "ς1→autonomous∧decoupled",
                    "ς9→interdependent∧synchronized",
                    "ς9→bottleneck vulnerability",
                    "ς→⋈ temporal",
                    "ς9→ε◻others"
                ]),
                ("⊢", [
                    "ς9 ⊢ coordination overhead ⊢ cascade risk",
                    "ς1 ⊢ autonomy ⊢ უ coordination benefit",
                    "ς9∧π9∧δ9 ⊢ system fragility"
                ]),
                ("⟷", ["ς⟷⋈", "ς⟷┃", "ς⟷ε"]),
            ],
            entity_type: None,
        },

        // λ - Consistency (Diagnostic Vector)
        ChronosOperatorDef {
            symbol: "λ",
            name: "consistency",
            category: ChronosOperatorCategory::Assessment,
            lines: lines![
                ("≡", ["consistency", "temporal_pattern_stability"]),
                ("≝", ["behavioral regularity", "diagnostic"]),
                ("⊡", "0≡erratic|5≡patterned|9≡invariant"),
                ("∂", [
                    "λ≠թ (temporal≠logical)",
                    "λ≠ delays (behavioral≠symbolic)",
                    "λ measures pattern∘time|թ measures internal logic"
                ]),
                ("⊛", [
                    "λ diagnostic (λ∉ω ∧ λ∉μ)",
                    "λ1→erratic∧unpredictable",
                    "λ9→stable∧reliable",
                    "λ⊧𝀏",
                    "λ9→⌺",
                    "λ1→Ց9",
                    "⋯∈λ1 (questions indicate inconsistency)"
                ]),
                ("⊢", [
                    "λ9 ⊢ ⟟ reliable ⊢ ◇planning",
                    "λ1 ⊢ ¬⟟ ⊢ reactive უ planning",
                    "λ⊰θ extension"
                ]),
                ("⟷", ["λ⟷𝀏", "λ⁻¹∝Ց", "λ⟷⌺", "λ⟷θ"]),
            ],
            entity_type: None,
        },

        // ϕ - Flux (Diagnostic Vector)
        ChronosOperatorDef {
            symbol: "ϕ",
            name: "flux",
            category: ChronosOperatorCategory::Assessment,
            lines: lines![
                ("≡", ["flux", "flow_state_measure"]),
                ("≝", ["dynamic fluidity", "diagnostic"]),
                ("⊡", "0≡blocked|5≡moving|9≡flowing"),
                ("∂", [
                    "ϕ≠𝀾 (ϕ measures|𝀾 names)",
                    "ϕ≠ε (general≠purposive)",
                    "ϕ diagnostic|ε teleological"
                ]),
                ("⊛", [
                    "ϕ diagnostic (ϕ∉ω ∧ ϕ∉μ)",
                    "ϕ1→blocked∧stagnant",
                    "ϕ9→flowing∧dynamic",
                    "ϕ measures 𝀾",
                    "ϕ1→𝁆 risk",
                    "ϕ9→𝀷 active"
                ]),
                ("⊢", [
                    "ϕ9 ⊢ momentum ⊢ 𝀾",
                    "ϕ1 ⊢ stagnation ⊢ 𝁆 risk",
                    "ϕ◻γ"
                ]),
                ("⟷", ["ϕ⟷𝀾", "ϕ⟷γ", "ϕ⟷δ (δ⁻¹∝ϕ)", "ϕ⟷ε"]),
            ],
            entity_type: None,
        },

        // γ - Capacity (Diagnostic Vector)
        ChronosOperatorDef {
            symbol: "γ",
            name: "capacity",
            category: ChronosOperatorCategory::Assessment,
            lines: lines![
                ("≡", ["capacity", "energy_reserve"]),
                ("≝", ["resource availability", "diagnostic"]),
                ("⊡", "0≡depleted|5≡adequate|9≡abundant"),
                ("∂", [
                    "γ≠Լ (energy≠duration)",
                    "γ≠ε (having≠using)",
                    "γ≠ϕ (reserve≠flow)"
                ]),
                ("⊛", [
                    "γ diagnostic (γ∉ω ∧ γ∉μ)",
                    "γ1→depleted∧exhausted",
                    "γ9→energized∧capable",
                    "γ⤋→𝁆",
                    "γ constrains ε ceiling",
                    "γ⊰ϕ",
                    "δ9∧ς9→γ⤋"
                ]),
                ("⊢", [
                    "γ9 ⊢ sustained performance ⊢ resilience",
                    "γ1 ⊢ ¬◇ε ⊢ 𝁆 imminent",
                    "γ⊰ε⊰ϕ"
                ]),
                ("⟷", [
                    "γ⟷ε (γ bounds ε)",
                    "γ⟷ϕ (γ⊰ϕ)",
                    "γ⟷δ (δ→γ⤋)",
                    "γ⟷𝁆",
                    "γ⟷Լ"
                ]),
            ],
            entity_type: None,
        },

        // ⊱ - Follows Resulting (Causal-Temporal)
        ChronosOperatorDef {
            symbol: "⊱",
            name: "follows_resulting",
            category: ChronosOperatorCategory::Causal,
            lines: lines![
                ("≡", ["follows_resulting", "emergent_causation"]),
                ("≝", ["consequential temporal flow", "resultant emergence through time"]),
                ("⊛", [
                    "action⊱reality",
                    "⟟⊱confirmation",
                    "⊱ emerges_from",
                    "𝀕⊱⧈",
                    "𝄃⊱⌂",
                    "𝀺⊱◬"
                ]),
                ("∂", ["⊱≠≻ resultant_not_sequential", "¬mere temporal succession", "requires causal emergence"]),
                ("⊢", "Causal action ⊢ ⊱ resulting state ⊢ temporal emergence"),
            ],
            entity_type: None,
        },

        // □ - Always (Modal-Temporal)
        ChronosOperatorDef {
            symbol: "□",
            name: "always",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["always", "invariant condition"]),
                ("≝", ["necessary truth", "universal constraint"]),
                ("∂", ["¬◇ (eventual/possible)", "¬◻ (contextual necessity)", "absolute invariance"]),
                ("⊛", ["□ mandatory requirements", "□ immutable properties", "□ validation assertions"]),
                ("⊢", "□ properties must hold in all states"),
            ],
            entity_type: None,
        },

        // ◇ - Eventually (Modal-Temporal)
        ChronosOperatorDef {
            symbol: "◇",
            name: "eventually",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["eventually", "future possibility"]),
                ("≝", ["temporal achievement", "goal realization"]),
                ("⊛", ["◇ workflow completion", "◇ validation goals", "◇ eventual consistency"]),
                ("∂", ["¬□ (immediate/always)", "temporal progression required"]),
            ],
            entity_type: None,
        },

        // ≺ - Precedes (Temporal Ordering)
        ChronosOperatorDef {
            symbol: "≺",
            name: "precedes",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["precedes", "ordered before"]),
                ("≝", ["sequential ordering", "temporal precedence"]),
                ("∂", ["¬⊰ (causal enabling)", "¬→ (logical implication)", "simple ordering only"]),
                ("⊛", ["workflow stages (○₁≺○₂≺○₃)", "≺ chains", "process dependencies"]),
            ],
            entity_type: None,
        },

        // ≻ - Succeeds (Temporal Ordering)
        ChronosOperatorDef {
            symbol: "≻",
            name: "succeeds",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["succeeds", "ordered after"]),
                ("≝", ["sequential succession", "temporal following"]),
                ("∂", ["¬(A≺B) ≡ (B≻A)", "inverse of ≺"]),
                ("⊛", [
                    "deployment≻testing ∧ parent≻child lifecycle ∧ future≻present",
                    "lifecycle stages where parent≻child necessarily holds"
                ]),
                ("⊢", "A≺B ⊢ B≻A ⊢ temporal ordering consistency"),
            ],
            entity_type: None,
        },

        // ⟟ - Prediction (Anticipatory Projection)
        ChronosOperatorDef {
            symbol: "⟟",
            name: "prediction",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["prediction", "anticipatory_projection"]),
                ("≝", ["future state modeling", "expectation formation"]),
                ("⊡", "Փ8Ե7"),
                ("⊛", [
                    "⟟→future_state",
                    "model⟟outcome",
                    "⟟⥅𝀞_perception",
                    "expectation⟟𝀕⟟result",
                    "⟟→⟚ when 𝄎",
                    "⟟→⛣ performative",
                    "⟟ 𝀐 what it predicts"
                ]),
                ("⊢", "⟟⊱confirmation (prediction results in confirmation)"),
            ],
            entity_type: None,
        },

        // 𝄃 - Repeats (Iterative Pattern)
        ChronosOperatorDef {
            symbol: "𝄃",
            name: "repeats",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["repeats", "iterates_pattern"]),
                ("≝", ["temporal iteration", "pattern recurrence"]),
                ("⊡", "Ր7Փ6"),
                ("⊛", [
                    "performativity via 𝄃",
                    "𝄃 𝆌 ⧈",
                    "ritual 𝄃",
                    "𝄃→sedimentation",
                    "⌂ formed by 𝄃",
                    "𝄃→⚘Judith Butler_gender"
                ]),
                ("⊢", "𝄃⊱⌂ (repetition results in infrastructure)"),
            ],
            entity_type: None,
        },

        // ⬡ - Simultaneous (Concurrent)
        ChronosOperatorDef {
            symbol: "⬡",
            name: "simultaneous",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["simultaneous", "concurrent"]),
                ("≝", ["temporal co-occurrence", "synchronic relation"]),
                ("⊛", [
                    "⧈⬡reality",
                    "naming⬡existence",
                    "⬡ co_constitutive",
                    "model⬡world",
                    "observer⬡observed⬡",
                    "𝀔⬡𝀕",
                    "⛫⬡subjects"
                ]),
                ("∂", ["¬sequential", "¬causal priority", "mutual co-presence"]),
            ],
            entity_type: None,
        },

        // ⏣ - State
        ChronosOperatorDef {
            symbol: "⏣",
            name: "state",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["state", "process_state"]),
                ("≝", ["workflow position", "current condition"]),
                ("⊡", "Ր9Լ8խ6"),
                ("⊛", [
                    "ნ⏣∈∘workflow_chain",
                    "○(⏣)→○⏣",
                    "⏣=position",
                    "track ⏣ through session",
                    "⑀⏣ (meta markers track state)"
                ]),
                ("⊢", "⏣ state ⊢ position tracking ⊢ workflow progression"),
            ],
            entity_type: None,
        },

        // ⟲ - Feedback Loop (Mechanism/Operator)
        ChronosOperatorDef {
            symbol: "⟲",
            name: "feedback_loop",
            category: ChronosOperatorCategory::Causal,
            lines: lines![
                ("≡", ["feedback_loop", "circular_causation"]),
                ("≝", ["output→input mechanism", "general self-reinforcement"]),
                ("⊡", "Ր9Տ8"),
                ("∂", [
                    "⟲⊅⟗ (⟲ mechanism | ⟗ named instance)",
                    "⟲ operator | ⟗ entity-taggable"
                ]),
                ("⊛", [
                    "⟟→outcome⟲⟟",
                    "⧈→𝀕⟲⧈",
                    "⟲ amplifies initial_conditions",
                    "market⟲sentiment⟲price",
                    "⟲ 𝀖 𝀺→◬",
                    "⟲ can 𝀶 or 𝆌"
                ]),
                ("⊢", "⟲ ⊢ non-linear dynamics ⊢ emergent behavior"),
            ],
            entity_type: None,
        },

        // ⟖ - Recursive Process (Specialized f(f(x)))
        ChronosOperatorDef {
            symbol: "⟖",
            name: "recursive_process",
            category: ChronosOperatorCategory::Causal,
            lines: lines![
                ("≡", ["recursive_process", "function_on_output"]),
                ("≝", ["f(f(x)) structure", "complexity generation"]),
                ("⊡", "Ր9Տ8"),
                ("∂", ["⟖⊂⟳ (⟖ specialized | ⟳ general)"]),
                ("⊛", [
                    "definition⟖self_reference⟖definition",
                    "⟖ generates_complexity",
                    "language⟖metalanguage⟖",
                    "consciousness⟖self_awareness⟖",
                    "⟖→𝄎 structures",
                    "⟖ 𝀙 with each cycle"
                ]),
                ("⊢", "⟖ ⊢ hierarchical complexity ⊢ self-organization"),
            ],
            entity_type: None,
        },

        // 𝄎 - Reflexive (Self-Referential)
        ChronosOperatorDef {
            symbol: "𝄎",
            name: "reflexive",
            category: ChronosOperatorCategory::Causal,
            lines: lines![
                ("≡", ["reflexive", "self_referential"]),
                ("≝", ["self-aware structure", "meta-level observation"]),
                ("⊡", "Ր8Տ7"),
                ("⊛", [
                    "𝄎 loops→⟗",
                    "𝄎 prediction→⟚",
                    "𝄎 awareness 𝀴",
                    "𝄎 𝀺→complexity",
                    "sociology≡𝄎"
                ]),
                ("⊢", "𝄎 ⊢ second-order observation ⊢ systems theory"),
            ],
            entity_type: None,
        },

        // 𝀺 - Process (Ongoing Activity)
        ChronosOperatorDef {
            symbol: "𝀺",
            name: "process",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["process", "ongoing_activity"]),
                ("≝", ["temporal unfolding", "dynamic becoming"]),
                ("⊡", "Փ9Գ8"),
                ("⊛", [
                    "reality≡𝀺 not thing",
                    "⧈ as 𝀺",
                    "𝀺 ontology→⚘Heraclitus",
                    "𝀺≫substance",
                    "social≡𝀺",
                    "𝀺→⚘Whitehead_philosophy",
                    "𝀺⊱◬"
                ]),
                ("⊢", "𝀺 ⊢ becoming over being ⊢ process metaphysics"),
            ],
            entity_type: None,
        },

        // 𝀃 - Prehension (πέτασθε)
        ChronosOperatorDef {
            symbol: "𝀃",
            name: "prehension",
            category: ChronosOperatorCategory::Becoming,
            lines: lines![
                ("≡", ["prehension", "feeling forth", "vectorial grasping"]),
                ("≝", ["spreading outward", "selective reaching", "pre-conscious feeling", "precedes 𝀷"]),
                ("∂", [
                    "¬perception",
                    "¬cognition",
                    "¬single direction",
                    "requires multiplicity of reach"
                ]),
                ("⊛", [
                    "occasion 𝀃 its world",
                    "positive 𝀃 (relevant) ∨ negative 𝀃 (excluded)",
                    "both 𝀃 modes→𝀗→occasion unity",
                    "vectorial: world→𝀃→occasion",
                    "unfurling before 𝀷 pressing",
                    "still airborne, not yet gathered",
                    "𝀃→𝀷→satisfaction",
                    "many directions at once",
                    "𝀷 integrates what 𝀃 gathers",
                    "πέταμαι: fly, spread out",
                    "∀ occasions 𝀃"
                ]),
                ("⊢", "data gathered ⊢ world felt ⊢ 𝀷 enabled"),
            ],
            entity_type: None,
        },

        // 𝀷 - Concrescence (πίασμα)
        ChronosOperatorDef {
            symbol: "𝀷",
            name: "concrescence",
            category: ChronosOperatorCategory::Becoming,
            lines: lines![
                ("≡", ["concrescence", "pressing into unity"]),
                ("≝", ["many→one integration", "felt pressure of becoming", "𝀃 integration"]),
                ("∂", [
                    "¬violence",
                    "¬completed satisfaction",
                    "¬mere collection",
                    "requires weighted relevance"
                ]),
                ("⊛", [
                    "𝀃→𝀷→satisfaction",
                    "many 𝀷 one",
                    "universe leaning on itself",
                    "weighted relevance gathering",
                    "𝀗 lifts contradictions into 𝀷 unity",
                    "⌻ via 𝀷",
                    "𝀷 precedes 𝀏",
                    "∀ actual occasions through 𝀷",
                    "πιάζω: grasp, press, seize"
                ]),
                ("⊢", "unified occasion ⊢ becoming complete"),
            ],
            entity_type: None,
        },

        // 𝆌 - Reinforces (Feedback Reinforcement)
        ChronosOperatorDef {
            symbol: "𝆌",
            name: "reinforces",
            category: ChronosOperatorCategory::Causal,
            lines: lines![
                ("≡", ["reinforces", "strengthens_pattern"]),
                ("≝", ["feedback reinforcement", "pattern strengthening through repetition"]),
                ("⊡", "Փ8Տ7"),
                ("⊛", [
                    "𝄃 𝆌 ⧈",
                    "⧈→𝀕→𝆌→⧈",
                    "usage patterns 𝆌 ⌺",
                    "⛫ 𝆌 itself",
                    "⟲ can 𝀶 or 𝆌",
                    "𝆌 stabilizes patterns",
                    "behavior→𝆌→⌂"
                ]),
                ("⊢", "𝆌 reinforcement ⊢ pattern stability ⊢ habitus formation"),
            ],
            entity_type: None,
        },

        // 𝀖 - Accelerates (Temporal Acceleration)
        ChronosOperatorDef {
            symbol: "𝀖",
            name: "accelerates",
            category: ChronosOperatorCategory::Temporal,
            lines: lines![
                ("≡", ["accelerates", "speeds_process"]),
                ("≝", ["temporal acceleration", "process intensification"]),
                ("⊡", "Փ8Ց7"),
                ("⊛", [
                    "⟲ 𝀖 𝀺→◬",
                    "⟚ 𝀖 via media",
                    "technology 𝀖 social change",
                    "𝀖 feedback loops",
                    "crisis 𝀖 transformation",
                    "𝀖 → compressed temporality"
                ]),
                ("⊢", "𝀖 acceleration ⊢ intensified dynamics ⊢ rapid emergence"),
            ],
            entity_type: None,
        },

        // 𝁤 - Subverts (Undermines From Within)
        ChronosOperatorDef {
            symbol: "𝁤",
            name: "subverts",
            category: ChronosOperatorCategory::Causal,
            lines: lines![
                ("≡", ["subverts", "undermines_from_within"]),
                ("≝", ["performative subversion", "norm undermining through iteration"]),
                ("⊡", "Ց8Փ7"),
                ("∂", [
                    "უdirect opposition",
                    "უexternal attack",
                    "requires working within structure"
                ]),
                ("⊛", [
                    "𝁤 expected_⧈",
                    "performative 𝁤",
                    "⟴ can 𝁤 norms",
                    "𝁤→new_possibilities",
                    "parody 𝁤 original",
                    "𝁤→⚘Judith Butler_drag"
                ]),
                ("⊢", "𝁤 subversion ⊢ internal undermining ⊢ new possibilities"),
            ],
            entity_type: None,
        },
    ]
}

/// Get entity type definitions from chronos operators
/// Returns entity types for operators that also function as entities
pub fn get_entity_type_definitions() -> Vec<EntityTypeDef> {
    get_chronos_operator_definitions()
        .into_iter()
        .filter_map(|op| op.entity_type)
        .collect()
}
