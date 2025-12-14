//! Person type data for Thai National ID.

/// Person type information
#[derive(Debug)]
pub struct PersonType {
    pub code: u8,
    pub description_th: &'static str,
    pub description_en: &'static str,
}

/// Get person type by code (digit 1 of ID)
pub fn get_person_type(code: u8) -> Option<PersonType> {
    let (description_th, description_en) = match code {
        1 => (
            "คนไทยที่แจ้งเกิดภายในกำหนด",
            "Thai citizen, birth registered on time",
        ),
        2 => (
            "คนไทยที่แจ้งเกิดเกินกำหนด",
            "Thai citizen, birth registered late",
        ),
        3 => (
            "คนไทยหรือต่างด้าวที่มีทะเบียนบ้านก่อน 31 พ.ค. 2527",
            "Thai or foreigner registered before May 31, 1984",
        ),
        4 => (
            "คนไทยหรือต่างด้าวที่ย้ายเข้าโดยไม่มีเลขประจำตัวในสมัยเริ่มแรก",
            "Thai or foreigner who moved in without ID number at initial period",
        ),
        5 => (
            "คนไทยที่เพิ่มชื่อในทะเบียนบ้านกรณีตกสำรวจ",
            "Thai citizen added to house registration (census omission)",
        ),
        6 => (
            "ผู้เข้าเมืองโดยไม่ถูกกฎหมายหรืออยู่ชั่วคราว",
            "Illegal immigrant or temporary resident",
        ),
        7 => (
            "บุตรของบุคคลประเภท 6 ที่เกิดในไทย",
            "Child of type 6 person, born in Thailand",
        ),
        8 => (
            "ต่างด้าวถูกกฎหมายหรือแปลงสัญชาติเป็นไทย",
            "Legal foreigner or naturalized Thai citizen",
        ),
        _ => return None,
    };
    Some(PersonType {
        code,
        description_th,
        description_en,
    })
}
