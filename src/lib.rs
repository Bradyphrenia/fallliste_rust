use md5;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::mem;

#[pyfunction]
fn sql_part_falliste(py: Python<'_>, mut line: Vec<String>) -> PyResult<PyObject> {
    if line.len() != 46 || line[0] == "Summe" {
        return Ok(PyTuple::empty_bound(py).into_py(py));
    }

    // Sanitize all fields in-place: trim and remove both quote types in one pass.
    for s in &mut line {
        // Trim (may allocate if we want to own the trimmed content)
        let trimmed = s.trim();
        if trimmed.len() != s.len() {
            *s = trimmed.to_owned();
        }
        // Remove quotes in-place without allocating a new String
        // (this keeps capacity; much cheaper than replace)
        s.retain(|c| c != '\'' && c != '"');
    }

    // Transform selected fields
    line[3] = correct_date_short(&line[3]);
    line[4] = correct_date_long(&line[4]);
    line[5] = correct_date_long(&line[5]);
    line[1] = md5_encode(&line[1]);
    line[2] = md5_encode(&line[2]);
    // v[0] = v[0].chars().take(8).collect();
    line[0] = line[0].chars().take(8).collect();

    let parse_i64 = |x: &str| x.trim().parse::<i64>().unwrap_or(0);

    // Move (not clone) owned Strings into Python objects where possible
    let items: [PyObject; 14] = [
        mem::take(&mut line[0]).into_py(py), // move
        mem::take(&mut line[1]).into_py(py),
        mem::take(&mut line[2]).into_py(py),
        mem::take(&mut line[3]).into_py(py),
        mem::take(&mut line[4]).into_py(py),
        mem::take(&mut line[5]).into_py(py),
        parse_i64(&line[6]).into_py(py), // cheap copy
        mem::take(&mut line[7]).into_py(py),
        mem::take(&mut line[11]).into_py(py),
        mem::take(&mut line[15]).into_py(py),
        mem::take(&mut line[22]).into_py(py),
        mem::take(&mut line[31]).into_py(py),
        mem::take(&mut line[32]).into_py(py),
        mem::take(&mut line[33]).into_py(py),
    ];

    Ok(PyTuple::new_bound(py, &items).into_py(py))
}

fn correct_date_short(date: &str) -> String {
    let parts: Vec<&str> = date.trim().split('.').collect();
    if parts.len() != 3 {
        return String::new();
    }
    format!("{}-{}-{}", parts[2], parts[1], parts[0])
}

fn correct_date_long(date: &str) -> String {
    let mut parts = date
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if parts.len() != 2 {
        return String::new();
    }
    parts[0] = correct_date_short(parts[0].trim());
    format!("{} {}", parts[0], parts[1])
}

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
