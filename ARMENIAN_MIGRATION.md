# Armenian Alphabet Migration

## Problem

The Armenian Unicode block (U+0530–U+058F) causes encoding issues when processed by some LLM systems. Characters may be misinterpreted, corrupted, or rendered incorrectly during tokenization.

## Goal

Migrate the 37 Armenian vector property symbols to a new, LLM-friendly alphabet while preserving semantic meaning and maintaining a supersession trail.

## Current Armenian Symbols

| Symbol | Name | Category | File |
|--------|------|----------|------|
| Ա | antisymmetric | Relational | antisymmetric.ron |
| Բ | boundary | Core | boundary.ron |
| Գ | generalizability | Core | generalizability.ron |
| Delays| explanatory | Core | explanatory.ron |
| Delays| intentionality | Core | intentionality.ron |
| խ | contextualization | Core | contextualization.ron |
| Լ | lifespan | Core | lifespan.ron |
| Փ | functional | Core | functional.ron |
| Վ | semantic_density | Core | semantic_density.ron |
| Ց | volatility | Core | volatility.ron |
| Տ | transitive | Relational | transitive.ron |
| Delays| symmetric | Relational | symmetric.ron |
| Ր | reflexive | Relational | reflexive.ron |
| Օ | total | Relational | total.ron |
| delays| symbolic_economy | Compression | symbolic_economy.ron |
| ռ | reference_efficiency | Compression | reference_efficiency.ron |
| ժ | recursion | SelfReference | recursion.ron |
| delays| coherence | SelfReference | coherence.ron |
| delays| naming | SelfReference | naming.ron |
| մ | metamodel | SelfReference | metamodel.ron |
| Delays| junction | SelfReference | junction.ron |
| ծ | conceptual_orthogonality | SystemCoherence | conceptual_orthogonality.ron |
| delays| knowledge_propagation | SystemCoherence | knowledge_propagation.ron |
| Մ | hierarchical_consistency | SystemCoherence | hierarchical_consistency.ron |
| Ձ | symbolic_stability | SystemCoherence | symbolic_stability.ron |
| ղ | quality_improvement | Evolution | quality_improvement.ron |
| Ճ | component_stability | Evolution | component_stability.ron |
| շ | simplicity_expressiveness | DesignBalance | simplicity_expressiveness.ron |
| Ո | orthogonality_integration | DesignBalance | orthogonality_integration.ron |
| Ք | consistency_specialization | DesignBalance | consistency_specialization.ron |
| Ֆ | formalization | SystemCoherence | formalization.ron |
| Պ | porosity | Relational | porosity.ron |
| Հ | hierarchy | SystemCoherence | hierarchy.ron |
| Շ | scope | Core | scope.ron |
| Ի | cohesion | SystemCoherence | cohesion.ron |
| Ղ | agency | Core | agency.ron |
| ⇀ | property_vector | Core | property_vector.ron |

## Migration Format

Each migrated definition will include a `supersedes` field:

```ron
(
    symbol: "X",           // New symbol from target alphabet
    name: "boundary",
    category: "Core",
    supersedes: "Բ",       // Original Armenian symbol
    // ... rest of definition
)
```

## Target Alphabet Candidates

| Option | Block | Range | Notes |
|--------|-------|-------|-------|
| Latin Extended | U+0100–U+024F | Ā-ɏ | Familiar, wide support |
| Cyrillic | U+0400–U+04FF | А-ӿ | Similar to Armenian issue? |
| Mathematical | U+1D400–U+1D7FF | 𝐀-𝟿 | Bold/italic variants |
| Box Drawing | U+2500–U+257F | ─-╿ | Geometric consistency |
| Geometric Shapes | U+25A0–U+25FF | ■-◿ | Visual clarity |
| Custom Ligatures | — | — | Composable from basic Latin |

## Migration Steps

1. **Select target alphabet** — Choose symbols with:
   - Wide Unicode support
   - LLM tokenization stability
   - Visual distinctiveness
   - Mnemonic connection to meaning

2. **Create mapping table** — Document old→new for all 37 symbols

3. **Update RON files** — Add `supersedes` field, change `symbol`

4. **Update loader** — Support `supersedes` field in `RonOperatorDef`

5. **Add migration CLI** — `eris migrate --check` to validate

6. **Update documentation** — CLAUDE.md, MIGRATION.md

7. **Create deprecation notice** — Old symbols remain valid for parsing but new output uses new symbols

## Backwards Compatibility

- Old tags with Armenian symbols continue to parse
- New output uses new alphabet
- `eris lookup` shows both old and new symbols
- Migration guide for existing tagged content

## Questions to Resolve

- [ ] Which target alphabet to use?
- [ ] Should we batch migrate or gradual rollout?
- [ ] How long to support dual-symbol recognition?
- [ ] Any symbols that should NOT migrate?

## Status

| Phase | Status |
|-------|--------|
| Problem identification | ✅ |
| Symbol inventory | ✅ |
| Target alphabet selection | ❌ |
| Mapping table | ❌ |
| Implementation | ❌ |
| Testing | ❌ |
| Documentation | ❌ |
