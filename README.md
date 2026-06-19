<div align="center">

<img src="https://img.shields.io/badge/Built%20on-Stellar-7B61FF?style=for-the-badge&logo=stellar&logoColor=white" />
<img src="https://img.shields.io/badge/Smart%20Contracts-Soroban-00B4D8?style=for-the-badge" />
<img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust&logoColor=white" />
<img src="https://img.shields.io/badge/Backend-NestJS-E0234E?style=for-the-badge&logo=nestjs&logoColor=white" />
<img src="https://img.shields.io/badge/Status-Testnet-yellow?style=for-the-badge" />

# 🌊 Wave-Kraal

**A decentralized automation protocol on Stellar — execute smart contract actions without manual calls.**

Wave-Kraal enables time-based and condition-based on-chain automation powered by an incentivized keeper network. Keepers monitor on-chain and off-chain events and trustlessly trigger transactions when predefined conditions are met — earning rewards for every successful execution.

[Overview](#-overview) · [Architecture](#-architecture) · [Contracts](#-smart-contracts) · [Getting Started](#-getting-started) · [Configuration](#-configuration) · [Development](#-development)

</div>

---

## 📖 Overview

| Feature | Description |
|---|---|
| ⏱ **Time-based triggers** | Schedule a contract call to execute at a specific ledger timestamp |
| 🔁 **Condition-based triggers** | Trigger actions when an on-chain or off-chain condition evaluates to true |
| 🔮 **Oracle triggers** | Fire tasks based on external data feeds |
| 🤖 **Keeper network** | Decentralized bots compete to execute tasks and earn XLM rewards |
| 🛡 **Slashing** | Misbehaving keepers lose stake and reputation |
| 💰 **Reward pool** | Task creators deposit rewards; distributed on successful execution |

---

## 🏗 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Wave-Kraal                           │
│                                                             │
│  ┌──────────────┐    ┌──────────────────────────────────┐  │
│  │  Task Creator │    │        Stellar / Soroban         │  │
│  │  (dApp / CLI) │───▶│                                  │  │
│  └──────────────┘    │  ┌─────────────┐  ┌───────────┐  │  │
│                       │  │TaskRegistry │  │RewardPool │  │  │
│  ┌──────────────┐    │  └──────┬──────┘  └─────┬─────┘  │  │
│  │  Keeper Bot   │    │         │               │        │  │
│  │  (Node.js)    │───▶│  ┌─────▼──────────────▼──────┐  │  │
│  └──────────────┘    │  │     ExecutionEngine         │  │  │
│                       │  └─────────────┬──────────────┘  │  │
│  ┌──────────────┐    │                │                  │  │
│  │   Backend     │    │  ┌─────────────▼──────────────┐  │  │
│  │   (NestJS)    │◀──▶│  │     KeeperNetwork          │  │  │
│  └──────────────┘    │  └────────────────────────────┘  │  │
│                       └──────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Flow

1. A creator registers a task via `TaskRegistry`, specifying trigger type, target contract, and reward.
2. The creator deposits a reward into `RewardPool` tied to the task ID.
3. Keeper bots poll `TaskRegistry` for pending tasks via the backend API or direct RPC.
4. When a trigger condition is met, a keeper calls `ExecutionEngine.execute_task()`.
5. `ExecutionEngine` prevents double execution, marks the task in `TaskRegistry`, and emits an event.
6. The admin finalizes the execution and `RewardPool` transfers the reward to the keeper.

---

## 📦 Smart Contracts

All contracts are written in **Rust** targeting the [Soroban](https://soroban.stellar.org) smart contract platform.

```
contracts/
├── task_registry/       # Task creation, updating, cancellation, state tracking
├── keeper_network/      # Keeper registration, staking, slashing, reputation
├── execution_engine/    # Task execution gate, double-execution prevention
└── reward_pool/         # Reward deposit, distribution, and refund logic
```

### `TaskRegistry`

Stores and manages automation tasks. Supports three trigger types:

- `Time` — execute after a given ledger timestamp
- `Condition` — execute when off-chain condition data is satisfied
- `Oracle` — execute on external data feed updates

**Key functions:**

| Function | Description |
|---|---|
| `create_task(creator, target, trigger_type, trigger_data, reward, execute_after)` | Register a new task, returns `task_id` |
| `update_task(task_id, trigger_data, reward, execute_after)` | Mutate a pending task (creator only) |
| `cancel_task(task_id)` | Cancel a pending task (creator only) |
| `mark_executed(task_id, execution_engine)` | Mark task as executed (engine only) |
| `get_task(task_id)` | Read a task by ID |

---

### `KeeperNetwork`

Manages the keeper registry, stake balances, and reputation scores.

**Key functions:**

| Function | Description |
|---|---|
| `register_keeper(address)` | Self-register as a keeper |
| `stake(address, amount)` | Deposit stake (minimum 1 XLM / 10,000,000 stroops) |
| `is_eligible(address)` | Returns `true` if keeper meets stake + reputation threshold |
| `slash_keeper(address, amount)` | Admin slashes stake and reduces reputation on failure |
| `record_success(address, caller)` | Increment reputation on successful execution |

---

### `ExecutionEngine`

Acts as the trusted execution gate — validates timing, prevents duplicate runs, and coordinates cross-contract finalization.

**Key functions:**

| Function | Description |
|---|---|
| `execute_task(keeper, task_id, ledger_time)` | Keeper submits execution; validates trigger time |
| `verify_condition(condition_data)` | Basic sanity check for condition-based triggers |
| `finalize_execution(task_id, keeper)` | Admin confirms and emits `FINALIZED` event |
| `is_executed(task_id)` | Check if a task has already been executed |

---

### `RewardPool`

Holds XLM (or any SEP-41 token) rewards escrowed per task. Rewards are only released upon confirmed execution.

**Key functions:**

| Function | Description |
|---|---|
| `deposit_rewards(depositor, task_id, amount)` | Lock reward funds for a task |
| `distribute_rewards(task_id, keeper)` | Release reward to keeper (admin only) |
| `refund_expired_tasks(task_id, creator)` | Refund creator for cancelled/expired tasks |
| `get_balance(task_id)` | Query escrowed reward for a task |

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/stellar-cli) (`stellar`)
- [Node.js](https://nodejs.org/) ≥ 18
- [PostgreSQL](https://www.postgresql.org/) ≥ 14

```bash
# Install Rust wasm target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Clone & Install

```bash
git clone https://github.com/wealth4ever123/wave-kraal.git
cd wave-kraal
npm install
```

### Build & Test Contracts

```bash
# Build all contracts
npm run build:contracts

# Run all contract unit tests
npm run test:contracts
```

### Deploy to Testnet

```bash
# Deploy task_registry
stellar contract deploy \
  --wasm contracts/task_registry/target/wasm32-unknown-unknown/release/task_registry.wasm \
  --network testnet \
  --source <YOUR_SECRET_KEY>

# Repeat for keeper_network, execution_engine, reward_pool
# Then update .env with the returned contract IDs
```

---

## ⚙️ Configuration

Copy `.env.example` to `.env` and fill in your values:

```bash
cp .env.example .env
```

| Variable | Description |
|---|---|
| `STELLAR_NETWORK` | `testnet` or `mainnet` |
| `STELLAR_RPC_URL` | Soroban RPC endpoint |
| `HORIZON_URL` | Horizon REST API endpoint |
| `NETWORK_PASSPHRASE` | Stellar network passphrase |
| `TASK_REGISTRY_CONTRACT_ID` | Deployed TaskRegistry contract address |
| `KEEPER_NETWORK_CONTRACT_ID` | Deployed KeeperNetwork contract address |
| `EXECUTION_ENGINE_CONTRACT_ID` | Deployed ExecutionEngine contract address |
| `REWARD_POOL_CONTRACT_ID` | Deployed RewardPool contract address |
| `KEEPER_SECRET_KEY` | Secret key for the keeper bot account |
| `KEEPER_POLL_INTERVAL_MS` | How often the keeper scans for tasks (ms) |
| `DATABASE_URL` | PostgreSQL connection string |
| `PORT` | Backend API port (default `3001`) |

---

## 🛠 Development

### Project Structure

```
wave-kraal/
├── contracts/           # Soroban smart contracts (Rust)
│   ├── task_registry/
│   ├── keeper_network/
│   ├── execution_engine/
│   └── reward_pool/
├── keeper/              # Keeper bot (Node.js)
│   └── src/
│       ├── index.js     # Entry point
│       ├── keeper.js    # Scan + execute loop
│       └── stellar.js   # Stellar SDK wrapper
├── backend/             # REST API (NestJS + TypeORM)
│   └── src/
│       ├── tasks/       # Task entity, CRUD endpoints
│       └── keepers/     # Keeper entity, registry endpoints
├── .env.example
└── package.json         # Workspace root
```

### Run Services

```bash
# Start keeper bot
npm run dev:keeper

# Start backend API
npm run dev:backend
```

### Backend API Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/tasks` | List all tasks |
| `POST` | `/tasks` | Create a task |
| `GET` | `/keepers` | List registered keepers |
| `POST` | `/keepers` | Register a keeper |

---

## 🔗 Resources

- [Stellar Developers](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Testnet Explorer](https://stellar.expert/explorer/testnet)
- [Horizon Testnet](https://horizon-testnet.stellar.org)
- [Soroban Testnet RPC](https://soroban-testnet.stellar.org)

---

## 📄 License

MIT — see [LICENSE](LICENSE) for details.

</div>
