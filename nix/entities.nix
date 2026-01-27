# ERIS Entity Definitions
# Generated from Rust definitions - do not edit manually
{
  person = {
    symbol = "⚘";
    name = "Person";
    description = "Named individual, human agent, biographical subject";
    category = "Primary";
    sortOrder = 1;
    lines = [
      { prefix = "≡"; content = "named_individual"; }
      { prefix = "≡"; content = "human_agent"; }
      { prefix = "≡"; content = "biographical_subject"; }
      { prefix = "≝"; content = "historical attestation ∧ proper name ∧ attributed author"; }
      { prefix = "∂"; content = "⚘⊅⧈ (⚘ individual | ⧈ category of people)"; }
      { prefix = "∂"; content = "⚘⊅⧊ (⚘ person | ⧊ role/concept)"; }
      { prefix = "∂"; content = "⚘⊅fictional (⚘ attested | fictional→∅)"; }
      { prefix = "⊛"; content = "⚘⦑C.S. Peirce|W.E.B. Du Bois|J.L. Austin|Hannah Arendt|Simone de Beauvoir⦒"; }
      { prefix = "⊛"; content = "⚘⦑Karl Marx|Max Weber|Émile Durkheim|Michel Foucault|Pierre Bourdieu⦒"; }
      { prefix = "⊛"; content = "⚘⦑Ibn Khaldun|Frantz Fanon|Gayatri Spivak|Edward Said|Amartya Sen⦒"; }
      { prefix = "⊛"; content = "⚘⦑Confucius|Mozi|Zhuangzi|Xuanzang|Wang Yangming⦒"; }
      { prefix = "⊛"; content = "⚘⦑Pytheas of Massalia|Hypatia of Alexandria|Avicenna|Maimonides⦒"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓⚘⦑J.L. Austin|W.E.B. Du Bois|C.S. Peirce⦒ — no space between initials"; }
      { prefix = "◻"; content = "  ✗⚘⦑J. L. Austin|W. E. B. Du Bois|C. S. Peirce⦒ — spaces between initials"; }
      { prefix = "◻"; content = "  toponym: ⚘⦑Pytheas of Massalia⦒ — complete_id ¬separate ⌖"; }
      { prefix = "◻"; content = "  ✗⚘⦑Person (Disambiguation)⦒ — no parenthetical"; }
      { prefix = "◻"; content = "  co-authors: ⚘⦑Author1∧Author2⦒"; }
      { prefix = "◻"; content = "compound citations:"; }
      { prefix = "◻"; content = "  ⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒ (book)"; }
      { prefix = "◻"; content = "  ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒ (article)"; }
      { prefix = "≟"; content = "test{named individual?→YES:⚘|NO:continue}"; }
      { prefix = "≟"; content = "test{'the [occupation]'?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{collective/category?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{role/concept?→YES:⧊|NO:continue}"; }
      { prefix = "≟"; content = "test{fictional?→YES:∅|NO:review}"; }
      { prefix = "⊨"; content = "⚘ ≡ historically_attested ∧ named_individual"; }
    ];
  };
  place = {
    symbol = "⌖";
    name = "Place";
    description = "Geographic entity with spatial extent and physical location";
    category = "Primary";
    sortOrder = 2;
    lines = [
      { prefix = "≡"; content = "geographic_entity"; }
      { prefix = "≡"; content = "spatial_extent"; }
      { prefix = "≡"; content = "physical_location"; }
      { prefix = "≝"; content = "literal spatial referent ∧ geographic specificity ∧ physical boundaries"; }
      { prefix = "∂"; content = "⌖⊅⧊ (⌖ literal | ⧊ metonymic system)"; }
      { prefix = "∂"; content = "⌖⊅⍚ (⌖ literal | ⍚ metonymic institution)"; }
      { prefix = "∂"; content = "⌖⊅⧖ (⌖ literal | ⧖ metonymic era)"; }
      { prefix = "∂"; content = "⌖⊅⌁ (⌖ place | ⌁ event at place)"; }
      { prefix = "∂"; content = "⌖⊅⍢ (⌖ city | ⍢ university there)"; }
      { prefix = "∂"; content = "⌖⊅⚘ (⌖ standalone | ¬separate when ∈⚘ name)"; }
      { prefix = "⊡"; content = "Շ variable: continent Շ9 | nation Շ7 | city Շ5 | neighborhood Շ3"; }
      { prefix = "⊛"; content = "⌖⦑Africa|Asia|Europe|North America|South America|Antarctica⦒"; }
      { prefix = "⊛"; content = "⌖⦑France|Japan|Brazil|Nigeria|India|Egypt|Mexico|Indonesia⦒"; }
      { prefix = "⊛"; content = "⌖⦑Midwest|Patagonia|Siberia|Sahel|Levant|Maghreb|Balkans|Punjab⦒"; }
      { prefix = "⊛"; content = "⌖⦑Rome|Tokyo|Lagos|São Paulo|Mumbai|Cairo|Berlin|Istanbul|Nairobi⦒"; }
      { prefix = "⊛"; content = "⌖⦑Gulf Stream|Nile River|Himalayas|Amazon Basin|Great Rift Valley⦒"; }
      { prefix = "⊛"; content = "⌖⦑Troy|Carthage|Constantinople|Tenochtitlan|Timbuktu|Angkor⦒"; }
      { prefix = "⊛"; content = "⌖⦑Silicon Valley|Rust Belt|City of London|Shenzhen|Bangalore⦒"; }
      { prefix = "⊛"; content = "⌖⦑Cambridge|Cambridge, Massachusetts|Athens|Athens, Georgia|Alexandria|Alexandria, Virginia⦒"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓⌖⦑Simple⦒|⌖⦑City, Region⦒ for disambiguation"; }
      { prefix = "◻"; content = "  ✗⌖⦑City, Country⦒ when unambiguous"; }
      { prefix = "◻"; content = "  ✗⌖ when ∈⚘ toponym"; }
      { prefix = "◻"; content = "ambiguous:"; }
      { prefix = "◻"; content = "  ⌖⦑Cambridge|Athens|Alexandria⦒ default: UK|Greece|Egypt"; }
      { prefix = "◻"; content = "  ⌖⦑Cambridge, Massachusetts|Athens, Georgia|Alexandria, Virginia⦒ for US"; }
      { prefix = "◻"; content = "  'at [X]'→likely ⍢ | 'in [X]'→likely ⌖"; }
      { prefix = "◻"; content = "  [Toponym] alone→⌖ unless ⍢|⍓|⎈ context"; }
      { prefix = "◻"; content = "metonymic:"; }
      { prefix = "◻"; content = "  '[X] system'→⧊ | '[X] era'→⧖ | '[X] believes'→⍚"; }
      { prefix = "≟"; content = "test{literal spatial?→YES:⌖|NO:continue}"; }
      { prefix = "≟"; content = "test{institution at place?→YES:⍢|⍓|⎈|NO:continue}"; }
      { prefix = "≟"; content = "test{metonymic?→YES:⧊|⍚|⧖|NO:continue}"; }
      { prefix = "≟"; content = "test{event at place?→YES:⌁|NO:continue}"; }
      { prefix = "≟"; content = "test{∈⚘ name?→YES:¬separate|NO:review}"; }
      { prefix = "⊨"; content = "⌖ ≡ literal_spatial_referent ∧ physical_geography"; }
    ];
  };
  era = {
    symbol = "⧖";
    name = "Era";
    description = "Named temporal span with historiographic legitimacy";
    category = "Primary";
    sortOrder = 3;
    lines = [
      { prefix = "≡"; content = "named_temporal_span"; }
      { prefix = "≡"; content = "historiographic_period"; }
      { prefix = "≡"; content = "bounded_duration"; }
      { prefix = "≝"; content = "scholarly legitimacy ∧ □{named,bounded,geographic,citable}"; }
      { prefix = "∂"; content = "⧖⊅⊙ (⧖ span | ⊙ point)"; }
      { prefix = "∂"; content = "⧖⊅⌁ (⧖ sustained | ⌁ discrete event)"; }
      { prefix = "∂"; content = "⧖⊅⌖ (⧖ temporal | ⌖ metonymic place→era)"; }
      { prefix = "⊡"; content = "⧖: Շ variable | Ֆ variable"; }
      { prefix = "⊡"; content = "century: Շ8Ֆ7 | decade: Շ7Ֆ5 | named era: Շ6Ֆ8"; }
      { prefix = "⊛"; content = "centuries: ⧖⦑Eighteenth Century|Nineteenth Century|Early Twentieth Century⦒"; }
      { prefix = "⊛"; content = "decades: ⧖⦑1920s|1960s|1970s|1980s|The Sixties|The Twenties⦒"; }
      { prefix = "⊛"; content = "named: ⧖⦑Victorian Era|Industrial Era|Gilded Age|Jazz Age|Meiji Era|Qing Dynasty⦒"; }
      { prefix = "⊛"; content = "periods: ⧖⦑Renaissance|Enlightenment|Romantic Period|Hellenistic Period|Warring States Period⦒"; }
      { prefix = "⊛"; content = "regional: ⧖⦑Italian Renaissance|Northern Renaissance|Weimar Republic|Tokugawa Period⦒"; }
      { prefix = "⊛"; content = "modern: ⧖⦑Cold War|Post-World War II Era|Interwar Period|Space Age⦒"; }
      { prefix = "⊛"; content = "ancient: ⧖⦑Archaic Greece|Classical Antiquity|Pre-Socratic Period|Bronze Age⦒"; }
      { prefix = "⊛"; content = "wars-as-eras: ⧖⦑World War I|World War II|Hundred Years' War|Thirty Years' War⦒"; }
      { prefix = "◻"; content = "4 criteria □∀⧖: named ∧ bounded ∧ geographic ∧ citable"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓⧖⦑Eighteenth Century⦒ | ✗⧖⦑18th Century|19th C.⦒"; }
      { prefix = "◻"; content = "  ✓⧖⦑1970s⦒ numeric | ✓⧖⦑The Sixties⦒ written"; }
      { prefix = "◻"; content = "  modifiers: Early|Mid-|Late"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  Δt>1yr∧sustained→⧖ | Δt<1yr∧discrete→⌁ | point→⊙"; }
      { prefix = "◻"; content = "  ⧖⦑World War II⦒ (era) vs ⌁⦑D-Day⦒ (event within)"; }
      { prefix = "◻"; content = "  ⧖⦑Bretton Woods Era⦒ vs ⌖⦑Bretton Woods⦒ (place)"; }
      { prefix = "≟"; content = "test{Δt>1yr∧sustained?→YES:⧖|NO:continue}"; }
      { prefix = "≟"; content = "test{Δt<1yr∧discrete?→YES:⌁|NO:continue}"; }
      { prefix = "≟"; content = "test{temporal point?→YES:⊙|NO:continue}"; }
      { prefix = "≟"; content = "test{unbounded?→YES:∅|NO:review}"; }
      { prefix = "⊨"; content = "⧖ ≡ historiographic_period ∧ □{named,bounded,geographic,citable}"; }
    ];
  };
  date = {
    symbol = "⊙";
    name = "Date";
    description = "Temporal point, publication metadata (YYYY, YYYY-MM, YYYY-MM-DD)";
    category = "Primary";
    sortOrder = 4;
    lines = [
      { prefix = "≡"; content = "temporal_point"; }
      { prefix = "≡"; content = "publication_metadata"; }
      { prefix = "≡"; content = "citation_component"; }
      { prefix = "≝"; content = "date specificity ∧ bibliographic marker ∧ numeric format"; }
      { prefix = "∂"; content = "⊙⊅⧖ (⊙ point | ⧖ named span)"; }
      { prefix = "∂"; content = "⊙⊅⌁ (⊙ date | ⌁ event — ¬separate ⊙ for events)"; }
      { prefix = "⊛"; content = "year: ⊙⦑1809|1848|1945|1968|1989|2001|2024⦒"; }
      { prefix = "⊛"; content = "month: ⊙⦑1848-03|1968-05|2024-11⦒"; }
      { prefix = "⊛"; content = "day: ⊙⦑1776-07-04|1945-08-06|1989-11-09|2024-03-15⦒"; }
      { prefix = "◻"; content = "format: YYYY|YYYY-MM|YYYY-MM-DD"; }
      { prefix = "◻"; content = "  ✓⊙⦑1848|1848-03|1848-03-15⦒"; }
      { prefix = "◻"; content = "  ✗⊙⦑March 1848|15 March 1848⦒ — numeric only"; }
      { prefix = "◻"; content = "compound citations:"; }
      { prefix = "◻"; content = "  ⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒ (book)"; }
      { prefix = "◻"; content = "  ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒ (article)"; }
      { prefix = "◻"; content = "precision: year default | month for periodicals | day for specific dates"; }
      { prefix = "≟"; content = "test{temporal point?→YES:⊙|NO:continue}"; }
      { prefix = "≟"; content = "test{named period?→YES:⧖|NO:continue}"; }
      { prefix = "≟"; content = "test{event?→YES:⌁ (¬separate ⊙)|NO:review}"; }
      { prefix = "⊨"; content = "⊙ ≡ temporal_point ∧ numeric_format"; }
    ];
  };
  event = {
    symbol = "⌁";
    name = "Event";
    description = "Discrete historical occurrence with bounded causality";
    category = "Primary";
    sortOrder = 5;
    lines = [
      { prefix = "≡"; content = "discrete_occurrence"; }
      { prefix = "≡"; content = "historical_significance"; }
      { prefix = "≡"; content = "bounded_causality"; }
      { prefix = "≝"; content = "specific occurrence ∧ causal unity ∧ ∃t₀happened(event,t₀)"; }
      { prefix = "∂"; content = "⌁⊅⧖ (⌁ discrete | ⧖ sustained)"; }
      { prefix = "∂"; content = "⌁⊅⊙ (⌁ named occurrence | ⊙ bare date)"; }
      { prefix = "∂"; content = "⌁⊅⧊ (⌁ specific occurrence | ⧊ category of occurrences)"; }
      { prefix = "⊡"; content = "⌁: Շ variable | Ֆ variable"; }
      { prefix = "⊡"; content = "battle: Շ4Ֆ5 | revolution: Շ7Ֆ9 | conference: Շ6Ֆ7"; }
      { prefix = "⊛"; content = "revolutions: ⌁⦑French Revolution|Haitian Revolution|Mexican Revolution|Russian Revolution|Iranian Revolution⦒"; }
      { prefix = "⊛"; content = "battles: ⌁⦑Battle of Amorgos|Battle of Plassey|Battle of Stalingrad|Fall of Constantinople⦒"; }
      { prefix = "⊛"; content = "conferences: ⌁⦑Bandung Conference|Congress of Vienna|Yalta Conference|Treaty of Westphalia⦒"; }
      { prefix = "⊛"; content = "crises: ⌁⦑Cuban Missile Crisis|Suez Crisis|Munich Crisis|Black Monday⦒"; }
      { prefix = "⊛"; content = "tests/firsts: ⌁⦑Soviet Atomic Bomb Test|Moon Landing|Sputnik Launch⦒"; }
      { prefix = "⊛"; content = "disasters: ⌁⦑1955 Le Mans Disaster|Sago Mine Disaster|Bhopal Disaster|Chernobyl⦒"; }
      { prefix = "⊛"; content = "political: ⌁⦑Partition of India|Meiji Restoration|Tiananmen Square|Fall of the Berlin Wall⦒"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓⌁⦑[Descriptive Name]⦒ | ✗⌁⦑March 15, 1963⦒ — no bare dates"; }
      { prefix = "◻"; content = "causal unity > temporal length:"; }
      { prefix = "◻"; content = "  ⌁⦑French Revolution⦒ multi-year single causal chain = one event"; }
      { prefix = "◻"; content = "  ⧖⦑Cold War⦒ sustained competition = era"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  'X occurred'→⌁ | 'during X'→⧖"; }
      { prefix = "◻"; content = "  'Car Accidents'→⧊ (category) | '1955 Le Mans Disaster'→⌁ (specific)"; }
      { prefix = "◻"; content = "  ⌁⦑Cuban Missile Crisis⦒ (discrete within ⧖⦑Cold War⦒)"; }
      { prefix = "≟"; content = "test{specific occurrence?→YES:⌁|category?→⧊}"; }
      { prefix = "≟"; content = "test{Δt<1yr∧discrete?→YES:⌁|NO:continue}"; }
      { prefix = "≟"; content = "test{single causal chain?→YES:⌁|sustained competition?→⧖}"; }
      { prefix = "≟"; content = "test{bare date?→YES:⊙|named occurrence?→⌁}"; }
      { prefix = "⊨"; content = "⌁ ≡ discrete_occurrence ∧ causal_unity ∧ ∂start ∧ ∂end"; }
    ];
  };
  field = {
    symbol = "❖";
    name = "Field";
    description = "Academic discipline with institutional markers (departments, degrees, journals)";
    category = "Primary";
    sortOrder = 6;
    lines = [
      { prefix = "≡"; content = "academic_discipline"; }
      { prefix = "≡"; content = "institutionalized_domain"; }
      { prefix = "≡"; content = "knowledge_container"; }
      { prefix = "≝"; content = "institutional boundaries ∧ scholarly legitimacy ∧ ∃{departments|degrees|journals|conferences}"; }
      { prefix = "∂"; content = "❖⊅⧏ (❖ field | ⧏ method within field)"; }
      { prefix = "∂"; content = "❖⊅⧊ (❖ institutional | ⧊ abstract concept)"; }
      { prefix = "⊡"; content = "❖: Ֆ7Շ6Բ7"; }
      { prefix = "⊡"; content = "core: Շ8Բ8 (physics, sociology)"; }
      { prefix = "⊡"; content = "subdiscipline: Շ5Բ6 (quantum mechanics)"; }
      { prefix = "⊡"; content = "interdiscipline: Շ6Բ5 (cognitive science)"; }
      { prefix = "⊛"; content = "core: ❖⦑Physics|Chemistry|Biology|Sociology|Philosophy|History|Economics|Psychology⦒"; }
      { prefix = "⊛"; content = "subdiscipline: ❖⦑Quantum Mechanics|Epistemology|Macroeconomics|Social Psychology⦒"; }
      { prefix = "⊛"; content = "interdiscipline: ❖⦑Cognitive Science|Science and Technology Studies|Computational Linguistics|Bioethics⦒"; }
      { prefix = "⊛"; content = "compound: ❖⦑Philosophy of Language|Sociology of Science|History of Mathematics|Philosophy of Mind⦒"; }
      { prefix = "⊛"; content = "specialized: ❖⦑Mormon History|Classical Philology|Media Theory|Subaltern Studies|Africana Studies⦒"; }
      { prefix = "⊛"; content = "regional: ❖⦑Sinology|Japanology|Indology|Slavic Studies|Latin American Studies⦒"; }
      { prefix = "◻"; content = "institutional markers (one sufficient):"; }
      { prefix = "◻"; content = "  'Department of X' | 'PhD in X' | 'Journal of X' | 'Conference on X'"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓❖⦑[Descriptive Name]⦒ | ✗abbreviations"; }
      { prefix = "◻"; content = "  ✓❖⦑Science and Technology Studies⦒ — unified field ¬separate tags"; }
      { prefix = "◻"; content = "  ✓❖⦑Philosophy of Language⦒ — compound valid"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  'study phenomenology'→❖ | 'apply phenomenological method'→⧏"; }
      { prefix = "◻"; content = "  'epistemology as field'→❖ | 'epistemological question'→⧊"; }
      { prefix = "≟"; content = "test{'Department of X' exists?→YES:❖|NO:continue}"; }
      { prefix = "≟"; content = "test{'Apply X method'?→YES:⧏|NO:continue}"; }
      { prefix = "≟"; content = "test{abstract concept?→YES:⧊|NO:review}"; }
      { prefix = "⊨"; content = "❖ ≡ institutionalized_domain ∧ □academic_legitimacy"; }
    ];
  };
  group = {
    symbol = "⧈";
    name = "Group";
    description = "Human classification (occupational, legal, diagnostic, social categories)";
    category = "Primary";
    sortOrder = 7;
    lines = [
      { prefix = "≡"; content = "human_classification"; }
      { prefix = "≡"; content = "social_category"; }
      { prefix = "≡"; content = "collective_label"; }
      { prefix = "≝"; content = "classificatory system ∧ group identity ∧ people-as-category"; }
      { prefix = "∂"; content = "⧈⊅⧊ (⧈ people | ⧊ abstract property)"; }
      { prefix = "∂"; content = "⧈⊅⚘ (⧈ category | ⚘ individual)"; }
      { prefix = "∂"; content = "⧈⊅⍚ (⧈ category | ⍚ organization)"; }
      { prefix = "∂"; content = "⧈⊅⚐ (⧈ supporters | ⚐ party)"; }
      { prefix = "∂"; content = "⧈⊅⌯ (⧈ adherents | ⌯ movement)"; }
      { prefix = "⊡"; content = "⧈: Շ variable | Ֆ variable (folk→formal)"; }
      { prefix = "⊡"; content = "occupational: Ֆ6Շ7"; }
      { prefix = "⊡"; content = "diagnostic: Ֆ8Շ5"; }
      { prefix = "⊡"; content = "ethnic: Ֆ3Շ6"; }
      { prefix = "⊛"; content = "occupational: ⧈⦑Engineers|Priests|Bureaucrats|Physicians|Programmers|Clerks⦒"; }
      { prefix = "⊛"; content = "legal/political: ⧈⦑Citizens|Metics|Slaves|Subjects|Refugees|Felons⦒"; }
      { prefix = "⊛"; content = "diagnostic: ⧈⦑Patients|Schizophrenics|Autistics|Diabetics⦒"; }
      { prefix = "⊛"; content = "social: ⧈⦑Early Adopters|Elites|Radicals|Intellectuals|Youth⦒"; }
      { prefix = "⊛"; content = "ethnic/cultural: ⧈⦑Hebrews|Bushmen|Puritans|Creoles|Mestizos⦒"; }
      { prefix = "⊛"; content = "partisan: ⧈⦑Democrats|Republicans|Tories|Labourites|Peronistas|Sandinistas⦒"; }
      { prefix = "◻"; content = "emic categories ∧ actors' terms ∂ etic judgments"; }
      { prefix = "◻"; content = "occupational ALWAYS ⧈:"; }
      { prefix = "◻"; content = "  ✓⧈⦑Engineers⦒ | ✗⧊⦑Engineers⦒"; }
      { prefix = "◻"; content = "  'the [occupation]'→⧈ people ≠ concept"; }
      { prefix = "◻"; content = "partisan vs party:"; }
      { prefix = "◻"; content = "  'the Democrats voted'→⧈ (people)"; }
      { prefix = "◻"; content = "  'the Democratic Party nominated'→⚐ (organization)"; }
      { prefix = "◻"; content = "context:"; }
      { prefix = "◻"; content = "  'the engineers'→⧈ | 'engineering mindset'→⧊"; }
      { prefix = "◻"; content = "  'the citizens'→⧈ | 'citizenship'→⧊"; }
      { prefix = "≟"; content = "test{'the [X]' = people?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{occupational?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{diagnostic?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{partisan supporters?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{party organization?→YES:⚐|NO:continue}"; }
      { prefix = "≟"; content = "test{abstract property?→YES:⧊|NO:review}"; }
      { prefix = "⊨"; content = "⧈ ≡ human_classification ∧ people-as-category ∧ occupational⊂⧈"; }
    ];
  };
  organization = {
    symbol = "⍚";
    name = "Organization";
    description = "Non-governmental coordination body with formal institutional structure";
    category = "Primary";
    sortOrder = 8;
    lines = [
      { prefix = "≡"; content = "organization"; }
      { prefix = "≡"; content = "coordinated_group"; }
      { prefix = "≡"; content = "structured_collective"; }
      { prefix = "≝"; content = "⛫_coordination ∧ membership ∧ ¬enforcement_authority ∧ ¬nominates_candidates"; }
      { prefix = "∂"; content = "⍚⊅⎈ (⍚ coordination | ⎈ enforcement)"; }
      { prefix = "∂"; content = "⍚⊅⍢ (⍚ general | ⍢ educational)"; }
      { prefix = "∂"; content = "⍚⊅⌯ (⍚ institutional | ⌯ diffuse)"; }
      { prefix = "∂"; content = "⍚⊅⧈ (⍚ membership | ⧈ ascribed category)"; }
      { prefix = "∂"; content = "⍚⊃⚐ (⚐ specialized ⍚ | nominates candidates)"; }
      { prefix = "⊡"; content = "⍚: Բ7Փ7Ֆ6Պ5Հ6Շ5Ի6Ղ7Ց5"; }
      { prefix = "⊡"; content = "gradients:"; }
      { prefix = "⊡"; content = "  →⎈: Բ↑Փ↑Ֆ↑Պ↓Հ↑Ղ↑Ց↓ | →⌯: Բ↓Ֆ↓Պ↑Հ↓Ի↓Ց↑ | →⚐: +nominates"; }
      { prefix = "⊛"; content = "corporate: ⍚⦑Apple|Toyota|Goldman Sachs|Samsung|Tata Group|Alibaba|HSBC⦒"; }
      { prefix = "⊛"; content = "nonprofit: ⍚⦑Red Cross|Doctors Without Borders|ACLU|Oxfam|Amnesty International⦒"; }
      { prefix = "⊛"; content = "research: ⍚⦑Bell Labs|RAND Corporation|Santa Fe Institute|Brookings Institution|Max Planck Society|Fraunhofer Society⦒"; }
      { prefix = "⊛"; content = "professional: ⍚⦑American Bar Association|IEEE|Royal Society|British Medical Association⦒"; }
      { prefix = "⊛"; content = "labor: ⍚⦑AFL-CIO|UAW|IG Metall|Solidarity|Congress of South African Trade Unions⦒"; }
      { prefix = "⊛"; content = "international: ⍚⦑United Nations|WHO|IMF|African Union⦒"; }
      { prefix = "⊛"; content = "historical: ⍚⦑East India Company|Hanseatic League|Académie française|Dutch East India Company⦒"; }
      { prefix = "◻"; content = "vector test: Բ≥6∧Ֆ≥5∧Պ≤6→institutional | Բ≤4∧Ֆ≤3∧Պ≥7→⌯"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⍚⦑United Nations|WHO|IMF⦒ (facilitates) vs ⎈⦑WTO|ICC⦒ (compels)"; }
      { prefix = "◻"; content = "  ⍚⦑JPMorgan Chase|Deutsche Bank⦒ (commercial) vs ⎈⦑Federal Reserve|ECB⦒ (regulatory)"; }
      { prefix = "≟"; content = "test{enforcement?→YES:⎈|NO:continue}"; }
      { prefix = "≟"; content = "test{educational primary?→YES:⍢|NO:continue}"; }
      { prefix = "≟"; content = "test{nominates candidates?→YES:⚐|NO:continue}"; }
      { prefix = "≟"; content = "test{coordination∧membership?→YES:⍚|NO:continue}"; }
      { prefix = "≟"; content = "test{diffuse ideological?→YES:⌯|NO:continue}"; }
      { prefix = "≟"; content = "test{ascribed category?→YES:⧈|NO:review}"; }
      { prefix = "⊨"; content = "⍚ ≡ ⛫_coordination ∧ membership ∧ ¬enforcement ∧ ¬nominates"; }
    ];
  };
  governmental-authority = {
    symbol = "⎈";
    name = "Governmental Authority";
    description = "State entity with enforcement power and sovereign or delegated authority";
    category = "Primary";
    sortOrder = 9;
    lines = [
      { prefix = "≡"; content = "state_entity"; }
      { prefix = "≡"; content = "enforcement_power"; }
      { prefix = "≡"; content = "sovereign_authority"; }
      { prefix = "≝"; content = "sovereign∨delegated authority ∧ public sector ∧ enforcement capacity"; }
      { prefix = "∂"; content = "⎈⊅⍚ (⎈ ⛫_enforcement | ⍚ ⛫_coordination)"; }
      { prefix = "∂"; content = "⎈⊅⚐ (⎈ holds∧exercises | ⚐ contests for power)"; }
      { prefix = "∂"; content = "⎈⊅⧈ (⎈ institution | ⧈ faction/supporters)"; }
      { prefix = "∂"; content = "⎈⊅⧊ (⎈ specific entity | ⧊ power concept)"; }
      { prefix = "⊡"; content = "⎈: Փ9Ֆ8Հ7Ղ8"; }
      { prefix = "⊡"; content = "regulatory: Փ8Ֆ9Հ6Ղ6 | legislative: Փ9Ֆ9Հ8Ղ9 | supranational: Փ8Ֆ7Հ7Ղ7"; }
      { prefix = "⊛"; content = "legislative: ⎈⦑U.S. Congress|Parliament (UK)|Bundestag|National People's Congress|Knesset|Diet of Japan⦒"; }
      { prefix = "⊛"; content = "judicial: ⎈⦑Supreme Court (US)|European Court of Justice|Constitutional Court (Germany)⦒"; }
      { prefix = "⊛"; content = "regulatory: ⎈⦑EPA|FDA|FCC|SEC|Ofcom|BaFin|CNIL⦒"; }
      { prefix = "⊛"; content = "military: ⎈⦑United States Army|People's Liberation Army|NATO|IDF|Bundeswehr⦒"; }
      { prefix = "⊛"; content = "monetary: ⎈⦑Federal Reserve|ECB|Bank of England|People's Bank of China|Bank of Japan⦒"; }
      { prefix = "⊛"; content = "supranational: ⎈⦑European Union|WTO|ICC|African Union Commission⦒"; }
      { prefix = "⊛"; content = "ancient: ⎈⦑Athenian Assembly|Roman Senate|Spartan Council⦒"; }
      { prefix = "◻"; content = "enforcement discriminator: statutory|monetary|military|legislative|judicial→⎈ | advisory→⍚"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  'European Powers'→⧊ | 'European Union'→⎈"; }
      { prefix = "◻"; content = "  'the state'→⧊ | 'the French state'→⎈"; }
      { prefix = "◻"; content = "  'Senate Republicans'→⧈ | 'U.S. Senate'→⎈"; }
      { prefix = "◻"; content = "  ⎈⦑WTO|ICC⦒ (can_compel) vs ⍚⦑UN|WHO⦒ (facilitates)"; }
      { prefix = "◻"; content = "  ⚐⦑Labour Party (UK)⦒ contests ∧ ⎈⦑UK Government⦒ governs — both valid"; }
      { prefix = "≟"; content = "test{specific entity?→continue | power concept?→⧊}"; }
      { prefix = "≟"; content = "test{enforcement power?→⎈ | advisory only?→⍚}"; }
      { prefix = "≟"; content = "test{public sector?→continue | private?→⍚}"; }
      { prefix = "≟"; content = "test{institution?→⎈ | faction?→⧈}"; }
      { prefix = "≟"; content = "test{contests elections?→⚐ | governs?→⎈}"; }
      { prefix = "⊨"; content = "⎈ ≡ ⛫_enforcement ∧ sovereign∨delegated ∧ public_sector"; }
    ];
  };
  tech = {
    symbol = "⌬";
    name = "Tech";
    description = "Technology with material/computational realization and operational capability";
    category = "Primary";
    sortOrder = 10;
    lines = [
      { prefix = "≡"; content = "technology"; }
      { prefix = "≡"; content = "technical_artifact"; }
      { prefix = "≡"; content = "implemented_system"; }
      { prefix = "≝"; content = "material|computational realization ∧ operational capability ∧ technical infrastructure"; }
      { prefix = "∂"; content = "⌬⊅⧊ (⌬ implemented | ⧊ abstract framework)"; }
      { prefix = "∂"; content = "⌬⊅⧏ (⌬ technical system | ⧏ analytical procedure)"; }
      { prefix = "∂"; content = "⌬⊅⍚ (⌬ artifact | ⍚ organization providing tech)"; }
      { prefix = "∂"; content = "⌬⊅⍓ (⌬ platform without curation | ⍓ editorial selection)"; }
      { prefix = "⊡"; content = "⌬: Բ7Փ8Ֆ6Պ5Հ6Շ6Ի7Ղ6Ց6"; }
      { prefix = "⊡"; content = "infrastructure: Բ8Փ9Շ8 (widespread, foundational)"; }
      { prefix = "⊡"; content = "platform: Բ6Փ7Պ6 (user-facing, permeable)"; }
      { prefix = "⊛"; content = "hardware: ⌬⦑Computer|Microprocessor|ENIAC|IBM System/360|Ferranti Mark 1|BESM-6⦒"; }
      { prefix = "⊛"; content = "software: ⌬⦑UNIX|Linux|Windows|LISP|COBOL|Multics⦒"; }
      { prefix = "⊛"; content = "networks: ⌬⦑Internet|Arpanet|SAGE System|Minitel|SWIFT|Fidonet⦒"; }
      { prefix = "⊛"; content = "platforms: ⌬⦑Facebook|WeChat|VKontakte|LINE|Tinder|Amazon KDP|Substack⦒"; }
      { prefix = "⊛"; content = "domains: ⌬⦑AI|Large Language Models|Blockchain|Nanotechnology|CRISPR⦒"; }
      { prefix = "⊛"; content = "industrial: ⌬⦑Numerical Control|Feedback Control|Assembly Line|Containerization⦒"; }
      { prefix = "⊛"; content = "historical: ⌬⦑Telegraph|Printing Press|Jacquard Loom|Difference Engine⦒"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓⌬⦑[Specific System]⦒: ⌬⦑UNIX|Arpanet|ENIAC⦒"; }
      { prefix = "◻"; content = "  ✓⌬⦑[Technical Domain]⦒: ⌬⦑AI|Blockchain|CRISPR⦒"; }
      { prefix = "◻"; content = "  expand acronyms when ambiguous: ⌬⦑Terminal Interface Message Processor (TIP)⦒"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⌬⦑AI⦒ (implemented domain) vs ⧊⦑Intelligence⦒ (abstract concept)"; }
      { prefix = "◻"; content = "  ⌬⦑Collaborative Filtering⦒ (algorithmic system) vs ⧏⦑Content Analysis⦒ (method)"; }
      { prefix = "◻"; content = "  ⌬⦑Facebook⦒ (platform) vs ⍚⦑Meta⦒ (company) — context determines"; }
      { prefix = "◻"; content = "  ⌬⦑Amazon KDP|Substack⦒ (platform) vs ⍓⦑Penguin⦒ (editorial selection)"; }
      { prefix = "≟"; content = "test{material|computational realization?→YES:⌬|NO:continue}"; }
      { prefix = "≟"; content = "test{operational capability∧technical infrastructure?→YES:⌬|NO:continue}"; }
      { prefix = "≟"; content = "test{abstract concept without implementation?→YES:⧊|NO:continue}"; }
      { prefix = "≟"; content = "test{analytical method without technical system?→YES:⧏|NO:continue}"; }
      { prefix = "≟"; content = "test{organization operating platform?→context: platform→⌬|company→⍚}"; }
      { prefix = "≟"; content = "test{platform without editorial selection?→YES:⌬|NO:⍓}"; }
      { prefix = "⊨"; content = "⌬ ≡ material|computational_realization ∧ operational_capability"; }
    ];
  };
  identifier = {
    symbol = "⎚";
    name = "Identifier";
    description = "Standardized persistent identifier with external authority (ISBN, DOI, ORCID)";
    category = "Institutional";
    sortOrder = 11;
    lines = [
      { prefix = "≡"; content = "unique_identifier"; }
      { prefix = "≡"; content = "persistent_reference"; }
      { prefix = "≡"; content = "deduplication_key"; }
      { prefix = "≝"; content = "standardized format ∧ external authority ∧ canonical registry"; }
      { prefix = "∂"; content = "⎚⊅⧊ (⎚ specific ID | ⧊ concept of identity)"; }
      { prefix = "∂"; content = "⎚⊅⧏ (⎚ instance | ⧏ identification system as method)"; }
      { prefix = "⊛"; content = "bibliographic: ⎚⦑ISBN 9780691059082|ISSN 0036-8075|LCCN 2012345678⦒"; }
      { prefix = "⊛"; content = "research: ⎚⦑DOI 10.1126/science.123456|ArXiv 2301.12345|PubMed 12345678⦒"; }
      { prefix = "⊛"; content = "personal: ⎚⦑ORCID 0000-0002-1234-5678|ResearcherID A-1234-2012⦒"; }
      { prefix = "⊛"; content = "institutional: ⎚⦑ROR 05dxps055|ISNI 0000000121032683|VIAF 12345678⦒"; }
      { prefix = "◻"; content = "criteria: standardized format ∧ persistent uniqueness ∧ external authority"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⎚⦑ISBN 9780691059082⦒ (specific) vs ⧊⦑Identity⦒ (concept)"; }
      { prefix = "◻"; content = "  ⎚⦑DOI 10.1126/...⦒ (instance) vs ⧏⦑DOI System⦒ (framework)"; }
      { prefix = "≟"; content = "test{external standardizing authority?→YES:⎚|NO:continue}"; }
      { prefix = "≟"; content = "test{persistent unique reference?→YES:⎚|NO:continue}"; }
      { prefix = "≟"; content = "test{concept about identity?→YES:⧊|NO:continue}"; }
      { prefix = "≟"; content = "test{naming system as method?→YES:⧏|NO:review}"; }
      { prefix = "⊨"; content = "⎚ ≡ standardized_identifier ∧ external_authority ∧ persistent_reference"; }
    ];
  };
  publisher = {
    symbol = "⍓";
    name = "Publisher";
    description = "Publishing house with content distribution as primary function";
    category = "Institutional";
    sortOrder = 12;
    lines = [
      { prefix = "≡"; content = "publishing_house"; }
      { prefix = "≡"; content = "content_distribution"; }
      { prefix = "≡"; content = "editorial_function"; }
      { prefix = "≝"; content = "editorial selection ∧ content production ∧ commercial/institutional distribution"; }
      { prefix = "∂"; content = "⍓⊂⍚ (⍓ specialized ⍚ | publishing function)"; }
      { prefix = "∂"; content = "⍓⊅⍢ (⍓ press | ⍢ university)"; }
      { prefix = "∂"; content = "⍓⊅⌬ (⍓ editorial selection | ⌬ platform without curation)"; }
      { prefix = "⊡"; content = "⍓: Բ6Փ7Ֆ5Պ5Հ5Շ5Ի6Ղ6Ց5"; }
      { prefix = "⊡"; content = "university_press: Փ7Ֆ6Շ4"; }
      { prefix = "⊡"; content = "commercial: Փ8Ֆ5Շ6"; }
      { prefix = "⊛"; content = "university: ⍓⦑MIT Press|Cambridge University Press|Oxford University Press|Harvard University Press|Princeton University Press|Éditions de l'EHESS⦒"; }
      { prefix = "⊛"; content = "commercial_anglophone: ⍓⦑Penguin Random House|HarperCollins|Norton|Faber|Verso|Bloomsbury⦒"; }
      { prefix = "⊛"; content = "commercial_european: ⍓⦑Gallimard|Suhrkamp|Feltrinelli|Tusquets|Actes Sud|Fischer Verlag⦒"; }
      { prefix = "⊛"; content = "commercial_global: ⍓⦑Kodansha|Shueisha|Commercial Press (China)|Planeta|Record (Brazil)⦒"; }
      { prefix = "⊛"; content = "academic/professional: ⍓⦑Springer|Elsevier|Wiley|IEEE|ACM|Taylor & Francis⦒"; }
      { prefix = "⊛"; content = "independent: ⍓⦑Verso|New Directions|Graywolf|Seven Stories|Dalkey Archive⦒"; }
      { prefix = "⊛"; content = "imprints: ⍓⦑Vintage|Knopf|Anchor Books|Picador|FSG⦒"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓⍓⦑MIT Press|Cambridge University Press⦒ (full name for university presses)"; }
      { prefix = "◻"; content = "  ✓⍓⦑Penguin|Faber|Gallimard⦒ (established short forms)"; }
      { prefix = "◻"; content = "  ✗⍓⦑MIT|Cambridge|Oxford⦒ when referring to publisher (ambiguous with university)"; }
      { prefix = "◻"; content = "  ✗⍓⦑University Presses|Commercial Publishers⦒ categorical plurals→DELETE"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⍓⦑MIT Press⦒ vs ⍢⦑MIT⦒ — publishing vs educational function"; }
      { prefix = "◻"; content = "  ⍓⦑IEEE⦒ (publishing) vs ⍚⦑IEEE⦒ (membership org) — context determines"; }
      { prefix = "◻"; content = "  ⍓⦑Vintage⦒ imprint ∧ ⍓⦑Random House⦒ parent — both valid"; }
      { prefix = "◻"; content = "  ⍓⦑Faber⦒ (editorial selection) vs ⌬⦑Amazon KDP|Substack⦒ (platform)"; }
      { prefix = "≟"; content = "test{editorial selection∧content distribution?→YES:⍓|NO:continue}"; }
      { prefix = "≟"; content = "test{university press vs university?→⍓ press ∧ ⍢ university}"; }
      { prefix = "≟"; content = "test{platform without editorial function?→YES:⌬|NO:continue}"; }
      { prefix = "≟"; content = "test{professional society as publisher?→YES:⍓|NO:⍚ membership org}"; }
      { prefix = "≟"; content = "test{generic organization?→YES:⍚|NO:review}"; }
      { prefix = "⊨"; content = "⍓ ≡ editorial_selection ∧ content_distribution ∧ ⍓⊂⍚"; }
    ];
  };
  university = {
    symbol = "⍢";
    name = "University";
    description = "Educational institution with degree-granting authority and teaching/research mission";
    category = "Institutional";
    sortOrder = 13;
    lines = [
      { prefix = "≡"; content = "educational_institution"; }
      { prefix = "≡"; content = "degree_granting"; }
      { prefix = "≡"; content = "teaching_research_mission"; }
      { prefix = "≝"; content = "higher education ∧ degree-granting authority ∧ research activity ∧ academic mission"; }
      { prefix = "∂"; content = "⍢⊂⍚ (⍢ specialized ⍚ | educational function)"; }
      { prefix = "∂"; content = "⍢⊅⍓ (⍢ institution | ⍓ press/publisher)"; }
      { prefix = "∂"; content = "⍢⊅⍚_research (⍢ degree-granting | ⍚ research-only)"; }
      { prefix = "⊡"; content = "⍢: Բ7Փ8Ֆ7Պ6Հ7Շ5Ի7Ղ6Ց4"; }
      { prefix = "⊡"; content = "research_university: Փ9Ֆ8Հ7"; }
      { prefix = "⊡"; content = "liberal_arts: Փ7Ֆ6Հ5"; }
      { prefix = "⊛"; content = "⍢⦑Harvard University|MIT|Stanford University|University of Chicago|Yale University⦒"; }
      { prefix = "⊛"; content = "⍢⦑Oxford|Cambridge|London School of Economics|Imperial College London|Edinburgh⦒"; }
      { prefix = "⊛"; content = "⍢⦑ETH Zurich|Sorbonne|Humboldt University|Leiden University|Uppsala University⦒"; }
      { prefix = "⊛"; content = "⍢⦑University of Tokyo|Tsinghua University|Peking University|National University of Singapore|IIT Bombay⦒"; }
      { prefix = "⊛"; content = "⍢⦑University of São Paulo|UNAM|University of Cape Town|Hebrew University of Jerusalem⦒"; }
      { prefix = "⊛"; content = "⍢⦑École Normale Supérieure|Sciences Po|Max Planck Institute⦒"; }
      { prefix = "⊛"; content = "colleges: ⍢⦑Williams College|Amherst College|Trinity College, Cambridge|Balliol College, Oxford⦒"; }
      { prefix = "⊛"; content = "subdivisions: ⍢⦑Harvard Business School|Yale Law School|MIT Sloan|MIT Media Lab⦒"; }
      { prefix = "⊛"; content = "historical: ⍢⦑Academy of Athens|University of Bologna|University of Paris|Al-Azhar University⦒"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓⍢⦑[Full Name]⦒: ⍢⦑Harvard University|University of Tokyo⦒"; }
      { prefix = "◻"; content = "  ✓⍢⦑[Established Short]⦒: ⍢⦑MIT|Oxford|Cambridge|Caltech⦒"; }
      { prefix = "◻"; content = "  ✓⍢⦑[Subdivision]⦒ when contextually relevant: ⍢⦑Harvard Business School⦒"; }
      { prefix = "◻"; content = "  ✗⍢⦑Harvard⦒ when ambiguous (university? press? location?)"; }
      { prefix = "◻"; content = "  ✗⍢⦑Universities|Colleges|Business Schools⦒ categorical plurals→DELETE"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⍢⦑MIT⦒ vs ⍓⦑MIT Press⦒ — educational vs publishing function"; }
      { prefix = "◻"; content = "  ⍢⦑Caltech⦒ vs ⍚⦑Bell Labs⦒ — degree-granting vs research-only"; }
      { prefix = "◻"; content = "  ⍢⦑UC System⦒ ∧ ⍢⦑UC Berkeley⦒ — system and campus both valid"; }
      { prefix = "≟"; content = "test{degree-granting authority?→YES:⍢|NO:continue}"; }
      { prefix = "≟"; content = "test{educational mission primary?→YES:⍢|NO:continue}"; }
      { prefix = "≟"; content = "test{research-only, no teaching?→YES:⍚|NO:continue}"; }
      { prefix = "≟"; content = "test{university press?→YES:⍓|NO:continue}"; }
      { prefix = "≟"; content = "test{non-educational org?→YES:⍚|NO:review}"; }
      { prefix = "⊨"; content = "⍢ ≡ degree_granting ∧ educational_mission ∧ ⍢⊂⍚"; }
    ];
  };
  language = {
    symbol = "⧩";
    name = "Language";
    description = "Natural language: references, non-trivial foreign terms, non-English passages";
    category = "Institutional";
    sortOrder = 14;
    lines = [
      { prefix = "≡"; content = "natural_language"; }
      { prefix = "≡"; content = "linguistic_system"; }
      { prefix = "≡"; content = "foreign_term_marker"; }
      { prefix = "≝"; content = "human communication system ∧ linguistic code ∧ non-English usage"; }
      { prefix = "∂"; content = "⧩⊅⧊ (⧩ specific language | ⧊ 'Language' as concept)"; }
      { prefix = "∂"; content = "⧩⊅⌬ (⧩ natural | ⌬ programming language)"; }
      { prefix = "⊛"; content = "major: ⧩⦑English|French|German|Spanish|Portuguese|Russian|Arabic|Mandarin|Japanese⦒"; }
      { prefix = "⊛"; content = "classical: ⧩⦑Greek|Latin|Sanskrit|Classical Arabic|Classical Chinese|Hebrew⦒"; }
      { prefix = "⊛"; content = "varieties: ⧩⦑Katharevousa|Demotic Greek|Classical Latin|Vulgar Latin|Old English⦒"; }
      { prefix = "⊛"; content = "regional: ⧩⦑Yoruba|Swahili|Hindi|Bengali|Tagalog|Quechua|Nahuatl⦒"; }
      { prefix = "◻"; content = "tagging triggers:"; }
      { prefix = "◻"; content = "  reference: 'written in Greek' | 'the French term' | 'from Arabic'"; }
      { prefix = "◻"; content = "  non-trivial terms: Aufhebung | Weltanschauung | habitus | ressentiment | Dasein"; }
      { prefix = "◻"; content = "  passages: block quotes in non-English | extended foreign text"; }
      { prefix = "◻"; content = "  ✗trivial: café | résumé | et cetera — fully assimilated"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⧩⦑German⦒ (language) vs ⧈⦑Germans⦒ (people)"; }
      { prefix = "◻"; content = "  ⧩⦑French⦒ (language) vs ⌖⦑France⦒ (place)"; }
      { prefix = "◻"; content = "  ⧩⦑Greek⦒ for 'λόγος' vs ⧊⦑Logos⦒ for concept discussion"; }
      { prefix = "≟"; content = "test{natural language reference?→YES:⧩|NO:continue}"; }
      { prefix = "≟"; content = "test{non-trivial foreign term?→YES:⧩|NO:continue}"; }
      { prefix = "≟"; content = "test{non-English passage?→YES:⧩|NO:continue}"; }
      { prefix = "≟"; content = "test{programming language?→YES:⌬|NO:continue}"; }
      { prefix = "≟"; content = "test{abstract 'Language' concept?→YES:⧊|NO:review}"; }
      { prefix = "⊨"; content = "⧩ ≡ natural_language ∧ (reference∨foreign_term∨passage)"; }
    ];
  };
  journal = {
    symbol = "𝄏";
    name = "Journal";
    description = "Periodical publication: academic journals, magazines, newspapers";
    category = "Institutional";
    sortOrder = 51;
    lines = [
      { prefix = "≡"; content = "periodical_publication"; }
      { prefix = "≡"; content = "serial_venue"; }
      { prefix = "≡"; content = "regular_issues"; }
      { prefix = "≝"; content = "serial format ∧ publication regularity ∧ institutional|commercial periodical"; }
      { prefix = "∂"; content = "𝄏⊅⊳ (𝄏 serial | ⊳ standalone work)"; }
      { prefix = "∂"; content = "𝄏⊅⍚ (𝄏 publication | ⍚ publishing organization)"; }
      { prefix = "∂"; content = "𝄏⊅⍓ (𝄏 periodical | ⍓ press/publisher)"; }
      { prefix = "⊛"; content = "academic: 𝄏⦑Nature|Science|Daedalus|Mind|Annales|Past & Present⦒"; }
      { prefix = "⊛"; content = "magazines: 𝄏⦑Harper's Magazine|Fortune|The New Yorker|Der Spiegel|L'Express⦒"; }
      { prefix = "⊛"; content = "newspapers: 𝄏⦑Financial Times|Asahi Shinbun|Le Monde|Frankfurter Allgemeine⦒"; }
      { prefix = "⊛"; content = "reviews: 𝄏⦑London Review of Books|New York Review of Books|Times Literary Supplement⦒"; }
      { prefix = "⊛"; content = "historical: 𝄏⦑Popular Science Monthly|Der Monat|Edinburgh Review|Mercure de France⦒"; }
      { prefix = "⊛"; content = "trade/underground: 𝄏⦑Byte|Datamation|2600: The Hacker Quarterly|FidoNews⦒"; }
      { prefix = "◻"; content = "naming:"; }
      { prefix = "◻"; content = "  ✓𝄏⦑[Full Name]⦒ | ✗abbreviations unless established"; }
      { prefix = "◻"; content = "  ✓𝄏⦑Dialogue: A Journal of Mormon Thought⦒ — include subtitle when official"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  𝄏⦑Daedalus⦒ (publication) vs ⍚⦑American Academy of Arts and Sciences⦒ (publisher)"; }
      { prefix = "◻"; content = "  periodic issues→𝄏 | one-time→⊳"; }
      { prefix = "◻"; content = "compound citation: ⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Article⦒"; }
      { prefix = "≟"; content = "test{periodic issues?→YES:𝄏|NO:continue}"; }
      { prefix = "≟"; content = "test{one-time publication?→YES:⊳|NO:continue}"; }
      { prefix = "≟"; content = "test{publishing organization?→YES:⍚|NO:review}"; }
      { prefix = "⊨"; content = "𝄏 ≡ serial_publication ∧ periodic_issues"; }
    ];
  };
  concept = {
    symbol = "⧊";
    name = "Concept";
    description = "Reality-organizing abstraction with cross-domain operation and organizing power";
    category = "Conceptual";
    sortOrder = 20;
    lines = [
      { prefix = "≡"; content = "reality_organizing_abstraction"; }
      { prefix = "≡"; content = "theoretical_construct"; }
      { prefix = "≡"; content = "cross_domain_operation"; }
      { prefix = "≝"; content = "domain transcendence ∧ organizing power ∧ ¬{people,institution,procedure,implementation}"; }
      { prefix = "∂"; content = "⧊⊅⧈ (⧊ abstraction | ⧈ people) — occupational ALWAYS ⧈"; }
      { prefix = "∂"; content = "⧊⊅❖ (⧊ cross-field | ❖ institutional)"; }
      { prefix = "∂"; content = "⧊⊅⧏ (⧊ framework | ⧏ procedural)"; }
      { prefix = "∂"; content = "⧊⊅⌬ (⧊ abstract | ⌬ implemented)"; }
      { prefix = "∂"; content = "⧊⊅⚐ (⧊ ideology | ⚐ party)"; }
      { prefix = "⊡"; content = "⧊: Շ variable (Շ9 universal→Շ3 domain-specific)"; }
      { prefix = "⊡"; content = "philosophical: Շ9Գ9 | political: Շ8Գ8 | analytical: Շ7Գ7"; }
      { prefix = "⊛"; content = "philosophical: ⧊⦑Free Will|Consciousness|Mind|Self|Being|Causation⦒"; }
      { prefix = "⊛"; content = "political: ⧊⦑Democracy|Capitalism|Sovereignty|Liberty|Republic|Citizenship⦒"; }
      { prefix = "⊛"; content = "social: ⧊⦑Class|Gender|Culture|Society|Power|Identity|Habitus⦒"; }
      { prefix = "⊛"; content = "epistemic: ⧊⦑Science|Knowledge|Rationality|Truth|Verification|Objectivity⦒"; }
      { prefix = "⊛"; content = "analytical: ⧊⦑Classification|Representation|Explanation|Performativity⦒"; }
      { prefix = "⊛"; content = "frameworks: ⧊⦑Engineering Mindset|Technical Rationality|Neoliberalism|Orientalism⦒"; }
      { prefix = "⊛"; content = "metaphors: ⧊⦑Black Box|Attention as Economic Resource|Social Contract⦒"; }
      { prefix = "⊛"; content = "non-western: ⧊⦑Qi|Dharma|Ubuntu|Tawhid|Ren|Li⦒"; }
      { prefix = "◻"; content = "CRITICAL: occupational∈⧈ (¬⧊)"; }
      { prefix = "◻"; content = "  ✗⧊⦑Engineers|Citizens|Patients⦒ → ⧈"; }
      { prefix = "◻"; content = "  ✓⧊⦑Engineering Mindset|Citizenship|Patienthood⦒ — abstraction ¬people"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  'The [X]'=people?→⧈ | abstraction?→⧊"; }
      { prefix = "◻"; content = "  'Department of [X]'?→❖ | cross-field?→⧊"; }
      { prefix = "◻"; content = "  'Apply [X]'?→⧏ | framework?→⧊"; }
      { prefix = "◻"; content = "  implemented system?→⌬ | abstract?→⧊"; }
      { prefix = "◻"; content = "domain transcendence: ⧊⦑Representation⦒ (philosophy,art,politics,science)"; }
      { prefix = "≟"; content = "test{'The [X]'=people?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{occupational?→YES:⧈|NO:continue}"; }
      { prefix = "≟"; content = "test{'Department of [X]'?→YES:❖|NO:continue}"; }
      { prefix = "≟"; content = "test{'Apply [X]' natural?→YES:⧏|NO:continue}"; }
      { prefix = "≟"; content = "test{material/computational?→YES:⌬|NO:continue}"; }
      { prefix = "≟"; content = "test{cross-domain∧organizing?→YES:⧊|NO:review}"; }
      { prefix = "⊨"; content = "⧊ ≡ reality_organizing ∧ cross_domain ∧ ⧊⊅{⧈,❖,⧏,⌬}"; }
    ];
  };
  method = {
    symbol = "⧏";
    name = "Method";
    description = "Analytical technique with procedural application and replicable steps";
    category = "Conceptual";
    sortOrder = 21;
    lines = [
      { prefix = "≡"; content = "analytical_technique"; }
      { prefix = "≡"; content = "systematic_procedure"; }
      { prefix = "≡"; content = "replicable_steps"; }
      { prefix = "≝"; content = "procedural application ∧ 'Apply [X]' natural ∧ HOW-writing"; }
      { prefix = "∂"; content = "⧏⊅❖ (⧏ technique | ❖ institutional field)"; }
      { prefix = "∂"; content = "⧏⊅⧊ (⧏ doing | ⧊ discussing/critiquing)"; }
      { prefix = "∂"; content = "⧏⊅⌬ (⧏ analytical | ⌬ material/computational)"; }
      { prefix = "⊡"; content = "⧏: Ֆ6Շ6Գ7Բ6"; }
      { prefix = "⊡"; content = "lab protocol: Ֆ9Շ3Բ9 | ethnographic: Ֆ5Շ7Բ4 | philosophical: Ֆ4Շ9Գ9"; }
      { prefix = "⊛"; content = "qualitative: ⧏⦑Content Analysis|Discourse Analysis|Grounded Theory|Thematic Analysis⦒"; }
      { prefix = "⊛"; content = "ethnographic: ⧏⦑Participant Observation|Thick Description|Life History Method⦒"; }
      { prefix = "⊛"; content = "historical: ⧏⦑Genealogical Method|Comparative Method|Prosopography|Archival Method⦒"; }
      { prefix = "⊛"; content = "quantitative: ⧏⦑Regression Analysis|Factor Analysis|Network Analysis|Time Series⦒"; }
      { prefix = "⊛"; content = "philosophical: ⧏⦑Dialectics|Phenomenological Method|Hermeneutic Circle|Deconstruction⦒"; }
      { prefix = "⊛"; content = "scientific: ⧏⦑Western Blot|PCR|Spectroscopy|Chromatography⦒"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  'apply content analysis'→⧏ | 'the idea of content analysis'→⧊"; }
      { prefix = "◻"; content = "  Popper critiquing 'holistic experiments'→⧊ | methods textbook teaching it→⧏"; }
      { prefix = "◻"; content = "  methodological (handbooks, protocols)→⧏ | epistemological (philosophy)→⧊"; }
      { prefix = "◻"; content = "tests:"; }
      { prefix = "◻"; content = "  'Apply [X]' natural?→⧏ | 'Department of [X]'?→❖ | implemented system?→⌬"; }
      { prefix = "≟"; content = "test{HOW-writing?→continue | WHAT-writing?→⧊}"; }
      { prefix = "≟"; content = "test{replicable steps?→continue | abstract?→⧊}"; }
      { prefix = "≟"; content = "test{departments/degrees?→❖ | technique?→continue}"; }
      { prefix = "≟"; content = "test{material/computational?→⌬ | analytical?→⧏}"; }
      { prefix = "⊨"; content = "⧏ ≡ systematic_procedure ∧ replicable ∧ 'Apply [X]' natural"; }
    ];
  };
  movement = {
    symbol = "⌯";
    name = "Movement";
    description = "Social movement with sustained mobilization and collective identity";
    category = "Conceptual";
    sortOrder = 22;
    lines = [
      { prefix = "≡"; content = "social_movement"; }
      { prefix = "≡"; content = "collective_action"; }
      { prefix = "≡"; content = "sustained_mobilization"; }
      { prefix = "≝"; content = "sustained mobilization ∧ collective identity ∧ ¬nominates_candidates"; }
      { prefix = "∂"; content = "⌯⊅⍚ (⌯ diffuse | ⍚ institutionalized)"; }
      { prefix = "∂"; content = "⌯⊅⚐ (⌯ ¬nominates | ⚐ nominates candidates)"; }
      { prefix = "∂"; content = "⌯⊅⧈ (⌯ movement | ⧈ adherents as category)"; }
      { prefix = "∂"; content = "⌯⊅⧖ (⌯ mobilization | ⧖ temporal period)"; }
      { prefix = "∂"; content = "⌯⊅❖ (⌯ activism | ❖ academic field)"; }
      { prefix = "⊡"; content = "⌯: Բ3Փ4Ֆ2Պ8Հ2Շ6Ի4Ղ5Ց7"; }
      { prefix = "⊡"; content = "→⍚|⚐: Բ↑Ֆ↑Պ↓ | →diffuse: Պ↑Ի↓Ց↑"; }
      { prefix = "⊛"; content = "⌯⦑Civil Rights Movement|Labor Movement|Anti-War Movement|Occupy|Black Lives Matter⦒"; }
      { prefix = "⊛"; content = "⌯⦑Feminism|Environmentalism|LGBTQ Rights Movement|Disability Rights Movement⦒"; }
      { prefix = "⊛"; content = "⌯⦑Solidarity (Poland)|Arab Spring|Umbrella Movement|Landless Workers' Movement (Brazil)⦒"; }
      { prefix = "⊛"; content = "⌯⦑Peronism|Gandhian Movement|Negritude|Pan-Africanism|Zionism (pre-state)⦒"; }
      { prefix = "⊛"; content = "⌯⦑Frankfurt School|Vienna Circle|Jansenism|Oxford Movement|Transcendentalism⦒"; }
      { prefix = "⊛"; content = "⌯⦑Protestantism|Pietism|Pentecostalism|Liberation Theology⦒"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⌯⦑Frankfurt School⦒ vs ⍚⦑Institute for Social Research⦒"; }
      { prefix = "◻"; content = "  ⌯⦑Zionism (pre-state)⦒ vs ⚐⦑Likud⦒"; }
      { prefix = "◻"; content = "  ⌯⦑Peronism⦒ vs ⚐⦑Partido Justicialista (PJ)⦒"; }
      { prefix = "◻"; content = "rejects_electoralism∧'party'_name→⌯: form decoupled from function"; }
      { prefix = "≟"; content = "test{nominates_candidates?→YES:⚐|NO:continue}"; }
      { prefix = "≟"; content = "test{sustained mobilization∧collective identity?→YES:⌯|NO:continue}"; }
      { prefix = "≟"; content = "test{formal organization∧membership?→YES:⍚|NO:continue}"; }
      { prefix = "≟"; content = "test{academic field?→YES:❖|NO:continue}"; }
      { prefix = "≟"; content = "test{adherents as people?→YES:⧈|NO:review}"; }
      { prefix = "⊨"; content = "⌯ ≡ sustained_mobilization ∧ collective_identity ∧ ¬nominates_candidates"; }
    ];
  };
  relation = {
    symbol = "⇋";
    name = "Relation";
    description = "Tripartite entrainment (agent-category-behavior) requiring ≥3 elements";
    category = "Relational";
    sortOrder = 30;
    lines = [
      { prefix = "≡"; content = "tripartite_entrainment"; }
      { prefix = "≡"; content = "mutual_influence"; }
      { prefix = "≡"; content = "structural_coupling"; }
      { prefix = "≝"; content = "agent∧category∧behavior ∧ elements≥3 ∧ mutual influence"; }
      { prefix = "∂"; content = "⇋⊅⧆ (⇋ tripartite≥3 | ⧆ dyadic binary)"; }
      { prefix = "∂"; content = "⇋⊅⟗ (⇋ structural coupling | ⟗ iterating feedback)"; }
      { prefix = "⊛"; content = "⇋⦑Classification-Person-Behavior|Diagnosis-Patient-Symptom|Label-Group-Action⦒"; }
      { prefix = "⊛"; content = "⇋⦑Market-Investor-Price|Institution-Role-Practice|Norm-Agent-Conduct⦒"; }
      { prefix = "◻"; content = "structure □mandatory:"; }
      { prefix = "◻"; content = "  agent (human/organizational) ∧ category (classificatory) ∧ behavior (action/response)"; }
      { prefix = "◻"; content = "  elements≥3 ∧ mutual influence evident"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⇋⦑Diagnosis-Patient-Symptom⦒ (tripartite) vs ⧆⦑Mind vs Body⦒ (dyadic)"; }
      { prefix = "◻"; content = "  elements=2→⧆ | elements≥3→⇋"; }
      { prefix = "≟"; content = "test{elements≥3∧mutual influence?→YES:⇋|NO:continue}"; }
      { prefix = "≟"; content = "test{dyadic binary?→YES:⧆|NO:review}"; }
      { prefix = "⊨"; content = "⇋ ≡ tripartite_entrainment ∧ □elements≥3"; }
    ];
  };
  tension = {
    symbol = "⧆";
    name = "Tension";
    description = "Binary productive opposition between competing frameworks (dyadic only)";
    category = "Relational";
    sortOrder = 31;
    lines = [
      { prefix = "≡"; content = "productive_opposition"; }
      { prefix = "≡"; content = "dyadic_contrast"; }
      { prefix = "≡"; content = "generative_friction"; }
      { prefix = "≝"; content = "binary competing frameworks ∧ elements=2 ∧ productive tension"; }
      { prefix = "∂"; content = "⧆⊅⇋ (⧆ dyadic=2 | ⇋ tripartite≥3)"; }
      { prefix = "∂"; content = "⧆⊅☯ (⧆ external opposition | ☯ internal contradiction)"; }
      { prefix = "∂"; content = "⧆⊅⟗ (⧆ static tension | ⟗ iterating cycle)"; }
      { prefix = "⊛"; content = "⧆⦑Individual vs Collective|Theory vs Practice|Structure vs Agency⦒"; }
      { prefix = "⊛"; content = "⧆⦑Quantitative vs Qualitative|Nature vs Nurture|Local vs Global⦒"; }
      { prefix = "⊛"; content = "⧆⦑Mind vs Body|Subject vs Object|Form vs Content⦒"; }
      { prefix = "⊛"; content = "⧆⦑Sacred vs Profane|Public vs Private|Universal vs Particular⦒"; }
      { prefix = "◻"; content = "naming: ⧆⦑A vs B⦒ format"; }
      { prefix = "◻"; content = "criteria: elements=2 exactly ∧ productive opposition ¬simple contradiction"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⧆⦑Mind vs Body⦒ (dyadic) vs ⇋⦑Agent-Category-Behavior⦒ (tripartite)"; }
      { prefix = "◻"; content = "  ⧆⦑Theory vs Practice⦒ (external) vs ☯⦑Knowing-by-Doing⦒ (internal paradox)"; }
      { prefix = "≟"; content = "test{elements=2∧competing?→YES:⧆|NO:continue}"; }
      { prefix = "≟"; content = "test{elements≥3?→YES:⇋|NO:continue}"; }
      { prefix = "≟"; content = "test{simultaneous within single?→YES:☯|NO:review}"; }
      { prefix = "⊨"; content = "⧆ ≡ dyadic_opposition ∧ elements=2 ∧ productive_tension"; }
    ];
  };
  loop = {
    symbol = "⟗";
    name = "Loop";
    description = "Feedback cycle with bidirectional causality and repeated iteration";
    category = "Relational";
    sortOrder = 32;
    lines = [
      { prefix = "≡"; content = "feedback_cycle"; }
      { prefix = "≡"; content = "named_loop"; }
      { prefix = "≡"; content = "repeated_iteration"; }
      { prefix = "≝"; content = "specific iterated pattern ∧ entity-taggable with ⦑⦒"; }
      { prefix = "∂"; content = "⟗⊂⟲ (⟗ instance | ⟲ mechanism)"; }
      { prefix = "∂"; content = "⟗⊅⬢ (⟗ iterating | ⬢ simultaneous t₀=t₀)"; }
      { prefix = "∂"; content = "⟗⊅⧃ (⟗ bidirectional | ⧃ unidirectional)"; }
      { prefix = "∂"; content = "⟗⊅⧊ (⟗ specific loop | ⧊ analyzing concept)"; }
      { prefix = "⊡"; content = "⟗: Տ7Բ8Գ4"; }
      { prefix = "⊡"; content = "⟗⦑Multiple Personality Formation Loop⦒: Տ9Բ8Գ5"; }
      { prefix = "⊛"; content = "⟗⦑Multiple Personality Formation Loop|Bandwagon Loop|Error-Confirmation Loop⦒"; }
      { prefix = "⊛"; content = "⟗⦑Hype Cycle|Boom-Bust Cycle|Vicious Circle|Virtuous Circle⦒"; }
      { prefix = "⊛"; content = "⟗⦑Self-Fulfilling Prophecy Loop|Expectations Loop|Stigma Loop⦒"; }
      { prefix = "◻"; content = "textual signals: 'fed back', 'reinforced', 'in turn', 'which then', 'cycle'"; }
      { prefix = "◻"; content = "criteria: bidirectional ∧ repeated ∧ visible iteration"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⟗⦑Multiple Personality Loop⦒: diagnosis→exhibit→expand→elicit (1983→1991 iteration)"; }
      { prefix = "◻"; content = "  ⬢⦑Making Up People⦒: classification constitutes classified (t₀=t₀, no iteration)"; }
      { prefix = "◻"; content = "  ⧃⦑Medicalization⦒: condition→medical frame (unidirectional, no return)"; }
      { prefix = "≟"; content = "test{specific loop?→continue | analyzing concept?→⧊}"; }
      { prefix = "≟"; content = "test{bidirectional∧repeated?→YES:⟗|NO:continue}"; }
      { prefix = "≟"; content = "test{unidirectional?→YES:⧃|NO:continue}"; }
      { prefix = "≟"; content = "test{simultaneous?→YES:⬢|NO:review}"; }
      { prefix = "⊨"; content = "⟗ ≡ feedback_cycle ∧ bidirectional ∧ repeated_iteration"; }
    ];
  };
  paradox = {
    symbol = "☯";
    name = "Paradox";
    description = "Simultaneous contradiction (A∧¬A) within single entity at same moment";
    category = "Relational";
    sortOrder = 33;
    lines = [
      { prefix = "≡"; content = "simultaneous_contradiction"; }
      { prefix = "≡"; content = "irresolvable_tension"; }
      { prefix = "≡"; content = "single_locus"; }
      { prefix = "≝"; content = "A∧¬A within single entity ∧ same moment ∧ genuine contradiction"; }
      { prefix = "∂"; content = "☯⊅⧆ (☯ internal contradiction | ⧆ external opposition)"; }
      { prefix = "∂"; content = "☯⊅⟗ (☯ simultaneous | ⟗ temporal alternation)"; }
      { prefix = "⊛"; content = "☯⦑Pyrrhic Victory|Catch-22|Double Bind|Bootstrap Paradox⦒"; }
      { prefix = "⊛"; content = "☯⦑Tolerance Paradox|Sovereignty Paradox|Observer's Paradox⦒"; }
      { prefix = "⊛"; content = "☯⦑Knowing-by-Doing|Planned Spontaneity|Organized Anarchism⦒"; }
      { prefix = "⊛"; content = "☯⦑Social Science of Change|Counterintuitive Policy|Unintended Consequences⦒"; }
      { prefix = "◻"; content = "criteria: A∧¬A ∧ same moment ∧ same entity ∧ irresolvable"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ☯⦑Double Bind⦒ (internal) vs ⧆⦑Mind vs Body⦒ (external)"; }
      { prefix = "◻"; content = "  ☯⦑Pyrrhic Victory⦒ (simultaneous) vs ⟗⦑Boom-Bust Cycle⦒ (alternating)"; }
      { prefix = "≟"; content = "test{A∧¬A simultaneously within single?→YES:☯|NO:continue}"; }
      { prefix = "≟"; content = "test{external binary opposition?→YES:⧆|NO:continue}"; }
      { prefix = "≟"; content = "test{temporal alternation?→YES:⟗|NO:review}"; }
      { prefix = "⊨"; content = "☯ ≡ simultaneous_contradiction ∧ single_locus ∧ irresolvable"; }
    ];
  };
  transformation = {
    symbol = "⧃";
    name = "Transformation";
    description = "Unidirectional temporal transformation with sequential developmental stages";
    category = "Process";
    sortOrder = 40;
    lines = [
      { prefix = "≡"; content = "transformation"; }
      { prefix = "≡"; content = "directional_change"; }
      { prefix = "≡"; content = "sequential_stages"; }
      { prefix = "≝"; content = "A→B→C unidirectional ∧ temporal progression ∧ irreversible"; }
      { prefix = "∂"; content = "⧃⊅⬢ (⧃ sequential | ⬢ simultaneous t₀=t₀)"; }
      { prefix = "∂"; content = "⧃⊅⟗ (⧃ unidirectional | ⟗ bidirectional feedback)"; }
      { prefix = "∂"; content = "⧃⊅⧆ (⧃ transforming | ⧆ static opposition)"; }
      { prefix = "∂"; content = "⧃⊅☯ (⧃ resolving | ☯ paradox persists)"; }
      { prefix = "∂"; content = "⧃⊅⧊ (⧃ specific transformation | ⧊ analyzing concept)"; }
      { prefix = "⊡"; content = "⧃: Շ7Գ6Բ8"; }
      { prefix = "⊡"; content = "⧃⦑Medicalization⦒: Շ8Գ7Բ9"; }
      { prefix = "⊛"; content = "⧃⦑Medicalization|Secularization|Professionalization|Rationalization|Bureaucratization⦒"; }
      { prefix = "⊛"; content = "⧃⦑Commodification|Financialization|Digitization|Globalization⦒"; }
      { prefix = "⊛"; content = "⧃⦑Democratization|Decolonization|Industrialization|Urbanization⦒"; }
      { prefix = "⊛"; content = "⧃⦑Magic→Religion→Science|Primitive→Modern Schema|Gemeinschaft→Gesellschaft⦒"; }
      { prefix = "◻"; content = "naming: ⧃⦑[Process Name]⦒ | ⧃⦑A→B→C⦒ for staged"; }
      { prefix = "◻"; content = "criteria: t₀<t₁<t₂ ∧ unidirectional→ ∧ irreversible"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⧃⦑Medicalization⦒: condition→medical frame (unidirectional, no return)"; }
      { prefix = "◻"; content = "  ⬢⦑Making Up People⦒: classification constitutes classified (t₀=t₀)"; }
      { prefix = "◻"; content = "  ⟗⦑Multiple Personality Loop⦒: diagnosis↔behavior (bidirectional)"; }
      { prefix = "◻"; content = "  ⧊⦑Transformation⦒: analyzing concept ¬specific process"; }
      { prefix = "≟"; content = "test{specific transformation?→continue | analyzing concept?→⧊}"; }
      { prefix = "≟"; content = "test{t₀<t₁<t₂ sequential?→YES:⧃|simultaneous?→⬢}"; }
      { prefix = "≟"; content = "test{unidirectional?→YES:⧃|bidirectional?→⟗}"; }
      { prefix = "≟"; content = "test{irreversible?→YES:⧃|cyclical?→⟗|static?→⧆}"; }
      { prefix = "⊨"; content = "⧃ ≡ unidirectional ∧ sequential ∧ irreversible"; }
    ];
  };
  symbolic-action = {
    symbol = "⟴";
    name = "Symbolic Action";
    description = "Burkean symbolic action with meaningful dimension and human agency";
    category = "Process";
    sortOrder = 41;
    lines = [
      { prefix = "≡"; content = "symbolic_action"; }
      { prefix = "≡"; content = "meaningful_intervention"; }
      { prefix = "≡"; content = "rhetorical_performance"; }
      { prefix = "≝"; content = "Burkean action ∧ agency-requiring ∧ interpretive dimension"; }
      { prefix = "∂"; content = "⟴⊅{mechanical, passive, automatic}"; }
      { prefix = "∂"; content = "⟴⊅⬢ (⟴ performs | ⬢ constitutes)"; }
      { prefix = "∂"; content = "⟴⊅⥅ (⟴ acts | ⥅ anticipates)"; }
      { prefix = "∂"; content = "⟴⊅◬ (⟴ agentive | ◬ emergent)"; }
      { prefix = "∂"; content = "⟴⊅⌁ (⟴ type of action | ⌁ specific occurrence)"; }
      { prefix = "⊛"; content = "Burkean: ⟴⦑Scapegoating|Mortification|Purification|Identification|Transcendence⦒"; }
      { prefix = "⊛"; content = "ritual: ⟴⦑Coronation|Inauguration|Ordination|Oath-Taking|Consecration⦒"; }
      { prefix = "⊛"; content = "speech: ⟴⦑Declaration|Proclamation|Denunciation|Confession|Testimony⦒"; }
      { prefix = "⊛"; content = "exclusion: ⟴⦑Excommunication|Censure|Banishment|Ostracism⦒"; }
      { prefix = "⊛"; content = "memory: ⟴⦑Commemoration|Dedication|Memorial|Canonization⦒"; }
      { prefix = "◻"; content = "symbolic_dimension ∧ human_agency"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  'X performs Y'→⟴ | 'X creates Y'→⬢ | 'X emerges'→◬"; }
      { prefix = "◻"; content = "  ⟴⦑Scapegoating⦒ (type) vs ⌁⦑Trial of Socrates⦒ (instance)"; }
      { prefix = "≟"; content = "test{meaningful∧interpretive?→continue | mechanical?→∅}"; }
      { prefix = "≟"; content = "test{human∧social_agent?→continue | automatic?→◬}"; }
      { prefix = "≟"; content = "test{type of action?→⟴ | specific occurrence?→⌁}"; }
      { prefix = "⊨"; content = "⟴ ≡ symbolic_action ∧ human_agency ∧ ⟴⊅{mechanical,automatic}"; }
    ];
  };
  effect = {
    symbol = "⬢";
    name = "Effect";
    description = "Performative reality-making with instantaneous constitution (t₀=t₀)";
    category = "Process";
    sortOrder = 42;
    lines = [
      { prefix = "≡"; content = "performative_effect"; }
      { prefix = "≡"; content = "reality_making"; }
      { prefix = "≡"; content = "instantaneous_constitution"; }
      { prefix = "≝"; content = "classification→reality simultaneity ∧ constitutive power ∧ t₀=t₀"; }
      { prefix = "∂"; content = "⬢⊅⥅ (⬢ simultaneous | ⥅ temporal gap)"; }
      { prefix = "∂"; content = "⬢⊅◬ (⬢ designed | ◬ emergent)"; }
      { prefix = "∂"; content = "⬢⊅⟗ (⬢ single | ⟗ iterating)"; }
      { prefix = "∂"; content = "⬢⊅⧃ (⬢ constitutes | ⧃ transforms)"; }
      { prefix = "∂"; content = "⬢⊅⧊ (⬢ specific effect | ⧊ analyzing concept of effect)"; }
      { prefix = "⊡"; content = "⬢: Պ7Փ5Տ4"; }
      { prefix = "⊡"; content = "⬢⦑Making Up People⦒: Պ8Փ6Տ3"; }
      { prefix = "⊛"; content = "⬢⦑Making Up People|Classification Creates Order|Ritual Creates Experience⦒"; }
      { prefix = "⊛"; content = "⬢⦑Prediction Creates Reality|Statistics Create Reality|Naming Creates Existence⦒"; }
      { prefix = "⊛"; content = "⬢⦑Bandwagon Effect|Pygmalion Effect|Observer Effect⦒"; }
      { prefix = "◻"; content = "simultaneity CRITICAL:"; }
      { prefix = "◻"; content = "  t₀classify=t₀create — 'classified AS X makes one X'"; }
      { prefix = "◻"; content = "  ✓⬢ 'ritual=reality' | ✗⬢ 'shapes future behavior'→⥅"; }
      { prefix = "◻"; content = "  ANY temporal gap disqualifies ⬢"; }
      { prefix = "◻"; content = "naming: 'X Creates Y' | gerunds | canonical effects"; }
      { prefix = "◻"; content = "  'creates'→simultaneity | 'leads to'→temporal gap"; }
      { prefix = "◻"; content = "discrimination:"; }
      { prefix = "◻"; content = "  ⬢⦑Making Up People⦒: classification constitutes classified (t₀=t₀)"; }
      { prefix = "◻"; content = "  ⟗⦑Multiple Personality Formation Loop⦒: same mechanism + iteration (1983→1991)"; }
      { prefix = "◻"; content = "  ⧃⦑Medicalization⦒: unidirectional transformation (t₀<t₁)"; }
      { prefix = "◻"; content = "  ⧊⦑Performativity⦒: analyzing concept ¬specific effect"; }
      { prefix = "≟"; content = "test{specific effect?→continue | analyzing concept?→⧊}"; }
      { prefix = "≟"; content = "test{t₀=t₀?→⬢|NO:continue}"; }
      { prefix = "≟"; content = "test{t₀<t₁ precedent?→⥅|NO:continue}"; }
      { prefix = "≟"; content = "test{emergent?→◬|NO:continue}"; }
      { prefix = "≟"; content = "test{iterating?→⟗|NO:continue}"; }
      { prefix = "≟"; content = "test{transforming?→⧃|NO:review}"; }
      { prefix = "⊨"; content = "⬢ ≡ performative_constitution ∧ t₀=t₀ ∧ ¬temporal_gap"; }
    ];
  };
  work = {
    symbol = "⊳";
    name = "Work";
    description = "Work title in compound citations (main title only, no subtitles)";
    category = "Compound";
    sortOrder = 50;
    lines = [
      { prefix = "≡"; content = "work_title"; }
      { prefix = "≡"; content = "citation_component"; }
      { prefix = "≡"; content = "compound_only"; }
      { prefix = "≝"; content = "bibliographic reference ∧ main title only ∧ □compound form"; }
      { prefix = "∂"; content = "⊳⊅𝄏 (⊳ standalone work | 𝄏 serial periodical)"; }
      { prefix = "∂"; content = "⊳⊅⍚ (⊳ work | ⍚ publisher organization)"; }
      { prefix = "∂"; content = "✗⊳⦑Title⦒ standalone — □ requires ⚘⊙ prefix"; }
      { prefix = "⊛"; content = "⚘⊙⊳⦑Thomas S. Kuhn⦒⦑1962⦒⦑The Structure of Scientific Revolutions⦒"; }
      { prefix = "⊛"; content = "⚘⊙⊳⦑Michel Foucault⦒⦑1975⦒⦑Discipline and Punish⦒"; }
      { prefix = "⊛"; content = "⚘⊙⊳⦑Edward Said⦒⦑1978⦒⦑Orientalism⦒"; }
      { prefix = "⊛"; content = "⚘⊙⊳⦑Kenneth Burke⦒⦑1945⦒⦑A Grammar of Motives⦒"; }
      { prefix = "⊛"; content = "⚘⊙𝄏⊳⦑C.S. Peirce⦒⦑1878⦒⦑Popular Science Monthly⦒⦑How to Make Our Ideas Clear⦒"; }
      { prefix = "◻"; content = "□ compound only: ✓⚘⊙⊳ | ✓⚘⊙𝄏⊳ | ✗⊳ standalone"; }
      { prefix = "◻"; content = "CRITICAL — main title only:"; }
      { prefix = "◻"; content = "  ✓⊳⦑A Matter of Justice⦒"; }
      { prefix = "◻"; content = "  ✗⊳⦑A Matter of Justice: The Legal System in Ferment⦒"; }
      { prefix = "◻"; content = "co-authors: ⚘⊙⊳⦑Author1∧Author2⦒⦑Year⦒⦑Title⦒"; }
      { prefix = "≟"; content = "test{compound form ⚘⊙?→YES:continue|NO:reject}"; }
      { prefix = "≟"; content = "test{one-time publication?→YES:⊳|NO:continue}"; }
      { prefix = "≟"; content = "test{periodic issues?→YES:𝄏|NO:review}"; }
      { prefix = "⊨"; content = "⊳ ≡ citation_component ∧ □compound_form ∧ main_title_only"; }
    ];
  };
  meta = {
    symbol = "⑀";
    name = "Meta";
    description = "User-defined meta marker for arbitrary classification";
    category = "UserDefined";
    sortOrder = 100;
    lines = [
      { prefix = "≡"; content = "status_marker"; }
      { prefix = "≡"; content = "arbitrary_classification"; }
      { prefix = "≡"; content = "flexible_tag"; }
      { prefix = "≝"; content = "user-defined semantics ∧ no fixed meaning"; }
      { prefix = "∂"; content = "∂ user-defined (¬ERIS detection)"; }
      { prefix = "∂"; content = "⑀⊅{⟡,⋯,◈} (⑀ arbitrary | ⟡⋯◈ structured)"; }
      { prefix = "◻"; content = "catch-all: workflow states | review flags | temporary markers"; }
      { prefix = "⊨"; content = "⑀ ≡ user_defined ∧ arbitrary_semantics"; }
    ];
  };
  question = {
    symbol = "⋯";
    name = "Question";
    description = "User-defined question marker for generative inquiry";
    category = "UserDefined";
    sortOrder = 101;
    lines = [
      { prefix = "≡"; content = "generative_inquiry"; }
      { prefix = "≡"; content = "fertile_unknown"; }
      { prefix = "≡"; content = "orienting_problem"; }
      { prefix = "≝"; content = "user's own question ∧ drives work ∧ anti-crystallization"; }
      { prefix = "∂"; content = "∂ user-defined (¬ERIS detection)"; }
      { prefix = "∂"; content = "⋯⊅⊟ (⋯ fertile | ⊟ needs resolution)"; }
      { prefix = "∂"; content = "⋯⊅⌾ (⋯ drives work | ⌾ seeks clarification)"; }
      { prefix = "∂"; content = "⋯⊅◈ (⋯ question | ◈ bounded project)"; }
      { prefix = "◻"; content = "lifecycle: ⋯⊰◈ (questions generate projects)"; }
      { prefix = "◻"; content = "accretes: ❧→⋯ (highlights gather to questions)"; }
      { prefix = "◻"; content = "persistence: ⋯ holds ¬⌺ | ⋯ may persist through ◈_⌺"; }
      { prefix = "⊨"; content = "⋯ ≡ user_owned_question ∧ generative ∧ user-defined"; }
    ];
  };
  project = {
    symbol = "◈";
    name = "Project";
    description = "User-defined project marker for bounded endeavors";
    category = "UserDefined";
    sortOrder = 102;
    lines = [
      { prefix = "≡"; content = "project_marker"; }
      { prefix = "≡"; content = "bounded_endeavor"; }
      { prefix = "≡"; content = "goal_oriented"; }
      { prefix = "≝"; content = "user's own project ∧ scoped work ∧ aggregates ❧"; }
      { prefix = "∂"; content = "∂ user-defined (¬ERIS detection)"; }
      { prefix = "∂"; content = "◈⊅⟡ (◈ bounded | ⟡ nascent idea)"; }
      { prefix = "∂"; content = "◈⊅⋯ (◈ project | ⋯ orienting question)"; }
      { prefix = "⊛"; content = "◈⦑Binding Time|LLMs and Feedforward|Tainted Software⦒"; }
      { prefix = "⊛"; content = "◈⦑Ronpaulization|The Intimate Machine|Classifier Stalinism⦒"; }
      { prefix = "⊛"; content = "◈⦑Context Widows|Politically Unreliable|Secularism Studies Project⦒"; }
      { prefix = "◻"; content = "lifecycle: ⟡→◈ (idea matures) | ◈→⟡ (project spawns idea)"; }
      { prefix = "◻"; content = "aggregates: ❧→◈ (highlights gather to projects)"; }
      { prefix = "◻"; content = "orients: ⋯⊰◈ (questions generate projects)"; }
      { prefix = "⊨"; content = "◈ ≡ user_owned_project ∧ bounded ∧ user-defined"; }
    ];
  };
  idea = {
    symbol = "⟡";
    name = "Idea";
    description = "User-defined idea marker for user's own conceptual seeds";
    category = "UserDefined";
    sortOrder = 103;
    lines = [
      { prefix = "≡"; content = "idea_marker"; }
      { prefix = "≡"; content = "conceptual_seed"; }
      { prefix = "≡"; content = "owned_thought"; }
      { prefix = "≝"; content = "user's own idea ∧ implementation potential"; }
      { prefix = "∂"; content = "∂ user-defined (¬ERIS detection)"; }
      { prefix = "∂"; content = "⟡⊅⧊ (⟡ mine | ⧊ in the world)"; }
      { prefix = "∂"; content = "⟡⊅◈ (⟡ nascent | ◈ bounded project)"; }
      { prefix = "◻"; content = "lifecycle: ⟡→◈ (idea matures) | ◈→⟡ (project spawns idea)"; }
      { prefix = "⊨"; content = "⟡ ≡ user_owned_idea ∧ user-defined"; }
    ];
  };
  section = {
    symbol = "§";
    name = "Section";
    description = "User-defined section marker for document divisions";
    category = "UserDefined";
    sortOrder = 104;
    lines = [
      { prefix = "≡"; content = "section_marker"; }
      { prefix = "≡"; content = "document_division"; }
      { prefix = "≡"; content = "organizational_unit"; }
      { prefix = "≝"; content = "bounded scope ∧ within ◈ or ⊳"; }
      { prefix = "∂"; content = "∂ user-defined (¬ERIS detection)"; }
      { prefix = "∂"; content = "§⊂◈ (§ divisions within projects)"; }
      { prefix = "∂"; content = "§⊂⊳ (§ divisions within works)"; }
      { prefix = "⊛"; content = "◈§⦑Binding Time⦒⦑The Archive Problem⦒"; }
      { prefix = "⊛"; content = "◈§⦑LLMs and Feedforward⦒⦑Richards and Anticipation⦒"; }
      { prefix = "⊛"; content = "◈§⦑Tainted Software⦒⦑Contagion Mechanics⦒"; }
      { prefix = "◻"; content = "notation: ◈§⦑Project⦒⦑Section⦒ | §⦑Section⦒ when ◈ implicit"; }
      { prefix = "⊨"; content = "§ ≡ user_defined ∧ subdivision ∧ bounded_scope"; }
    ];
  };
}
