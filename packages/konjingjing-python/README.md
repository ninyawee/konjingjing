# konjingjing

Thai Citizen ID validation library for Python (Rust-powered).

The library name 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

## Installation

```bash
pip install konjingjing
```

Or with uv:

```bash
uv add konjingjing
```

## Usage

```python
from konjingjing import verify_id

assert verify_id('1112034563562')        # Valid
assert not verify_id('1112034563563')    # Invalid checksum
assert not verify_id('11120345635')      # Too short
```

## License

ISC
