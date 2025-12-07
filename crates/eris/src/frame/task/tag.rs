//! initTagging workflow
//!
//! Systematic tagging workflow for human-AI collaborative validation.

use crate::lines;

/// Render lines with operator-style formatting
fn render_lines(lines: &[(&str, &str)], base_indent: &str) -> String {
    if lines.is_empty() {
        return String::new();
    }

    let mut result = format!("{}{} {}", base_indent, lines[0].0, lines[0].1);
    let mut prev_prefix = lines[0].0;

    for line in &lines[1..] {
        let prefix_display = if line.0 == prev_prefix {
            " ".repeat(line.0.chars().count())
        } else {
            line.0.to_string()
        };
        result.push_str(&format!("\n{}{} {}", base_indent, prefix_display, line.1));
        prev_prefix = line.0;
    }

    result
}

/// Workflow step - uses same lines pattern as operators
#[derive(Debug, Clone)]
pub struct WorkflowStep {
    pub symbol: &'static str,
    pub name: &'static str,
    pub lines: Vec<(&'static str, &'static str)>,
}

impl WorkflowStep {
    fn to_eris_text(&self) -> String {
        let symbol_width = self.symbol.chars().count();
        let indent = " ".repeat(symbol_width + 1);

        let mut result = format!("{} {}", self.symbol, self.name);
        if !self.lines.is_empty() {
            result.push('\n');
            result.push_str(&render_lines(&self.lines, &indent));
        }
        result
    }
}

/// Validation dispatch rule
#[derive(Debug, Clone)]
pub struct DispatchRule {
    pub condition: &'static str,
    pub result: &'static str,
}

/// Validation formula - dispatch format
#[derive(Debug, Clone)]
pub struct ValidationFormula {
    pub symbol: &'static str,
    pub rules: Vec<DispatchRule>,
}

impl ValidationFormula {
    fn to_eris_text(&self) -> String {
        let mut result = format!("{} ⊨", self.symbol);
        for rule in &self.rules {
            result.push_str(&format!("\n  {} → {}", rule.condition, rule.result));
        }
        result
    }
}

/// Discrimination reference - points to entity ≟ sections
#[derive(Debug, Clone)]
pub struct DiscriminationRef {
    pub symbols: &'static [&'static str],
    pub description: &'static str,
}

impl DiscriminationRef {
    fn to_eris_text(&self) -> String {
        format!("  {} → {}", self.description, self.symbols.join(" ∧ "))
    }
}

/// Principle - uses same lines pattern as operators
#[derive(Debug, Clone)]
pub struct Principle {
    pub name: &'static str,
    pub lines: Vec<(&'static str, &'static str)>,
}

impl Principle {
    fn to_eris_text(&self) -> String {
        let mut result = self.name.to_string();
        if !self.lines.is_empty() {
            result.push('\n');
            result.push_str(&render_lines(&self.lines, "  "));
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct InitTaggingWorkflow {
    pub name: &'static str,
    pub description: &'static str,
    pub principle: Option<Principle>,
    pub steps: Vec<WorkflowStep>,
    pub validations: Vec<ValidationFormula>,
    pub discriminations: Vec<DiscriminationRef>,
}

impl InitTaggingWorkflow {
    pub fn to_eris_text(&self) -> String {
        let mut result = format!("{}\n  ≡ {}\n", self.name, self.description);

        if let Some(ref principle) = self.principle {
            result.push_str(&format!("\n{}\n", principle.to_eris_text()));
        }

        result.push_str("\nWorkflow Sequence\n");
        for step in &self.steps {
            result.push_str(&format!("\n{}\n", step.to_eris_text()));
        }

        if !self.validations.is_empty() {
            result.push_str("\nValidation Formulas\n");
            for val in &self.validations {
                result.push_str(&format!("\n{}\n", val.to_eris_text()));
            }
        }

        if !self.discriminations.is_empty() {
            result.push_str("\n≟ Discrimination: `eris <symbol>` for ≟ section\n");
            for disc in &self.discriminations {
                result.push_str(&format!("\n{}\n", disc.to_eris_text()));
            }
        }

        result
    }
}

impl std::fmt::Display for InitTaggingWorkflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_eris_text())
    }
}

/// Get the initTagging workflow
pub fn get_inittagging_workflow() -> InitTaggingWorkflow {
    InitTaggingWorkflow {
        name: "initTagging",
        description: "systematic tagging workflow ∧ human-AI collaborative validation",
        principle: Some(Principle {
            name: "Highlight Validity Principle",
            lines: lines![
                ("≡", "Every highlight warrants entity tagging"),
                ("∂", ["¬dismissing bibliographic content", "¬dismissing metadata"]),
                ("□", "Extraction occurred → entities should be detected"),
            ],
        }),
        steps: vec![
            WorkflowStep {
                symbol: "○₁",
                name: "Query untagged",
                lines: lines![
                    ("≝", "highlights requiring entity tags"),
                    ("◩", "`rwq --untagged --sample 10 --format validation`"),
                    ("◻", "batch size 10 (LLM context ∧ precision ∧ human review session)"),
                    ("⊨", "output: {highlight_id, text, author, work_title, tags}"),
                    ("⊸", "count=0 → `rwq --partial --sample-random 10 --format validation`"),
                    ("∂", "¬manual selection ∧ ¬cherry-picking"),
                ],
            },
            WorkflowStep {
                symbol: "○₂",
                name: "Compound citations",
                lines: lines![
                    ("≝", "bibliographic attribution ∧ entity composition"),
                    ("◻", [
                        "⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒ books",
                        "⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒ articles",
                        "□ main title only ∧ ¬subtitle ∧ ¬publisher ∧ ¬edition",
                        "□ co-authors: ⚘⊙⊳⦑Author1∧Author2⦒⦑Year⦒⦑Title⦒",
                        "□ partial when work unknown: ⚘⊙⦑Author⦒⦑Year⦒",
                    ]),
                    ("⊛", [
                        "component tags (always include with compound):",
                        "⚘⦑Author⦒ standalone person tag",
                        "⊙⦑YYYY-MM⦒ when month specified in text",
                        "𝄏⦑Journal⦒ for article citation context",
                        "⍓⦑Publisher⦒ when visible",
                    ]),
                    ("⊛", [
                        "works mentioned in discourse:",
                        "italic *Title* → create compound ⚘⊙⊳ when author∧year knowable",
                        "∂ standalone ⊳⦑Work⦒ (must have compound form)",
                    ]),
                ],
            },
            WorkflowStep {
                symbol: "○₃",
                name: "Basic entities",
                lines: lines![
                    ("≝", "foundational entity detection"),
                    ("⊛", [
                        "⚘⦑Name⦒ geographic disambiguation when needed",
                        "⊙⦑YYYY|YYYY-MM|YYYY-MM-DD⦒ numeric ∧ standalone when MM|DD present",
                        "⎚⦑ISBN|DOI|ORCID⦒ identifiers",
                        "⌖⦑Location⦒ literal spatial ∂metonymic",
                        "⍓⦑Publisher⦒ ∧ ⍢⦑University⦒",
                        "⍚⦑Organization⦒ ∧ ⎈⦑Agency⦒ (governmental authority)",
                        "❖⦑Field⦒ institutional discipline",
                        "⧈⦑Group⦒ human classification ∧ occupational categories",
                    ]),
                ],
            },
            WorkflowStep {
                symbol: "○₄",
                name: "Complex entities",
                lines: lines![
                    ("≝", "sophisticated patterns ∧ relational structures"),
                    ("◻", "precision≫pollution ∧ verify against existing tags"),
                    ("⊛", [
                        "⧖⦑Period⦒ iff □4{named,bounded,geographic,citable}",
                        "⌁⦑Event⦒ discrete occurrence ∧ specific date",
                        "⧊⦑Concept⦒ cross-domain abstraction ∂occupational→⧈",
                        "⧏⦑Method⦒ systematic procedure",
                        "⌯⦑Movement⦒ sustained mobilization",
                    ]),
                    ("⊛", [
                        "relational (verify ⇋◬⟴𝄎⬢⥅ against existing):",
                        "⇋⦑Relation⦒ tripartite entrainment ◻3 elements",
                        "⧆⦑Tension⦒ binary productive opposition",
                        "⟗⦑Loop⦒ bidirectional feedback",
                        "☯⦑Paradox⦒ simultaneous contradiction",
                        "⧃⦑Evolution⦒ unidirectional transformation",
                    ]),
                    ("⊛", [
                        "process/effect:",
                        "⬢⦑Effect⦒ performative reality t₀=t₀",
                        "⥅⦑Revelation⦒ feedforward t₀→t₁",
                        "⟴⦑Action⦒ Burkean symbolic action",
                    ]),
                    ("∂", "¬bulk detection without validation"),
                ],
            },
            WorkflowStep {
                symbol: "○₅",
                name: "Generate JSON",
                lines: lines![
                    ("≝", "proposed_tags mode array"),
                    ("◩", "`mcq --schema batch_proposed_tags` → /tmp/tagging_batch.json"),
                    ("◻", "array format ∧ \"proposed_tags\" mode ∧ separate elements"),
                    ("∂", "¬highlight text in JSON (DB fetch)"),
                ],
            },
            WorkflowStep {
                symbol: "○₆",
                name: "Interactive validation",
                lines: lines![
                    ("≝", "human approval via TUI"),
                    ("◩", "`mcq -i /tmp/tagging_batch.json`"),
                    ("◻", "¬pipe mcq output (breaks TUI)"),
                    ("⊨", "approve → API+DB ∧ human_validated=1 (metadata → machine_validated=1)"),
                    ("∂", "¬manual inscribe (mcq applies approved tags automatically)"),
                    ("⊸", "⑀⦑Partial Tagging⦒ ∈ existing → `amanuensis inscribe by-id <ID> --remove ⑀⦑Partial Tagging⦒`"),
                ],
            },
            WorkflowStep {
                symbol: "○₇",
                name: "Repeat",
                lines: lines![
                    ("→", "○₁"),
                    ("◇", "until user terminates"),
                ],
            },
        ],
        validations: vec![
            ValidationFormula {
                symbol: "⥅",
                rules: vec![
                    DispatchRule { condition: "∂pre-configures", result: "∅ ∨ ⬢/⟗" },
                    DispatchRule { condition: "performative", result: "⬢" },
                    DispatchRule { condition: "∂AT-event", result: "⬢" },
                    DispatchRule { condition: "duplicate", result: "consolidate" },
                    DispatchRule { condition: "else", result: "⥅" },
                ],
            },
            ValidationFormula {
                symbol: "◬",
                rules: vec![
                    DispatchRule { condition: "designed∨planned", result: "∅" },
                    DispatchRule { condition: "∂spontaneous", result: "∅" },
                    DispatchRule { condition: "∂unpredictable", result: "∅" },
                    DispatchRule { condition: "individual", result: "∅" },
                    DispatchRule { condition: "∂organic-growth", result: "∅" },
                    DispatchRule { condition: "else", result: "◬" },
                ],
            },
            ValidationFormula {
                symbol: "⟗",
                rules: vec![
                    DispatchRule { condition: "∂feedback", result: "∅" },
                    DispatchRule { condition: "∂awareness-Δ-behavior", result: "∅" },
                    DispatchRule { condition: "captured-by-⇋", result: "remove" },
                    DispatchRule { condition: "unidirectional", result: "⥅" },
                    DispatchRule { condition: "else", result: "⟗" },
                ],
            },
            ValidationFormula {
                symbol: "⬢",
                rules: vec![
                    DispatchRule { condition: "self-fulfilling", result: "⟚" },
                    DispatchRule { condition: "∂creates-reality", result: "remove" },
                    DispatchRule { condition: "feedforward", result: "⥅" },
                    DispatchRule { condition: "spontaneous", result: "◬" },
                    DispatchRule { condition: "duplicate", result: "consolidate" },
                    DispatchRule { condition: "else", result: "⬢" },
                ],
            },
            ValidationFormula {
                symbol: "⧆",
                rules: vec![
                    DispatchRule { condition: "∂competing", result: "remove" },
                    DispatchRule { condition: "entrainment", result: "⇋" },
                    DispatchRule { condition: "A∧¬A", result: "☯" },
                    DispatchRule { condition: "dichotomy→evolution", result: "⧆∧⧃" },
                    DispatchRule { condition: "∂A-vs-B-format", result: "reformat" },
                    DispatchRule { condition: "else", result: "⧆" },
                ],
            },
        ],
        discriminations: vec![
            DiscriminationRef {
                symbols: &["⧖", "⊙", "⌁"],
                description: "temporal",
            },
            DiscriminationRef {
                symbols: &["⎈", "⍚", "⧈", "⌯"],
                description: "institutional",
            },
            DiscriminationRef {
                symbols: &["⍓", "⍢"],
                description: "specialized orgs",
            },
            DiscriminationRef {
                symbols: &["⧊", "❖", "⧏", "⌬"],
                description: "conceptual",
            },
            DiscriminationRef {
                symbols: &["⧈", "⚘"],
                description: "human categories",
            },
            DiscriminationRef {
                symbols: &["⇋", "⧆", "⟗", "☯"],
                description: "relational",
            },
            DiscriminationRef {
                symbols: &["⧃", "⬢", "⟴"],
                description: "process/effect",
            },
            DiscriminationRef {
                symbols: &["𝄏", "⊳"],
                description: "publications",
            },
            DiscriminationRef {
                symbols: &["⎚"],
                description: "identifiers",
            },
            DiscriminationRef {
                symbols: &["⌖"],
                description: "spatial",
            },
            DiscriminationRef {
                symbols: &["⧩"],
                description: "language",
            },
        ],
    }
}
