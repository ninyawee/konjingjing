# Algorithm

## Thai National ID Structure

A Thai National ID consists of 13 digits with the following structure:

```
┌─┬──┬──┬───────┬─┐
│1│23│45│6789012│3│
└─┴──┴──┴───────┴─┘
 │ │  │    │     └─ Check digit (digit 13)
 │ │  │    └─────── Serial number (digits 6-12)
 │ │  └──────────── District code (digits 4-5)
 │ └─────────────── Province code (digits 2-3)
 └───────────────── Person type (digit 1)
```

### Digit Breakdown

| Position | Name | Description |
|----------|------|-------------|
| 1 | Person Type | Category of person (1-8) |
| 2-3 | Province | Province code (10-96) |
| 4-5 | District | Amphoe code (continues from province) |
| 6-12 | Serial | Unique serial number |
| 13 | Check Digit | Checksum validation digit |

## Checksum Algorithm

The check digit (digit 13) is calculated from the first 12 digits using a weighted sum algorithm.

### Steps

1. **Validate format**: Ensure the input is exactly 13 ASCII digits
2. **Calculate weighted sum**: Multiply each of the first 12 digits by a weight
3. **Compute check digit**: Apply modulo operations to get the expected check digit
4. **Compare**: Verify the calculated check digit matches digit 13

### Formula

```
sum = Σ (digit[i] × (13 - i))  for i = 0 to 11

check_digit = (11 - (sum mod 11)) mod 10
```

### Example Calculation

For ID `1112034563562`:

| Position (i) | Digit | Weight (13-i) | Product |
|--------------|-------|---------------|---------|
| 0 | 1 | 13 | 13 |
| 1 | 1 | 12 | 12 |
| 2 | 1 | 11 | 11 |
| 3 | 2 | 10 | 20 |
| 4 | 0 | 9 | 0 |
| 5 | 3 | 8 | 24 |
| 6 | 4 | 7 | 28 |
| 7 | 5 | 6 | 30 |
| 8 | 6 | 5 | 30 |
| 9 | 3 | 4 | 12 |
| 10 | 5 | 3 | 15 |
| 11 | 6 | 2 | 12 |
| **Total** | | | **207** |

```
sum = 207
sum mod 11 = 207 mod 11 = 9
11 - 9 = 2
2 mod 10 = 2

Expected check digit: 2
Actual digit 13: 2 ✓
```

### Rust Implementation

```rust
pub fn verify_id(id: &str) -> bool {
    // Check length and all digits
    if id.len() != 13 || !id.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }

    let digits: Vec<u32> = id.bytes().map(|b| (b - b'0') as u32).collect();

    // Calculate weighted checksum
    let total: u32 = digits[..12]
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (13 - i as u32))
        .sum();

    let calculated_checksum = (11 - total % 11) % 10;

    calculated_checksum == digits[12]
}
```

## Validation Rules

An ID is considered **invalid** if:

- Length is not exactly 13 characters
- Contains any non-digit characters
- The check digit does not match the calculated value

!!! note
    The `verify_id` function only validates the checksum. It does not verify that the province or district codes are real. Use `get_id_meaning` to check if the embedded codes correspond to known locations.
