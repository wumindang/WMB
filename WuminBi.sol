// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0; // Solidity版本

import "@openzeppelin/contracts/token/ERC20/ERC20.sol"; // ERC20标准合约
import "@openzeppelin/contracts/access/Ownable.sol"; // Ownable合约，便于管理

contract WuminBi is ERC20, Ownable {
    struct LockInfo {
        uint256 amount; // 购买的锁仓代币数量
        uint256 purchaseTime; // 购买时间戳
    }

    mapping(address => LockInfo) public lockedTokens; // 记录用户的锁仓信息

    constructor() ERC20("WuminBi", "WMB") Ownable(msg.sender) { 
        _mint(msg.sender, 1443497378 * 10 ** decimals()); // 铸造总供应量
    }

    function decimals() public pure override returns (uint8) {
        return 2; // 代币精度为2（即100分 = 1元）
    }

    function buyTokens(uint256 amount) public {
        require(amount > 0, "Purchase amount must be greater than 0"); // 购买金额必须大于0

        // 代币从合约所有者转移到用户
        _transfer(owner(), msg.sender, amount);

        // 记录锁仓信息
        lockedTokens[msg.sender] = LockInfo({
            amount: lockedTokens[msg.sender].amount + amount,
            purchaseTime: block.timestamp
        });
    }

    function unlockTokens() public {
        require(lockedTokens[msg.sender].amount > 0, "No tokens available for unlocking"); // 没有可解锁的代币

        uint256 lockDuration = block.timestamp - lockedTokens[msg.sender].purchaseTime;
        uint256 unlockableAmount = 0;

        if (lockDuration >= 1095 days) {
            unlockableAmount = lockedTokens[msg.sender].amount; // 全部解锁
            // 额外奖励 2 倍 WMB，但奖励不在本合约上发放，而是在五民币区块链上执行
        } else if (lockDuration >= 730 days) {
            unlockableAmount = lockedTokens[msg.sender].amount; // 全部解锁
        } else if (lockDuration >= 365 days) {
            unlockableAmount = lockedTokens[msg.sender].amount / 2; // 解锁50%
        }

        require(unlockableAmount > 0, "Unlock conditions not met"); // 未达到解锁条件

        // 更新锁仓信息
        lockedTokens[msg.sender].amount -= unlockableAmount;

        // 发送可解锁代币
        _transfer(owner(), msg.sender, unlockableAmount);
    }
}