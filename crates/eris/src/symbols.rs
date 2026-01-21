//! ERIS symbol constants.
//!
//! Use these instead of hardcoding Unicode symbols in application code.

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

// Institutional
pub const IDENTIFIER: &str = "⎚";
pub const PUBLISHER: &str = "⍓";
pub const UNIVERSITY: &str = "⍢";
pub const LANGUAGE: &str = "⧩";
pub const JOURNAL: &str = "𝄏";

// Conceptual
pub const CONCEPT: &str = "⧊";
pub const METHOD: &str = "⧏";
pub const MOVEMENT: &str = "⌯";

// Relational/dynamic
pub const RELATION: &str = "⇋";
pub const TENSION: &str = "⧆";
pub const LOOP: &str = "⟗";
pub const PARADOX: &str = "☯";

// Process/effect
pub const EVOLUTION: &str = "⧃";
pub const ACTION: &str = "⟴";
pub const EFFECT: &str = "⬢";

// Compound citation components
pub const WORK: &str = "⊳";

// ML/recommendation symbols
pub const ATTRACTOR: &str = "✱";
pub const TAG: &str = "⌕";
pub const COMMIT: &str = "⊞";

// User-defined markers
pub const META: &str = "⑀";
pub const QUESTION: &str = "⋯";
pub const PROJECT: &str = "◈";
pub const IDEA: &str = "⟡";
pub const SECTION: &str = "§";

// Tag brackets - entities (simple and compound)
pub const BRACKET_OPEN: char = '⦑';
pub const BRACKET_CLOSE: char = '⦒';

// Vector brackets - Armenian property vectors
pub const VECTOR_OPEN: char = '⟨';
pub const VECTOR_CLOSE: char = '⟩';

// Reference brackets - ID references
pub const REF_OPEN: char = '⟦';
pub const REF_CLOSE: char = '⟧';

// Logical operators
pub const CONJUNCTION: char = '∧';
pub const DISJUNCTION: char = '∨';
pub const NEGATION: char = '¬';
pub const IMPLIES: char = '→';
pub const IFF: char = '↔';
pub const FOR_ALL: char = '∀';
pub const EXISTS: char = '∃';
pub const THEREFORE: char = '∴';
pub const SUBSET: char = '⊂';
pub const SUPERSET: char = '⊃';
pub const ELEMENT_OF: char = '∈';
pub const NOT_ELEMENT_OF: char = '∉';
pub const INTERSECTION: char = '∩';
pub const UNION: char = '∪';
pub const EQUIVALENCE: char = '≡';
pub const DEFINED_AS: char = '≝';
pub const BOUNDARY: char = '∂';
pub const ENTAILS: char = '⊢';
pub const VALIDATES: char = '⊨';
pub const NECESSITY: char = '◻';
pub const TESTING: char = '≟';
pub const UNCERTAIN: char = '⊟';
pub const PRECEDES_ENABLING: char = '⊰';
pub const COMPOSE: char = '∘';

// Workflow operators
pub const AWAITING: char = '⊐';
pub const OWING: char = '⊏';
pub const DORMANT: char = '⊔';
pub const ORPHANED: char = '⊬';

// Temporal operators
pub const ALWAYS: char = '□';
pub const EVENTUALLY: char = '◇';
pub const PRECEDES: char = '≺';
pub const SUCCEEDS: char = '≻';
pub const NEXT_STATE: char = '○';

// Georgian workflow markers
pub const PAST: char = 'დ';
pub const NOW: char = 'ნ';
pub const FUTURE: char = 'წ';
pub const GROUNDING: char = 'გ';
pub const OPERATION: char = 'ო';
pub const SHIFT: char = 'შ';
pub const WITHOUT: char = 'უ';

// Semantic stability states
pub const STABLE_WELL: char = '⌺';
pub const FORMING_WELL: char = '⌻';
pub const DISSOLVING_WELL: char = '⌼';

// Ontology operators
pub const CREATES: char = '𝀐';
pub const CONSTITUTES: char = '𝁚';
pub const CRYSTALLIZE: char = '𝀏';
pub const FLOW: char = '𝀾';
pub const DISSOLUTION: char = '𝁆';
pub const SUBLATION: char = '𝀗';
pub const CONCRESCENCE: char = '𝀷';

// Semantic operators
pub const CONTEXT: char = '⯐';
pub const PERSPECTIVE: char = '⊚';
pub const EMERGENCE: char = '◬';
pub const FEEDFORWARD: char = '⥅';
pub const INTERANIMATION: char = '⥈';
pub const UNDERSTANDING: char = '☊';
pub const MEANING: &str = "ℳ";

// Chronos operators
pub const TELEOLOGICAL_ANCHOR: char = '⍜';
pub const PREDICTION: char = '⟟';
pub const FEEDBACK_LOOP: char = '⟲';
pub const RECURSIVE_PROCESS: char = '⟖';
pub const REFLEXIVE: char = '𝄎';
pub const PROCESS: char = '𝀺';
pub const STATE: char = '⏣';

// Armenian property vector indicator
pub const PROPERTY_VECTOR: char = '⊡';

// Meta operators
pub const PATTERN: &str = "ᛝ";
pub const SYMBOL: &str = "ꕥ";
pub const SCHEMA: char = '⋕';

// Reference symbols
pub const HIGHLIGHT_REF: char = '❧';
