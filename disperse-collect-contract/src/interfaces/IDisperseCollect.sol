// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IDisperseCollect {
    // * Errors *
    error InvalidLengths();
    error InvalidMsgValue();
    error TransferError();

    // * Events *
    event Collected(address indexed from, address indexed to, uint256 amount);

    event CollectedERC20(
        address indexed from,
        address indexed to,
        address indexed token,
        uint256 amount
    );

    event Dispersed(address indexed from, address[] to, uint256[] amounts);

    event DispersedERC20(
        address indexed from,
        address[] tokens,
        address[] to,
        uint256[] amounts
    );

    // * Functions *

    function changeCollectToAddress(address newTo) external;

    function collectETH(uint256 amount, uint256 percents) external payable;

    function collectERC20(
        address token,
        uint256 amount,
        uint256 percents
    ) external;

    function disperseETH(
        address[] calldata to,
        uint256[] calldata amounts,
        uint256 percents
    ) external payable;

    function disperseERC20(
        address[] calldata tokens,
        address[] calldata to,
        uint256[] calldata amounts,
        uint256 percents
    ) external;
}
