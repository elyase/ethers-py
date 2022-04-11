use ethers::prelude::*;
use ethers::providers::Http;
use num_bigint::{BigInt, Sign};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;
use pythonize::pythonize;
use std::convert::TryFrom;
use std::str::FromStr;
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

#[derive(FromPyObject)]
pub enum PyBlockId {
    #[pyo3(transparent, annotation = "str")]
    Str(String),
    #[pyo3(transparent, annotation = "int")]
    Int(u64),
}

impl Into<BlockId> for PyBlockId {
    fn into(self) -> BlockId {
        match self {
            PyBlockId::Str(s) => BlockId::Hash(H256::from_str(s.as_str()).unwrap()),
            PyBlockId::Int(i) => BlockId::from(i),
        }
    }
}

fn u256_to_bigint(u: U256) -> BigInt {
    let mut bytes = [0u8; 32];
    u.to_big_endian(&mut bytes);
    BigInt::from_bytes_be(Sign::Plus, &bytes)
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

    /// Gets the block at `block_number_or_hash`
    ///
    /// Args:
    ///     block_number_or_hash (Union[str, int]): The block number or hash.
    ///
    /// Returns:
    ///     dict: Block object
    #[pyo3(text_signature = "(self, block_number_or_hash)")]
    pub fn get_block<'p>(&self, py: Python<'p>, block_id: PyBlockId) -> PyResult<&'p PyAny> {
        let provider = Arc::clone(&self.provider);
        future_into_py(py, async move {
            let block_number: BlockId = block_id.into();

            let block = provider
                .get_block(block_number)
                .await
                .map_err(to_py_exception)?;
            Ok(Python::with_gil(|py| pythonize(py, &block).unwrap()))
        })
    }

    /// Gets the latest ato_little_endianlock number via the `eth_BlockNumber` API
    ///
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

    /// Returns the account's balance of the given address or ENS name
    ///
    /// Args:
    ///     address (string): The account's address or ENS name
    ///
    /// Returns:
    ///     int: account's balance
    #[pyo3(text_signature = "(self, address, block_id)")]
    pub fn get_balance<'p>(
        &self,
        py: Python<'p>,
        address: String,
        block_id: PyBlockId,
    ) -> PyResult<&'p PyAny> {
        let provider = Arc::clone(&self.provider);
        future_into_py(py, async move {
            let block_id: BlockId = block_id.into();
            let address = address.parse::<Address>().unwrap();
            let balance = provider
                .get_balance(address, Some(block_id))
                .await
                .map_err(to_py_exception)?;

            let balance = u256_to_bigint(balance);
            Ok(Python::with_gil(|py| balance.to_object(py)))
        })
    }
}

#[pymodule]
pub fn providers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HTTPProvider>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use ethers::types::U256;
    use super::*;

    #[test]
    fn test() {
        let u = U256::from_dec_str("12").unwrap();
        let b = u256_to_bigint(u);
        println!("BigInt {}", b);
        println!("U256 {}", u);
    }
}
