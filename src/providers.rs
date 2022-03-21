use ethers::prelude::*;
use ethers::providers::Http;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use std::convert::TryFrom;
use std::sync::Arc;

#[pyclass]
struct HTTPProvider {
    provider: Arc<ethers::providers::Provider<Http>>,
}

pub fn to_py_exception(err: impl std::fmt::Display) -> PyErr {
    PyException::new_err(format!("{}", err))
}

#[pymethods]
impl HTTPProvider {

    #[new]
    pub fn new(endpoint: String) -> Self {
        let provider = ethers::providers::Provider::<Http>::try_from(endpoint.as_str()).unwrap();
        Self {
            provider: Arc::new(provider),
        }
    }

    /// Decode the given list of tokens to a final string Gets the latest block number via the `eth_BlockNumber` API
    ///
    /// Args:
    ///     tokens (:obj:`List[str]`):
    ///         The list of tokens to decode
    ///
    /// Returns:
    ///     :obj:`str`: The decoded string
    pub fn get_block_number<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let provider = Arc::clone(&self.provider);
        future_into_py(py, async move {
            let bn = provider.get_block_number().await.map_err(to_py_exception)?.as_u64();
            Ok(Python::with_gil(|py| bn.to_object(py)))
        })
    }
}

#[pymodule]
pub fn providers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HTTPProvider>()?;
    Ok(())
}
