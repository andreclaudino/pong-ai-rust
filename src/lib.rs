mod game_state;
mod constants;
mod entity;

mod game_entry;
use game_entry::start_pong;

use pyo3::prelude::{pyfunction, pymodule, Python, PyModule, PyResult};
use pyo3::wrap_pyfunction;

/// Starts pong game
#[pyfunction]
pub fn start_pypong() -> PyResult<()> {
  start_pong().expect("Error loading game");
  Ok(())
}

/// A Python to interact with pong game
#[pymodule]
fn pong_lib(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_wrapped(wrap_pyfunction!(start_pypong))?;

    Ok(())
}