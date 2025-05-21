pub mod json_lib;
mod resolve_array_and_table;
pub mod toml_lib;
pub mod xml_lib;
pub mod yaml_lib;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

/// Reads a TOML file and returns its contents as a Python dict.
#[pyfunction]
fn read_toml(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    toml_lib::toml_to_py(py, path)
}

/// Reads a YAML file and returns its contents as a Python dict.
#[pyfunction]
fn read_yaml(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    yaml_lib::yaml_to_py(py, path)
}

/// Reads a JSON file and returns its contents as a Python dict.
#[pyfunction]
fn read_json(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    json_lib::json_to_py(py, path)
}

/// Reads an XML file and returns its contents as a Python dict.
#[pyfunction]
fn read_xml(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    xml_lib::xml_to_py(py, path)
}

/// A General method to read a file and return its contents as a Python dict
/// based on the file extension.
#[pyfunction]
fn read(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    match path.split('.').last() {
        Some("toml") => read_toml(py, path),
        Some("yaml") => read_yaml(py, path),
        Some("json") => read_json(py, path),
        Some("xml") => read_xml(py, path),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "Unsupported file extension".to_string(),
        )),
    }
}

/// Writes a TOML file from a Python dictionary.
#[pyfunction]
fn write_toml(py: Python<'_>, dict: HashMap<String, PyObject>, path: &str) -> PyResult<()> {
    let toml_table = resolve_array_and_table::resolve_table(py, dict);
    let toml_string = toml::to_string(&toml_table).unwrap();
    std::fs::write(path, toml_string)?;

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn config_lang_serder(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_toml, m)?)?;
    m.add_function(wrap_pyfunction!(read_yaml, m)?)?;
    m.add_function(wrap_pyfunction!(read_json, m)?)?;
    m.add_function(wrap_pyfunction!(read_xml, m)?)?;
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(write_toml, m)?)?;
    Ok(())
}
