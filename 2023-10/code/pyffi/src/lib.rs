use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;

mod part1;
mod part2;
mod part3;
mod part4;
mod part5;
mod part6;

/// A Python module implemented in Rust.
#[pymodule]
fn pyffi(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(part1::sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(part2::comma_join, m)?)?;
    m.add_function(wrap_pyfunction!(part3::comma_join_nonempty, m)?)?;
    m.add_function(wrap_pyfunction!(part4::comma_join_py, m)?)?;
    m.add_function(wrap_pyfunction!(part5::make_struct, m)?)?;
    m.add_function(wrap_pyfunction!(part6::type_test, m)?)?;
    Ok(())
}
