# Konjingjing

Thai Citizen ID validation library available in Rust, Python, Node.js, and WebAssembly (browser).

The library name **kon-jing-jing** (คนจริงจริง) means "real person" in Thai.

## Features

- Validate Thai National ID checksums
- Extract demographic information from IDs:
    - Person type (8 categories)
    - Province (77 provinces)
    - District (amphoe)
- Bilingual support (Thai and English)

## Quick Install

=== "Rust"

    ```bash
    cargo add konjingjing
    ```

=== "Python"

    ```bash
    pip install konjingjing
    ```

=== "Node.js"

    ```bash
    npm install konjingjing
    ```

=== "Browser (WASM)"

    ```bash
    npm install konjingjing-wasm
    ```

## Quick Example

=== "Rust"

    ```rust
    use konjingjing::{verify_id, get_id_meaning};

    fn main() {
        // Validate an ID
        let is_valid = verify_id("1101700230703");
        println!("Valid: {}", is_valid);

        // Get ID meaning
        if let Some(meaning) = get_id_meaning("1101700230703") {
            println!("Person type: {}", meaning.person_type.description_en);
            if let Some(province) = meaning.province {
                println!("Province: {}", province.name_en);
            }
        }
    }
    ```

=== "Python"

    ```python
    from konjingjing import verify_id, get_id_meaning

    # Validate an ID
    is_valid = verify_id("1101700230703")
    print(f"Valid: {is_valid}")

    # Get ID meaning
    meaning = get_id_meaning("1101700230703")
    if meaning:
        print(f"Person type: {meaning['person_type_description_en']}")
        print(f"Province: {meaning['province_name_en']}")
    ```

=== "Node.js"

    ```typescript
    import { verifyId, getIdMeaning } from 'konjingjing';

    // Validate an ID
    const isValid = verifyId("1101700230703");
    console.log(`Valid: ${isValid}`);

    // Get ID meaning
    const meaning = getIdMeaning("1101700230703");
    if (meaning) {
        console.log(`Person type: ${meaning.personTypeDescriptionEn}`);
        console.log(`Province: ${meaning.provinceNameEn}`);
    }
    ```

=== "Browser (WASM)"

    ```typescript
    import init, { verifyId, getIdMeaning } from 'konjingjing-wasm';

    // Initialize WASM module (required once)
    await init();

    // Validate an ID
    const isValid = verifyId("1101700230703");
    console.log(`Valid: ${isValid}`);

    // Get ID meaning
    const meaning = getIdMeaning("1101700230703");
    if (meaning) {
        console.log(`Person type: ${meaning.personTypeDescriptionEn}`);
        console.log(`Province: ${meaning.provinceNameEn}`);
    }
    ```

## License

ISC
