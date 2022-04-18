import { HardhatRuntimeEnvironment } from "hardhat/types"
import { HardhatRuntimeEnvironmentToken } from "./injectionTokens"
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers"
import {container, inject, injectable} from "tsyringe"
import {createSignerWithAddresss, isHardhatRuntimeEnvironment} from "./hardhatSupport"
import { Signer } from "ethers"
import * as hardhat from "hardhat";

/**
 * The accounts necessary for testing a sifchain system
 */
export class SifchainAccounts {
  constructor(
    readonly operatorAccount: SignerWithAddress,
    readonly ownerAccount: SignerWithAddress,
    readonly pauserAccount: SignerWithAddress,
    readonly validatatorAccounts: Array<SignerWithAddress>,
    readonly availableAccounts: Array<SignerWithAddress>
  ) {}
}

/**
 * Note that the hardhat environment provides accounts as promises, so
 * we need to wrap a SifchainAccounts in a promise.
 */
@injectable()
export class SifchainAccountsPromise {
  accounts: Promise<SifchainAccounts>

  constructor(accounts: Promise<SifchainAccounts>)
  constructor(
    @inject(HardhatRuntimeEnvironmentToken)
    hardhatOrAccounts: HardhatRuntimeEnvironment | Promise<SifchainAccounts>
  ) {
    const accts = new SifchainAccounts(
        createSignerWithAddresss("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", hardhat.ethers.provider),
        createSignerWithAddresss("59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d", hardhat.ethers.provider),
        createSignerWithAddresss("5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a", hardhat.ethers.provider),
        [createSignerWithAddresss("7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6", hardhat.ethers.provider)],
        [createSignerWithAddresss("47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a", hardhat.ethers.provider)],
    )
    if (isHardhatRuntimeEnvironment(hardhatOrAccounts)) {
      this.accounts = hreToSifchainAccountsAsync(hardhatOrAccounts)
    } else {
      this.accounts = hardhatOrAccounts
    }
    this.accounts = Promise.resolve(accts)
  }
}

export async function hreToSifchainAccountsAsync(
  hardhat: HardhatRuntimeEnvironment
): Promise<SifchainAccounts> {
  const accounts = await hardhat.ethers.getSigners()
  const [operatorAccount, ownerAccount, pauserAccount, validator1Account, ...extraAccounts] =
    accounts
  return new SifchainAccounts(
    operatorAccount,
    ownerAccount,
    pauserAccount,
    [validator1Account],
    extraAccounts
  )
}
