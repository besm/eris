# Property Vector Symbol Migration

## Problem

The Armenian Unicode block (U+0530–U+058F) causes encoding issues when processed by some LLM systems. Characters may be misinterpreted, corrupted, or rendered incorrectly during tokenization.

## Goal

Migrate the 36 Armenian vector property symbols to Tifinagh (U+2D30–U+2D7F) while preserving semantic meaning and maintaining a supersession trail.

## Target Alphabet

**Tifinagh** (U+2D30–U+2D7F) — 59 characters available, 36 needed.

- Berber/Tuareg script with geometric aesthetic
- Good BMP font support (Noto Sans Tifinagh)
- No collisions with existing ERIS symbols
- 23 spare characters for future expansion

## Migration Status

| Cluster | Count | Status | Directory |
|---------|-------|--------|-----------|
| Stability | 4 | ✅ | `vectors/stability/` |
| Relational | 6 | ✅ | `vectors/relational/` |
| Core | 10 | ❌ | — |
| SelfReference | 5 | ❌ | — |
| SystemCoherence | 6 | ❌ | — |
| Compression | 2 | ❌ | — |
| Evolution | 1 | ❌ | — |
| DesignBalance | 3 | ❌ | — |

**Progress:** 10/36 (28%)

## Completed Migrations

### Stability (4 symbols)

| Old | New | Name | File |
|-----|-----|------|------|
| Ց | ⵥ | volatility | `stability/volatility.ron` |
| Լ | ⵜ | lifespan | `stability/lifespan.ron` |
| Ճ | ⵛ | component_stability | `stability/component.ron` |
| Ձ | ⵙ | symbolic_stability | `stability/symbolic.ron` |

### Relational (6 symbols)

| Old | New | Name | File |
|-----|-----|------|------|
| Ա | ⵔ | antisymmetric | `relational/antisymmetric.ron` |
| Տ | ⵕ | transitive | `relational/transitive.ron` |
| Ս | ⵖ | symmetric | `relational/symmetric.ron` |
| Ր | ⵅ | reflexive | `relational/reflexive.ron` |
| Օ | ⵀ | total | `relational/total.ron` |
| Պ | ⵒ | porosity | `relational/porosity.ron` |

## Pending Migrations

### Core (10 symbols)

| Old | New | Name |
|-----|-----|------|
| Բ | — | boundary |
| Գ | — | generalizability |
| Delays | — | explanatory |
| Ը | — | intentionality |
| խ | — | contextualization |
| Փ | — | functional |
| Վ | — | semantic_density |
| Շ | — | scope |
| Ղ | — | agency |
| ⇀ | ⇀ | property_vector (keep) |

### SelfReference (5 symbols)

| Old | New | Name |
|-----|-----|------|
| delays | — | recursion |
| delays | — | coherence |
| delays | — | naming |
| delays | — | metamodel |
| Յ | — | junction |

### SystemCoherence (6 symbols)

| Old | New | Name |
|-----|-----|------|
| ծ | — | conceptual_orthogonality |
| delays | — | knowledge_propagation |
| Մ | — | hierarchical_consistency |
| Ֆ | — | formalization |
| Delays | — | hierarchy |
| Ի | — | cohesion |

### Compression (2 symbols)

| Old | New | Name |
|-----|-----|------|
| delays | — | symbolic_economy |
| ռ | — | reference_efficiency |

### Evolution (1 symbol)

| Old | New | Name |
|-----|-----|------|
| ղ | — | quality_improvement |

### DesignBalance (3 symbols)

| Old | New | Name |
|-----|-----|------|
| շ | — | simplicity_expressiveness |
| Ո | — | orthogonality_integration |
| Ք | — | consistency_specialization |

## Implementation

### RON Format

```ron
(
    symbol: "ⵥ",
    name: "volatility",
    category: "Stability",
    supersedes: Some("Ց"),
    lines: [...],
)
```

### Loader Support

- `supersedes` field added to `RonOperatorDef`
- `get_armenian_operator()` finds by new OR old symbol
- Categories renamed: `ARMENIAN_CATEGORIES` → `PROPERTY_CATEGORIES`

### Directory Structure

```
defs/vectors/
├── stability/           # Migrated
│   ├── volatility.ron   # ⵥ
│   ├── lifespan.ron     # ⵜ
│   ├── component.ron    # ⵛ
│   └── symbolic.ron     # ⵙ
├── relational/          # Planned
├── core/                # Planned
├── selfreference/       # Planned
├── coherence/           # Planned
├── compression/         # Planned
├── evolution/           # Planned
├── balance/             # Planned
└── *.ron                # Legacy (unmigrated)
```

## Backwards Compatibility

- Old tags with Armenian symbols continue to parse via `supersedes` lookup
- New output uses Tifinagh symbols
- `eris lookup Ց` returns the ⵥ definition
- Migration guide for existing tagged content (future)

## Questions Resolved

- [x] Which target alphabet? → **Tifinagh**
- [x] Batch or gradual? → **Gradual by semantic cluster**
- [ ] How long to support dual-symbol recognition? → TBD
- [x] Any symbols that should NOT migrate? → **⇀ (property_vector)** stays
