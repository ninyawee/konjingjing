use serde::Serialize;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

/// Verify a Thai Citizen Card ID.
#[wasm_bindgen(js_name = verifyId)]
pub fn verify_id(id: &str) -> bool {
    konjingjing::verify_id(id)
}

/// Result of extracting meaning from a Thai National ID.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(rename_all = "camelCase")]
pub struct IdMeaning {
    pub person_type_code: u8,
    pub person_type_description: String,
    pub person_type_description_en: String,
    pub province_code: Option<u8>,
    pub province_name_th: Option<String>,
    pub province_name_en: Option<String>,
    pub amphoe_code: Option<u16>,
    pub amphoe_name: Option<String>,
    pub is_valid: bool,
}

/// Extract meaning from a Thai National ID.
///
/// Returns an object with person type, province, amphoe, and validity info.
/// Returns null if the ID format is invalid.
#[wasm_bindgen(js_name = getIdMeaning)]
pub fn get_id_meaning(id: &str) -> Option<IdMeaning> {
    let meaning = konjingjing::get_id_meaning(id)?;
    Some(IdMeaning {
        person_type_code: meaning.person_type.code,
        person_type_description: meaning.person_type.description_th.to_string(),
        person_type_description_en: meaning.person_type.description_en.to_string(),
        province_code: meaning.province.as_ref().map(|p| p.code),
        province_name_th: meaning.province.as_ref().map(|p| p.name_th.to_string()),
        province_name_en: meaning.province.as_ref().map(|p| p.name_en.to_string()),
        amphoe_code: meaning.amphoe.as_ref().map(|a| a.code),
        amphoe_name: meaning.amphoe.as_ref().map(|a| a.name_th.to_string()),
        is_valid: meaning.is_valid,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    // verifyId tests
    #[wasm_bindgen_test]
    fn test_verify_id_valid() {
        assert!(verify_id("1112034563562"));
    }

    #[wasm_bindgen_test]
    fn test_verify_id_invalid_checksum() {
        assert!(!verify_id("1101700230705"));
    }

    #[wasm_bindgen_test]
    fn test_verify_id_too_short() {
        assert!(!verify_id("110170023073"));
    }

    #[wasm_bindgen_test]
    fn test_verify_id_contains_letters() {
        assert!(!verify_id("11017002070d3"));
        assert!(!verify_id("rytege54fsfsf"));
    }

    #[wasm_bindgen_test]
    fn test_verify_id_single_char() {
        assert!(!verify_id("0"));
        assert!(!verify_id("-"));
    }

    #[wasm_bindgen_test]
    fn test_verify_id_empty() {
        assert!(!verify_id(""));
    }

    #[wasm_bindgen_test]
    fn test_verify_id_garbage() {
        assert!(!verify_id("blablabla"));
    }

    // getIdMeaning tests
    #[wasm_bindgen_test]
    fn test_get_id_meaning_valid() {
        let result = get_id_meaning("1112034563562");
        assert!(result.is_some());
        let meaning = result.unwrap();
        assert_eq!(meaning.person_type_code, 1);
        assert_eq!(meaning.province_code, Some(11));
        assert!(meaning.is_valid);
    }

    #[wasm_bindgen_test]
    fn test_get_id_meaning_bangkok() {
        let result = get_id_meaning("1101700230703");
        assert!(result.is_some());
        let meaning = result.unwrap();
        assert_eq!(meaning.province_code, Some(10));
        assert_eq!(meaning.province_name_en, Some("Bangkok".to_string()));
        assert_eq!(meaning.amphoe_code, Some(1017));
    }

    #[wasm_bindgen_test]
    fn test_get_id_meaning_invalid_format() {
        assert!(get_id_meaning("").is_none());
        assert!(get_id_meaning("123").is_none());
        assert!(get_id_meaning("abcdefghijklm").is_none());
    }

    #[wasm_bindgen_test]
    fn test_get_id_meaning_invalid_person_type() {
        assert!(get_id_meaning("0101700230704").is_none());
        assert!(get_id_meaning("9101700230701").is_none());
    }
}
