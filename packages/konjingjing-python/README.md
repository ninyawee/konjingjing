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

### Verify ID

```python
from konjingjing import verify_id

assert verify_id('1112034563562')        # Valid
assert not verify_id('1112034563563')    # Invalid checksum
assert not verify_id('11120345635')      # Too short
```

### Extract ID Meaning

```python
from konjingjing import get_id_meaning

result = get_id_meaning('1101700230703')
# {
#     'person_type_code': 1,
#     'person_type_description': 'คนไทยที่แจ้งเกิดภายในกำหนด',
#     'person_type_description_en': 'Thai citizen, birth registered on time',
#     'province_code': 10,
#     'province_name_th': 'กรุงเทพมหานคร',
#     'province_name_en': 'Bangkok',
#     'amphoe_code': 1017,
#     'amphoe_name': 'ห้วยขวาง',
#     'is_valid': True
# }

get_id_meaning('invalid')  # None
```

## License

ISC
