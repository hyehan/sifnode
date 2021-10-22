// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.0;

import "../interfaces/IBlocklist.sol";

/**
 * @title Ethereum WhiteList
 * @dev WhiteList contract records the ERC 20 list that can be locked in BridgeBank.
 */
contract EthereumWhiteList {
    /**
     * @dev has the contract been initialized?
     */
    bool private _initialized;

    /**
     * @dev mapping to keep track of whitelisted tokens
     */
    mapping(address => bool) private _ethereumTokenWhiteList;

    /**
     * @dev the blocklist contract
     */
    IBlocklist public blocklist;

    /**
     * @dev is the blocklist active?
     */
    bool public hasBlocklist;

    /**
     * @dev gap of storage for future upgrades
     */
    uint256[98] private ____gap;

    /**
     * @notice Event emitted when the whitelist is updated
     */
    event LogWhiteListUpdate(address _token, bool _value);

    /**
     * @notice Initializer
     */
    function initialize() public {
        require(!_initialized, "Initialized");
        _ethereumTokenWhiteList[address(0)] = true;
        _initialized = true;
    }

    /**
     * @dev Modifier to restrict EVM addresses
     */
    modifier onlyNotBlocklisted(address account) {
        if (hasBlocklist) {
            require(
                !blocklist.isBlocklisted(account),
                "Address is blocklisted"
            );
        }
        _;
    }

    /**
     * @dev Set the token address in whitelist
     * @param _token: ERC 20's address
     * @param _inList: Set the _token in list or not
     * @return New value of if _token in whitelist
     */
    function setTokenInEthWhiteList(address _token, bool _inList)
        internal
        returns (bool)
    {
        _ethereumTokenWhiteList[_token] = _inList;
        emit LogWhiteListUpdate(_token, _inList);
        return _inList;
    }

    /**
     * @notice Is `_token` in Ethereum Whitelist?
     * @dev Get if the token in whitelist
     * @param _token ERC 20's address
     * @return If _token in whitelist
     */
    function getTokenInEthWhiteList(address _token) public view returns (bool) {
        return _ethereumTokenWhiteList[_token];
    }

    /**
     * @notice Lets the operator set the blocklist address
     * @param blocklistAddress The address of the blocklist contract
     */
    function _setBlocklist(address blocklistAddress) internal {
        blocklist = IBlocklist(blocklistAddress);
        hasBlocklist = true;
    }
}
