import {
  UnclaimedFundClaimed as UnclaimedFundClaimedEvent,
  UnclaimedFundRegistered as UnclaimedFundRegisteredEvent,
  UnclaimedFundVoided as UnclaimedFundVoidedEvent,
  UnclaimedStateClaimed as UnclaimedStateClaimedEvent,
  UnclaimedStateRegistered as UnclaimedStateRegisteredEvent,
  UnclaimedStateVoided as UnclaimedStateVoidedEvent,
} from "../generated/UnclaimsHandler/UnclaimsHandler";
import { UnclaimedFund, UnclaimedState } from "../generated/schema";

export function handleUnclaimedFundRegistered(event: UnclaimedFundRegisteredEvent): void {
  let unclaimedFund = new UnclaimedFund(event.params.id.toString());
  unclaimedFund.tokenAddr = event.params.tokenAddr;
  unclaimedFund.amount = event.params.amount;
  unclaimedFund.sender = event.params.sender;
  unclaimedFund.expiryTime = event.params.expiryTime;
  unclaimedFund.commitmentRandomness = event.params.commitmentRandomness;
  unclaimedFund.emailAddr = event.params.emailAddr;

  unclaimedFund.createdAt = event.block.number;
  unclaimedFund.save();
}

export function handleUnclaimedFundClaimed(event: UnclaimedFundClaimedEvent): void {
  let unclaimedFund = UnclaimedFund.load(event.params.id.toString());
  if (unclaimedFund == null) {
    throw new Error("UnclaimedFund not found");
  }

  unclaimedFund.recipient = event.params.recipient;
  unclaimedFund.claimedAt = event.block.number;
  unclaimedFund.save();
}

export function handleUnclaimedFundVoided(event: UnclaimedFundVoidedEvent): void {
  let unclaimedFund = UnclaimedFund.load(event.params.id.toString());
  if (unclaimedFund == null) {
    throw new Error("UnclaimedFund not found");
  }

  unclaimedFund.voidedAt = event.block.number;
  unclaimedFund.save();
}

export function handleUnclaimedStateRegistered(event: UnclaimedStateRegisteredEvent): void {
  let unclaimedState = new UnclaimedState(event.params.id.toString());
  unclaimedState.extensionAddr = event.params.extensionAddr;
  unclaimedState.sender = event.params.sender;
  unclaimedState.expiryTime = event.params.expiryTime;
  unclaimedState.state = event.params.state;
  unclaimedState.commitmentRandomness = event.params.commitmentRandomness;
  unclaimedState.emailAddr = event.params.emailAddr;

  unclaimedState.createdAt = event.block.number;
  unclaimedState.save();
}

export function handleUnclaimedStateClaimed(event: UnclaimedStateClaimedEvent): void {
  let unclaimedState = UnclaimedState.load(event.params.id.toString());
  if (unclaimedState == null) {
    throw new Error("UnclaimedState not found");
  }

  unclaimedState.recipient = event.params.recipient;
  unclaimedState.claimedAt = event.block.number;
  unclaimedState.save();
}

export function handleUnclaimedStateVoided(event: UnclaimedStateVoidedEvent): void {
  let unclaimedState = UnclaimedState.load(event.params.id.toString());
  if (unclaimedState == null) {
    throw new Error("UnclaimedState not found");
  }

  unclaimedState.voidedAt = event.block.number;
  unclaimedState.save();
}
