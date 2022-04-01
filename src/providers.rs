use ethers::prelude::*;
use ethers::providers::Http;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use pythonize::pythonize;
use std::convert::TryFrom;
use std::sync::Arc;

/// A HTTPProvider is an abstraction of a connection to the Ethereum network, providing a concise, consistent interface to standard Ethereum node functionality.
///
/// Args:
///     endpoint (:obj:`str`, `optional`):
///         The http endpoint to connect to (ex: `http://localhost:8545` or `https://mainnet.infura.io/v3/YOUR_PROJECT_ID`).
#[pyclass(module = "ethers.providers")]
#[pyo3(text_signature = "(self, endpoint)")]
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

    /// Gets the block at `block_number`
    ///
    /// Args:
    ///     block_number (:obj:`int`):
    ///         The block number to get.
    ///
    /// Returns:
    ///     :obj:`dict`: Block object
    #[pyo3(text_signature = "(self, block_number)")]
    pub fn get_block<'p>(&self, py: Python<'p>, block_number: u64) -> PyResult<&'p PyAny> {
        let provider = Arc::clone(&self.provider);
        future_into_py(py, async move {
            let block = provider
                .get_block(block_number)
                .await
                .map_err(to_py_exception)?;
            Ok(Python::with_gil(|py| pythonize(py, &block).unwrap()))
        })
    }


    /// Gets the latest block number via the `eth_BlockNumber` API
    /// Returns:
    ///     :obj:`int`: latest block number
    #[pyo3(text_signature = "(self)")]
    pub fn get_block_number<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let provider = Arc::clone(&self.provider);
        future_into_py(py, async move {
            let bn = provider
                .get_block_number()
                .await
                .map_err(to_py_exception)?
                .as_u64();
            Ok(Python::with_gil(|py| bn.to_object(py)))
        })
    }
}

#[pymodule]
pub fn providers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HTTPProvider>()?;
    Ok(())
}
