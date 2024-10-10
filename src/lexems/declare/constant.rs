// Examples, (I'd like to do this with spaces between values):
// bool Disabled false
// byte Key V2arjN0mVGd9457Zow6q4uJmBe0=
// string LocalHostname nbb-reports

use pyo3::pyclass;

#[pyclass]
pub struct Constant {
    r#type: String,
    key: String,
    value: String,
}
