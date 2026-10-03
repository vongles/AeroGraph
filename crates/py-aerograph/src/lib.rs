use pyo3::prelude::*;

#[pyclass]
pub struct PyAeroGraphGym {
    num_envs: usize,
}

#[pymethods]
impl PyAeroGraphGym {
    #[new]
    fn new(num_envs: usize) -> Self {
        Self { num_envs }
    }

    fn step(&mut self) -> usize {
        self.num_envs
    }
}

#[pymodule]
fn py_aerograph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyAeroGraphGym>()?;
    Ok(())
}
