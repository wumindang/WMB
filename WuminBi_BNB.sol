// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0; // 指定 Solidity 版本为 0.8.0 或更高版本

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol"; // 导入 OpenZeppelin ERC20 可升级合约库
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol"; // 导入 OpenZeppelin Ownable 可升级合约库
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol"; // 导入 OpenZeppelin UUPS 可升级合约库
import "@openzeppelin/contracts/token/ERC20/IERC20.sol"; // 导入 OpenZeppelin IERC20 接口，用于与 USDT 交互
import "@openzeppelin/contracts/utils/math/SafeMath.sol"; // 导入 OpenZeppelin SafeMath 库，用于安全数学运算
import "@openzeppelin/contracts/utils/math/Math.sol"; // 导入 OpenZeppelin Math 库，用于数学运算

contract WuminBi is ERC20Upgradeable, OwnableUpgradeable, UUPSUpgradeable { // 定义 WuminBi 合约，继承自 ERC20Upgradeable, OwnableUpgradeable, UUPSUpgradeable
    using SafeMath for uint256; // 使用 SafeMath 库，为 uint256 类型提供安全数学运算
    using Math for uint256; // 使用 Math 库，为 uint256 类型提供数学运算

    IERC20 public usdtToken; // 定义 USDT 代币接口，用于与 USDT 合约交互
    uint256 public constant USDT_RATE = 100; // 定义 USDT 兑换 WMB 的汇率，1 USDT = 100 WMB
    uint256 public constant TOTAL_SUPPLY = 1443497378 * 10 ** 2; // 定义 WMB 总发行量，1,443,497,378.00 WMB，2 位小数

    uint256 public mintedAmount; // 定义已铸造的 WMB 数量
    mapping(address => uint256) public purchaseRecords; // 定义购买记录映射，仅记录通过合约购买的 WMB 数量

    event WMBPurchased(address indexed user, uint256 usdtAmount, uint256 wmbAmount); // 定义 WMB 购买事件，记录购买信息

    function initialize(address _usdtAddress) public initializer { // 定义初始化函数，用于初始化合约
        __ERC20_init("WuminBi", "WMB"); // 初始化 ERC20，设置代币名称和符号
        __Ownable_init(msg.sender); // 初始化 Ownable，设置合约所有者
        __UUPSUpgradeable_init(); // 初始化 UUPS，启用合约升级功能

        require(_usdtAddress != address(0), "USDT address cannot be zero"); // 验证 USDT 地址不为零地址
        require(address(_usdtAddress).code.length > 0, "USDT address must be a contract"); // 验证 USDT 地址必须是合约地址

        usdtToken = IERC20(_usdtAddress); // 设置 USDT 代币地址
        mintedAmount = TOTAL_SUPPLY; // 设置已铸造的 WMB 数量为总发行量
        _mint(msg.sender, TOTAL_SUPPLY); // 铸造全部 WMB，给合约所有者
    }

    function decimals() public pure override returns (uint8) { // 定义 decimals 函数，返回代币小数位数
        return 2; // 设置代币小数位数为 2
    }

    function buyWMB(uint256 usdtAmount) public { // 定义购买 WMB 函数
        require(usdtAmount > 0, "Amount must be greater than 0"); // 验证购买数量大于 0

        uint256 wmbAmount = usdtAmount.mul(USDT_RATE); // 计算购买的 WMB 数量
        require(balanceOf(owner()) >= wmbAmount, "Not enough WMB in contract"); // 验证合约所有者有足够的 WMB

        require(usdtToken.transferFrom(msg.sender, owner(), usdtAmount), "USDT transfer failed"); // 从用户转移 USDT 到合约所有者

        _transfer(owner(), msg.sender, wmbAmount); // 从合约所有者转移 WMB 给用户
        purchaseRecords[msg.sender] = purchaseRecords[msg.sender].add(wmbAmount); // 记录用户购买的 WMB 数量

        emit WMBPurchased(msg.sender, usdtAmount, wmbAmount); // 发出 WMB 购买事件
    }

    function getPurchaseRecord(address user) public view returns (uint256 amount) { // 定义查询购买记录函数
        return purchaseRecords[user]; // 返回用户通过合约购买的 WMB 数量
    }

    function _authorizeUpgrade(address newImplementation) internal override onlyOwner {} // 定义合约升级授权函数，只有合约所有者可以升级
}