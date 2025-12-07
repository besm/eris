//! ERIS operator definitions
//!
//! The operator system provides symbolic vocabulary for expressing relationships,
//! processes, and properties that recur across knowledge domains.
//!
//! ## Operator Categories
//!
//! **Armenian** — A numeric rating system for evaluating properties of concepts
//! and relationships. When you need to say how generalizable something is, whether
//! it works in both directions, or how well it compresses—Armenian gives you
//! a 0-9 scale encoded in letters. Think of it as a property vector for ideas.
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

pub mod armenian;
pub mod chronos;
pub mod georgian;
pub mod logical;
pub mod meta;
pub mod ontology;
pub mod semantic;

macro_rules! aggregate_operators {
    ($($mod:ident :: $defs:ident / $get:ident),+ $(,)?) => {
        pub fn get_all_definitions() -> Vec<String> {
            let mut defs = Vec::new();
            $(for op in $mod::$defs() { defs.push(op.to_eris_text()); })+
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
    armenian::get_armenian_operator_definitions/get_armenian_operator,
    chronos::get_chronos_operator_definitions/get_chronos_operator,
    georgian::get_georgian_operator_definitions/get_georgian_operator,
    logical::get_logical_operator_definitions/get_logical_operator,
    meta::get_meta_operator_definitions/get_meta_operator,
    ontology::get_ontology_operator_definitions/get_ontology_operator,
    semantic::get_semantic_operator_definitions/get_semantic_operator
);
