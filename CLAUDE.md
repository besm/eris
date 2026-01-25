# ERIS

ERIS (Entity Reference and Information System) is a notation system for semantic tagging using Unicode symbols. It provides a formal vocabulary for classifying entities, expressing relationships, and structuring knowledge.

## Build & Test

```bash
cargo build              # Build all crates
cargo test               # Run tests
cargo run -p eris-cli    # Run CLI
```

## Post-Build Install

After building, install binaries to `~/bin`:

```bash
cargo build --release -p eris-cli -p eris-mcp
cp target/release/eris ~/bin/eris
cp target/release/eris-mcp ~/bin/eris-mcp
```

## Project Structure

```
crates/
├── eris/                # Core library
│   └── src/
│       ├── entities/    # 30 entity types (person, place, concept, etc.)
│       ├── operators/   # 7 operator categories (~60+ operators)
│       ├── notation/    # Tag parsing (compound, vector, reference, temporal)
│       ├── frame/       # Composable context specs (role, context, task)
│       ├── export.rs    # LLM prompt integration with prefix caching
│       ├── parsers/     # Tag validation utilities
│       ├── symbols.rs   # Unicode symbol constants
│       ├── macros.rs    # Code generation macros
│       └── lib.rs       # Public API
└── eris-cli/            # CLI binary
```

## Entity Types

30 entity types organized by category:

| Category | Entities |
|----------|----------|
| Primary | ⚘ Person, ⌖ Place, ⧖ Era, ⊙ Date, ⌁ Event, ❖ Field, ⧈ Group, ⍚ Organization, ⎈ Agency, ⌬ Tech |
| Institutional | ⎚ Identifier, ⍓ Publisher, ⍢ University, ⧩ Language, 𝄏 Journal |
| Conceptual | ⧊ Concept, ⧏ Method, ⌯ Movement |
| Relational | ⇋ Relation, ⧆ Tension, ⟗ Loop, ☯ Paradox |
| Process | ⧃ Evolution, ⟴ Action, ⬢ Effect |
| Compound | ⊳ Work (used in citations) |
| User-defined | ⑀ Meta, ⋯ Question, ◈ Project, ⟡ Idea |

## Operator Categories

| Category | Purpose | Examples |
|----------|---------|----------|
| Armenian | Property vectors (0-9 scale) | Փ (performativity), Գ (generalizability), Վ (validity) |
| Chronos | Temporal/teleological | ⍜ (purpose), ω (teleological vector), ι (initiation) |
| Georgian | Workflow states | Past/present/future contexts |
| Logical | Mathematical notation | ≡, ≝, →, ∧, ∨, ∀, ∃, ⊂, ◻, ⊨ |
| Meta | Self-referential | Documentation, schemas, examples |
| Ontology | Constitution/grounding | Creation, performativity, crystallization |
| Semantic | Meaning dynamics | Gravity wells, stability states |

## Notation Types

| Type | Brackets | Example |
|------|----------|---------|
| Entity | `⦑⦒` | `⚘⦑Mary Douglas⦒` |
| Compound | `⦑⦒` | `⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒` |
| Vector | `⟨⟩` | `⊡⟨Փ9Գ8⟩` |
| Reference | `⟦⟧` | `❧⟦12345⟧` |
| Date | `⊙⦑⦒` | `⊙⦑1984⦒`, `⊙⦑2023-05-15⦒` |
| Era | `⧖⦑⦒` | `⧖⦑1980s⦒`, `⧖⦑Nineteenth Century⦒` |

### Compound Notation Patterns

Defined in `notation/entity.rs`:

- **BookCitation**: `⚘⊙⊳⦑Author⦒⦑Year⦒⦑Title⦒`
- **ArticleCitation**: `⚘⊙𝄏⊳⦑Author⦒⦑Year⦒⦑Journal⦒⦑Title⦒`
- **OrgBookCitation**: `⍚⊙⊳⦑Org⦒⦑Year⦒⦑Title⦒`
- **OrgArticleCitation**: `⍚⊙𝄏⊳⦑Org⦒⦑Year⦒⦑Journal⦒⦑Title⦒`
- **DatedEvent**: `⌁⊙⦑Event⦒⦑Year⦒`

Multi-author uses `∧` conjunction: `⚘⊙⊳⦑Lakoff∧Johnson⦒⦑1980⦒⦑Title⦒`

## Definition Line Prefixes

All entity/operator definitions use standardized prefixes:

| Prefix | Meaning |
|--------|---------|
| `≡` | Equivalence/name |
| `≝` | Defined as/essence |
| `∂` | Boundary/exclusions |
| `⊛` | Pattern examples |
| `◻` | Constraints/requirements |
| `≟` | Discrimination rules |
| `⊨` | Validation/evidence |
| `⊡` | Armenian property vector |

## Frame System

Composable operational framing (distinct from entities):

- **Role** (Ψ): nav, pln, evl, crt, itg
- **Context** (⯐): ann, wfl, str, eps
- **Task** (τ): validate, tag, review, migrate, query

Composed spec: `⟜⟨Ψ.nav⊗⯐.wfl⊗τ.tag⟩`

Accessed via CLI flags, NOT included in `eris all`.

## Adding Entities

1. Create module in `entities/` using `define_entity_module!`:

```rust
define_entity_module! {
    Entity {
        Primary => "Primary entity types",
    }
}

pub fn get_entity_definitions() -> Vec<EntityDef> {
    vec![
        EntityDef {
            symbol: "⚘",
            name: "Person",
            description: "Named individual",
            sort_order: 1,
            category: EntityCategory::Primary,
            lines: lines![
                ("≡", "person ∧ named individual"),
                ("≝", "human agent"),
                // ...
            ],
        },
    ]
}
```

2. Register in `entities/mod.rs` via `aggregate_entities!`

## Adding Operators

1. Create module in `operators/` using `define_operator_module!`:

```rust
define_operator_module! {
    Logical {
        Core => "Core logical operators",
        Quantifier => "Universal/existential",
    }
}

pub fn get_logical_operator_definitions() -> Vec<LogicalOperatorDef> {
    vec![
        LogicalOperatorDef {
            symbol: "≡",
            name: "Equivalence",
            category: LogicalOperatorCategory::Core,
            lines: lines![("≡", "identity ∧ interchangeable")],
        },
    ]
}
```

2. Register in `operators/mod.rs` via `aggregate_operators!`

## CLI Usage

```bash
eris list                    # List all symbols
eris all                     # Output all definitions
eris ops                     # Operators only
eris entities                # Entities only
eris ⚘                       # Look up single symbol
eris define <file>           # Definitions for symbols used in file
eris closure [-c]            # Find undefined symbols (with counts)
eris workflow tag            # Get workflow spec

# Frame flags
eris --role evl --task tag   # Compose frame specification
eris --roles                 # List all roles
eris --contexts              # List all contexts
eris --tasks                 # List all tasks
```

## LLM Integration

`export.rs` provides cached definitions for LLM prompts:

```rust
use eris::{definitions_for_prompt, system_prompt_base};

// Full definitions (cached)
let defs = definitions_for_prompt();

// Base system prompt with definitions
let prompt = system_prompt_base();
```

Optimized for DeepSeek prefix caching (~10% cost with cache hits).

## Key Types

```rust
// Parsed compound tag
struct CompoundTag {
    symbols: Vec<char>,      // ['⚘', '⊙', '⊳']
    components: Vec<String>, // ["Author", "Year", "Title"]
}

// Entity type enum (generated by aggregate_entities!)
enum EntityType {
    Person, Place, Era, Date, Event, ...
}

// Frame specification
struct Frame {
    role: Option<Role>,
    context: Option<Context>,
    task: Option<Task>,
}
```

## Testing

Tests are inline in each module. Key test areas:

- Tag parsing (simple, compound, multi-author)
- Notation pattern matching and named access
- Entity type detection and discrimination
- Unicode and special character handling
- Roundtrip rendering
- Implied tag generation
