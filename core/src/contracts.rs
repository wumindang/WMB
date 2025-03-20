// 文件路径: WMB/core/src/contracts.rs

use evm::backend::{MemoryVicinity, Backend, Basic};
use evm::{Config, Context, executor::Executor, ExitReason, ExitSucceed, Handler};
use primitive_types::{H160, H256, U256};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ContractState {
    storage: HashMap<H256, H256>,
    balances: HashMap<H160, f64>,
}

pub struct ContractExecutor {
    state: ContractState,
    evm: Executor<MemoryBackend>,
}

impl ContractExecutor {
    pub fn new() -> Self {
        let vicinity = MemoryVicinity {
            gas_price: U256::from(10),
            origin: H160::zero(),
            chain_id: U256::one(),
            block_hashes: Vec::new(),
            block_number: U256::zero(),
            block_coinbase: H160::zero(),
            block_timestamp: U256::zero(),
            block_difficulty: U256::zero(),
            block_gas_limit: U256::from(1000000),
            block_base_fee_per_gas: U256::zero(),
            block_randomness: None,
        };
        let backend = MemoryBackend::new(vicinity, HashMap::new());
        let config = Config::istanbul();
        let evm = Executor::new(backend, config);
        ContractExecutor { state: ContractState::default(), evm }
    }

    pub fn deploy_contract(&mut self, caller: H160, bytecode: Vec<u8>, value: f64) -> Result<H160, String> {
        let context = Context {
            address: H160::zero(),
            caller,
            apparent_value: U256::from((value * 100.0) as u128),
        };
        let mut handler = self.evm.handler();
        match handler.create(context, bytecode.len(), &bytecode, None) {
            ExitReason::Succeed(ExitSucceed::Returned) => {
                let contract_address = self.generate_contract_address(&caller);
                self.state.balances.insert(contract_address, value);
                Ok(contract_address)
            }
            ExitReason::Error(err) => Err(format!("部署失败: {:?}", err)),
            _ => Err("未知错误".to_string()),
        }
    }

    pub fn call_contract(&mut self, caller: H160, contract_addr: H160, data: Vec<u8>, value: f64) -> Result<Vec<u8>, String> {
        let context = Context {
            address: contract_addr,
            caller,
            apparent_value: U256::from((value * 100.0) as u128),
        };
        let mut handler = self.evm.handler();
        match handler.call(context, data.len(), &data, None) {
            ExitReason::Succeed(ExitSucceed::Returned) => Ok(handler.output().to_vec()),
            ExitReason::Error(err) => Err(format!("调用失败: {:?}", err)),
            _ => Err("未知错误".to_string()),
        }
    }

    fn generate_contract_address(&self, caller: &H160) -> H160 {
        let mut hasher = Keccak256::new();
        hasher.update(caller.as_bytes());
        hasher.update((self.state.storage.len() as u64).to_be_bytes());
        let hash = hasher.finalize();
        H160::from_slice(&hash[12..32])
    }

    pub fn get_balance(&self, address: H160) -> f64 {
        *self.state.balances.get(&address).unwrap_or(&0.0)
    }
}

#[derive(Debug)]
struct MemoryBackend {
    vicinity: MemoryVicinity,
    balances: HashMap<H160, f64>,
}

impl MemoryBackend {
    fn new(vicinity: MemoryVicinity, balances: HashMap<H160, f64>) -> Self {
        MemoryBackend { vicinity, balances }
    }
}

impl Backend for MemoryBackend {
    fn gas_price(&self) -> U256 { self.vicinity.gas_price }
    fn origin(&self) -> H160 { self.vicinity.origin }
    fn block_hash(&self, _number: U256) -> H256 { H256::zero() }
    fn block_number(&self) -> U256 { self.vicinity.block_number }
    fn block_coinbase(&self) -> H160 { self.vicinity.block_coinbase }
    fn block_timestamp(&self) -> U256 { self.vicinity.block_timestamp }
    fn block_difficulty(&self) -> U256 { self.vicinity.block_difficulty }
    fn block_randomness(&self) -> Option<H256> { self.vicinity.block_randomness }
    fn block_gas_limit(&self) -> U256 { self.vicinity.block_gas_limit }
    fn block_base_fee_per_gas(&self) -> U256 { self.vicinity.block_base_fee_per_gas }
    fn chain_id(&self) -> U256 { self.vicinity.chain_id }
    fn exists(&self, _address: H160) -> bool { true }
    fn basic(&self, address: H160) -> Basic {
        Basic {
            balance: U256::from((*self.balances.get(&address).unwrap_or(&0.0) * 100.0) as u128),
            nonce: U256::zero(),
        }
    }
    fn code(&self, _address: H160) -> Vec<u8> { Vec::new() }
    fn storage(&self, _address: H160, _key: H256) -> H256 { H256::zero() }
    fn original_storage(&self, _address: H160, _key: H256) -> Option<H256> { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_execution() {
        let mut executor = ContractExecutor::new();
        let caller = H160::from_low_u64_be(1);
        let bytecode = vec![0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x00, 0xF3];
        let contract_addr = executor.deploy_contract(caller, bytecode, 1.00).expect("部署失败");
        let input = vec![0x01, 0x02, 0x03, 0x04];
        let result = executor.call_contract(caller, contract_addr, input.clone(), 0.00).expect("调用失败");
        assert_eq!(result, input);
        let balance = executor.get_balance(contract_addr);
        assert_eq!(balance, 1.00);
        println!("合约地址: {:?}", contract_addr);
    }
}