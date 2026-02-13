//! ERIS operator definitions
//!
//! The operator system provides symbolic vocabulary for expressing relationships,
//! processes, and properties that recur across knowledge domains.
//!
//! ## Operator Categories
//!
//! **Property Vectors** — A numeric rating system for evaluating properties of concepts
//! and relationships. When you need to say how generalizable something is, whether
//! it works in both directions, or how well it compresses—property vectors give you
//! a 0-9 scale encoded in Tifinagh symbols. Think of it as a property vector for ideas.
//!
//! **Chronos** — Time and purpose intertwined. These operators express not just
//! when things happen but why they're aimed at particular ends. The vocabulary
//! for discussing teleology, causation, phases of development, and the way
//! moments accumulate into meaning.
//!
//! **Georgian** — The machinery of workflow. Past, present, and future as active
//! contexts rather than mere timestamps. Verbs for guiding, operating, dissolving.
//! What you reach for when modeling how processes unfold through states.
//!
//! **Logical** — Mathematical notation adapted for knowledge work. The grammar
//! underlying everything else: definition, implication, conjunction, necessity.
//! If you're composing claims or enforcing constraints, you're using these.
//!
//! **Meta** — Operators that talk about operators. Documentation, schemas,
//! examples, tags. Also epistemic visibility—how much of something is transparent
//! versus hidden. The self-referential layer that lets the system describe itself.
//!
//! **Ontology** — How realities get made. Creation, constitution, grounding.
//! The dynamics of crystallizing into stable forms or dissolving back into flux.
//! Performativity—when naming something brings it into being. The philosophical
//! heavy machinery for discussing what exists and how it came to exist.
//!
//! **Semantic** — Meaning as a physical system. Semantic gravity wells that
//! attract related content, stability states that form or dissolve, institutional
//! forces that shape what can be thought. Where sociology of knowledge meets
//! dynamical systems.

pub mod chronos;
pub mod loader;
pub mod georgian;
pub mod logical;
pub mod meta;
pub mod ontology;
pub mod semantic;

/// Operators whose standalone definition blocks are omitted in reduced mode.
/// Their meaning is already internalized through pervasive use as line prefixes,
/// or mirrors conventional mathematical/logical usage that LLMs already know.
pub const REDUCED_SYMBOLS: &[char] = &[
    // Well-defined through use (appear as prefixes in every definition)
    crate::symbols::EQUIVALENCE,     // ≡
    crate::symbols::DEFINED_AS,      // ≝
    // Standard logic (conventional use, identical to textbook)
    crate::symbols::IMPLIES,         // →
    crate::symbols::CONJUNCTION,     // ∧
    crate::symbols::DISJUNCTION,     // ∨
    crate::symbols::NEGATION,        // ¬
    crate::symbols::FOR_ALL,         // ∀
    crate::symbols::EXISTS,          // ∃
    crate::symbols::VALIDATES,       // ⊨
    crate::symbols::ENTAILS,         // ⊢
    // Standard set theory (conventional use)
    crate::symbols::SUBSET,          // ⊂
    crate::symbols::SUPERSET,        // ⊃
    crate::symbols::NOT_SUBSET,      // ⊅
    // Well-defined through use (meaning obvious from 80+ uses as prefix)
    crate::symbols::PROPERTY_VECTOR, // ⊡
    // Standard modal/temporal logic
    crate::symbols::ALWAYS,          // □
    crate::symbols::EVENTUALLY,      // ◇
    // Standard order relations
    crate::symbols::PRECEDES,        // ≺
    crate::symbols::SUCCEEDS,        // ≻
    // Conventional symbols
    crate::symbols::BIDIRECTIONAL,   // ⟷
    crate::symbols::INFINITY,        // ∞
    crate::symbols::STRUCTURED_COMBINATION, // ⊕
];

/// Operators reduced to their def line only (symbol ≡ name) in reduced mode.
/// Meaning is clear from the name but the symbol still appears for closure.
pub const REDUCED_DEF_ONLY: &[char] = &[
    crate::symbols::STRONG_PREFERENCE, // ≫
    crate::symbols::REPEATS,           // 𝄃
    crate::symbols::NEXT_STATE,        // ○
];

fn is_reduced(symbol: &str) -> bool {
    REDUCED_SYMBOLS.iter().any(|s| symbol == s.to_string())
}

fn is_def_only(symbol: &str) -> bool {
    REDUCED_DEF_ONLY.iter().any(|s| symbol == s.to_string())
}

/// Render just the first line of a definition (symbol ≡ name)
fn def_line_only(text: &str) -> String {
    text.lines().next().unwrap_or(text).to_string()
}

macro_rules! aggregate_operators {
    ($($mod:ident :: $defs:ident / $get:ident),+ $(,)?) => {
        pub fn get_all_definitions() -> Vec<String> {
            let mut defs = Vec::new();
            $(for op in $mod::$defs() { defs.push(op.to_eris_text()); })+
            defs
        }

        pub fn get_all_definitions_reduced() -> Vec<String> {
            let mut defs = Vec::new();
            $(for op in $mod::$defs() {
                let sym = op.symbol.to_string();
                if is_reduced(&sym) {
                    // omit entirely
                } else if is_def_only(&sym) {
                    defs.push(def_line_only(&op.to_eris_text()));
                } else {
                    defs.push(op.to_eris_text());
                }
            })+
            defs
        }

        pub fn get_all_symbols() -> Vec<String> {
            let mut syms = Vec::new();
            $(for op in $mod::$defs() { syms.push(op.symbol.to_string()); })+
            syms
        }

        pub fn lookup(symbol: &str) -> Option<String> {
            $(if let Some(op) = $mod::$get(symbol) { return Some(op.to_eris_text()); })+
            None
        }
    };
}

aggregate_operators!(
    loader::get_property_vector_definitions/get_property_vector,
    chronos::get_chronos_operator_definitions/get_chronos_operator,
    georgian::get_georgian_operator_definitions/get_georgian_operator,
    logical::get_logical_operator_definitions/get_logical_operator,
    meta::get_meta_operator_definitions/get_meta_operator,
    ontology::get_ontology_operator_definitions/get_ontology_operator,
    semantic::get_semantic_operator_definitions/get_semantic_operator
);
