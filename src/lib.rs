mod lexems;
use pyo3::prelude::*;

#[pyfunction]
fn parse_config(a: &str) -> PyResult<(String, String)> {
    Ok((format!("hello, {a}"), "world!".to_string()))
}

#[pymodule]
fn vpn_config_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_config, m)?)?;
    // m.add_function(wrap_pyfunction!(lexems::parse_hash_comment, m)?)?;
    Ok(())
}
