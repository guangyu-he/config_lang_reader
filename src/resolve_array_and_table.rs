use pyo3::prelude::*;
use pyo3::PyObject;
use std::collections::HashMap;
use toml::Table;
use toml::Value;

/// Converts a `Vec<PyObject>` into a `Vec<Value>`.
///
/// This function takes a `Vec<PyObject>` and converts it into a `Vec<Value>`.
/// The conversion is done in the following way:
///
/// * If the `PyObject` is a boolean, it is converted to a `toml::Value::Boolean`.
/// * If the `PyObject` is a string, it is converted to a `toml::Value::String`.
/// * If the `PyObject` is an integer, it is converted to a `toml::Value::Integer`.
/// * If the `PyObject` is a float, it is converted to a `toml::Value::Float`.
/// * If the `PyObject` is a list, it is converted to a `toml::Value::Array`.
/// * If the `PyObject` is a dictionary, it is converted to a `toml::Value::Table`.
/// * If the `PyObject` is of any other type, it is converted to a `toml::Value::String` containing the string "UNSUPPORTED TYPE".
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

/// Converts a `HashMap<String, PyObject>` into a `toml::Table`.
///
/// This function takes a `HashMap<String, PyObject>` and converts it into a `toml::Table`.
/// The conversion is done in the following way:
///
/// * If the `PyObject` is a boolean, it is converted to a `toml::Value::Boolean`.
/// * If the `PyObject` is a string, it is converted to a `toml::Value::String`.
/// * If the `PyObject` is an integer, it is converted to a `toml::Value::Integer`.
/// * If the `PyObject` is a float, it is converted to a `toml::Value::Float`.
/// * If the `PyObject` is a vector of `PyObject`s, it is converted to a `toml::Value::Array`.
/// * If the `PyObject` is a `HashMap<String, PyObject>`, it is converted to a `toml::Value::Table`.
/// * If the `PyObject` is of any other type, it is converted to a `toml::Value::String` with the value of the string being `"UNSUPPORTED TYPE"`.
///
/// # Arguments
///
/// * `py` - A Python object, used to extract the value from the `PyObject`.
/// * `value` - The `HashMap<String, PyObject>` to be converted.
///
/// # Returns
///
/// A `toml::Table` that represents the converted `HashMap<String, PyObject>`.
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
