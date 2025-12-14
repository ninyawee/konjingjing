# Rust API

## Installation

```toml
[dependencies]
konjingjing = "0.2"
```

## Functions

### `verify_id`

Verify a Thai Citizen Card ID checksum.

```rust
pub fn verify_id(id: &str) -> bool
```

**Arguments:**

- `id` - A string slice containing the 13-digit citizen ID

**Returns:**

- `true` if the ID passes checksum validation
- `false` otherwise (wrong length, non-digits, or invalid checksum)

**Example:**

```rust
use konjingjing::verify_id;

assert!(verify_id("1112034563562"));   // Valid
assert!(!verify_id("1112034563563"));  // Invalid checksum
assert!(!verify_id("11120345635"));    // Too short
```

---

### `get_id_meaning`

Extract meaning from a Thai National ID.

```rust
pub fn get_id_meaning(id: &str) -> Option<IdMeaning>
```

**Arguments:**

- `id` - A string slice containing the 13-digit citizen ID

**Returns:**

- `Some(IdMeaning)` with extracted information
- `None` if the ID is not 13 digits or contains non-digit characters

**Example:**

```rust
use konjingjing::get_id_meaning;

let meaning = get_id_meaning("1101700230703").unwrap();
assert_eq!(meaning.person_type.code, 1);
assert_eq!(meaning.province.unwrap().code, 10);
```

## Types

### `IdMeaning`

Result of extracting meaning from a Thai National ID.

```rust
pub struct IdMeaning {
    /// Person type information (digit 1)
    pub person_type: PersonType,
    /// Province information (digits 2-3), if found
    pub province: Option<Province>,
    /// Amphoe/district information (digits 2-5), if found
    pub amphoe: Option<Amphoe>,
    /// Whether the ID passes checksum validation
    pub is_valid: bool,
}
```

---

### `PersonType`

Person type information extracted from digit 1.

```rust
pub struct PersonType {
    pub code: u8,
    pub description_th: &'static str,
    pub description_en: &'static str,
}
```

---

### `Province`

Province information extracted from digits 2-3.

```rust
pub struct Province {
    pub code: u8,
    pub name_th: &'static str,
    pub name_en: &'static str,
}
```

---

### `Amphoe`

District (amphoe) information extracted from digits 2-5.

```rust
pub struct Amphoe {
    pub code: u16,
    pub name_th: &'static str,
}
```

## Complete Example

```rust
use konjingjing::{verify_id, get_id_meaning};

fn main() {
    let id = "1101700230703";

    // Simple validation
    if verify_id(id) {
        println!("ID is valid");
    }

    // Full information extraction
    if let Some(meaning) = get_id_meaning(id) {
        println!("Person Type: {} - {}",
            meaning.person_type.code,
            meaning.person_type.description_en);

        if let Some(province) = meaning.province {
            println!("Province: {} ({})", province.name_en, province.name_th);
        }

        if let Some(amphoe) = meaning.amphoe {
            println!("District: {}", amphoe.name_th);
        }

        println!("Checksum valid: {}", meaning.is_valid);
    }
}
```
