import pytest
from ethers.providers import HTTPProvider

ENDPOINT = "https://mainnet.infura.io/v3/670cc38391324e12bbcf893d84c175c3"


@pytest.fixture
def provider():
    return HTTPProvider(ENDPOINT)


async def test_get_block_number(provider):
    block_number = await provider.get_block_number()
    assert block_number > 0


async def test_get_block(provider) -> None:
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

async def test_get_balance(provider) -> None:

    # https://etherscan.io/txs?block=14430335&p=5
    address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
    block_number = await provider.get_block_number()
    ens = "vitalik.eth"
    balance = await provider.get_balance(address, block_number) 
    assert balance > 0
