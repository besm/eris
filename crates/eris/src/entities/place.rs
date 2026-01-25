//! ⌖ Place entity type

define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

type PlaceDef = EntityDef;

pub fn get_entity_definitions() -> Vec<PlaceDef> {
    vec![
        PlaceDef {
            symbol: "⌖",
            name: "Place",
            description: "Geographic entity with spatial extent and physical location",
            sort_order: 2,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", ["geographic_entity", "spatial_extent", "physical_location"]),
                ("≝", "literal spatial referent ∧ geographic specificity ∧ physical boundaries"),
                ("∂", [
                    "⌖⊅⧊ (⌖ literal place | ⧊ metonymic system)",
                    "⌖⊅⍚ (⌖ literal place | ⍚ metonymic institution)",
                    "⌖⊅⧖ (⌖ literal place | ⧖ metonymic era)",
                    "⌖⊅⌁ (⌖ place | ⌁ event at place)",
                    "⌖⊅⍢ (⌖ city | ⍢ university there)",
                    "⌖⊅⚘ (⌖ standalone | ¬separate when ∈⚘ name)"
                ]),
                ("⊡", [
                    "⌖: Շ variable (parochial→universal)",
                    "continent: Շ9",
                    "nation: Շ7",
                    "city: Շ5",
                    "neighborhood: Շ3"
                ]),
                ("⊛", [
                    "continents: ⌖⦑Africa|Asia|Europe|North America|South America|Antarctica⦒",
                    "nations: ⌖⦑France|Japan|Brazil|Nigeria|India|Egypt|Mexico⦒",
                    "regions: ⌖⦑Midwest|Patagonia|Siberia|Sahel|Levant|Maghreb|Balkans⦒",
                    "cities: ⌖⦑Rome|Tokyo|Lagos|São Paulo|Mumbai|Cairo|Berlin|Istanbul⦒",
                    "features: ⌖⦑Gulf Stream|Nile River|Himalayas|Amazon Basin|Great Rift Valley⦒",
                    "historical: ⌖⦑Troy|Carthage|Constantinople|Tenochtitlan|Timbuktu⦒",
                    "cultural: ⌖⦑Silicon Valley|Rust Belt|City of London|Shenzhen⦒",
                    "local: ⌖⦑Watts, Los Angeles|Kreuzberg, Berlin|Dharavi, Mumbai⦒",
                    "ambiguous: ⌖⦑Cambridge|Cambridge, Massachusetts|Athens|Athens, Georgia|Alexandria|Alexandria, Virginia⦒"
                ]),
                ("◻", [
                    "naming:",
                    "  ✓⌖⦑[Simple Name]⦒: ⌖⦑Rome|Tokyo|Siberia⦒",
                    "  ✓⌖⦑[City, Region]⦒ when disambiguation needed: ⌖⦑Cambridge, Massachusetts|Athens, Georgia⦒",
                    "  ✗⌖⦑Ottawa, Canada⦒ when unambiguous — just ⌖⦑Ottawa⦒",
                    "  ✗separate ⌖ when part of ⚘ name: ⚘⦑Pytheas of Massalia⦒ ¬⌖⦑Massalia⦒",
                    "ambiguous toponyms:",
                    "  Cambridge: ⌖⦑Cambridge⦒ (UK) vs ⌖⦑Cambridge, Massachusetts⦒ (US)",
                    "    ⍢⦑University of Cambridge⦒ — prefer full name for university",
                    "    ⍓⦑Cambridge University Press⦒ — always full name",
                    "    'at Cambridge'→likely ⍢ | 'in Cambridge'→likely ⌖",
                    "  Athens: ⌖⦑Athens⦒ (Greece) vs ⌖⦑Athens, Georgia⦒ (US)",
                    "    ⍢⦑Academy of Athens⦒ — institution ¬⌖",
                    "    ⎈⦑Athenian Assembly⦒ — governmental body ¬⌖",
                    "  Alexandria: ⌖⦑Alexandria⦒ (Egypt) vs ⌖⦑Alexandria, Virginia⦒ (US)",
                    "    ⍢⦑Library of Alexandria⦒ — institution ¬⌖",
                    "  pattern: [Toponym] alone→⌖ unless context indicates ⍢|⍓|⎈",
                    "metonymic discrimination:",
                    "  'in/at [Place]'→likely ⌖",
                    "  '[Place] system/model'→⧊: ⧊⦑Westphalian System⦒ ¬⌖⦑Westphalia⦒",
                    "  '[Place] era'→⧖: ⧖⦑Bretton Woods Era⦒ ¬⌖⦑Bretton Woods⦒",
                    "  '[Place] believes/wants'→⍚: ⍚⦑Wall Street⦒ ¬⌖⦑Wall Street⦒"
                ]),
                ("≟", [
                    "test{literal spatial referent?→YES:⌖|NO:continue}",
                    "test{institution at place?→YES:⍢|⍓|⎈|NO:continue}",
                    "test{metonymic system/model?→YES:⧊|NO:continue}",
                    "test{metonymic institution?→YES:⍚|NO:continue}",
                    "test{metonymic era?→YES:⧖|NO:continue}",
                    "test{event at place?→YES:⌁|NO:continue}",
                    "test{part of person name?→YES:¬separate ⌖|NO:review}"
                ]),
                ("⊨", "⌖ ≡ literal_spatial_referent ∧ physical_geography")
            ],
        },
    ]
}