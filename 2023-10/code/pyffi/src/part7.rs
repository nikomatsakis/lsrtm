use pyo3::prelude::*;

#[pyclass(frozen)]
pub struct Character {
    name: String,
    age: u32,
}

#[pymethods]
impl Character {
    #[new]
    fn new(name: String) -> Self {
        Self { name, age: 22 }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    // fn set_age(&mut self, age: u32) {
    //     self.age = age;
    // }
}
