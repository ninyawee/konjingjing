# Python API

## Installation

```bash
pip install konjingjing
```

Requires Python 3.9+.

## Functions

### `verify_id`

Verify a Thai Citizen Card ID checksum.

```python
def verify_id(id: str) -> bool
```

**Arguments:**

- `id` - A string containing the 13-digit citizen ID

**Returns:**

- `True` if the ID passes checksum validation
- `False` otherwise

**Example:**

```python
from konjingjing import verify_id

verify_id("1112034563562")   # True
verify_id("1112034563563")   # False (invalid checksum)
verify_id("11120345635")     # False (too short)
verify_id("abc")             # False (non-digits)
```

---

### `get_id_meaning`

Extract meaning from a Thai National ID.

```python
def get_id_meaning(id: str) -> dict | None
```

**Arguments:**

- `id` - A string containing the 13-digit citizen ID

**Returns:**

- A dictionary with ID information, or `None` if the format is invalid

**Return Dictionary Keys:**

| Key | Type | Description |
|-----|------|-------------|
| `person_type_code` | `int` | Person type code (1-8) |
| `person_type_description` | `str` | Thai description |
| `person_type_description_en` | `str` | English description |
| `province_code` | `int \| None` | Province code |
| `province_name_th` | `str \| None` | Thai province name |
| `province_name_en` | `str \| None` | English province name |
| `amphoe_code` | `int \| None` | District code |
| `amphoe_name` | `str \| None` | Thai district name |
| `is_valid` | `bool` | Checksum validation result |

**Example:**

```python
from konjingjing import get_id_meaning

meaning = get_id_meaning("1101700230703")
if meaning:
    print(meaning["person_type_code"])           # 1
    print(meaning["person_type_description_en"]) # "Thai citizen, birth registered on time"
    print(meaning["province_name_en"])           # "Bangkok"
    print(meaning["is_valid"])                   # True
```

## Complete Example

```python
from konjingjing import verify_id, get_id_meaning

def check_thai_id(id: str) -> None:
    """Check a Thai National ID and print information."""

    # Quick validation
    if not verify_id(id):
        print(f"ID {id} is invalid")
        return

    # Get detailed information
    meaning = get_id_meaning(id)
    if not meaning:
        print("Could not parse ID")
        return

    print(f"Person Type: {meaning['person_type_code']} - {meaning['person_type_description_en']}")

    if meaning["province_name_en"]:
        print(f"Province: {meaning['province_name_en']} ({meaning['province_name_th']})")

    if meaning["amphoe_name"]:
        print(f"District: {meaning['amphoe_name']}")

    print(f"Valid: {meaning['is_valid']}")


# Usage
check_thai_id("1101700230703")
```

## Type Hints

For better IDE support, you can use TypedDict:

```python
from typing import TypedDict

class IdMeaning(TypedDict):
    person_type_code: int
    person_type_description: str
    person_type_description_en: str
    province_code: int | None
    province_name_th: str | None
    province_name_en: str | None
    amphoe_code: int | None
    amphoe_name: str | None
    is_valid: bool
```
