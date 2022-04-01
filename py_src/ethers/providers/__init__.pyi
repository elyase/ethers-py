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
    def get_block(self, block_number):
        """
        /// Gets the block at `block_number`

        Args:
            block_number (:obj:`int`):
                The block number to get.

        Returns:
            :obj:`dict`: Block object
        """
        pass
    def get_block_number(self):
        """
        Gets the latest block number via the `eth_BlockNumber` API
        Returns:
            :obj:`int`: latest block number
        """
        pass
