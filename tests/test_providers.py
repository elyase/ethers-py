import pytest
from ethers.providers import HTTPProvider


async def test_get_block_number() -> None:
    provider = HTTPProvider(
        "https://mainnet.infura.io/v3/670cc38391324e12bbcf893d84c175c3"
    )
    block_number = await provider.get_block_number()
    assert block_number > 0
