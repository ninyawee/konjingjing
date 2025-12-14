# Getting Started

## Installation

=== "Rust"

    Add to your `Cargo.toml`:

    ```toml
    [dependencies]
    konjingjing = "0.2"
    ```

    Or use cargo:

    ```bash
    cargo add konjingjing
    ```

=== "Python"

    Requires Python 3.9+:

    ```bash
    pip install konjingjing
    ```

=== "Node.js"

    ```bash
    npm install konjingjing
    ```

    Or with other package managers:

    ```bash
    yarn add konjingjing
    pnpm add konjingjing
    bun add konjingjing
    ```

=== "Browser (WASM)"

    ```bash
    npm install konjingjing-wasm
    ```

    Or use directly from a CDN:

    ```html
    <script type="module">
      import init, { verifyId } from 'https://unpkg.com/konjingjing-wasm/konjingjing_wasm.js';
      await init();
    </script>
    ```

## Basic Usage

### Validating an ID

The `verify_id` function checks if a Thai National ID has a valid checksum:

=== "Rust"

    ```rust
    use konjingjing::verify_id;

    assert!(verify_id("1112034563562"));   // Valid
    assert!(!verify_id("1112034563563"));  // Invalid checksum
    assert!(!verify_id("11120345635"));    // Too short
    ```

=== "Python"

    ```python
    from konjingjing import verify_id

    verify_id("1112034563562")   # True
    verify_id("1112034563563")   # False (invalid checksum)
    verify_id("11120345635")     # False (too short)
    ```

=== "Node.js"

    ```typescript
    import { verifyId } from 'konjingjing';

    verifyId("1112034563562");   // true
    verifyId("1112034563563");   // false (invalid checksum)
    verifyId("11120345635");     // false (too short)
    ```

=== "Browser (WASM)"

    ```typescript
    import init, { verifyId } from 'konjingjing-wasm';

    await init();  // Required once before using any function

    verifyId("1112034563562");   // true
    verifyId("1112034563563");   // false (invalid checksum)
    verifyId("11120345635");     // false (too short)
    ```

### Extracting ID Information

The `get_id_meaning` function extracts demographic information:

=== "Rust"

    ```rust
    use konjingjing::get_id_meaning;

    if let Some(meaning) = get_id_meaning("1101700230703") {
        println!("Person type: {} ({})",
            meaning.person_type.description_en,
            meaning.person_type.code);

        if let Some(province) = meaning.province {
            println!("Province: {} ({})", province.name_en, province.code);
        }

        println!("Valid checksum: {}", meaning.is_valid);
    }
    ```

=== "Python"

    ```python
    from konjingjing import get_id_meaning

    meaning = get_id_meaning("1101700230703")
    if meaning:
        print(f"Person type: {meaning['person_type_description_en']}")
        print(f"Province: {meaning['province_name_en']}")
        print(f"Valid: {meaning['is_valid']}")
    ```

=== "Node.js"

    ```typescript
    import { getIdMeaning } from 'konjingjing';

    const meaning = getIdMeaning("1101700230703");
    if (meaning) {
        console.log(`Person type: ${meaning.personTypeDescriptionEn}`);
        console.log(`Province: ${meaning.provinceNameEn}`);
        console.log(`Valid: ${meaning.isValid}`);
    }
    ```

=== "Browser (WASM)"

    ```typescript
    import init, { getIdMeaning } from 'konjingjing-wasm';

    await init();

    const meaning = getIdMeaning("1101700230703");
    if (meaning) {
        console.log(`Person type: ${meaning.personTypeDescriptionEn}`);
        console.log(`Province: ${meaning.provinceNameEn}`);
        console.log(`Valid: ${meaning.isValid}`);
    }
    ```

## Error Handling

Both functions handle invalid input gracefully:

- `verify_id` returns `false` for invalid input
- `get_id_meaning` returns `None`/`null` for invalid input

Invalid input includes:
- Non-13-digit strings
- Strings containing non-digit characters
- Empty strings
