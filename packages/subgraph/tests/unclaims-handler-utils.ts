import { newMockEvent } from "matchstick-as"
import { ethereum, Address, BigInt, Bytes } from "@graphprotocol/graph-ts"
import {
  OwnershipTransferred,
  UnclaimedFundClaimed,
  UnclaimedFundRegistered,
  UnclaimedFundVoided,
  UnclaimedStateClaimed,
  UnclaimedStateRegistered,
  UnclaimedStateVoided
} from "../generated/UnclaimsHandler/UnclaimsHandler"

export function createOwnershipTransferredEvent(
  previousOwner: Address,
  newOwner: Address
): OwnershipTransferred {
  let ownershipTransferredEvent = changetype<OwnershipTransferred>(
    newMockEvent()
  )

  ownershipTransferredEvent.parameters = new Array()

  ownershipTransferredEvent.parameters.push(
    new ethereum.EventParam(
      "previousOwner",
      ethereum.Value.fromAddress(previousOwner)
    )
  )
  ownershipTransferredEvent.parameters.push(
    new ethereum.EventParam("newOwner", ethereum.Value.fromAddress(newOwner))
  )

  return ownershipTransferredEvent
}

export function createUnclaimedFundClaimedEvent(
  id: BigInt,
  emailAddrCommit: Bytes,
  tokenAddr: Address,
  amount: BigInt,
  recipient: Address
): UnclaimedFundClaimed {
  let unclaimedFundClaimedEvent = changetype<UnclaimedFundClaimed>(
    newMockEvent()
  )

  unclaimedFundClaimedEvent.parameters = new Array()

  unclaimedFundClaimedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )
  unclaimedFundClaimedEvent.parameters.push(
    new ethereum.EventParam(
      "emailAddrCommit",
      ethereum.Value.fromFixedBytes(emailAddrCommit)
    )
  )
  unclaimedFundClaimedEvent.parameters.push(
    new ethereum.EventParam("tokenAddr", ethereum.Value.fromAddress(tokenAddr))
  )
  unclaimedFundClaimedEvent.parameters.push(
    new ethereum.EventParam("amount", ethereum.Value.fromUnsignedBigInt(amount))
  )
  unclaimedFundClaimedEvent.parameters.push(
    new ethereum.EventParam("recipient", ethereum.Value.fromAddress(recipient))
  )

  return unclaimedFundClaimedEvent
}

export function createUnclaimedFundRegisteredEvent(
  id: BigInt,
  emailAddrCommit: Bytes,
  tokenAddr: Address,
  amount: BigInt,
  sender: Address,
  expiryTime: BigInt,
  commitmentRandomness: BigInt,
  emailAddr: string
): UnclaimedFundRegistered {
  let unclaimedFundRegisteredEvent = changetype<UnclaimedFundRegistered>(
    newMockEvent()
  )

  unclaimedFundRegisteredEvent.parameters = new Array()

  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )
  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam(
      "emailAddrCommit",
      ethereum.Value.fromFixedBytes(emailAddrCommit)
    )
  )
  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam("tokenAddr", ethereum.Value.fromAddress(tokenAddr))
  )
  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam("amount", ethereum.Value.fromUnsignedBigInt(amount))
  )
  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam("sender", ethereum.Value.fromAddress(sender))
  )
  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam(
      "expiryTime",
      ethereum.Value.fromUnsignedBigInt(expiryTime)
    )
  )
  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam(
      "commitmentRandomness",
      ethereum.Value.fromUnsignedBigInt(commitmentRandomness)
    )
  )
  unclaimedFundRegisteredEvent.parameters.push(
    new ethereum.EventParam("emailAddr", ethereum.Value.fromString(emailAddr))
  )

  return unclaimedFundRegisteredEvent
}

export function createUnclaimedFundVoidedEvent(
  id: BigInt,
  emailAddrCommit: Bytes,
  tokenAddr: Address,
  amount: BigInt,
  sender: Address
): UnclaimedFundVoided {
  let unclaimedFundVoidedEvent = changetype<UnclaimedFundVoided>(newMockEvent())

  unclaimedFundVoidedEvent.parameters = new Array()

  unclaimedFundVoidedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )
  unclaimedFundVoidedEvent.parameters.push(
    new ethereum.EventParam(
      "emailAddrCommit",
      ethereum.Value.fromFixedBytes(emailAddrCommit)
    )
  )
  unclaimedFundVoidedEvent.parameters.push(
    new ethereum.EventParam("tokenAddr", ethereum.Value.fromAddress(tokenAddr))
  )
  unclaimedFundVoidedEvent.parameters.push(
    new ethereum.EventParam("amount", ethereum.Value.fromUnsignedBigInt(amount))
  )
  unclaimedFundVoidedEvent.parameters.push(
    new ethereum.EventParam("sender", ethereum.Value.fromAddress(sender))
  )

  return unclaimedFundVoidedEvent
}

export function createUnclaimedStateClaimedEvent(
  id: BigInt,
  emailAddrCommit: Bytes,
  recipient: Address
): UnclaimedStateClaimed {
  let unclaimedStateClaimedEvent = changetype<UnclaimedStateClaimed>(
    newMockEvent()
  )

  unclaimedStateClaimedEvent.parameters = new Array()

  unclaimedStateClaimedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )
  unclaimedStateClaimedEvent.parameters.push(
    new ethereum.EventParam(
      "emailAddrCommit",
      ethereum.Value.fromFixedBytes(emailAddrCommit)
    )
  )
  unclaimedStateClaimedEvent.parameters.push(
    new ethereum.EventParam("recipient", ethereum.Value.fromAddress(recipient))
  )

  return unclaimedStateClaimedEvent
}

export function createUnclaimedStateRegisteredEvent(
  id: BigInt,
  emailAddrCommit: Bytes,
  extensionAddr: Address,
  sender: Address,
  expiryTime: BigInt,
  state: Bytes,
  commitmentRandomness: BigInt,
  emailAddr: string
): UnclaimedStateRegistered {
  let unclaimedStateRegisteredEvent = changetype<UnclaimedStateRegistered>(
    newMockEvent()
  )

  unclaimedStateRegisteredEvent.parameters = new Array()

  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )
  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam(
      "emailAddrCommit",
      ethereum.Value.fromFixedBytes(emailAddrCommit)
    )
  )
  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam(
      "extensionAddr",
      ethereum.Value.fromAddress(extensionAddr)
    )
  )
  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam("sender", ethereum.Value.fromAddress(sender))
  )
  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam(
      "expiryTime",
      ethereum.Value.fromUnsignedBigInt(expiryTime)
    )
  )
  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam("state", ethereum.Value.fromBytes(state))
  )
  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam(
      "commitmentRandomness",
      ethereum.Value.fromUnsignedBigInt(commitmentRandomness)
    )
  )
  unclaimedStateRegisteredEvent.parameters.push(
    new ethereum.EventParam("emailAddr", ethereum.Value.fromString(emailAddr))
  )

  return unclaimedStateRegisteredEvent
}

export function createUnclaimedStateVoidedEvent(
  id: BigInt,
  emailAddrCommit: Bytes,
  sender: Address
): UnclaimedStateVoided {
  let unclaimedStateVoidedEvent = changetype<UnclaimedStateVoided>(
    newMockEvent()
  )

  unclaimedStateVoidedEvent.parameters = new Array()

  unclaimedStateVoidedEvent.parameters.push(
    new ethereum.EventParam("id", ethereum.Value.fromUnsignedBigInt(id))
  )
  unclaimedStateVoidedEvent.parameters.push(
    new ethereum.EventParam(
      "emailAddrCommit",
      ethereum.Value.fromFixedBytes(emailAddrCommit)
    )
  )
  unclaimedStateVoidedEvent.parameters.push(
    new ethereum.EventParam("sender", ethereum.Value.fromAddress(sender))
  )

  return unclaimedStateVoidedEvent
}
