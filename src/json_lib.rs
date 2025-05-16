use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString};
use pyo3::{PyObject, PyResult, Python};
use serde_json::Value as Json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn json_to_py(py: Python<'_>, path: &str) -> PyResult<Py<PyDict>> {
    match read_json_to_hashmap(&PathBuf::from(path)) {
        Ok(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map.into_iter() {
                dict.set_item(k, json_value_to_py(py, v)?)?;
            }
            Ok(dict.into())
        }
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e.to_string())),
    }
}

/// Reads a JSON file from the given path and returns a HashMap<String, Value> representing the top-level keys.
fn read_json_to_hashmap(path: &PathBuf) -> Result<HashMap<String, Json>> {
    let content = fs::read_to_string(path)?;
    let value: Json = serde_json::from_str(&content)?;
    let mut map = HashMap::new();
    if let Some(obj) = value.as_object() {
        for (key, value) in obj.iter() {
            map.insert(key.clone(), value.clone());
        }
    }
    Ok(map)
}

fn json_value_to_py(py: Python<'_>, v: Json) -> PyResult<PyObject> {
    match v {
        Json::Null => Ok(py.None()),
        Json::Bool(b) => Ok(PyBool::new(py, b).to_owned().into()),
        Json::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(PyInt::new(py, i).into())
            } else if let Some(u) = n.as_u64() {
                Ok(PyInt::new(py, u as i64).into())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into())
            } else {
                Err(pyo3::exceptions::PyValueError::new_err(
                    "Unsupported JSON number type".to_string(),
                ))
            }
        }
        Json::String(s) => Ok(PyString::new(py, &s).into()),
        Json::Array(arr) => {
            let list = PyList::empty(py);
            for v in arr {
                list.append(json_value_to_py(py, v)?)?;
            }
            Ok(list.into())
        }
        Json::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_value_to_py(py, v)?)?;
            }
            Ok(dict.into())
        }
    }
}
