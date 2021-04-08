use pyo3::create_exception;
use pyo3::exceptions::*;
use pyo3::prelude::*;
use pyo3::types::PyUnicode;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Exception when you try to add small numbers
create_exception!(randomos, SmallNumberError, PyException);

/// Says hello to the name given and welcome to the city. Returns a string.
/// Takes a dictrionary as argument.
#[pyfunction]
#[text_signature = "(data)"]
fn hello(data: HashMap<String, String>) -> PyResult<String> {
    let name = data.get("name").unwrap();
    let city = data.get("city").unwrap();
    let answer = format!("Hello {}, welcome to {}", name, city);
    Ok(answer)
}

/// Adds two numbers and returns the result
#[pyfunction]
#[text_signature = "(a, b)"]
fn add_numbers(a: i64, b: i64) -> PyResult<i64> {
    if a < 10 {
        return Err(SmallNumberError::new_err(format!(
            "Too small number: {}, think big.",
            a
        )));
    }
    Ok(a + b)
}

/// Reads a file as Python Bytes
#[pyfunction]
#[text_signature = "(filename)"]
fn read_file(py: Python, filename: String) -> PyResult<PyObject> {
    let mut file = File::open(filename).expect("no file found");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let result = std::str::from_utf8(&buf).unwrap();
    let res = PyUnicode::new(py, &result);
    Ok(res.into())
}

///Class Rpath for path related methods
#[pyclass]
struct Rpath {
    original_path: String,
}

#[pymethods]
impl Rpath {
    #[new]
    fn new(original_path: String) -> Self {
        Rpath { original_path }
    }

    pub fn exists(&mut self) -> PyResult<bool> {
        let p = &self.original_path.as_str();
        let path = Path::new(p);
        Ok(path.exists())
    }
    pub fn is_dir(&mut self) -> PyResult<bool> {
        let p = &self.original_path.as_str();
        let path = Path::new(p);
        Ok(path.is_dir())
    }
    pub fn is_file(&mut self) -> PyResult<bool> {
        let p = &self.original_path.as_str();
        let path = Path::new(p);
        Ok(path.is_file())
    }
}

/// A Python module implemented in Rust with random OS things.
#[pymodule]
fn randomos(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(read_file, m)?)?;
    m.add("SmallNumberError", _py.get_type::<SmallNumberError>())?;
    m.add_class::<Rpath>()?;

    Ok(())
}
