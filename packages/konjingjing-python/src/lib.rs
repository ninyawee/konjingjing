use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Verify a Thai Citizen Card ID.
#[pyfunction]
fn verify_id(id: &str) -> bool {
    ::konjingjing::verify_id(id)
}

/// Extract meaning from a Thai National ID.
///
/// Returns a dictionary with:
/// - person_type_code: int
/// - person_type_description: str
/// - province_code: int (optional)
/// - province_name_th: str (optional)
/// - province_name_en: str (optional)
/// - amphoe_code: int (optional)
/// - amphoe_name: str (optional)
/// - is_valid: bool
///
/// Returns None if the ID format is invalid.
#[pyfunction]
fn get_id_meaning(py: Python<'_>, id: &str) -> PyResult<Option<Py<PyDict>>> {
    let meaning = match ::konjingjing::get_id_meaning(id) {
        Some(m) => m,
        None => return Ok(None),
    };

    let dict = PyDict::new(py);

    dict.set_item("person_type_code", meaning.person_type.code)?;
    dict.set_item("person_type_description", meaning.person_type.description_th)?;
    dict.set_item("person_type_description_en", meaning.person_type.description_en)?;

    if let Some(province) = meaning.province {
        dict.set_item("province_code", province.code)?;
        dict.set_item("province_name_th", province.name_th)?;
        dict.set_item("province_name_en", province.name_en)?;
    } else {
        dict.set_item("province_code", py.None())?;
        dict.set_item("province_name_th", py.None())?;
        dict.set_item("province_name_en", py.None())?;
    }

    if let Some(amphoe) = meaning.amphoe {
        dict.set_item("amphoe_code", amphoe.code)?;
        dict.set_item("amphoe_name", amphoe.name_th)?;
    } else {
        dict.set_item("amphoe_code", py.None())?;
        dict.set_item("amphoe_name", py.None())?;
    }

    dict.set_item("is_valid", meaning.is_valid)?;

    Ok(Some(dict.into()))
}

#[pymodule]
fn konjingjing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(verify_id, m)?)?;
    m.add_function(wrap_pyfunction!(get_id_meaning, m)?)?;
    Ok(())
}
