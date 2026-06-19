#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    token, Address, Env, Symbol,
};

#[contracttype]
pub enum DataKey {
    Balance(u32),   // task_id -> reward amount
    Admin,
    Token,
}

const REWARD_DEPOSITED: Symbol = symbol_short!("REW_DEP");
const REWARD_PAID: Symbol = symbol_short!("REW_PAY");

#[contract]
pub struct RewardPool;

#[contractimpl]
impl RewardPool {
    pub fn init(env: Env, admin: Address, token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    pub fn deposit_rewards(env: Env, depositor: Address, task_id: u32, amount: i128) {
        depositor.require_auth();
        assert!(amount > 0, "amount must be positive");
        let token: Address = env.storage().instance().get(&DataKey::Token).expect("not initialized");
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);
        let current: i128 = env.storage().persistent().get(&DataKey::Balance(task_id)).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Balance(task_id), &(current + amount));
        env.events().publish((REWARD_DEPOSITED,), (task_id, amount));
    }

    pub fn distribute_rewards(env: Env, task_id: u32, keeper: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        admin.require_auth();
        let amount: i128 = env.storage().persistent().get(&DataKey::Balance(task_id)).unwrap_or(0);
        assert!(amount > 0, "no reward available");
        let token: Address = env.storage().instance().get(&DataKey::Token).expect("not initialized");
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &keeper, &amount);
        env.storage().persistent().remove(&DataKey::Balance(task_id));
        env.events().publish((REWARD_PAID,), (task_id, keeper, amount));
    }

    pub fn refund_expired_tasks(env: Env, task_id: u32, creator: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        admin.require_auth();
        let amount: i128 = env.storage().persistent().get(&DataKey::Balance(task_id)).unwrap_or(0);
        if amount > 0 {
            let token: Address = env.storage().instance().get(&DataKey::Token).expect("not initialized");
            let token_client = token::Client::new(&env, &token);
            token_client.transfer(&env.current_contract_address(), &creator, &amount);
            env.storage().persistent().remove(&DataKey::Balance(task_id));
        }
    }

    pub fn get_balance(env: Env, task_id: u32) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(task_id)).unwrap_or(0)
    }
}
