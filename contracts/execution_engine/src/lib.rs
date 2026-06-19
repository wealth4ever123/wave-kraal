#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

#[contracttype]
pub enum DataKey {
    Executed(u32),
    Admin,
    TaskRegistry,
    KeeperNetwork,
    RewardPool,
}

const TASK_EXECUTED: Symbol = symbol_short!("TASK_EXE");

#[contract]
pub struct ExecutionEngine;

#[contractimpl]
impl ExecutionEngine {
    pub fn init(
        env: Env,
        admin: Address,
        task_registry: Address,
        keeper_network: Address,
        reward_pool: Address,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TaskRegistry, &task_registry);
        env.storage().instance().set(&DataKey::KeeperNetwork, &keeper_network);
        env.storage().instance().set(&DataKey::RewardPool, &reward_pool);
    }

    /// Keeper calls this to execute a task
    pub fn execute_task(env: Env, keeper: Address, task_id: u32, ledger_time: u64) {
        keeper.require_auth();

        // Prevent double execution
        if env.storage().persistent().has(&DataKey::Executed(task_id)) {
            panic!("already executed");
        }

        // Validate time trigger
        assert!(ledger_time >= env.ledger().timestamp(), "trigger time not reached");

        env.storage().persistent().set(&DataKey::Executed(task_id), &true);
        env.events().publish((TASK_EXECUTED,), (task_id, keeper.clone()));
    }

    pub fn verify_condition(_env: Env, condition_data: soroban_sdk::Bytes) -> bool {
        // Off-chain conditions are verified by keeper before submission;
        // on-chain we validate the data is non-empty as a basic sanity check.
        !condition_data.is_empty()
    }

    pub fn finalize_execution(env: Env, task_id: u32, keeper: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        admin.require_auth();
        assert!(
            env.storage().persistent().has(&DataKey::Executed(task_id)),
            "task not executed"
        );
        env.events().publish((symbol_short!("FINALIZED"),), (task_id, keeper));
    }

    pub fn is_executed(env: Env, task_id: u32) -> bool {
        env.storage().persistent().has(&DataKey::Executed(task_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_execute_task_once() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, ExecutionEngine);
        let client = ExecutionEngineClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let keeper = Address::generate(&env);
        client.init(&admin, &admin, &admin, &admin);
        client.execute_task(&keeper, &1_u32, &0_u64);
        assert!(client.is_executed(&1_u32));
    }

    #[test]
    #[should_panic(expected = "already executed")]
    fn test_no_double_execution() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, ExecutionEngine);
        let client = ExecutionEngineClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let keeper = Address::generate(&env);
        client.init(&admin, &admin, &admin, &admin);
        client.execute_task(&keeper, &1_u32, &0_u64);
        client.execute_task(&keeper, &1_u32, &0_u64); // should panic
    }
}
