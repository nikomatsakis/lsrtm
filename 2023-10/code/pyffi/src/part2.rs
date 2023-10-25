use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn comma_join(a: Vec<String>) -> PyResult<String> {
    Ok(a.join(", "))
}
