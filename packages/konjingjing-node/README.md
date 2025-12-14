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

```typescript
import { verifyId } from 'konjingjing';

verifyId('1112034563562');  // true
verifyId('1112034563563');  // false (invalid checksum)
verifyId('11120345635');    // false (too short)
```

### Snake case alias

For cross-language consistency with Python/Rust:

```typescript
import { verify_id } from 'konjingjing';

verify_id('1112034563562');  // true
```

## License

ISC
