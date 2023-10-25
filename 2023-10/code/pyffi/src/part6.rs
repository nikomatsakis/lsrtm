use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// From Py Object will try out the variants...
#[derive(FromPyObject, Debug)]
pub enum TypeTest {
    /// Can it be converted to a u32?
    IsInt(u32),

    /// Can it be converted to a string?
    IsString(String),

    /// Does it have x, y fields that can be converted to integers?
    Point { x: u32, y: u32 },
}

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn type_test(a: TypeTest) -> PyResult<String> {
    Ok(format!("{a:?}"))
}
