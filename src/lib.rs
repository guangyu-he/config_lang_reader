pub mod json_lib;
mod resolve_array_and_table;
pub mod toml_lib;
pub mod xml_lib;
pub mod yaml_lib;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use resolve_array_and_table::resolve_table;

/// A General method to read a file and return its contents as a Python dict
/// based on the file extension.
#[pyfunction]
fn read(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    match path.split('.').last() {
        Some("toml") => toml_lib::toml_to_py(py, path),
        Some("yaml") => yaml_lib::yaml_to_py(py, path),
        Some("json") => json_lib::json_to_py(py, path),
        Some("xml") => xml_lib::xml_to_py(py, path),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "Unsupported file extension".to_string(),
        )),
    }
}

/// Writes a Python dictionary to a file based on the file extension.
#[pyfunction]
fn write(py: Python<'_>, dict: HashMap<String, PyObject>, path: &str) -> PyResult<()> {
    let toml_table = resolve_table(py, dict);
    let string = match path.split('.').last() {
        Some("toml") => toml::to_string(&toml_table).unwrap(),
        Some("yaml") => serde_yaml::to_string(&toml_table).unwrap(),
        Some("json") => serde_json::to_string_pretty(&toml_table).unwrap(),
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Unsupported file extension".to_string(),
            ))
        }
    };
    std::fs::write(path, string)?;

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn config_lang_serder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(write, m)?)?;
    Ok(())
}
