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
    block_hash = "0xa562a578a2903a0bf17eba66a72915b8aadc794f782555701ea978be71ce8bd7"
    block = await provider.get_block(block_hash)
    assert block["hash"] == block_hash

    block_number = 14430335
    block = await provider.get_block(block_number)
    assert block["hash"] == block_hash

    transactions = block["transactions"]
    first_tx_hash = "0x9d6d096e442c7725679c81fb7068919e1027d2a7b3da7d96383039d4cd9341e5"
    assert transactions[0] == first_tx_hash
