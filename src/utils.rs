extern crate cpython;
extern crate serde_json;

use cpython::*;
use serde_json::Value;

pub fn fromjson(py: Python, json: Value) -> PyObject {
    macro_rules! obj {
        ($x:ident) => {
            $x.into_py_object(py).into_object()
        }
    }

    match json {
        Value::Number(x) => {
            if let Some(n) = x.as_u64() {
                obj!(n)
            } else if let Some(n) = x.as_i64() {
                obj!(n)
            } else if let Some(n) = x.as_f64() {
                obj!(n)
            } else {
                panic!("Número inválido")
            }
        }
        Value::String(x) => PyUnicode::new(py, &x).into_object(),
        Value::Bool(x) => obj!(x),
        Value::Array(vec) => {
            let mut elements = Vec::new();
            for item in vec {
                elements.push(fromjson(py, item));
            }
            PyList::new(py, &elements[..]).into_object()
        }
        Value::Object(map) => {
            let dict = PyDict::new(py);
            for (key, value) in map {
                dict.set_item(py, key, fromjson(py, value));
            }
            dict.into_object()
        }
        Value::Null => py.None(),
    }
}