//! ERIS macro definitions

/// High-level macro to set up an operator module
#[macro_export]
macro_rules! define_operator_module {
    (
        $base_name:ident {
            $($variant:ident => $doc:expr),* $(,)?
        }
        $(, extra_fields: { $($(#[$field_meta:meta])* $field_name:ident : $field_type:ty),* $(,)? })?
    ) => {
        paste::paste! {
            /// Category classification for operators
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum [<$base_name OperatorCategory>] {
                $(
                    #[doc = $doc]
                    $variant
                ),*
            }

            define_operator_struct! {
                pub struct [<$base_name OperatorDef>] {
                    category_enum: [<$base_name OperatorCategory>],
                    definitions_fn: [<get_ $base_name:lower _operator_definitions>],
                    $(extra_fields: {
                        $($(#[$field_meta])* $field_name : $field_type),*
                    })?
                }
            }

            /// Get operator by symbol
            pub fn [<get_ $base_name:lower _operator>](symbol: &str) -> Option<[<$base_name OperatorDef>]> {
                get_operator(symbol)
            }
        }
    };
}

/// Macro to define an operator struct with standard fields
#[macro_export]
macro_rules! define_operator_struct {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            category_enum: $category_enum:ident,
            definitions_fn: $defs_fn:ident,
            $(extra_fields: {
                $($(#[$field_meta:meta])* $field_name:ident : $field_type:ty),* $(,)?
            })?
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Debug, Clone)]
        $vis struct $name {
            /// Operator symbol
            pub symbol: &'static str,
            /// Operator name
            pub name: &'static str,
            /// Flexible sequence of definition lines as (prefix, content) tuples
            pub lines: Vec<(&'static str, &'static str)>,
            /// Category classification
            pub category: $category_enum,
            $($(
                $(#[$field_meta])*
                pub $field_name: $field_type,
            )*)?
        }

        impl $name {
            /// Render operator definition as formatted ERIS text
            pub fn to_eris_text(&self) -> String {
                let symbol = self.symbol;
                let symbol_width = symbol.chars().count();

                if self.lines.is_empty() {
                    return format!("{} {}", symbol, self.name);
                }

                let mut result = format!(
                    "{} {} {}",
                    symbol,
                    self.lines[0].0,
                    self.lines[0].1
                );

                // Indent subsequent lines to align with first line's prefix
                let indent = " ".repeat(symbol_width + 1);
                let mut prev_prefix = self.lines[0].0;
                
                for line in &self.lines[1..] {
                    let prefix_display = if line.0 == prev_prefix {
                        // Same prefix as previous line: replace with spaces for alignment
                        " ".repeat(line.0.chars().count())
                    } else {
                        // Different prefix: show it
                        line.0.to_string()
                    };
                    result.push_str(&format!("\n{}{} {}", indent, prefix_display, line.1));
                    prev_prefix = line.0;
                }

                result
            }
        }

        /// Get operator by symbol (internal)
        fn get_operator(symbol: &str) -> Option<$name> {
            $defs_fn()
                .into_iter()
                .find(|op| op.symbol == symbol)
        }

        /// Get all operators in category
        pub fn get_operators_by_category(category: $category_enum) -> Vec<$name> {
            $defs_fn()
                .into_iter()
                .filter(|op| op.category == category)
                .collect()
        }
    };
}

/// Macro to generate public getter function with custom name
#[macro_export]
macro_rules! public_operator_getter {
    ($fn_name:ident, $struct_name:ident) => {
        #[doc = concat!("Get ", stringify!($struct_name), " by symbol")]
        pub fn $fn_name(symbol: &str) -> Option<$struct_name> {
            get_operator(symbol)
        }
    };
}

/// Helper macro for building lines with array expansion
#[macro_export]
macro_rules! lines {
    (@expand ($prefix:expr, [$($content:expr),* $(,)?])) => {
        vec![$( ($prefix, $content) ),*]
    };
    (@expand ($prefix:expr, $content:expr)) => {
        vec![($prefix, $content)]
    };
    
    ($($item:tt),* $(,)?) => {{
        let mut all = Vec::new();
        $(all.extend(lines!(@expand $item));)*
        all
    }};
}

/// Macro to define an entity type module (for ERIS entity types like Person, Place, etc.)
#[macro_export]
macro_rules! define_entity_module {
    (
        $base_name:ident {
            $($variant:ident => $doc:expr),* $(,)?
        }
    ) => {
        paste::paste! {
            use $crate::entities::types::EntityTypeDef;

            /// Category classification for entity types
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum [<$base_name Category>] {
                $(
                    #[doc = $doc]
                    $variant
                ),*
            }

            /// Entity type definition for ERIS operators
            #[derive(Debug, Clone)]
            pub struct [<$base_name Def>] {
                /// Entity symbol
                pub symbol: &'static str,
                /// Entity name
                pub name: &'static str,
                /// Short description
                pub description: &'static str,
                /// Sort order for database
                pub sort_order: i32,
                /// Flexible sequence of definition lines as (prefix, content) tuples
                pub lines: Vec<(&'static str, &'static str)>,
                /// Category classification
                pub category: [<$base_name Category>],
            }

            impl [<$base_name Def>] {
                /// Render entity definition as formatted ERIS text
                pub fn to_eris_text(&self) -> String {
                    let symbol = self.symbol;
                    let symbol_width = symbol.chars().count();

                    if self.lines.is_empty() {
                        return format!("{} {}", symbol, self.name);
                    }

                    let mut result = format!(
                        "{} {} {}",
                        symbol,
                        self.lines[0].0,
                        self.lines[0].1
                    );

                    // Indent subsequent lines to align with first line's prefix
                    let indent = " ".repeat(symbol_width + 1);
                    let mut prev_prefix = self.lines[0].0;
                    
                    for line in &self.lines[1..] {
                        let prefix_display = if line.0 == prev_prefix {
                            " ".repeat(line.0.chars().count())
                        } else {
                            line.0.to_string()
                        };
                        result.push_str(&format!("\n{}{} {}", indent, prefix_display, line.1));
                        prev_prefix = line.0;
                    }

                    result
                }

                /// Convert to EntityTypeDef for database consumption
                pub fn to_entity_type_def(&self) -> EntityTypeDef {
                    EntityTypeDef {
                        symbol: self.symbol,
                        name: self.name,
                        description: self.description,
                        sort_order: self.sort_order,
                    }
                }
            }

            /// Get entity by symbol
            pub fn [<get_ $base_name:lower>](symbol: &str) -> Option<[<$base_name Def>]> {
                [<get_ $base_name:lower _definitions>]()
                    .into_iter()
                    .find(|e| e.symbol == symbol)
            }

            /// Get all entities as EntityTypeDef for database
            pub fn [<get_ $base_name:lower _type_definitions>]() -> Vec<EntityTypeDef> {
                [<get_ $base_name:lower _definitions>]()
                    .into_iter()
                    .map(|e| e.to_entity_type_def())
                    .collect()
            }
        }
    };
}
