mod lexems;
use lexems::{parse_declare, Constant, Declare};
use pyo3::prelude::*;

// python: from vpn_config_parser import parse_config
#[pyfunction]
fn parse_config(a: &str) -> PyResult<Option<Declare>> {
    match parse_declare(a) {
        Ok((_, result)) => Ok(Some(result)),
        Err(_) => Ok(None),
    }
}

#[pymodule]
fn vpn_config_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_config, m)?)?;
    m.add_class::<Declare>()?;
    m.add_class::<Constant>()?;
    Ok(())
}
