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
                ("≡", ["state entity", "enforcement power"]),
                ("≝", ["sovereign or delegated authority", "public sector", "enforcement capacity"]),
                ("∂", [
                    "⎈⊅{⍚ non-governmental, ⧈ human categories}",
                    "⎈⊅⧊_power_concepts (power relations ≠ power-wielding entities)",
                    "specific entity→⎈ | abstract power concept→⧊"
                ]),
                ("⊡", [
                    "typical: Փ9Ֆ8Հ7Ղ8",
                    "⎈⦑EPA⦒: Փ8Ֆ9Հ6Ղ6 (regulatory, delegated)",
                    "⎈⦑U.S. Congress⦒: Փ9Ֆ9Հ8Ղ9 (legislative, sovereign)",
                    "⎈⦑NATO⦒: Փ8Ֆ7Հ7Ղ7 (supranational, coordinated)"
                ]),
                ("⊛", [
                    "⎈⦑Congress|EPA|Federal Reserve|Supreme Court⦒",
                    "⎈⦑United States Army|NATO|European Union⦒"
                ]),
                ("⧠", [
                    "'European Powers'→⧊ (concept about power relations)",
                    "'European Union'→⎈ (specific governmental entity)",
                    "'Great Powers'→⧊ (IR theory concept)",
                    "'UN Security Council'→⎈ (specific body)",
                    "'the state'→⧊ (abstract)",
                    "'the French state'→⎈ (specific polity)"
                ]),
                ("◻", [
                    "enforcement authority ◻□discriminator:",
                    "  statutory/regulatory: {⎈⦑EPA|FDA|FCC|SEC⦒}",
                    "  monetary: {⎈⦑Federal Reserve⦒} (regulatory powers required)",
                    "  military: {⎈⦑United States Army|NATO⦒}",
                    "  legislative: {⎈⦑Congress|Parliament⦒}",
                    "  judicial: {⎈⦑Supreme Court⦒}",
                    "  supranational: {⎈⦑European Union|OECD⦒}",
                    "vs ⍚: test{enforcement power?→⎈ | advisory only?→⍚}",
                    "  ✓⎈⦑Federal Reserve⦒ ✗⍚⦑National Academy of Sciences⦒",
                    "vs ⧈: test{institution?→⎈ | partisan faction?→⧈}",
                    "  ✓⎈⦑U.S. Senate⦒ ✗⧈⦑Senate Republicans⦒",
                    "vs ⧊: test{specific entity?→⎈ | power concept?→⧊}"
                ]),
                ("≟", [
                    "○₁ specificity: specific entity?→continue | power concept?→⧊",
                    "○₂ enforcement: statutory/regulatory power?→⎈ | advisory?→⍚",
                    "○₃ governmental: public sector?→continue | private?→⍚",
                    "○₄ human: institution?→⎈ | faction/caucus?→⧈",
                    "party_in_government: ⚐ contests, ⎈ governs — both valid, context determines"
                ]),
                ("⊨", "⎈⊂governmental authority ∧ enforcement power ∧ ⎈⊂⍚")
            ],
        },
    ]
}