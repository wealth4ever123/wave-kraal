const sdk = require('@stellar/stellar-sdk');

class StellarSdk {
  constructor({ rpcUrl, networkPassphrase, secretKey, taskRegistryId, executionEngineId }) {
    this.server = new sdk.rpc.Server(rpcUrl);
    this.networkPassphrase = networkPassphrase;
    this.keypair = sdk.Keypair.fromSecret(secretKey);
    this.taskRegistryId = taskRegistryId;
    this.executionEngineId = executionEngineId;
  }

  async getPendingTasks() {
    // Fetch task count then retrieve tasks
    try {
      const countResult = await this.server.simulateTransaction(
        await this._buildCall(this.taskRegistryId, 'task_count', [])
      );
      const count = sdk.scValToNative(countResult.result.retval);
      const tasks = [];
      for (let i = 1; i <= count; i++) {
        try {
          const res = await this.server.simulateTransaction(
            await this._buildCall(this.taskRegistryId, 'get_task', [
              sdk.nativeToScVal(i, { type: 'u32' }),
            ])
          );
          tasks.push(sdk.scValToNative(res.result.retval));
        } catch (_) {}
      }
      return tasks;
    } catch (err) {
      console.error('[stellar] getPendingTasks error:', err.message);
      return [];
    }
  }

  async buildExecuteTaskTx(taskId, ledgerTime) {
    return this._buildCall(this.executionEngineId, 'execute_task', [
      new sdk.Address(this.keypair.publicKey()).toScVal(),
      sdk.nativeToScVal(taskId, { type: 'u32' }),
      sdk.nativeToScVal(ledgerTime, { type: 'u64' }),
    ]);
  }

  async submitTx(tx) {
    const account = await this.server.getAccount(this.keypair.publicKey());
    const built = new sdk.TransactionBuilder(account, {
      fee: '100',
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(sdk.Operation.invokeContractFunction({
        contract: tx.contract,
        function: tx.function,
        args: tx.args,
      }))
      .setTimeout(30)
      .build();
    built.sign(this.keypair);
    const result = await this.server.sendTransaction(built);
    return result;
  }

  async _buildCall(contractId, method, args) {
    return { contract: contractId, function: method, args };
  }
}

module.exports = { StellarSdk };
