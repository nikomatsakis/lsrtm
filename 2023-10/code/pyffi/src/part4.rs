use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn comma_join_py(a: &PyList) -> PyResult<String> {
    // c.f. https://docs.rs/pyo3/latest/pyo3/types/struct.PyList.html
    if a.len() == 0 {
        return Err(PyValueError::new_err("empty list"));
    }

    // We can convert to a Rust type manually:
    let a: Vec<String> = a.extract()?;

    Ok(a.join(", "))
}
