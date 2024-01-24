import { afterEach, beforeEach, test } from "vitest";
import { SContract, SWallet, SWorld, assertAccount, e } from "xsuite";

let world: SWorld;
let deployer: SWallet;
let contract: SContract;

let admin: SWallet;

beforeEach(async () => {
  world = await SWorld.start();
  deployer = await world.createWallet();
  admin = await world.createWallet({
    balance: 10_000_000_000n,
  });

  ({ contract } = await deployer.deployContract({
    code: "file:output/contract.wasm",
    codeMetadata: [],
    gasLimit: 10_000_000,
  }));
});

afterEach(async () => {
  await world.terminate();
});

test("Test", async () => {
  assertAccount(await contract.getAccountWithKvs(), {
    balance: 0n,
    kvs: [],
  });
});
test("supply", async () => {
  const result = await admin.callContract({
    callee: contract,
    gasLimit: 5_000_000,
    funcName: "supply",
    value: 1_000, // egld to send in the transaction
  });

  // assertAccount(await contract.getAccountWithKvs(), {
  //   balance: 1_000,
  //   kvs: [],
  // });
  const account = await contract.getAccountWithKvs();

  assertAccount(account, {
    balance: 1_000,
    kvs: [e.kvs.Mapper("supply_amount").Value(e.U(1_000))],
  });
});

test("set_users", async () => {
  const result = await admin.callContract({
    callee: contract,
    gasLimit: 5_000_000,
    funcName: "set_users",
    value: 0n, // egld to send in the transaction
    funcArgs: [e.List(e.Addr(admin.toString()), e.Addr(deployer.toString()))],
  });

  const account = await contract.getAccountWithKvs();

  assertAccount(account, {
    balance: 0n,
    kvs: [
      e.kvs
        .Mapper("registered_users")
        .UnorderedSet([e.Addr(admin.toString()), e.Addr(deployer.toString())]),
    ],
  });
});
