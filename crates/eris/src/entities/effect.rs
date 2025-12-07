//! ⬢ Effect entity type

define_entity_module! {
    Entity {
        Complex => "Complex/process entity types",
    }
}

type EffectDef = EntityDef;

pub fn get_entity_definitions() -> Vec<EffectDef> {
    vec![
        EffectDef {
            symbol: "⬢",
            name: "Effect",
            description: "Performative reality-making with instantaneous constitution (t₀=t₀)",
            sort_order: 42,
            category: EntityCategory::Complex,
            lines: lines![
                ("≡", "effect ∧ performative reality-making"),
                ("≝", "classification→reality simultaneity ∧ constitutive power ∧ performative creation ∧ instantaneous constitution"),
                ("∂", [
                    "⬢⊅{⥅temporal gap,◬gradual,⟗cyclical,⧃transformation sequence}",
                    "⬢⊅⧊_effect_concepts (analyzing performativity ≠ describing specific effect)",
                    "◻{simultaneity t₀=t₀,performative power}",
                    "∂¬descriptive"
                ]),
                ("⊛", "⬢⦑Making Up People|Classification Creates Order|Ritual Creates Experience|Performativity|Bandwagon Effect⦒"),
                ("⧠", [
                    "⬢⦑Making Up People⦒: classification constitutes classified (simultaneity, 'classified AS X makes one X')",
                    "⟗⦑Multiple Personality Formation Loop⦒: same mechanism but visible iteration 1983→1991 ('fed back', cycles)",
                    "⧃⦑Medicalization⦒: condition→medical frame (unidirectional transformation, no constitution)",
                    "⧊⦑Performativity⦒: Butler analyzing the concept (discourse about effect, not specific effect)"
                ]),
                ("⊡", "typical: Պ7Փ5Տ4"),
                ("⊡", "⬢⦑Making Up People⦒: Պ8Փ6Տ3"),
                ("◻", [
                    "\"X Creates Y\" structure dominant|performative gerunds:",
                    "  creation{⬢⦑Classification Creates Order|Ritual Creates Experience|Prediction Creates Reality⦒}",
                    "  gerunds{⬢⦑Classifying Humans|Creating Economic/Noneconomic Distinction⦒}",
                    "  canonical{⬢⦑Making Up People⦒Hacking,⬢⦑Performativity⦒Austin/Butler}",
                    "  descriptive{⬢⦑Categories Creating People|Social Interaction Creates Classification⦒}",
                    "  \"creates\"emphasizes simultaneity(¬\"leads to\"temporal gap)",
                    "Simultaneity requirement CRITICAL:",
                    "  t₀classify=t₀create",
                    "  valid{\"Classified AS X makes one X\"immediate constitution}",
                    "  invalid{\"Classification shapes future behavior\"temporal gap→use ⥅}",
                    "  test{simultaneous?→⬢|precedent?→⥅|delayed?→⧃}",
                    "  ∂ANY temporal gap disqualifies ⬢",
                    "Performative power classification CONSTITUTES¬describes reality:",
                    "  valid{⬢⦑Ritual Creates Experience⦒ritual=reality¬symbol}",
                    "  valid{⬢⦑Statistics Create Reality⦒quantification makes real}",
                    "  invalid{\"Classification reflects reality\"descriptive→∅}",
                    "Quadripartite algorithm:",
                    "  ○₁temporal{t₀=t₀?→⬢|t₀<t₁?→continue|∫?→◬}",
                    "  ○₂directionality{X→Y?→⥅∨⬢|X↔Y?→⟗}",
                    "  ○₃spontaneity{designed?→⥅∨⬢|organic?→◬}",
                    "  ○₄iteration{single?→⬢∨⥅|repeated?→⟗}"
                ]),
                ("≟", [
                    "○₁ discourse check: describing specific effect?→continue | analyzing effect concept?→⧊",
                    "○₂ \"Classified as X makes one X\"→⬢",
                    "○₃ \"X shapes before Y occurs\"→⥅",
                    "○₄ \"X emerges from interactions\"→◬",
                    "○₅ \"X cycles through feedback\"→⟗"
                ]),
                ("⊨", "⬢⊂performative effects ∧ instantaneous constitution ∧ classification→reality ∧ simultaneity t₀=t₀ ∧ ⬢⊅{temporal gap,descriptive,gradual,cyclical}"),
            ],
        },
    ]
}