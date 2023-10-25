use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn comma_join_nonempty(a: Vec<String>) -> PyResult<String> {
    if a.is_empty() {
        return Err(PyValueError::new_err("empty list"));
    }
    Ok(a.join(", "))
}
