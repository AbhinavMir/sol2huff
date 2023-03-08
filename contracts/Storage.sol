// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title Storage
 * @dev Simple contract that demonstrates how to use storage variables.
 */
contract Storage {
    uint256 private value;
    
    /**
     * @dev Sets the value of the storage variable.
     * @param newValue The new value to set.
     */
    function setValue(uint256 newValue) public {
        value = newValue;
    }
    
    /**
     * @dev Returns the current value of the storage variable.
     * @return The current value.
     */
    function getValue() public view returns (uint256) {
        return value;
    }
}