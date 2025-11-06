use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn convert_to_sql(line: &str) -> String {
    Some(line)
        .map(|l| l.split(';').map(str::to_owned).collect::<Vec<_>>())
        .filter(|splitted| splitted.len() == 46 && splitted[0] != "Summe")
        .map(|mut splitted| {
            splitted[3] = correct_date_short(&splitted[3]);
            splitted[4] = correct_date_long(&splitted[4]);
            splitted[5] = correct_date_long(&splitted[5]);
            generate_sql_string(splitted)
        })
        .unwrap_or_default()
}



fn correct_date_short(date: &str) -> String {
    let parts: Vec<&str> = date.trim().split('.').collect();
    if parts.len() != 3 {
        return String::new();
    }
    format!("{}-{}-{}", parts[2], parts[1], parts[0])
}

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

fn generate_sql_string(list: Vec<String>) -> String {
    let mut sql = String::new();
    sql.push_str("INSERT INTO fallliste (\"Fallnummer\", \"Nachname\", \"Vorname\", \"Geburtsdatum\", \"Aufnahme\", \"Entlassung\", \"Behandlungstage  gesamt\", \"DRG Nr\", \"untere Grenzverweildauer\", \"mittlere Verweildauer\", \"obere Grenzverweildauer\", \"Hauptfachabteilung\", \"Fachabteilung\", \"Station\")");
    let substr = format!(
        " VALUES ('{}', '{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}');",
        list[0],
        list[1],
        list[2],
        list[3],
        list[4],
        list[5],
        list[6],
        list[7],
        list[11],
        list[15],
        list[22],
        list[31],
        list[32],
        list[33]
    );
    sql.push_str(substr.as_str());
    sql
}

/// A Python module implemented in Rust.
#[pymodule]
fn fallliste_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_to_sql, m)?)?;
    Ok(())
}