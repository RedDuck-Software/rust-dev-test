// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {IDisperseCollect} from "./interfaces/IDisperseCollect.sol";

import "openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

contract DisperseCollect is Ownable, IDisperseCollect {
    using SafeERC20 for IERC20;

    uint256 public constant PERCENTAGES = 10000; // 100%

    address private _collectAddress;

    constructor() Ownable(msg.sender) {
        _collectAddress = msg.sender;
    }

    modifier validPercents(uint256 percents) {
        require(percents <= PERCENTAGES);

        _;
    }

    function changeCollectToAddress(address newTo) external onlyOwner {
        _collectAddress = newTo;
    }

    function collectETH(
        uint256 amount,
        uint256 percents
    ) external payable validPercents(percents) {
        if (msg.value != amount) {
            revert InvalidMsgValue();
        }

        uint256 amountToSend = _getAmountWithPercents(amount, percents);

        _sendETH(_collectAddress, amountToSend);

        emit Collected(msg.sender, _collectAddress, amountToSend);
    }

    function collectERC20(
        address token,
        uint256 amount,
        uint256 percents
    ) external validPercents(percents) {
        uint256 amountToSend = _getAmountWithPercents(amount, percents);

        IERC20(token).safeTransferFrom(
            msg.sender,
            _collectAddress,
            amountToSend
        );

        emit CollectedERC20(msg.sender, _collectAddress, token, amountToSend);
    }

    function disperseETH(
        address[] calldata to,
        uint256[] calldata amounts,
        uint256 percents
    ) external payable validPercents(percents) {
        if (to.length != amounts.length) {
            revert InvalidLengths();
        }

        uint256 total;

        for (uint256 i = 0; i < to.length; i++) {
            uint256 amountToSend = _getAmountWithPercents(amounts[i], percents);

            if (address(this).balance < amountToSend) {
                break;
            }

            total += amountToSend;

            _sendETH(to[i], amountToSend);
        }

        if (total != msg.value) {
            revert InvalidMsgValue();
        }

        emit Dispersed(msg.sender, to, amounts);
    }

    function disperseERC20(
        address[] calldata tokens,
        address[] calldata to,
        uint256[] calldata amounts,
        uint256 percents
    ) external validPercents(percents) {
        if (!_validateParams(tokens, to, amounts)) {
            revert InvalidLengths();
        }

        for (uint256 i = 0; i < tokens.length; i++) {
            uint256 amountToSend = _getAmountWithPercents(amounts[i], percents);

            IERC20(tokens[i]).safeTransferFrom(msg.sender, to[i], amountToSend);
        }

        emit DispersedERC20(msg.sender, tokens, to, amounts);
    }

    function _sendETH(address to, uint256 amount) private {
        (bool success, ) = to.call{value: amount}("");

        if (!success) {
            revert TransferError();
        }
    }

    function _validateParams(
        address[] calldata tokens,
        address[] calldata to,
        uint256[] calldata amounts
    ) private pure returns (bool) {
        return tokens.length == to.length && to.length == amounts.length;
    }

    function _getAmountWithPercents(
        uint256 amount,
        uint256 percents
    ) private pure returns (uint256) {
        return percents == 0 ? amount : (amount * percents) / PERCENTAGES;
    }
}
