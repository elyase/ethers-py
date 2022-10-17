# Generated content DO NOT EDIT
class HTTPProvider:
    """
    A HTTPProvider is an abstraction of a connection to the Ethereum network, providing a concise, consistent interface to standard Ethereum node functionality.

    Args:
        endpoint (:obj:`str`, `optional`):
            The http endpoint to connect to (ex: `http://localhost:8545` or `https://mainnet.infura.io/v3/YOUR_PROJECT_ID`).
    """

    def __init__(self, endpoint):
        pass
    def get_balance(self, address, block_id):
        """
        Returns the account's balance of the given address or ENS name

        Args:
            address (string): The account's address or ENS name

        Returns:
            int: account's balance
        """
        pass
    def get_block(self, block_number_or_hash):
        """
        Gets the block at `block_number_or_hash`

        Args:
            block_number_or_hash (Union[str, int]): The block number or hash.

        Returns:
            dict: Block object
        """
        pass
    def get_block_number(self):
        """
        Gets the latest ato_little_endianlock number via the `eth_BlockNumber` API

        Returns:
            :obj:`int`: latest block number
        """
        pass
