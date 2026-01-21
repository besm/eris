//! ERIS symbol constants.
//!
//! Use these instead of hardcoding Unicode symbols in application code.

// =============================================================================
// ENTITY SYMBOLS
// =============================================================================

// Primary entities
pub const PERSON: &str = "⚘";
pub const PLACE: &str = "⌖";
pub const ERA: &str = "⧖";
pub const DATE: &str = "⊙";
pub const EVENT: &str = "⌁";
pub const FIELD: &str = "❖";
pub const GROUP: &str = "⧈";
pub const ORGANIZATION: &str = "⍚";
pub const AGENCY: &str = "⎈";
pub const TECH: &str = "⌬";

// Institutional entities
pub const IDENTIFIER: &str = "⎚";
pub const PUBLISHER: &str = "⍓";
pub const UNIVERSITY: &str = "⍢";
pub const LANGUAGE: &str = "⧩";
pub const JOURNAL: &str = "𝄏";

// Conceptual entities
pub const CONCEPT: &str = "⧊";
pub const METHOD: &str = "⧏";
pub const MOVEMENT: &str = "⌯";

// Relational entities
pub const RELATION: &str = "⇋";
pub const TENSION: &str = "⧆";
pub const LOOP: &str = "⟗";
pub const PARADOX: &str = "☯";

// Process entities
pub const EVOLUTION: &str = "⧃";
pub const ACTION: &str = "⟴";
pub const EFFECT: &str = "⬢";

// Compound citation components
pub const WORK: &str = "⊳";

// User-defined markers
pub const META: &str = "⑀";
pub const QUESTION: &str = "⋯";
pub const PROJECT: &str = "◈";
pub const IDEA: &str = "⟡";
pub const SECTION: &str = "§";

// =============================================================================
// BRACKET SYMBOLS
// =============================================================================

// Tag brackets - entities (simple and compound)
pub const BRACKET_OPEN: char = '⦑';
pub const BRACKET_CLOSE: char = '⦒';

// Vector brackets - Armenian property vectors
pub const VECTOR_OPEN: char = '⟨';
pub const VECTOR_CLOSE: char = '⟩';

// Reference brackets - ID references
pub const REF_OPEN: char = '⟦';
pub const REF_CLOSE: char = '⟧';

// Property vector marker
pub const PROPERTY_VECTOR: char = '⊡';

// =============================================================================
// CHRONOS OPERATORS - Time, Purpose, Teleology
// =============================================================================

// Teleological
pub const TELEOLOGICAL_ANCHOR: char = '⍜';
pub const TELEOLOGICAL_VECTOR: char = 'ω';
pub const INTENTIONALITY: char = 'ι';
pub const ALIGNMENT: char = 'α';
pub const EXECUTION: char = 'ε';

// Physics/temporal vector
pub const PHYSICS_VECTOR: char = 'μ';
pub const HORIZON: char = 'θ';
pub const DENSITY: char = 'δ';
pub const PRECISION: char = 'π';
pub const SYNC: char = 'ς';

// Assessment/diagnostic
pub const CONSISTENCY: char = 'λ';
pub const FLUX: char = 'ϕ';
pub const CAPACITY: char = 'γ';

// Causal-temporal
pub const FOLLOWS_RESULTING: char = '⊱';
pub const FEEDBACK_LOOP: char = '⟲';
pub const RECURSIVE_PROCESS: char = '⟖';
pub const REFLEXIVE: char = '𝄎';

// Temporal ordering
pub const ALWAYS: char = '□';
pub const EVENTUALLY: char = '◇';
pub const PRECEDES: char = '≺';
pub const SUCCEEDS: char = '≻';
pub const NEXT_STATE: char = '○';
pub const SIMULTANEOUS: char = '⬡';

// Process/state
pub const PROCESS: char = '𝀺';
pub const STATE: char = '⏣';
pub const PREDICTION: char = '⟟';
pub const REPEATS: char = '𝄃';

// Becoming
pub const PREHENSION: char = '𝀃';
pub const CONCRESCENCE: char = '𝀷';
pub const REINFORCES: char = '𝆌';
pub const ACCELERATES: char = '𝀖';
pub const SUBVERTS: char = '𝁤';

// =============================================================================
// GEORGIAN OPERATORS - Workflow States
// =============================================================================

// Temporal flow
pub const PAST: char = 'დ';
pub const NOW: char = 'ნ';
pub const FUTURE: char = 'წ';

// Operational core
pub const GROUNDING: char = 'გ';
pub const OPERATION: char = 'ო';
pub const SHIFT: char = 'შ';

// Structural markers
pub const WITHOUT: char = 'უ';
pub const SOURCE: char = 'ჭ';

// =============================================================================
// LOGICAL OPERATORS
// =============================================================================

// Core logical
pub const EQUIVALENCE: char = '≡';
pub const DEFINED_AS: char = '≝';
pub const IMPLIES: char = '→';
pub const CONJUNCTION: char = '∧';
pub const DISJUNCTION: char = '∨';
pub const NEGATION: char = '¬';
pub const FOR_ALL: char = '∀';
pub const EXISTS: char = '∃';
pub const THEREFORE: char = '∴';
pub const IFF: char = '↔';
pub const BOTTOM: char = '⊥';
pub const INFINITY: char = '∞';

// Set operations
pub const SUBSET: char = '⊂';
pub const SUPERSET: char = '⊃';
pub const NOT_SUBSET: char = '⊅';
pub const SUBSET_OR_EQUAL: char = '⊆';
pub const ELEMENT_OF: char = '∈';
pub const NOT_ELEMENT_OF: char = '∉';
pub const INTERSECTION: char = '∩';
pub const UNION: char = '∪';
pub const BIG_UNION: char = '⋃';
pub const BIG_INTERSECTION: char = '⋂';
pub const JOIN: char = '⊔';
pub const EMPTY_SET: char = '∅';

// Mathematical
pub const SUMMATION: char = '∑';
pub const INTEGRATION: char = '∫';
pub const CONGRUENT: char = '≅';
pub const ISOMORPHISM: char = '≃';
pub const APPROXIMATELY_EQUALS: char = '≈';
pub const DENSITY_OP: char = '≐';
pub const PROPORTIONAL_TO: char = '∝';
pub const GRADIENT: char = '∇';
pub const INTEGER: char = 'ℤ';

// Workflow logical
pub const NECESSITY: char = '◻';
pub const VALIDATES: char = '⊨';
pub const STRONG_PREFERENCE: char = '≫';
pub const PATTERN_DETECTION: char = '⊛';
pub const TESTING: char = '≟';
pub const UNCERTAIN: char = '⊟';
pub const ENTAILS: char = '⊢';
pub const POLICY_ENFORCEMENT: char = '⊩';
pub const EMBODIES: char = '⊧';
pub const TRIANGULATES: char = '⊿';
pub const IMPORTS: char = '←';
pub const IMPORT: char = '⊸';
pub const BOUNDARY: char = '∂';

// Workflow state operators
pub const AWAITING: char = '⊐';
pub const OWING: char = '⊏';
pub const DORMANT_OP: char = '⊔';
pub const ORPHANED: char = '⊬';

// Process operators
pub const PRECEDES_ENABLING: char = '⊰';
pub const BIDIRECTIONAL: char = '⟷';
pub const ATTRACTOR_OP: char = '✱';
pub const ANALYZE: char = '⩕';
pub const PERFORMANCE: char = '↗';
pub const COMMAND: char = '◩';
pub const MANAGEMENT: char = '⏈';

// Composition operators
pub const COMPOSE: char = '∘';
pub const STRUCTURED_COMBINATION: char = '⊕';
pub const CONNECTS: char = '↬';
pub const TRANSLATION: char = '⌇';
pub const RECIPROCAL_FLOW: char = '↭';
pub const PROPERTY_DIFFERENCE: char = '⊖';
pub const PROPERTY_PROJECTION: char = '↓';
pub const PROPERTY_COMPATIBILITY: char = '⋄';
pub const METAPHORIZE: char = '⫍';
pub const COMMIT_OP: char = '⊞';
pub const RESOLVE: char = '∮';
pub const INTERACTION: char = '⎄';
pub const INTEGRATION_OP: char = '⨝';
pub const MERGER: char = '⎊';
pub const EXTRACT: char = '⇌';

// =============================================================================
// META OPERATORS
// =============================================================================

pub const DESCRIPTION: char = '⟓';
pub const EXAMPLES: char = '⧠';
pub const TAG: &str = "⌕";
pub const SCHEMA: char = '⋕';
pub const COLLECTION: char = '⌹';
pub const META_SYMBOL: char = '⧰';
pub const TRANSPARENT: char = '▢';
pub const PARTIALLY_KNOWN: char = '◐';
pub const MOSTLY_HIDDEN: char = '◔';
pub const ORIGIN: char = '𝟎';
pub const SYMBOL: &str = "ꕥ";
pub const PATTERN: &str = "ᛝ";
pub const HIGHLIGHT_REF: char = '❧';

// =============================================================================
// ONTOLOGY OPERATORS
// =============================================================================

// Creation
pub const CREATES: char = '𝀐';
pub const SPEECH_ACT: char = '⛣';

// Constitution
pub const CONSTITUTES: char = '𝁚';
pub const EXTENSION: char = '𝀼';
pub const SHAPES: char = '𝀞';
pub const HABITUS: char = '⌂';
pub const SHAPES_THROUGH_INFLUENCE: char = '⤑';
pub const PERFORMATIVITY_CHAIN: char = '⬟';
pub const INTERACTIVE_KINDS: char = '⥎';

// Grounding
pub const GROUNDS: char = '⟒';
pub const MEMORY: char = '⛁';
pub const ENTITY_NODE: char = '⩎';

// Dynamics
pub const CRYSTALLIZE: char = '𝀏';
pub const FLOW: char = '𝀾';
pub const PERTURBATION: char = '𝀸';
pub const BEHAVIOR: char = '𝀕';

// States
pub const EQUILIBRIUM: char = '𝀆';
pub const OSCILLATION: char = '𝁀';

// Transitions
pub const DISSOLUTION: char = '𝁆';
pub const SUBLATION: char = '𝀗';
pub const DESTROYS: char = '𝀶';
pub const TRANSFORMS: char = '𝀴';

// =============================================================================
// SEMANTIC OPERATORS
// =============================================================================

// Stability states
pub const STABLE_WELL: char = '⌺';
pub const FORMING_WELL: char = '⌻';
pub const DISSOLVING_WELL: char = '⌼';

// Dynamics
pub const DECLINE: char = '⤋';
pub const EMPHASIZES: char = '𝀋';
pub const THOUGHT: char = '𝀔';
pub const THEME: char = '𝀭';
pub const DEEPENS: char = '𝀙';
pub const INTERANIMATION: char = '⥈';
pub const UNDERSTANDING: char = '☊';
pub const MEANING: &str = "ℳ";
pub const LANGUAGE_OP: char = '◭';
pub const SHARED: char = '⩍';

// Processes
pub const RECURSIVE: char = '⟳';
pub const FEEDFORWARD: char = '⥅';
pub const DEEP_TRANSFORM: char = '⤇';

// Emergence
pub const EMERGENCE: char = '◬';
pub const BOUNDARY_OBJECT: char = '┃';

// Institutional
pub const INSTITUTIONAL: char = '⛫';

// Performative
pub const SELF_FULFILLING_PROPHECY: char = '⟚';
pub const INSTITUTIONAL_SHAPING: char = '⟛';

// Relations
pub const RELATION_OP: char = '⋈';
pub const QUESTIONS: char = '⌾';

// Contextual
pub const CONTEXT: char = '⯐';
pub const PERSPECTIVE: char = '⊚';
