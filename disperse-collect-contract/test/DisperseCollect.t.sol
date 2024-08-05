// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import {DisperseCollect} from "../src/DisperseCollect.sol";
import {MockedERC20} from "./MockedERC20.sol";

contract DisperseCollectTest is Test {
    DisperseCollect public disperseCollect;
    MockedERC20 public token;
    address deployer;
    address user;

    function setUp() public {
        deployer = 0xacd40F6e41A44816649f3Fe003D54B781b0f17ED;
        user = 0x9d2294d7c90164E4E1F682F230362d47D7B5403D;

        token = new MockedERC20();
        disperseCollect = new DisperseCollect();

        vm.deal(deployer, 10 ether);

        disperseCollect.changeCollectToAddress(user);
    }

    function testETHCollect() public {
        uint256 transferAmount = 1 * 10 ** 18;

        disperseCollect.collectETH{value: transferAmount}(transferAmount, 0);

        assertEq(address(user).balance, transferAmount);
    }
}
