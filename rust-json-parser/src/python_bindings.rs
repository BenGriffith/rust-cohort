use crate::{JsonError, JsonParser, JsonValue};
use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

impl<'py> IntoPyObject<'py> for JsonValue {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            JsonValue::Null => Ok(py.None().into_pyobject(py)?),
            JsonValue::Boolean(b) => Ok(b.into_pyobject(py)?.to_owned().into_any()),
            JsonValue::Number(n) => Ok(n.into_pyobject(py)?.to_owned().into_any()),
            JsonValue::String(s) => Ok(s.into_pyobject(py)?.into_any()),
            JsonValue::Array(arr) => {
                let py_list = PyList::empty(py);
                for item in arr {
                    py_list.append(item.into_pyobject(py)?)?;
                }
                Ok(py_list.into_any())
            }
            JsonValue::Object(obj) => {
                let py_dict = PyDict::new(py);
                for (key, value) in obj {
                    py_dict.set_item(key, value.into_pyobject(py)?)?;
                }
                Ok(py_dict.into_any())
            }
        }
    }
}

impl From<JsonError> for PyErr {
    fn from(err: JsonError) -> PyErr {
        match err {
            JsonError::UnexpectedToken {
                expected,
                found,
                position,
            } => PyValueError::new_err(format!(
                "Unexpected token at position {}: expected {}, found {}",
                position, expected, found
            )),
            JsonError::UnexpectedEndOfInput { expected, position } => {
                PyValueError::new_err(format!(
                    "Unexpected end of input at position {}: expected {}",
                    position, expected
                ))
            }
            JsonError::InvalidNumber { value, position } => PyValueError::new_err(format!(
                "Invalid number: {} at position {}:",
                value, position
            )),
            JsonError::InvalidEscape { char, position } => PyValueError::new_err(format!(
                "Invalid escape character: {} at position {}:",
                char, position
            )),
            JsonError::InvalidUnicode { sequence, position } => PyValueError::new_err(format!(
                "Invalid unicode: {} at position {}:",
                sequence, position
            )),
            JsonError::InvalidPosition { position } => {
                PyValueError::new_err(format!("Invalid position: {position}"))
            }
            JsonError::ExpectedColon { position } => {
                PyValueError::new_err(format!("Expected Colon at position: {}", position))
            }
            JsonError::IOError { message } => PyIOError::new_err(message.to_string()),
            JsonError::FileNotFound { path } => {
                PyIOError::new_err(format!("File not found: {}", path))
            }
        }
    }
}

/// Parses a JSON string into a native Python object.
///
/// This function executes the Rust-based recursive descent parser to transform a raw string
/// into an equivalent Python structure (dict, list, float, etc.).
///
/// Args:
///     input (str): The raw JSON string to be parsed.
///
/// Returns:
///     Any: A Python object representation of the JSON data.
///
/// Raises:
///     ValueError: If the input string contains invalid JSON syntax or
///                 unsupported escape sequences.
///
/// Example:
///     >>> import _rust_json_parser
///     >>> data = _rust_json_parser.parse_json('{"status": "ok", "count": 5}')
///     >>> print(data["status"])
///     ok
#[pyfunction]
fn parse_json<'py>(py: Python<'py>, input: &str) -> PyResult<Bound<'py, PyAny>> {
    let mut parser = JsonParser::new(input)?;
    let result = parser.parse()?;
    let py_result = result.into_pyobject(py)?;
    Ok(py_result)
}

/// Reads a file from the filesystem and parses its JSON content.
///
/// This is a convenience function that handles file I/O in Rust before passing
/// the content to the parser.
///
/// Args:
///     path (str): The absolute or relative path to the .json file.
///
/// Returns:
///     Any: A Python object representation of the file's content.
///
/// Raises:
///     IOError: If the file does not exist or cannot be read.
///     ValueError: If the file content is not valid JSON.
///
/// Example:
///     >>> import _rust_json_parser
///     >>> data = _rust_json_parser.parse_json_file("config.json")
///     >>> print(type(data))
///     <class 'dict'>
#[pyfunction]
fn parse_json_file<'py>(py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
    let input = read_to_string(path)?;
    let mut parser = JsonParser::new(&input)?;
    let result = parser.parse()?;
    let py_result = result.into_pyobject(py)?;
    Ok(py_result)
}

fn py_to_json_value(obj: &Bound<PyAny>) -> PyResult<JsonValue> {
    if obj.is_none() {
        return Ok(JsonValue::Null);
    }

    if let Ok(b) = obj.extract::<bool>() {
        return Ok(JsonValue::Boolean(b));
    }

    if let Ok(n) = obj.extract::<f64>() {
        return Ok(JsonValue::Number(n));
    }

    if let Ok(s) = obj.extract::<String>() {
        return Ok(JsonValue::String(s));
    }

    if let Ok(list) = obj.cast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_to_json_value(&item)?);
        }
        return Ok(JsonValue::Array(arr));
    }

    if let Ok(dict) = obj.cast::<PyDict>() {
        let mut py_dict: HashMap<String, JsonValue> = HashMap::new();
        for (key, value) in dict {
            let k = key.extract::<String>()?;
            let v = py_to_json_value(&value)?;
            py_dict.insert(k, v);
        }
        return Ok(JsonValue::Object(py_dict));
    }

    Err(PyValueError::new_err(
        "Unsupported type for JSON conversion",
    ))
}

#[pyfunction]
#[pyo3(signature = (obj, indent=None))]
fn dumps(obj: &Bound<PyAny>, indent: Option<usize>) -> PyResult<String> {
    let json_value = py_to_json_value(obj)?;
    match indent {
        Some(n) => Ok(json_value.pretty_print(n)),
        None => Ok(json_value.to_string()),
    }
}

fn analyze(times: &Vec<Duration>) -> Duration {
    let mut times_sorted: Vec<_> = times.iter().collect();
    times_sorted.sort();
    *times_sorted[times_sorted.len() / 2]
}

#[pyfunction]
#[pyo3(signature = (json_str, iterations=4000))]
fn benchmark_performance(
    py: Python<'_>,
    json_str: &str,
    iterations: u32,
) -> PyResult<(f64, f64, f64)> {
    let mut rust_times = Vec::with_capacity(iterations as usize);
    let mut json_module_times = Vec::with_capacity(iterations as usize);
    let mut simplejson_times = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {
        let rust_start = Instant::now();
        let _ = parse_json(py, json_str);
        rust_times.push(rust_start.elapsed());
    }

    let json_module = py.import("json")?;
    let json_loads = json_module.getattr("loads")?;

    for _ in 0..iterations {
        let json_module_start = Instant::now();
        let _ = json_loads.call1((json_str,))?;
        json_module_times.push(json_module_start.elapsed());
    }

    let simplejson_module = py.import("simplejson")?;
    let simplejson_loads = simplejson_module.getattr("loads")?;

    for _ in 0..iterations {
        let simplejson_start = Instant::now();
        let _ = simplejson_loads.call1((json_str,))?;
        simplejson_times.push(simplejson_start.elapsed());
    }

    let rust_median = analyze(&rust_times);
    let json_module_median = analyze(&json_module_times);
    let simplejson_median = analyze(&simplejson_times);

    Ok((
        rust_median.as_secs_f64(),
        json_module_median.as_secs_f64(),
        simplejson_median.as_secs_f64(),
    ))
}

#[pymodule]
fn _rust_json_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_json, m)?)?;
    m.add_function(wrap_pyfunction!(parse_json_file, m)?)?;
    m.add_function(wrap_pyfunction!(dumps, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_performance, m)?)?;
    Ok(())
}
