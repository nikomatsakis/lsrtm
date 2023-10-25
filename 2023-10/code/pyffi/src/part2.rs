use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn comma_join(a: Vec<String>) -> PyResult<String> {
    Ok(a.join(", "))
}
