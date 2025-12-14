# konjingjing

Thai Citizen ID validation library available in Rust, Python, and Node.js.

The library name 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

## Packages

| Language | Package | Install |
|----------|---------|---------|
| Rust | [packages/konjingjing](./packages/konjingjing) | `cargo add konjingjing` |
| Python | [packages/konjingjing-python](./packages/konjingjing-python) | `pip install konjingjing` |
| Node.js | [packages/konjingjing-node](./packages/konjingjing-node) | `npm install konjingjing` |

## API

All packages export a `verify_id` function:

```
verify_id(id: string) -> boolean
```

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

## License

ISC
