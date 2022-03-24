import pytest
from ethers.providers import HTTPProvider

ENDPOINT = "https://mainnet.infura.io/v3/670cc38391324e12bbcf893d84c175c3"


async def test_get_block_number() -> None:
    provider = HTTPProvider(ENDPOINT)
    block_number = await provider.get_block_number()
    assert block_number > 0


async def test_get_block() -> None:
    provider = HTTPProvider(ENDPOINT)

    # https://etherscan.io/txs?block=14430335&p=5
    block_number = await provider.get_block(14430335)
    assert (
        block_number["transactions"][0]
        == "0x9d6d096e442c7725679c81fb7068919e1027d2a7b3da7d96383039d4cd9341e5"
    )
