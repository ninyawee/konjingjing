# konjingjing

Thai Citizen ID validation library available in Rust, Python, Node.js, and WebAssembly.

The library name 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

📖 **[Documentation](https://ninyawee.github.io/konjingjing/)**

## Packages

| Language | Package | Install |
|----------|---------|---------|
| Rust | [packages/konjingjing](./packages/konjingjing) | `cargo add konjingjing` |
| Python | [packages/konjingjing-python](./packages/konjingjing-python) | `pip install konjingjing` |
| Node.js | [packages/konjingjing-node](./packages/konjingjing-node) | `npm install konjingjing` |
| WASM | [packages/konjingjing-wasm](./packages/konjingjing-wasm) | `npm install konjingjing-wasm` |

### Node.js vs WASM: Which to use?

| Use Case | Package | Why |
|----------|---------|-----|
| **Node.js / Bun / Deno** | `konjingjing` | Native binding, fastest performance |
| **Browser** | `konjingjing-wasm` | Runs in browser, no server needed |
| **Edge / Serverless** | `konjingjing-wasm` | Portable, no native dependencies |
| **Cloudflare Workers** | `konjingjing-wasm` | WASM supported, native bindings not |

> **TL;DR**: Use `konjingjing` for server-side Node.js. Use `konjingjing-wasm` for browsers and edge runtimes.

## API

All packages export these functions:

### `verify_id(id: string) -> boolean`

Validates a Thai National ID checksum.

### `get_id_meaning(id: string) -> IdMeaning | null`

Extracts meaning from a Thai National ID:

| Field | Type | Description |
|-------|------|-------------|
| `person_type_code` | `number` | Person type (1-8) |
| `person_type_description` | `string` | Thai description |
| `person_type_description_en` | `string` | English description |
| `province_code` | `number?` | Province code (10-96) |
| `province_name_th` | `string?` | Thai province name |
| `province_name_en` | `string?` | English province name |
| `amphoe_code` | `number?` | District code |
| `amphoe_name` | `string?` | District name (Thai) |
| `is_valid` | `boolean` | Checksum validity |

Returns `null` if the ID format is invalid (not 13 digits).

## Development

Uses [mise](https://mise.jdx.dev/) for task running.

```bash
mise run test          # Run all tests
mise run test:rust     # Run Rust tests
mise run test:python   # Run Python tests
mise run test:node     # Run Node tests
```

## Algorithm

1. Validate input is exactly 13 digits
2. Calculate weighted checksum: `sum(digit[i] * (13 - i))` for i in 0..12
3. Verify: `(11 - sum % 11) % 10 == digit[12]`

## Support

[![Ko-fi](https://img.shields.io/badge/Ko--fi-Support%20me%20☕-ff5f5f?logo=ko-fi&logoColor=white)](https://ko-fi.com/ninyawee)

## License

ISC
