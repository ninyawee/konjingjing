# konjingjing

Thai Citizen ID validation library for Rust.

The library name 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

## Installation

```bash
cargo add konjingjing
```

## Usage

```rust
use konjingjing::verify_id;

fn main() {
    assert!(verify_id("1112034563562"));        // Valid
    assert!(!verify_id("1112034563563"));       // Invalid checksum
    assert!(!verify_id("11120345635"));         // Too short
}
```

## License

ISC
