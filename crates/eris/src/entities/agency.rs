//! ⎈ Governmental Authority entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type GovernmentalAuthorityDef = EntityDef;

pub fn get_entity_definitions() -> Vec<GovernmentalAuthorityDef> {
    vec![
        GovernmentalAuthorityDef {
            symbol: "⎈",
            name: "Governmental Authority",
            description: "State entity with enforcement power and sovereign or delegated authority",
            sort_order: 9,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["state_entity", "enforcement_power", "sovereign_authority"]),
                ("≝", "sovereign or delegated authority ∧ public sector ∧ enforcement capacity"),
                ("∂", [
                    "⎈⊅⍚ (⎈ ⛫_enforcement | ⍚ ⛫_coordination)",
                    "⎈⊅⚐ (⎈ holds∧exercises power | ⚐ contests for power)",
                    "⎈⊅⧈ (⎈ institution | ⧈ faction/supporters)",
                    "⎈⊅⧊ (⎈ specific entity | ⧊ power concept)"
                ]),
                ("⊡", [
                    "typical: Փ9Ֆ8Հ7Ղ8",
                    "regulatory (delegated): Փ8Ֆ9Հ6Ղ6",
                    "legislative (sovereign): Փ9Ֆ9Հ8Ղ9",
                    "supranational (coordinated): Փ8Ֆ7Հ7Ղ7"
                ]),
                ("⊛", [
                    "legislative: ⎈⦑U.S. Congress|Parliament of the United Kingdom|Bundestag|National People's Congress|Knesset|Diet of Japan⦒",
                    "judicial: ⎈⦑Supreme Court (United States)|European Court of Justice|Constitutional Court of Germany|High Court of Australia⦒",
                    "regulatory: ⎈⦑EPA|FDA|FCC|SEC|Ofcom|BaFin|CNIL⦒",
                    "military: ⎈⦑United States Army|People's Liberation Army|NATO|IDF|Bundeswehr⦒",
                    "monetary: ⎈⦑Federal Reserve|ECB|Bank of England|People's Bank of China|Bank of Japan|Reserve Bank of India⦒",
                    "supranational: ⎈⦑European Union|WTO|ICC|European Court of Human Rights|African Union Commission⦒",
                    "ancient: ⎈⦑Athenian Assembly|Roman Senate|Spartan Council⦒"
                ]),
                ("◻", [
                    "enforcement_authority discriminator:",
                    "  statutory/regulatory power→⎈",
                    "  monetary policy∧regulatory authority→⎈",
                    "  military command→⎈",
                    "  legislative authority→⎈",
                    "  judicial authority→⎈",
                    "  supranational∧can_compel→⎈",
                    "  advisory only→⍚"
                ]),
                ("⧠", [
                    "'European Powers'→⧊ (concept) | 'European Union'→⎈ (entity)",
                    "'the state'→⧊ (abstract) | 'the French state'→⎈ (polity)",
                    "'Senate Republicans'→⧈ (faction) | 'U.S. Senate'→⎈ (institution)"
                ]),
                ("≟", [
                    "test{specific entity?→continue|power concept?→⧊}",
                    "test{enforcement/regulatory power?→⎈|advisory only?→⍚}",
                    "test{public sector?→continue|private?→⍚}",
                    "test{institution?→⎈|faction/caucus?→⧈}",
                    "test{contests elections?→⚐|governs?→⎈} (both valid)",
                    "edge_cases:",
                    "  international: ⎈⦑WTO|ICC⦒ (can_compel) vs ⍚⦑UN|WHO⦒ (facilitates)",
                    "  party∧government: ⚐⦑Labour Party (UK)⦒ contests ∧ ⎈⦑UK Government⦒ governs"
                ]),
                ("⊨", "⎈ ≡ ⛫_enforcement ∧ sovereign∨delegated_authority ∧ public_sector")
            ],
        },
    ]
}