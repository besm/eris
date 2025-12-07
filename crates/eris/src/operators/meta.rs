//! ≡ ERIS meta-level operator system
//! ≝ Operators for documentation, assessment, and quality evaluation

define_operator_module! {
    Meta {
        Documentation => "Description and specification operators",
        Assessment => "Value mapping and quality assessment",
    },
    extra_fields: {
        /// Armenian property vector rating (if applicable)
        property_vector: Option<&'static str>,
    }
}

/// Get all meta-level operator definitions
pub fn get_meta_operator_definitions() -> Vec<MetaOperatorDef> {
    vec![
        // ⟓ - Description
        MetaOperatorDef {
            symbol: "⟓",
            name: "description",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["description", "definition_field"]),
                ("≝", ["specification clarity", "metadata structure"]),
                ("⊛", [
                    "⟓→specification_clarity ∧ ⟓∈metadata_structure",
                    "⩎⟓comprehensive_documentation ∧ ⟓→semantic_precision",
                    "API⟓design ∧ schema⟓completeness"
                ]),
            ],
        },
        // ⊡ - Value Map
        MetaOperatorDef {
            symbol: "⊡",
            name: "value_map",
            category: MetaOperatorCategory::Assessment,
            property_vector: None,
            lines: lines![
                ("≡", ["value_map", "quality_assessment"]),
                ("≝", ["value assessment notation", "quality evaluation scale"]),
                ("⊛", [
                    "⊡ property_vector notation ∧ Armenian 0-9 scale",
                    "⊡⟨Փ9Գ8⟩ indicates high value assessment",
                    "⊡ prefix for value_map scores across operators"
                ]),
            ],
        },
        // ⧠ - Examples
        MetaOperatorDef {
            symbol: "⧠",
            name: "examples",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["examples", "concrete_instances"]),
                ("≝", ["concrete cases", "manifestations"]),
                ("⊛", [
                    "⧏⧠concrete_cases",
                    "⧊⧠manifestations",
                    "pattern⧠occurrences",
                    "⋕⧠implementations"
                ]),
            ],
        },
        // ⌕ - Tag
        MetaOperatorDef {
            symbol: "⌕",
            name: "tag",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["tag", "classification_marker"]),
                ("≝", ["entity annotation", "highlight-to-concept bridge"]),
                ("∂", [
                    "⌕⊅⧊ (⌕ applied | ⧊ abstract)",
                    "⌕⊅❧ (⌕ annotation | ❧ passage)"
                ]),
                ("⊛", [
                    "❧ ⊰ ⌕ (highlights carry tags)",
                    "⌕⦑entity⦒ notation",
                    "⌕ binds ❧↬⧊",
                    "⌕ binds ❧↬⚘",
                    "⌕ binds ❧↬◈",
                    "shared ⌕ → ❧⥈❧",
                    "⌕.validated ∈ {machine, human, both}"
                ]),
                ("⊢", "⌕ tag ⊢ ❧ classified ⊢ ☊ retrievable"),
            ],
        },
        // ⋕ - Schema
        MetaOperatorDef {
            symbol: "⋕",
            name: "schema",
            category: MetaOperatorCategory::Documentation,
            property_vector: Some("⟨Բ9Գ8Վ7⟩"),
            lines: lines![
                ("≡", ["schema", "structural_template"]),
                ("≝", ["structural template", "design pattern"]),
                ("⊡", "⟨Բ9Գ8Վ7⟩"),
                ("⊛", [
                    "⋕→გ_guide",
                    "⋕∈design_ᛝ",
                    "⋕.constraints→⊩"
                ]),
            ],
        },
        // ⌹ - Collection
        MetaOperatorDef {
            symbol: "⌹",
            name: "collection",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["collection", "organized_set"]),
                ("≝", ["organized set", "curated grouping"]),
                ("⊛", [
                    "⌹∑",
                    "⌹∈ℳ_⍚",
                    "⌹.curation→quality"
                ]),
            ],
        },
        // ⧰ - Meta Symbol
        MetaOperatorDef {
            symbol: "⧰",
            name: "meta_symbol",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["meta_symbol", "level_marker"]),
                ("≝", ["notation discussing notation", "self-reference indicator"]),
                ("∂", [
                    "⧰⊅⧊ (⧰ flags level | ⧊ first-order concept)",
                    "⧰⊅𝄎 (⧰ marker | 𝄎 reflexive property)"
                ]),
                ("⊛", [
                    "⧰ flags ◭_discussing_◭",
                    "⧰ marks ꕥ_defining_ꕥ",
                    "⧰ indicates system_describing_system",
                    "operator_registry ∈ ⧰",
                    "⧰ enables ⟖ discourse"
                ]),
                ("⊢", "⧰ meta ⊢ level_shift ⊢ self-description enabled"),
            ],
        },
        // ▢ - Transparent
        MetaOperatorDef {
            symbol: "▢",
            name: "transparent",
            category: MetaOperatorCategory::Assessment,
            property_vector: Some("Ե9Բ8"),
            lines: lines![
                ("≡", ["transparent", "open"]),
                ("≝", ["epistemic transparency", "fully visible"]),
                ("⊡", "Ե9Բ8"),
                ("⊛", [
                    "▢ ⧈_visible",
                    "power_relations▢",
                    "▢ mechanisms_explicit",
                    "▢≫◔ for understanding",
                    "⛫_𝀺▢",
                    "𝀭 analysis→▢"
                ]),
            ],
        },
        // ◐ - Partially Known
        MetaOperatorDef {
            symbol: "◐",
            name: "partially_known",
            category: MetaOperatorCategory::Assessment,
            property_vector: Some("Ե6Բ5"),
            lines: lines![
                ("≡", ["partially_known", "semi-transparent"]),
                ("≝", ["partial visibility", "requires investigation"]),
                ("⊡", "Ե6Բ5"),
                ("⊛", [
                    "reality_making◐visible",
                    "◐ some_mechanisms_hidden",
                    "⧈_effects◐",
                    "◐ requires_investigation",
                    "power◐transparent"
                ]),
            ],
        },
        // ◔ - Mostly Hidden
        MetaOperatorDef {
            symbol: "◔",
            name: "mostly_hidden",
            category: MetaOperatorCategory::Assessment,
            property_vector: Some("Ց3Բ2"),
            lines: lines![
                ("≡", ["mostly_hidden", "opaque"]),
                ("≝", ["epistemic opacity", "naturalized invisibility"]),
                ("⊡", "Ց3Բ2"),
                ("⊛", [
                    "⛫_power◔",
                    "⧈_origins◔",
                    "◔ naturalized_as_given",
                    "reality_construction◔",
                    "◔◻ archeological_work",
                    "⌂ operates◔",
                    "◔→𝁤 potential"
                ]),
            ],
        },

        // 𝟎 - Origin (foundation)
        MetaOperatorDef {
            symbol: "𝟎",
            name: "origin",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["origin", "foundation"]),
                ("≝", ["starting point", "foundational source"]),
                ("∂", [
                    "𝟎≠⌺ (source vs stability)",
                    "𝟎≠⧊ (origin vs concept)"
                ]),
                ("⊛", [
                    "𝟎→⧊",
                    "𝟎=beginning",
                    "𝟎 of ◭=first_principle",
                    "from 𝟎 ☊ all"
                ]),
                ("⊢", "𝟎 origin ⊢ foundational source ⊢ first principle"),
            ],
        },

        // ꕥ - Symbol (notation)
        MetaOperatorDef {
            symbol: "ꕥ",
            name: "symbol",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["symbol", "notation"]),
                ("≝", ["sign vehicle", "carrier of meaning"]),
                ("∂", [
                    "ꕥ≠ℳ (carrier vs content)",
                    "ꕥ≠◭ (unit vs system)"
                ]),
                ("⊛", [
                    "ꕥ→ℳ",
                    "ℳ/ꕥ→∞",
                    "ꕥ gain ℳ through use↬",
                    "ꕥ→ꕥ↬→use",
                    "ancient_ꕥ⊗math⊗⧊",
                    "ꕥ_system=meta-discussion",
                    "ꕥ→ო→გ via ⬟"
                ]),
                ("⊢", "ꕥ symbol ⊢ notation unit ⊢ meaning carrier"),
            ],
        },

        // ᛝ - Pattern (template)
        MetaOperatorDef {
            symbol: "ᛝ",
            name: "pattern",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["pattern", "template"]),
                ("≝", ["recurring structure", "generative form"]),
                ("∂", [
                    "ᛝ≠⩎ (abstract vs instance)",
                    "ᛝ≠⧊ (structure vs concept)"
                ]),
                ("⊛", [
                    "⫍→ᛝ",
                    "ᛝ→გ⩎",
                    "ᛝ→⊿⊨→გ⩎",
                    "recurring_ᛝ detected by ⊛",
                    "ᛝ crystallizes forms",
                    "usage_ᛝ in elaborations",
                    "abstract_ᛝ⊧concrete_implementation"
                ]),
                ("⊢", "ᛝ pattern ⊢ recurring structure ⊢ template application"),
            ],
        },
        // ❧ - Highlight
        MetaOperatorDef {
            symbol: "❧",
            name: "highlight",
            category: MetaOperatorCategory::Documentation,
            property_vector: None,
            lines: lines![
                ("≡", ["highlight", "extracted_passage"]),
                ("≝", ["citation unit", "tagged datum"]),
                ("∂", [
                    "❧⊅⊳ (❧ fragment | ⊳ whole)",
                    "❧⊅⋯ (❧ captured | ⋯ question)"
                ]),
                ("⊛", [
                    "❧⟦id⟧ reference notation",
                    "⊳ ⊃ {❧,❧,❧...}",
                    "❧ ⊰ ⌕ (highlights carry tags)",
                    "◈ ⊰ ❧ (projects aggregate highlights)",
                    "❧ ≡ evidence_atom",
                    "❧⥈❧ via shared ⌕"
                ]),
                ("⊢", "❧ highlight ⊢ passage extracted ⊢ datum for ☊"),
            ],
        },
    ]
}
