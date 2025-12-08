# ERIS

Crazy person stuff, don't look.


ERIS (Entity Reference and Information System) is a notation system for semantic tagging using Unicode symbols.

## Overview

ERIS provides:

- **Entity types** - symbols for semantic categories (⚘ Person, ⌖ Place, ⧊ Concept, etc.)
- **Operators** - relationship and property markers
- **Notation parsing** - parse and validate ERIS tag formats
- **Frames** - composable context specifications for LLM workflows

## Notation Types

| Type | Brackets | Example |
|------|----------|---------|
| Entity | `⦑⦒` | `⚘⦑Mary Douglas⦒` |
| Compound | `⦑⦒` | `⚘⊙⊳⦑George Orwell⦒⦑1949⦒⦑Nineteen Eighty-Four⦒` |
| Vector | `⟨⟩` | `⊡⟨Փ9Գ8⟩` |
| Reference | `⟦⟧` | `❧⟦12345⟧` |
| Date | `⊙⦑⦒` | `⊙⦑1984⦒`, `⊙⦑2023-05-15⦒` |
| Era | `⧖⦑⦒` | `⧖⦑1980s⦒`, `⧖⦑Nineteenth Century⦒` |

## Usage

### Library

```rust
use eris::notation::{CompoundTag, parse_date_tag, is_era_tag};

// Parse a compound tag
let tag = CompoundTag::parse("⚘⊙⊳⦑Author⦒⦑1984⦒⦑Title⦒").unwrap();
assert_eq!(tag.get(0), Some("Author"));

// Parse a date tag
assert_eq!(parse_date_tag("⊙⦑1984⦒"), Some(1984));

// Check era tags
assert!(is_era_tag("⧖⦑1980s⦒"));
```

### CLI

```bash
# List all symbols
eris list

# Get all definitions
eris all

# Look up a symbol
eris ⚘

# Get workflow specification
eris workflow tag
```

## License

MIT
