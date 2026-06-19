#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum TriggerType {
    Time,
    Condition,
    Oracle,
}

#[contracttype]
#[derive(Clone)]
pub enum TaskStatus {
    Pending,
    Executed,
    Cancelled,
}

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub task_id: u32,
    pub creator: Address,
    pub target_contract: Address,
    pub trigger_type: TriggerType,
    pub trigger_data: Bytes,
    pub reward: i128,
    pub execute_after: u64,
    pub status: TaskStatus,
}

#[contracttype]
pub enum DataKey {
    Task(u32),
    TaskCount,
}

const TASK_CREATED: Symbol = symbol_short!("TASK_CRE");
const TASK_CANCELLED: Symbol = symbol_short!("TASK_CAN");

#[contract]
pub struct TaskRegistry;

#[contractimpl]
impl TaskRegistry {
    pub fn create_task(
        env: Env,
        creator: Address,
        target_contract: Address,
        trigger_type: TriggerType,
        trigger_data: Bytes,
        reward: i128,
        execute_after: u64,
    ) -> u32 {
        creator.require_auth();
        let count: u32 = env.storage().instance().get(&DataKey::TaskCount).unwrap_or(0);
        let task_id = count + 1;
        let task = Task {
            task_id,
            creator,
            target_contract,
            trigger_type,
            trigger_data,
            reward,
            execute_after,
            status: TaskStatus::Pending,
        };
        env.storage().persistent().set(&DataKey::Task(task_id), &task);
        env.storage().instance().set(&DataKey::TaskCount, &task_id);
        env.events().publish((TASK_CREATED,), task_id);
        task_id
    }

    pub fn update_task(
        env: Env,
        task_id: u32,
        trigger_data: Bytes,
        reward: i128,
        execute_after: u64,
    ) {
        let mut task: Task = env.storage().persistent().get(&DataKey::Task(task_id)).expect("task not found");
        task.creator.require_auth();
        matches!(task.status, TaskStatus::Pending) || panic!("task not pending");
        task.trigger_data = trigger_data;
        task.reward = reward;
        task.execute_after = execute_after;
        env.storage().persistent().set(&DataKey::Task(task_id), &task);
    }

    pub fn cancel_task(env: Env, task_id: u32) {
        let mut task: Task = env.storage().persistent().get(&DataKey::Task(task_id)).expect("task not found");
        task.creator.require_auth();
        task.status = TaskStatus::Cancelled;
        env.storage().persistent().set(&DataKey::Task(task_id), &task);
        env.events().publish((TASK_CANCELLED,), task_id);
    }

    pub fn get_task(env: Env, task_id: u32) -> Task {
        env.storage().persistent().get(&DataKey::Task(task_id)).expect("task not found")
    }

    pub fn mark_executed(env: Env, task_id: u32, execution_engine: Address) {
        execution_engine.require_auth();
        let mut task: Task = env.storage().persistent().get(&DataKey::Task(task_id)).expect("task not found");
        matches!(task.status, TaskStatus::Pending) || panic!("task not pending");
        task.status = TaskStatus::Executed;
        env.storage().persistent().set(&DataKey::Task(task_id), &task);
    }

    pub fn task_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::TaskCount).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_create_and_get_task() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, TaskRegistry);
        let client = TaskRegistryClient::new(&env, &contract_id);
        let creator = Address::generate(&env);
        let target = Address::generate(&env);
        let id = client.create_task(
            &creator, &target,
            &TriggerType::Time,
            &Bytes::from_slice(&env, &[1, 2, 3]),
            &1000, &9999,
        );
        assert_eq!(id, 1);
        let task = client.get_task(&id);
        assert_eq!(task.reward, 1000);
    }
}
