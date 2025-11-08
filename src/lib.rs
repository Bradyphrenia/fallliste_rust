use md5;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

// takes a list and returns a Python tuple
#[pyfunction]
fn sql_part_falliste(py: Python<'_>, line: Vec<String>) -> PyResult<PyObject> {
    if line.len() != 46 || line[0] == "Summe" {
        //return empty tuple
        return Ok(PyTuple::empty_bound(py).into_py(py));
    }
    let remove_single_quotes = |x: &str| x.trim().replace("'", "");
    let remove_double_quotes = |x: &str| x.trim().replace("\"", "");
    let mut v: Vec<String> = line
        .iter()
        .map(|x| {
            remove_double_quotes(x);
            remove_single_quotes(x)
        })
        .collect();
    v[3] = correct_date_short(&v[3]);
    v[4] = correct_date_long(&v[4]);
    v[5] = correct_date_long(&v[5]);
    v[1] = md5_encode(&v[1]);
    v[2] = md5_encode(&v[2]);
    v[0] = v[0].chars().take(8).collect();

    let convert_to_int = |x: &str| x.trim().parse::<i64>().unwrap_or(0);
    let items = vec![
        v[0].clone().into_py(py),
        v[1].clone().into_py(py),
        v[2].clone().into_py(py),
        v[3].clone().into_py(py),
        v[4].clone().into_py(py),
        v[5].clone().into_py(py),
        convert_to_int(&v[6]).into_py(py),
        v[7].clone().into_py(py),
        v[11].clone().into_py(py),
        v[15].clone().into_py(py),
        v[22].clone().into_py(py),
        v[31].clone().into_py(py),
        v[32].clone().into_py(py),
        v[33].clone().into_py(py),
    ];
    // generate a Python tuple from the tuple
    Python::with_gil(|py| Ok(PyTuple::new_bound(py, items).into_py(py)))
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
    let mut parts = date
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if parts.len() != 2 {
        return "".to_string();
    }
    parts[0] = correct_date_short(parts[0].trim());
    format!("{} {}", parts[0], parts[1])
}

//helper function for generating md5 hash
fn md5_encode(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

// Python module falliste_rust with function convert_to_sql implemented in Rust
#[pymodule]
fn fallliste_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sql_part_falliste, m)?)?;
    Ok(())
}
