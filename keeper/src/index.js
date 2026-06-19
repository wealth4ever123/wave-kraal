require('dotenv').config();
const { KeeperBot } = require('./keeper');

async function main() {
  const bot = new KeeperBot({
    rpcUrl: process.env.STELLAR_RPC_URL,
    networkPassphrase: process.env.NETWORK_PASSPHRASE,
    secretKey: process.env.KEEPER_SECRET_KEY,
    taskRegistryId: process.env.TASK_REGISTRY_CONTRACT_ID,
    executionEngineId: process.env.EXECUTION_ENGINE_CONTRACT_ID,
    pollIntervalMs: parseInt(process.env.KEEPER_POLL_INTERVAL_MS || '5000'),
  });
  await bot.start();
}

main().catch(console.error);
