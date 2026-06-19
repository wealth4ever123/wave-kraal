const { StellarSdk } = require('./stellar');

class KeeperBot {
  constructor(config) {
    this.config = config;
    this.stellar = new StellarSdk(config);
    this.running = false;
  }

  async start() {
    console.log('[keeper] starting...');
    this.running = true;
    while (this.running) {
      try {
        await this.scanAndExecute();
      } catch (err) {
        console.error('[keeper] scan error:', err.message);
      }
      await sleep(this.config.pollIntervalMs);
    }
  }

  async scanAndExecute() {
    const tasks = await this.stellar.getPendingTasks();
    const now = Math.floor(Date.now() / 1000);
    for (const task of tasks) {
      if (this.isTriggerable(task, now)) {
        console.log(`[keeper] executing task ${task.task_id}`);
        await this.executeTask(task).catch(err =>
          console.error(`[keeper] task ${task.task_id} failed:`, err.message)
        );
      }
    }
  }

  isTriggerable(task, now) {
    if (task.status !== 'Pending') return false;
    if (task.trigger_type === 'Time') return now >= task.execute_after;
    if (task.trigger_type === 'Condition' || task.trigger_type === 'Oracle') {
      // Off-chain evaluation: treat non-empty trigger_data as satisfied
      return task.trigger_data && task.trigger_data.length > 0;
    }
    return false;
  }

  async executeTask(task) {
    const tx = await this.stellar.buildExecuteTaskTx(
      task.task_id,
      Math.floor(Date.now() / 1000)
    );
    const result = await this.stellar.submitTx(tx);
    console.log(`[keeper] task ${task.task_id} executed, hash: ${result.hash}`);
    return result;
  }

  stop() { this.running = false; }
}

function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }

module.exports = { KeeperBot };
