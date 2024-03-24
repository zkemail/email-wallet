import { Bytes } from "@graphprotocol/graph-ts";
import { EmailOpHandled as EmailOpHandledEvent } from "../generated/EmailWalletCore/EmailWalletCore";
import { EmailOp } from "../generated/schema";

export function handleEmailOpHandled(event: EmailOpHandledEvent): void {
  let entity = new EmailOp(event.transaction.hash.concatI32(event.logIndex.toI32()));

  entity.success = event.params.success;
  entity.registeredUnclaimId = event.params.registeredUnclaimId;
  entity.emailNullifier = event.params.emailNullifier;
  entity.walletSalt = event.params.walletSalt;
  // entity.emailAddrPointer = event.params.emailAddrPointer;

  if (event.params.recipientEmailAddrCommit != Bytes.empty()) {
    entity.hasRecipient = true;
    entity.hasEmailRecipient = true;
  }

  if (event.params.recipientETHAddr != Bytes.empty()) {
    entity.hasRecipient = true;
    entity.hasEmailRecipient = false;
  }

  entity.createdAt = event.block.timestamp;
  entity.blockNumber = event.block.number;
  entity.transactionHash = event.transaction.hash;

  entity.save();
}
