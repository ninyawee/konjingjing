# Node.js API

## Installation

```bash
npm install konjingjing
```

Or with other package managers:

```bash
yarn add konjingjing
pnpm add konjingjing
bun add konjingjing
```

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
import { verifyId } from 'konjingjing';

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
import { getIdMeaning } from 'konjingjing';

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

```typescript
import { verifyId, getIdMeaning, IdMeaning } from 'konjingjing';

function checkThaiId(id: string): void {
    // Quick validation
    if (!verifyId(id)) {
        console.log(`ID ${id} is invalid`);
        return;
    }

    // Get detailed information
    const meaning = getIdMeaning(id);
    if (!meaning) {
        console.log("Could not parse ID");
        return;
    }

    console.log(`Person Type: ${meaning.personTypeCode} - ${meaning.personTypeDescriptionEn}`);

    if (meaning.provinceNameEn) {
        console.log(`Province: ${meaning.provinceNameEn} (${meaning.provinceNameTh})`);
    }

    if (meaning.amphoeName) {
        console.log(`District: ${meaning.amphoeName}`);
    }

    console.log(`Valid: ${meaning.isValid}`);
}

// Usage
checkThaiId("1101700230703");
```

## CommonJS Usage

```javascript
const { verifyId, getIdMeaning } = require('konjingjing');

const isValid = verifyId("1112034563562");
console.log(isValid); // true
```
