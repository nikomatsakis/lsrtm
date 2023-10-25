use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

// ---- PART 1

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// ---- PART 2

/// Formats the sum of two numbers as string.
#[pyfunction]
fn comma_join(a: Vec<String>) -> PyResult<String> {
    Ok(a.join(", "))
}

// ---- PART 3

/// Formats the sum of two numbers as string.
#[pyfunction]
fn comma_join_nonempty(a: Vec<String>) -> PyResult<String> {
    if a.is_empty() {
        return Err(PyValueError::new_err("empty list"));
    }
    Ok(a.join(", "))
}

// ---- PART 3

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

// ---- PART 4

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

// ---- PART 5

/// From Py Object will try out the variants...
#[derive(FromPyObject, Debug)]
enum TypeTest {
    /// Can it be converted to a u32?
    IsInt(u32),

    /// Can it be converted to a string?
    IsString(String),

    /// Does it have x, y fields that can be converted to integers?
    Point { x: u32, y: u32 },
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn type_test(a: TypeTest) -> PyResult<String> {
    Ok(format!("{a:?}"))
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyffi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(comma_join, m)?)?;
    m.add_function(wrap_pyfunction!(comma_join_nonempty, m)?)?;
    m.add_function(wrap_pyfunction!(comma_join_py, m)?)?;
    m.add_function(wrap_pyfunction!(make_struct, m)?)?;
    m.add_function(wrap_pyfunction!(type_test, m)?)?;
    Ok(())
}
