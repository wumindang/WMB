// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC20/ERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/utils/math/Math.sol";

contract WuminBi is ERC20Upgradeable, AccessControlUpgradeable, UUPSUpgradeable {
    using SafeMath for uint256;
    using Math for uint256;

    // 定义合约角色
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE"); // 暂停角色
    bytes32 public constant UPGRADER_ROLE = keccak256("UPGRADER_ROLE"); // 升级角色

    IERC20 public usdtToken; // USDT 代币接口
    address public ownerWallet; // 合约所有者钱包地址
    uint256 public constant USDT_RATE = 100; // 1 元 = 100 分，1 分 = 0.0001 USDT，1 元 = 0.01 USDT
    uint256 public constant TOTAL_SUPPLY = 1443497378 * 10 ** 2; // WMB 总发行量 (1,443,497,378.00 WMB)

    uint256 public mintedAmount; // 已铸造的 WMB 数量
    mapping(address => PurchaseRecord[]) public purchaseRecords; // 购买记录

    // 定义购买记录结构体
    struct PurchaseRecord {
        uint256 usdtAmount; // 购买的 USDT 数量
        uint256 wmbAmount; // 购买的 WMB 数量
        uint256 timestamp; // 购买时间戳
        bytes32 transactionHash; // 交易哈希
    }

    // 定义事件
    event WMBPurchased(address indexed user, uint256 usdtAmount, uint256 wmbAmount);
    event ContractPaused(address account);
    event ContractUnpaused(address account);

    // 初始化合约
    function initialize(address _usdtAddress, address _ownerWallet) public initializer {
        __ERC20_init("WuminBi", "WMB");
        __AccessControl_init();
        __UUPSUpgradeable_init();

        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender); // 设置默认管理员角色
        _setupRole(PAUSER_ROLE, msg.sender); // 设置暂停角色
        _setupRole(UPGRADER_ROLE, msg.sender); // 设置升级角色

        require(_usdtAddress != address(0), "USDT address cannot be zero"); // 验证 USDT 地址
        require(address(_usdtAddress).code.length > 0, "USDT address must be a contract"); // 验证 USDT 地址是合约地址
        require(_ownerWallet != address(0), "Owner wallet address cannot be zero"); // 验证 ownerWallet 地址

        usdtToken = IERC20(_usdtAddress); // 设置 USDT 代币地址
        ownerWallet = _ownerWallet; // 设置 ownerWallet 地址
        mintedAmount = TOTAL_SUPPLY; // 设置已铸造的 WMB 数量
        _mint(msg.sender, TOTAL_SUPPLY); // 铸造全部 WMB
    }

    // 返回代币小数位数
    function decimals() public pure override returns (uint8) {
        return 2;
    }

    // 购买 WMB
    function buyWMB(uint256 amountInFen) public whenNotPaused {
        require(amountInFen >= 10000, "Amount must be greater than or equal to 100 元"); // 验证购买数量（最小单位为 100 元）

        uint256 usdtAmount = amountInFen / USDT_RATE; // 计算购买需要的 USDT 数量
        uint256 wmbAmount = amount