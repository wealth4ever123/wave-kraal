#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Keeper {
    pub address: Address,
    pub stake: i128,
    pub reputation: u32,
    pub successful_executions: u32,
    pub failed_executions: u32,
}

#[contracttype]
pub enum DataKey {
    Keeper(Address),
    AssignedTask(u32),
    Admin,
}

const MIN_STAKE: i128 = 10_000_000; // 1 XLM in stroops
const KEEPER_REGISTERED: Symbol = symbol_short!("KEEPER_R");
const KEEPER_SLASHED: Symbol = symbol_short!("KEEPER_S");

#[contract]
pub struct KeeperNetwork;

#[contractimpl]
impl KeeperNetwork {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn register_keeper(env: Env, keeper_address: Address) {
        keeper_address.require_auth();
        if env.storage().persistent().has(&DataKey::Keeper(keeper_address.clone())) {
            panic!("already registered");
        }
        let keeper = Keeper {
            address: keeper_address.clone(),
            stake: 0,
            reputation: 100,
            successful_executions: 0,
            failed_executions: 0,
        };
        env.storage().persistent().set(&DataKey::Keeper(keeper_address.clone()), &keeper);
        env.events().publish((KEEPER_REGISTERED,), keeper_address);
    }

    pub fn stake(env: Env, keeper_address: Address, amount: i128) {
        keeper_address.require_auth();
        assert!(amount >= MIN_STAKE, "below minimum stake");
        let mut keeper: Keeper = env.storage().persistent().get(&DataKey::Keeper(keeper_address.clone())).expect("keeper not found");
        keeper.stake += amount;
        env.storage().persistent().set(&DataKey::Keeper(keeper_address), &keeper);
    }

    pub fn slash_keeper(env: Env, keeper_address: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        admin.require_auth();
        let mut keeper: Keeper = env.storage().persistent().get(&DataKey::Keeper(keeper_address.clone())).expect("keeper not found");
        keeper.stake = (keeper.stake - amount).max(0);
        keeper.reputation = keeper.reputation.saturating_sub(10);
        keeper.failed_executions += 1;
        env.storage().persistent().set(&DataKey::Keeper(keeper_address.clone()), &keeper);
        env.events().publish((KEEPER_SLASHED,), keeper_address);
    }

    pub fn assign_task(env: Env, task_id: u32, keeper_address: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        admin.require_auth();
        env.storage().persistent().set(&DataKey::AssignedTask(task_id), &keeper_address);
    }

    pub fn validate_execution(env: Env, task_id: u32, keeper_address: Address) -> bool {
        let assigned: Option<Address> = env.storage().persistent().get(&DataKey::AssignedTask(task_id));
        match assigned {
            Some(a) => a == keeper_address,
            None => false,
        }
    }

    pub fn record_success(env: Env, keeper_address: Address, caller: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("not initialized");
        // allow execution engine (caller must be admin)
        admin.require_auth_for_args((&caller,).into_val(&env));
        let mut keeper: Keeper = env.storage().persistent().get(&DataKey::Keeper(keeper_address.clone())).expect("keeper not found");
        keeper.successful_executions += 1;
        keeper.reputation = (keeper.reputation + 1).min(1000);
        env.storage().persistent().set(&DataKey::Keeper(keeper_address), &keeper);
    }

    pub fn get_keeper(env: Env, keeper_address: Address) -> Keeper {
        env.storage().persistent().get(&DataKey::Keeper(keeper_address)).expect("keeper not found")
    }

    pub fn is_eligible(env: Env, keeper_address: Address) -> bool {
        match env.storage().persistent().get::<DataKey, Keeper>(&DataKey::Keeper(keeper_address)) {
            Some(k) => k.stake >= MIN_STAKE && k.reputation > 0,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_register_and_stake() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, KeeperNetwork);
        let client = KeeperNetworkClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let keeper = Address::generate(&env);
        client.init(&admin);
        client.register_keeper(&keeper);
        client.stake(&keeper, &10_000_000_i128);
        assert!(client.is_eligible(&keeper));
    }
}
