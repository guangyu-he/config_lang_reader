use pyo3::prelude::*;
use pyo3::PyObject;
use std::collections::HashMap;
use toml::Table;
use toml::Value;

pub fn resolve_array(py: Python<'_>, value: Vec<PyObject>) -> Vec<Value> {
    let mut value_vec: Vec<Value> = Vec::new();
    for each_obj in value {
        if let Ok(value) = each_obj.extract::<bool>(py) {
            value_vec.push(Value::Boolean(value));
        } else if let Ok(value) = each_obj.extract::<String>(py) {
            value_vec.push(Value::String(value));
        } else if let Ok(value) = each_obj.extract::<i64>(py) {
            value_vec.push(Value::Integer(value));
        } else if let Ok(value) = each_obj.extract::<f64>(py) {
            value_vec.push(Value::Float(value));
        } else if let Ok(value) = each_obj.extract::<Vec<PyObject>>(py) {
            value_vec.push(Value::Array(resolve_array(py, value)));
        } else if let Ok(value) = each_obj.extract::<HashMap<String, PyObject>>(py) {
            value_vec.push(Value::Table(resolve_table(py, value)));
        } else {
            value_vec.push(Value::String("UNSUPPORTED TYPE".to_string()));
        }
    }
    value_vec
}

pub fn resolve_table(py: Python<'_>, value: HashMap<String, PyObject>) -> Table {
    let mut value_table = Table::new();
    for (key, value) in value {
        if let Ok(value) = value.extract::<bool>(py) {
            value_table.insert(key, Value::Boolean(value));
        } else if let Ok(value) = value.extract::<String>(py) {
            value_table.insert(key, Value::String(value));
        } else if let Ok(value) = value.extract::<i64>(py) {
            value_table.insert(key, Value::Integer(value));
        } else if let Ok(value) = value.extract::<f64>(py) {
            value_table.insert(key, Value::Float(value));
        } else if let Ok(value) = value.extract::<Vec<PyObject>>(py) {
            value_table.insert(key, Value::Array(resolve_array(py, value)));
        } else if let Ok(value) = value.extract::<HashMap<String, PyObject>>(py) {
            value_table.insert(key, Value::Table(resolve_table(py, value)));
        } else {
            value_table.insert(key, Value::String("UNSUPPORTED TYPE".to_string()));
        }
    }
    value_table
}
