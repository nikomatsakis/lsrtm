use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::cell::RefCell;

thread_local! {
    static LIST: RefCell<Option<Py<PyList>>> = Default::default()
}

/// Set current list.
#[pyfunction]
pub fn stash_list(a: Py<PyList>) -> PyResult<()> {
    LIST.with(|list| {
        *list.borrow_mut() = Some(a);
    });
    Ok(())
}

/// Get current list;
#[pyfunction]
pub fn get_list() -> PyResult<Py<PyList>> {
    LIST.with(|list| match &*list.borrow() {
        Some(l) => Ok(l.clone()),
        None => Err(PyValueError::new_err("no list")),
    })
}

/// Length of the current list;
#[pyfunction]
pub fn get_list_len() -> PyResult<usize> {
    todo!()
}
