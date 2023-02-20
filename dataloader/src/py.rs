use super::{graph_traffic, probability_of_traffic, Float};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "volume_data")]
fn py_volume_data(mut array: numpy::PyReadwriteArray2<Float>, path: &str) -> PyResult<()> {
    let dims = array.dims();
    Ok(graph_traffic(array.as_array_mut(), dims[1], path)?)
}

#[pyfunction]
#[pyo3(name = "frequency_data")]
fn py_frequency_data(
    mut array: numpy::PyReadwriteArray2<Float>,
    path: &str,
    threshold: f32,
) -> PyResult<()> {
    let dims = array.dims();
    Ok(probability_of_traffic(
        array.as_array_mut(),
        dims[1],
        path,
        threshold,
    )?)
}

#[pymodule]
fn dataloader(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_volume_data, m)?)?;
    m.add_function(wrap_pyfunction!(py_frequency_data, m)?)?;
    Ok(())
}
