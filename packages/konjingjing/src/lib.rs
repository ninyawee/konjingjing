//! Thai Citizen ID validation library.
//!
//! The library name 'kon-jing-jing' (คนจริงจริง) means 'real person' in Thai.

pub mod data;

use data::{get_amphoe, get_person_type, get_province, Amphoe, PersonType, Province};

/// Result of extracting meaning from a Thai National ID.
#[derive(Debug)]
pub struct IdMeaning {
    /// Person type information (digit 1)
    pub person_type: PersonType,
    /// Province information (digits 2-3), if found
    pub province: Option<Province>,
    /// Amphoe/district information (digits 2-5), if found
    pub amphoe: Option<Amphoe>,
    /// Whether the ID passes checksum validation
    pub is_valid: bool,
}

/// Extract meaning from a Thai National ID.
///
/// Returns `None` if the ID is not 13 digits or contains non-digit characters.
///
/// # Arguments
/// * `id` - A string slice containing the 13-digit citizen ID
///
/// # Returns
/// * `Some(IdMeaning)` with extracted information, or `None` if parsing fails
///
/// # Examples
/// ```
/// use konjingjing::get_id_meaning;
///
/// let meaning = get_id_meaning("1101700230703").unwrap();
/// assert_eq!(meaning.person_type.code, 1);
/// assert_eq!(meaning.province.unwrap().code, 10);
/// ```
pub fn get_id_meaning(id: &str) -> Option<IdMeaning> {
    // Check basic format
    if id.len() != 13 || !id.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }

    let person_type_code = id[0..1].parse::<u8>().ok()?;
    let province_code = id[1..3].parse::<u8>().ok()?;
    let amphoe_code = id[1..5].parse::<u16>().ok()?;

    let person_type = get_person_type(person_type_code)?;
    let province = get_province(province_code);
    let amphoe = get_amphoe(amphoe_code);
    let is_valid = verify_id(id);

    Some(IdMeaning {
        person_type,
        province,
        amphoe,
        is_valid,
    })
}

/// Verify a Thai Citizen Card ID.
///
/// # Arguments
/// * `id` - A string slice containing the 13-digit citizen ID
///
/// # Returns
/// * `true` if the ID is valid, `false` otherwise
///
/// # Examples
/// ```
/// use konjingjing::verify_id;
///
/// assert!(verify_id("1112034563562"));
/// assert!(!verify_id("1112034563563")); // Invalid checksum
/// assert!(!verify_id("11120345635"));   // Too short
/// ```
pub fn verify_id(id: &str) -> bool {
    // Check length and all digits
    if id.len() != 13 || !id.bytes().all(|b| b.is_ascii_digit()) {
        return false;
    }

    let digits: Vec<u32> = id.bytes().map(|b| (b - b'0') as u32).collect();

    // Calculate weighted checksum
    let total: u32 = digits[..12]
        .iter()
        .enumerate()
        .map(|(i, &d)| d * (13 - i as u32))
        .sum();

    let calculated_checksum = (11 - total % 11) % 10;

    calculated_checksum == digits[12]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_id() {
        assert!(verify_id("1112034563562"));
    }

    #[test]
    fn test_invalid_checksum() {
        assert!(!verify_id("1101700230705"));
    }

    #[test]
    fn test_too_short() {
        assert!(!verify_id("110170023073"));
    }

    #[test]
    fn test_contains_letters() {
        assert!(!verify_id("11017002070d3"));
        assert!(!verify_id("rytege54fsfsf"));
    }

    #[test]
    fn test_single_char() {
        assert!(!verify_id("0"));
        assert!(!verify_id("-"));
    }

    #[test]
    fn test_empty() {
        assert!(!verify_id(""));
    }

    #[test]
    fn test_garbage() {
        assert!(!verify_id("blablabla"));
    }
}
