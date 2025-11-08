use md5;
use pyo3::{prelude::*, PyObject};

// takes a string slice and returns a Python tuple
#[pyfunction]
fn sql_part_falliste(line: &str) -> PyResult<PyObject> {
    Some(line)
        .map(|l| l.split(';').map(str::to_owned).collect::<Vec<_>>())
        .filter(|splitted| splitted.len() == 46 && splitted[0] != "Summe")
        .map(|mut splitted| {
            splitted[3] = correct_date_short(&splitted[3]);
            splitted[4] = correct_date_long(&splitted[4]);
            splitted[5] = correct_date_long(&splitted[5]);
            splitted[1] = md5_encode(&splitted[1]);
            splitted[2] = md5_encode(&splitted[2]);
            return_tuple(splitted)
        })
        .unwrap()
}

//helper function for formatting date string
fn correct_date_short(date: &str) -> String {
    let parts: Vec<&str> = date.trim().split('.').collect();
    if parts.len() != 3 {
        return String::new();
    }
    format!("{}-{}-{}", parts[2], parts[1], parts[0])
}

//helper function for formatting date string
fn correct_date_long(date: &str) -> String {
    let corrected = date.to_string();
    let mut parts = corrected
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if parts.len() != 2 {
        return "".to_string();
    }
    let ds = parts[0].as_str();
    parts[0] = correct_date_short(ds.trim());
    let date_new = format!("{} {}", parts[0], parts[1]);
    date_new
}
//helper function for converting a list to tuple
fn return_tuple(list: Vec<String>) -> PyResult<PyObject> {
    if list.len() != 14 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Erwartete Länge {}, erhalten {}",
            14,
            list.len()
        )));
    }

    let tuple = (
        list[0].unwrap().to_owned(),
        list[1].unwrap().to_owned(),
        list[2].unwrap().to_owned(),
        list[3].unwrap().to_owned(),
        list[4].unwrap().to_owned(),
        list[5].unwrap().to_owned(),
        list[6].unwrap().to_owned(),
        list[7].unwrap().to_owned(),
        list[8].unwrap().to_owned(),
        list[9].unwrap().to_owned(),
        list[10].unwrap().to_owned(),
        list[11].unwrap().to_owned(),
        list[12].unwrap().to_owned(),
        list[13].unwrap().to_owned(),
    );
    Ok(tuple.into_py(py))
}

//helper function for generating md5 hash
fn md5_encode(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    // hex encode
    let hex = format!("{:x}", digest);
    hex
}

// Python module falliste_rust with function convert_to_sql implemented in Rust
#[pymodule]
fn fallliste_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sql_part_falliste, m)?)?;
    Ok(())
}
