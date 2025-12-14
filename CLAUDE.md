# konjingjing

Thai Citizen ID validation library with Rust core and Python/Node.js bindings.

## Project Structure

```
packages/
├── konjingjing/           # Core Rust library
├── konjingjing-python/    # Python bindings (PyO3 + maturin)
└── konjingjing-node/      # Node.js bindings (napi-rs)
```

## Development

Uses **mise** for task running and tool management.

### Common Tasks

```bash
mise run test           # Run all tests
mise run test:rust      # Run Rust tests only
mise run test:python    # Run Python tests only
mise run test:node      # Run Node.js tests only
mise run build          # Build all packages
mise run lint           # Run clippy
mise run fmt            # Format Rust code
```

### Building Individual Packages

**Python:**
```bash
cd packages/konjingjing-python
uv run --with maturin maturin develop
```

**Node.js:**
```bash
cd packages/konjingjing-node
bun install && bun run build
```

## API

- `verify_id(id)` - Validate Thai National ID checksum
- `get_id_meaning(id)` - Extract meaning (person type, province, amphoe, validity)

## Notes

- Node.js exports use camelCase (`verifyId`, `getIdMeaning`) with snake_case aliases
- Python exports use snake_case (`verify_id`, `get_id_meaning`)
- Core Rust library uses snake_case
