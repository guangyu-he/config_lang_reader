use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};
use pyo3::{PyObject, PyResult, Python};
use serde_yaml::Value as Yaml;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn yaml_to_py(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    match read_yaml_to_hashmap(&PathBuf::from(path)) {
        Ok(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map.into_iter() {
                dict.set_item(k, yaml_value_to_py(py, v)?)?;
            }
            Ok(dict.into())
        }
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}

/// Reads a YAML file from the given path and returns a HashMap<String, Value> representing the top-level keys.
fn read_yaml_to_hashmap(path: &PathBuf) -> Result<HashMap<String, Yaml>> {
    let content = fs::read_to_string(path)?;
    let value: Yaml = serde_yaml::from_str(&content)?;
    let mut map = HashMap::new();
    if let Some(obj) = value.as_mapping() {
        for (key, value) in obj.iter() {
            if let Some(key_str) = key.as_str() {
                map.insert(key_str.to_string(), value.clone());
            }
        }
    }
    Ok(map)
}

fn yaml_value_to_py(py: Python<'_>, v: Yaml) -> PyResult<PyObject> {
    match v {
        Yaml::Bool(b) => Ok(PyBool::new(py, b).to_owned().into()),
        Yaml::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(PyInt::new(py, i).into())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into())
            } else {
                Err(pyo3::exceptions::PyValueError::new_err(
                    "Unsupported YAML number type".to_string(),
                ))
            }
        }
        Yaml::String(s) => Ok(PyString::new(py, &s).into()),
        Yaml::Sequence(seq) => {
            let list = PyList::empty(py);
            for v in seq {
                list.append(yaml_value_to_py(py, v)?)?;
            }
            Ok(list.into())
        }
        Yaml::Mapping(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                if let Some(key_str) = k.as_str() {
                    dict.set_item(key_str, yaml_value_to_py(py, v)?)?;
                }
            }
            Ok(dict.into())
        }
        Yaml::Null => Ok(py.None()),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "Unsupported YAML value type".to_string(),
        )),
    }
}
