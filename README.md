# ethers-py
Python library for interacting with the Ethereum Blockchain and bindings for the great [ethers-rs](https://github.com/gakonst/ethers-rs)

## Usage/Examples

Until [wheels for pypi are available](https://github.com/pypa/pypi-support/issues/1794) you need to [install the rust toolchain](https://www.rust-lang.org/tools/install) and then run:

```bash
pip install git+https://github.com/elyase/ethers-py
```

## Getting started

```python
from ethers.providers import HTTPProvider

provider = HTTPProvider("https://mainnet.infura.io/v3/your_infura_id")
block_number = await provider.get_block_number()
# 14429070
```

## Documentation

[Documentation](https://linktodocumentation)

## Features

- [ ] Ethereum JSON-RPC Client
- [ ] Interacting and deploying smart contracts
- [ ] Type safe smart contract bindings code generation
- [ ] Querying past events
- [ ] Event monitoring as `Stream`s
- [ ] ENS as a first class citizen
- [ ] Celo support
- [ ] Polygon support 
- [ ] Avalanche support 
- [ ] Websockets / `eth_subscribe`
- [ ] Hardware Wallet Support
- [ ] Parity APIs (`tracing`, `parity_blockWithReceipts`)
- [ ] Geth TxPool API
- [ ] WASM Bindings (see note)
- [ ] FFI Bindings (see note)
- [ ] CLI for common operations
