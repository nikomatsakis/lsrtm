
use pyo3::prelude::*;

#[derive(FromPyObject)]
struct RustyStruct {
    my_string: String,
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn make_struct(a: RustyStruct) -> PyResult<String> {
    Ok(a.my_string)
}