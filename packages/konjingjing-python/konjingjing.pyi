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

def verify_id(id: str) -> bool:
    """Verify a Thai Citizen Card ID."""
    ...

def get_id_meaning(id: str) -> IdMeaning | None:
    """
    Extract meaning from a Thai National ID.

    Returns a dict with person type, province, amphoe, and validity info.
    Returns None if the ID format is invalid.
    """
    ...
