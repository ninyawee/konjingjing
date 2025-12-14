#[macro_use]
extern crate napi_derive;

/// Verify a Thai Citizen Card ID.
#[napi]
pub fn verify_id(id: String) -> bool {
    konjingjing::verify_id(&id)
}

/// Result of extracting meaning from a Thai National ID.
#[napi(object)]
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
#[napi]
pub fn get_id_meaning(id: String) -> Option<IdMeaning> {
    let meaning = konjingjing::get_id_meaning(&id)?;

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
