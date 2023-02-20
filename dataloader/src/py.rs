use super::{graph_traffic, Float};
use pyo3::prelude::*;

#[pyfunction]
fn render_py(mut array: numpy::PyReadwriteArray2<Float>, path: &str) -> PyResult<()> {
    let dims = array.dims();
    Ok(graph_traffic(array.as_array_mut(), dims[1], path)?)
}

#[pymodule]
fn dataloader(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_py, m)?)?;
    Ok(())
}
