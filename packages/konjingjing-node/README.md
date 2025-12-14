# konjingjing

Thai Citizen ID validation library for Node.js (Rust-powered via napi-rs).

The library name 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

## Installation

```bash
npm install konjingjing
# or
bun add konjingjing
```

## Usage

### Verify ID

```typescript
import { verifyId } from 'konjingjing';

verifyId('1112034563562');  // true
verifyId('1112034563563');  // false (invalid checksum)
verifyId('11120345635');    // false (too short)
```

### Extract ID Meaning

```typescript
import { getIdMeaning } from 'konjingjing';

const result = getIdMeaning('1101700230703');
// {
//   personTypeCode: 1,
//   personTypeDescription: 'คนไทยที่แจ้งเกิดภายในกำหนด',
//   personTypeDescriptionEn: 'Thai citizen, birth registered on time',
//   provinceCode: 10,
//   provinceNameTh: 'กรุงเทพมหานคร',
//   provinceNameEn: 'Bangkok',
//   amphoeCode: 1017,
//   amphoeName: 'ห้วยขวาง',
//   isValid: true
// }

getIdMeaning('invalid');  // null
```

### Snake case aliases

For cross-language consistency with Python/Rust:

```typescript
import { verify_id, get_id_meaning } from 'konjingjing';

verify_id('1112034563562');  // true
get_id_meaning('1112034563562');  // IdMeaning object
```

## License

ISC
