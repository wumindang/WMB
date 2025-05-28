bytes memory data,
        uint256 value,
        string memory errorMessage
    ) internal returns (bytes memory) {
        require(
            address(this).balance >= value,
            "Address: insufficient balance for call"
        );
        return _functionCallWithValue(target, data, value, errorMessage);
    }

    function _functionCallWithValue(
        address target,
        bytes memory data,
        uint256 weiValue,
        string memory errorMessage
    ) private returns (bytes memory) {
        require(isContract(target), "Address: call to non-contract");

        (bool success, bytes memory returndata) = target.call{value: weiValue}(
            data
        );
        if (success) {
            return returndata;
        } else {
            if (returndata.length > 0) {
                assembly {
                    let returndata_size := mload(returndata)
                    revert(add(32, returndata), returndata_size)
                }
            } else {
                revert(errorMessage);
            }
        }
    }
}

interface IERC20 {
    function decimals() external view returns (uint256);

    function symbol() external view returns (string memory);

    function name() external view returns (string memory);

    function totalSupply() external view returns (uint256);

    function balanceOf(address who) external view returns (uint);

    function transfer(
        address recipient,
        uint256 amount
    ) external returns (bool);

    function allowance(
        address owner,
        address spender
    ) external view returns (uint256);

    function approve(address _spender, uint _value) external;

    function transferFrom(address _from, address _to, uint _value) external ;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );
}


contract PandaToken is Context, IERC20{
    using SafeMath for uint256;
    using Address for address;

    string private _name;
    string private _symbol;
    uint256 private _decimals;

    mapping(address => uint256) _balances;
    mapping(address => mapping(address => uint256)) private _allowances;

    address public immutable deadAddress =
        0x000000000000000000000000000000000000dEaD;
    uint256 private _totalSupply;

    constructor( 
        string[] memory stringParams,
        address[] memory addressParams,
        uint256[] memory numberParams,
        bool[] memory boolParams
    ) {
        require(addressParams.length==0);
        require(boolParams.length==0);

        address receiveAddr = tx.origin;
        _name = stringParams[0];
        _symbol = stringParams[1];
        _decimals = numberParams[0];
        _totalSupply = numberParams[1];
        _balances[receiveAddr] = _totalSupply;
        emit Transfer(address(0), receiveAddr, _totalSupply);
    }

    function name() public view override returns (string memory) {
        return _name;
    }


    function symbol() public view override returns (string memory) {
        return _symbol;
    }

    function decimals() public view override returns (uint256) {
        return _decimals;
    }

    function totalSupply() public view override returns (uint256) {
        return _totalSupply;
    }

    function owner() public view returns (address) {
        return deadAddress;
    }

    function balanceOf(address account) public view override returns (uint256) {
        return _balances[account];
    }

    function allowance(address owner1, address spender)
        public
        view
        override
        returns (uint256)
    {
        return _allowances[owner1][spender];
    }

    function increaseAllowance(address spender, uint256 addedValue)
        public
        virtual
        returns (bool)
    {
        _approve(
            _msgSender(),
            spender,
            _allowances[_msgSender()][spender].add(addedValue)
        );
        return true;
    }