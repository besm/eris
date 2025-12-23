//! ERIS entity system

use paste::paste;

pub mod types;

pub mod person;
pub mod place;
pub mod era;
pub mod date;
pub mod event;
pub mod field;
pub mod group;
pub mod organization;
pub mod agency;
pub mod tech;
pub mod identifier;
pub mod publisher;
pub mod university;
pub mod language;
pub mod concept;
pub mod method;
pub mod movement;
pub mod relation;
pub mod tension;
pub mod r#loop;
pub mod paradox;
pub mod evolution;
pub mod action;
pub mod effect;
pub mod work;
pub mod journal;
pub mod meta;
pub mod question;
pub mod project;
pub mod idea;
pub mod section;

macro_rules! aggregate_entities {
    ($($mod:ident),+ $(,)?) => {
        paste! {
            /// Entity type enum generated from all entity modules
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum EntityType {
                $([<$mod:camel>]),+
            }

            impl EntityType {
                /// Look up entity type from its symbol character
                pub fn from_symbol(s: char) -> Option<Self> {
                    let sym = s.to_string();
                    $(if $mod::get_entity(&sym).is_some() { return Some(Self::[<$mod:camel>]); })+
                    None
                }

                /// Get the symbol for this entity type
                pub fn symbol(&self) -> char {
                    match self {
                        $(Self::[<$mod:camel>] => $mod::get_entity_definitions()[0].symbol.chars().next().unwrap()),+
                    }
                }

                /// Get the name of this entity type
                pub fn name(&self) -> &'static str {
                    match self {
                        $(Self::[<$mod:camel>] => $mod::get_entity_definitions()[0].name),+
                    }
                }

                /// Get all entity types
                pub fn all() -> &'static [Self] {
                    &[$(Self::[<$mod:camel>]),+]
                }
            }
        }

        pub fn get_all_definitions() -> Vec<String> {
            let mut defs = Vec::new();
            $(for e in $mod::get_entity_definitions() { defs.push(e.to_eris_text()); })+
            defs
        }

        pub fn get_all_symbols() -> Vec<String> {
            let mut syms = Vec::new();
            $(for e in $mod::get_entity_definitions() { syms.push(e.symbol.to_string()); })+
            syms
        }

        pub fn lookup(symbol: &str) -> Option<String> {
            $(if let Some(e) = $mod::get_entity(symbol) { return Some(e.to_eris_text()); })+
            None
        }
    };
}

aggregate_entities!(
    person, place, era, date, event, field, group, organization, agency,
    tech, identifier, publisher, university, language, concept, method,
    movement, relation, tension, r#loop, paradox, evolution, action,
    effect, work, journal, meta, question, project, idea, section
);
