//! ⎚ Identifier entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type IdentifierDef = EntityDef;

pub fn get_entity_definitions() -> Vec<IdentifierDef> {
    vec![
        IdentifierDef {
            symbol: "⎚",
            name: "Identifier",
            description: "Standardized persistent identifier with external authority (ISBN, DOI, ORCID)",
            sort_order: 11,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "unique identifier ∧ persistent reference"),
                ("≝", "standardized identification systems ∧ canonical entity references ∧ deduplication keys"),
                ("∂", "⎚⊅{⧊concepts about identity,⧏methods of identification,mere names without standard}"),
                ("◻", [
                    "◻{standardized format,persistent uniqueness,external authority|canonical registry}"
                ]),
                ("⊛", "⎚⦑ISBN 9780691059082|DOI 10.1126/science.123456|ORCID 0000-0002-1234-5678⦒"),
                ("◻", [
                    "Standard identifier ≫ arbitrary label:",
                    "  bibliographic{⎚⦑ISBN 9780691059082|ISSN 0036-8075|LCCN 2012345678⦒}",
                    "  research{⎚⦑DOI 10.1126/science.123456|ArXiv 2301.12345|PubMed 12345678⦒}",
                    "  personal{⎚⦑ORCID 0000-0002-1234-5678|ResearcherID A-1234-2012⦒}",
                    "  institutional{⎚⦑ROR 05dxps055|ISNI 0000000121032683⦒}",
                    "Identifier vs concept identity→⎚|notion→⧊:",
                    "  ⎚⦑ISBN 9780691059082⦒(specific book instance identifier)",
                    "  ⧊⦑Identity⦒(philosophical/conceptual notion)",
                    "  ∂conceptual discussions of identity→⧊",
                    "  persistent reference criterion(must have external standard)",
                    "Identifier vs naming system→⎚|framework→⧏:",
                    "  ⎚⦑DOI 10.1126/science.123456⦒(specific persistent identifier)",
                    "  ⧏⦑Digital Object Identifier (DOI) System⦒(framework/methodology)",
                    "  ∂identification systems as frameworks→⧏",
                    "  standardization discriminator"
                ]),
                ("≟", [
                    "concept about IDs→⧊",
                    "naming system→⧏",
                    "organizational ID (without standard)→metadata ∂ERIS",
                    "test{external standardizing authority?→YES:⎚|NO:review}",
                    "test{persistent unique reference?→YES:⎚|NO:review}"
                ]),
                ("⊨", "⎚⊂standardized identifiers ∧ deduplication protocol ∧ ⎚→canonical reference"),
            ],
        },
    ]
}
