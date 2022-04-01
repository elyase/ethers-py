# ethers-py
Python library for interacting with the Ethereum Blockchain and bindings for the great ethers-rs


## Usage/Examples

```python
from ethers.providers import HTTPProvider

provider = HTTPProvider("https://mainnet.infura.io/v3/your_infura_id")
block_number = await provider.get_block_number()
# 14429070
```

```{toctree}
:maxdepth: 2
:hidden:
:caption: "Contents:"
   
providers
```

   