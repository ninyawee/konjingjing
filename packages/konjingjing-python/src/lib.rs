use pyo3::prelude::*;

/// Verify a Thai Citizen Card ID.
#[pyfunction]
fn verify_id(id: &str) -> bool {
    ::konjingjing::verify_id(id)
}

#[pymodule]
fn konjingjing(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(verify_id, m)?)?;
    Ok(())
}
