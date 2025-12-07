//! ⟴ Action entity type

define_entity_module! {
    Entity {
        Complex => "Complex/process entity types",
    }
}

type ActionDef = EntityDef;

pub fn get_entity_definitions() -> Vec<ActionDef> {
    vec![
        ActionDef {
            symbol: "⟴",
            name: "Symbolic Action",
            description: "Burkean symbolic action with meaningful dimension and human agency",
            sort_order: 41,
            category: EntityCategory::Complex,
            lines: lines![
                ("≡", ["symbolic_action", "meaningful_intervention"]),
                ("≝", ["Burkean action ∧ rhetorical performance", "agency-requiring deed"]),
                ("∂", [
                    "⟴⊅{mechanical, passive, automatic}",
                    "⟴⊅⬢ (⟴ performs | ⬢ constitutes)",
                    "⟴⊅⥅ (⟴ acts | ⥅ anticipates)",
                    "⟴⊅◬ (⟴ agentive | ◬ emergent)"
                ]),
                ("◻", "symbolic_dimension ∧ human_agency"),
                ("⊛", [
                    "gerunds: ⟴⦑Branding⦒ ∧ ⟴⦑Ordination⦒",
                    "compounds: ⟴⦑Symbolic Enactment⦒ ∧ ⟴⦑Rhetorical Performance⦒",
                    "X_as_Y: ⟴⦑Definition as Symbolic Act⦒ ∧ ⟴⦑Taboo as Symbolic Action⦒",
                    "functional: ⟴⦑Priest-Prophet Function⦒",
                    "⚘⦑Kenneth Burke⦒ Dramatism ∧ pentadic_analysis"
                ]),
                ("≟", [
                    "meaningful∧interpretive?→continue | mechanical?→∅",
                    "human∧social_agent?→continue | automatic?→◬_if_emergent",
                    "\"X performs Y\"→⟴",
                    "\"X creates Y\"→⬢",
                    "\"X reveals Y\"→⥅",
                    "\"X emerges\"→◬"
                ]),
                ("⊨", "⟴⊂symbolic_actions ∧ human_agency ∧ ⟴⊅{mechanical,automatic}"),
            ],
        },
    ]
}
