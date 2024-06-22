import { AccountCreated as AccountCreatedEvent } from "../generated/AccountHandler/AccountHandler";
import { Account, Relayer } from "../generated/schema";

export function handleAccountCreated(event: AccountCreatedEvent): void {
  let account = new Account(event.params.accountSalt);
  account.accountSalt = event.params.accountSalt;
  // account.walletAddr = event.params.accountCodeCommit;
  // account.accountSalt = event.params.accountSalt;
  account.createdAt = event.block.timestamp;
  account.save();

  let relayer = Relayer.load(event.transaction.from);
  if (relayer == null) {
    throw new Error("Relayer not found");
  }
  relayer.relayerAccounts.push(account.id);
  relayer.save();

  // let relayerAccount = new RelayerAccount(event.params.accountCodeCommit);
  // relayerAccount.emailAddrPointer = event.params.emailAddrPointer;
  // relayerAccount.accountCodeCommit = event.params.accountCodeCommit;
  // relayerAccount.psiPoint = event.params.psiPoint;
  // relayerAccount.account = account.id;
  // relayerAccount.relayer = relayer.id;
  // relayerAccount.isInitialized = false;
  // relayerAccount.createdAt = event.block.timestamp;
  // relayerAccount.save();
}

// export function handleAccountInitialized(event: AccountInitializedEvent): void {
//     let relayerAccount = RelayerAccount.load(event.params.accountCodeCommit);
//     if (relayerAccount == null) {
//         throw new Error("RelayerAccount not found");
//     }

//     relayerAccount.isInitialized = true;
//     relayerAccount.initializedAt = event.block.timestamp;
//     relayerAccount.save();
// }

// export function handleAccountTransported(event: AccountTransportedEvent): void {
//     let oldRelayerAccount = RelayerAccount.load(event.params.oldAccountCodeCommit);
//     if (oldRelayerAccount == null) {
//         throw new Error("Existing RelayerAccount not found");
//     }

//     let newRelayerAccount = new RelayerAccount(event.params.newAccountCodeCommit);
//     newRelayerAccount.emailAddrPointer = event.params.newEmailAddrPointer;
//     newRelayerAccount.accountCodeCommit = event.params.newAccountCodeCommit;
//     newRelayerAccount.psiPoint = event.params.newPSIPoint;
//     newRelayerAccount.account = oldRelayerAccount.account;
//     newRelayerAccount.relayer = event.transaction.from;
//     newRelayerAccount.isInitialized = true;
//     newRelayerAccount.initializedAt = event.block.timestamp;
//     newRelayerAccount.save();
// }
