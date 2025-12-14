# WebAssembly (Browser) API

## Installation

```bash
npm install konjingjing-wasm
```

Or use directly from a CDN:

```html
<script type="module">
  import init, { verifyId, getIdMeaning } from 'https://unpkg.com/konjingjing-wasm/konjingjing_wasm.js';
  await init();
</script>
```

## Initialization

!!! warning "Required"
    You must call `init()` once before using any other functions.

```typescript
import init from 'konjingjing-wasm';

await init();
```

The `init()` function loads and instantiates the WASM module. It only needs to be called once, typically at application startup.

## Functions

### `verifyId`

Verify a Thai Citizen Card ID checksum.

```typescript
function verifyId(id: string): boolean
```

**Arguments:**

- `id` - A string containing the 13-digit citizen ID

**Returns:**

- `true` if the ID passes checksum validation
- `false` otherwise

**Example:**

```typescript
import init, { verifyId } from 'konjingjing-wasm';

await init();

verifyId("1112034563562");   // true
verifyId("1112034563563");   // false (invalid checksum)
verifyId("11120345635");     // false (too short)
verifyId("abc");             // false (non-digits)
```

---

### `getIdMeaning`

Extract meaning from a Thai National ID.

```typescript
function getIdMeaning(id: string): IdMeaning | null
```

**Arguments:**

- `id` - A string containing the 13-digit citizen ID

**Returns:**

- An `IdMeaning` object, or `null` if the format is invalid

**Example:**

```typescript
import init, { getIdMeaning } from 'konjingjing-wasm';

await init();

const meaning = getIdMeaning("1101700230703");
if (meaning) {
    console.log(meaning.personTypeCode);           // 1
    console.log(meaning.personTypeDescriptionEn);  // "Thai citizen, birth registered on time"
    console.log(meaning.provinceNameEn);           // "Bangkok"
    console.log(meaning.isValid);                  // true
}
```

## Types

### `IdMeaning`

```typescript
interface IdMeaning {
    personTypeCode: number;
    personTypeDescription: string;      // Thai
    personTypeDescriptionEn: string;    // English
    provinceCode?: number;
    provinceNameTh?: string;
    provinceNameEn?: string;
    amphoeCode?: number;
    amphoeName?: string;
    isValid: boolean;
}
```

| Property | Type | Description |
|----------|------|-------------|
| `personTypeCode` | `number` | Person type code (1-8) |
| `personTypeDescription` | `string` | Thai description |
| `personTypeDescriptionEn` | `string` | English description |
| `provinceCode` | `number?` | Province code (optional) |
| `provinceNameTh` | `string?` | Thai province name (optional) |
| `provinceNameEn` | `string?` | English province name (optional) |
| `amphoeCode` | `number?` | District code (optional) |
| `amphoeName` | `string?` | Thai district name (optional) |
| `isValid` | `boolean` | Checksum validation result |

## Complete Example

### Vanilla JavaScript

```html
<!DOCTYPE html>
<html>
<head>
    <title>Thai ID Validator</title>
</head>
<body>
    <input type="text" id="idInput" placeholder="Enter Thai ID" maxlength="13">
    <button onclick="validate()">Validate</button>
    <div id="result"></div>

    <script type="module">
        import init, { verifyId, getIdMeaning } from 'https://unpkg.com/konjingjing-wasm/konjingjing_wasm.js';

        await init();

        window.validate = function() {
            const id = document.getElementById('idInput').value;
            const resultDiv = document.getElementById('result');

            if (!verifyId(id)) {
                resultDiv.textContent = 'Invalid ID';
                return;
            }

            const meaning = getIdMeaning(id);
            if (meaning) {
                resultDiv.innerHTML = `
                    <p>Valid ID</p>
                    <p>Person Type: ${meaning.personTypeDescriptionEn}</p>
                    <p>Province: ${meaning.provinceNameEn || 'N/A'}</p>
                `;
            }
        };
    </script>
</body>
</html>
```

### React

```tsx
import { useEffect, useState } from 'react';
import init, { verifyId, getIdMeaning } from 'konjingjing-wasm';

function ThaiIdValidator() {
    const [ready, setReady] = useState(false);
    const [id, setId] = useState('');
    const [result, setResult] = useState<string | null>(null);

    useEffect(() => {
        init().then(() => setReady(true));
    }, []);

    const validate = () => {
        if (!ready) return;

        if (!verifyId(id)) {
            setResult('Invalid ID');
            return;
        }

        const meaning = getIdMeaning(id);
        if (meaning) {
            setResult(`Valid - ${meaning.personTypeDescriptionEn}, ${meaning.provinceNameEn}`);
        }
    };

    return (
        <div>
            <input
                value={id}
                onChange={(e) => setId(e.target.value)}
                placeholder="Enter Thai ID"
                maxLength={13}
            />
            <button onClick={validate} disabled={!ready}>
                Validate
            </button>
            {result && <p>{result}</p>}
        </div>
    );
}
```

## Node.js vs WASM

| Feature | `konjingjing` (Node) | `konjingjing-wasm` (Browser) |
|---------|---------------------|------------------------------|
| Runtime | Node.js only | Browser + Node.js |
| Binary | Native `.node` | Universal WASM |
| Speed | Faster (native) | Slightly slower |
| Size | ~200KB per platform | 74KB universal |
| Init | Sync | Async (`await init()`) |

Use the Node.js package for server-side applications where performance is critical. Use the WASM package for browsers or when you need a universal package.
