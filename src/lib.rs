mod lexems;
use pyo3::prelude::*;

// from vpn_config_parser import parse_config
#[pyfunction]
#[pyo3(signature = (a = None))]
fn parse_config(a: Option<&str>) -> PyResult<(String, String)> {
    match a {
        Some(value) => Ok(("Hello, ".to_string(), value.to_string())),
        None => Ok(("Hello, ".to_string(), "world!".to_string())),
    }
}

#[pymodule]
fn vpn_config_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_config, m)?)?;
    // m.add_function(wrap_pyfunction!(lexems::parse_hash_comment, m)?)?;
    Ok(())
}
