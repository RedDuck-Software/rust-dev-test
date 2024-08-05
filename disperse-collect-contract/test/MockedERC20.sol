// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {ERC20} from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract MockedERC20 is ERC20 {
    constructor() ERC20("Test", "T") {
        _mint(msg.sender, 100_000_000 * 10 ** 18);
    }
}
