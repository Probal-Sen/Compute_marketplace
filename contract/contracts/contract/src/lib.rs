#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol, Vec,
};

// =========================
// Data Structures
// =========================

#[derive(Clone)]
#[contracttype]
pub struct Task {
    pub creator: Address,
    pub description: Symbol,
    pub reward: i128,
    pub completed: bool,
    pub worker: Option<Address>,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Tasks,
}

// =========================
// Contract
// =========================

#[contract]
pub struct ComputeMarketplace;

#[contractimpl]
impl ComputeMarketplace {
    // Create a new task
    pub fn create_task(
        env: Env,
        creator: Address,
        description: Symbol,
        reward: i128,
    ) -> u32 {
        creator.require_auth();

        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&DataKey::Tasks)
            .unwrap_or(Vec::new(&env));

        let task = Task {
            creator: creator.clone(),
            description,
            reward,
            completed: false,
            worker: None,
        };

        tasks.push_back(task);

        let task_id = tasks.len() - 1;

        env.storage().instance().set(&DataKey::Tasks, &tasks);

        task_id
    }

    // Accept a task
    pub fn accept_task(env: Env, worker: Address, task_id: u32) {
        worker.require_auth();

        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&DataKey::Tasks)
            .unwrap();

        let mut task = tasks.get(task_id).unwrap();

        if task.worker.is_some() {
            panic!("Task already taken");
        }

        task.worker = Some(worker);

        tasks.set(task_id, task);

        env.storage().instance().set(&DataKey::Tasks, &tasks);
    }

    // Complete a task
    pub fn complete_task(env: Env, creator: Address, task_id: u32) {
        creator.require_auth();

        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&DataKey::Tasks)
            .unwrap();

        let mut task = tasks.get(task_id).unwrap();

        if task.creator != creator {
            panic!("Only creator can complete");
        }

        if task.worker.is_none() {
            panic!("Task not assigned");
        }

        task.completed = true;

        tasks.set(task_id, task);

        env.storage().instance().set(&DataKey::Tasks, &tasks);
    }

    // View all tasks
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&DataKey::Tasks)
            .unwrap_or(Vec::new(&env))
    }
}