use pyo3::prelude::*;
/// Implements pointer conversions between Rust and Python tskit objects
mod ffi;
/// Implements a haploid Wright-Fisher simulation
mod haploid_wright_fisher;

/// Export tskit errors
#[derive(Debug)]
enum Error {
    Tskit(tskit::TskitError),
    Message(String),
}

impl From<tskit::TskitError> for Error {
    fn from(value: tskit::TskitError) -> Self {
        Self::Tskit(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tskit(e) => write!(f, "{e}"),
            Self::Message(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for Error {}

/// A Python module implemented in Rust.
#[pymodule]
mod tskit_maturin_starter {
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;

    use crate::ffi;
    use crate::haploid_wright_fisher;

    /// Run a haploid Wright-Fisher simulation
    #[pyfunction]
    pub fn sim_haploid_wright_fisher(
        py: Python<'_>,
        population_size: usize,
        num_generations: usize,
        random_seed: u64,
        simplify_interval: usize,
    ) -> PyResult<Py<PyAny>> {
        let tables = haploid_wright_fisher::simulate(
            random_seed,
            population_size,
            num_generations as i32,
            simplify_interval as i32,
            true,
        )
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        ffi::table_collection_into_python_tree_sequence(py, tables)
    }
}
