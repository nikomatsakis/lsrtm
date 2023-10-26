use pyo3::prelude::*;

#[derive(FromPyObject)]
pub struct RustyStruct {
    my_string: String,
}

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn make_struct(a: RustyStruct) -> PyResult<String> {
    Ok(a.my_string)
}

#[derive(FromPyObject)]
pub struct RustyTupleStruct(String, String);

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn make_tuple_struct(a: RustyTupleStruct) -> PyResult<String> {
    Ok(format!("{:?} + {:?}", a.0, a.1))
}
