// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import {DisperseCollect} from "../src/DisperseCollect.sol";

contract DeployDisperseCollect is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(privateKey);
        DisperseCollect contract = new DisperseCollect();

        console.log("DisperseCollect deployed at:", address(contract));

        vm.stopBroadcast();
    }
}
