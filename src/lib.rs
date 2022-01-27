use pyo3::prelude::*;

pub mod window;

/// A simple python UI module but is rusty at its core 🦀✨.
#[pymodule]
fn rui(_py: Python, m: &PyModule) -> PyResult<()> {
    window::add_window_class(&m)?;

    Ok(())
}