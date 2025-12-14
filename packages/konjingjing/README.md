# konjingjing

Thai Citizen ID validation library for Rust.

The library name 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

## Installation

```bash
cargo add konjingjing
```

## Usage

### Verify ID

```rust
use konjingjing::verify_id;

assert!(verify_id("1112034563562"));        // Valid
assert!(!verify_id("1112034563563"));       // Invalid checksum
assert!(!verify_id("11120345635"));         // Too short
```

### Extract ID Meaning

```rust
use konjingjing::get_id_meaning;

let result = get_id_meaning("1101700230703").unwrap();
assert_eq!(result.person_type.code, 1);
assert_eq!(result.person_type.description_th, "คนไทยที่แจ้งเกิดภายในกำหนด");
assert_eq!(result.person_type.description_en, "Thai citizen, birth registered on time");
assert_eq!(result.province.unwrap().name_en, "Bangkok");
assert_eq!(result.amphoe.unwrap().name_th, "ห้วยขวาง");
assert!(result.is_valid);

assert!(get_id_meaning("invalid").is_none());
```

## License

ISC
