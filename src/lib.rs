use md5;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

// takes a string slice and returns a Python tuple or None
#[pyfunction]
fn sql_part_falliste(py: Python<'_>, line: &str) -> PyResult<Option<PyObject>> {
    let mut splitted: Vec<String> = line.split(';').map(str::to_owned).collect();

    if splitted.len() != 46 || splitted.get(0).map(|s| s.as_str()) == Some("Summe") {
        return Ok(None);
    }

    // transform fields
    splitted[3] = correct_date_short(&splitted[3]);
    splitted[4] = correct_date_long(&splitted[4]);
    splitted[5] = correct_date_long(&splitted[5]);
    splitted[1] = md5_encode(&splitted[1]);
    splitted[2] = md5_encode(&splitted[2]);

    let items = splitted.into_iter().map(|s| s.into_py(py));
    let tuple = PyTuple::new_bound(py, items);
    Ok(Some(tuple.into()))
}

//helper function for formatting date string
fn correct_date_short(date: &str) -> String {
    let parts: Vec<&str> = date.trim().split('.').collect();
    if parts.len() != 3 {
        return String::new();
    }
    let (d, m, y) = (parts[0].trim(), parts[1].trim(), parts[2].trim());
    if d.is_empty() || m.is_empty() || y.is_empty() {
        return String::new();
    }
    format!("{}-{}-{}", y, m, d)
}

//helper function for formatting date string
fn correct_date_long(date: &str) -> String {
    let mut parts = date
        .splitn(2, ' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if parts.len() != 2 {
        return String::new();
    }
    parts[0] = correct_date_short(parts[0].trim());
    format!("{} {}", parts[0], parts[1])
}

//helper function for generating md5 hash
fn md5_encode(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

// Python module fallliste_rust with function sql_part_falliste implemented in Rust
#[pymodule]
fn fallliste_rust(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sql_part_falliste, m)?)?;
    Ok(())
}
