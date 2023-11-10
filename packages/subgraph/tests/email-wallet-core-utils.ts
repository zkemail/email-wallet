import { newMockEvent } from "matchstick-as"
import { ethereum, BigInt, Bytes, Address } from "@graphprotocol/graph-ts"
import { EmailOpHandled } from "../generated/EmailWalletCore/EmailWalletCore"

export function createEmailOpHandledEvent(
  success: boolean,
  registeredUnclaimId: BigInt,
  emailNullifier: Bytes,
  emailAddrPointer: Bytes,
  recipientEmailAddrCommit: Bytes,
  recipientETHAddr: Address,
  err: Bytes
): EmailOpHandled {
  let emailOpHandledEvent = changetype<EmailOpHandled>(newMockEvent())

  emailOpHandledEvent.parameters = new Array()

  emailOpHandledEvent.parameters.push(
    new ethereum.EventParam("success", ethereum.Value.fromBoolean(success))
  )
  emailOpHandledEvent.parameters.push(
    new ethereum.EventParam(
      "registeredUnclaimId",
      ethereum.Value.fromUnsignedBigInt(registeredUnclaimId)
    )
  )
  emailOpHandledEvent.parameters.push(
    new ethereum.EventParam(
      "emailNullifier",
      ethereum.Value.fromFixedBytes(emailNullifier)
    )
  )
  emailOpHandledEvent.parameters.push(
    new ethereum.EventParam(
      "emailAddrPointer",
      ethereum.Value.fromFixedBytes(emailAddrPointer)
    )
  )
  emailOpHandledEvent.parameters.push(
    new ethereum.EventParam(
      "recipientEmailAddrCommit",
      ethereum.Value.fromFixedBytes(recipientEmailAddrCommit)
    )
  )
  emailOpHandledEvent.parameters.push(
    new ethereum.EventParam(
      "recipientETHAddr",
      ethereum.Value.fromAddress(recipientETHAddr)
    )
  )
  emailOpHandledEvent.parameters.push(
    new ethereum.EventParam("err", ethereum.Value.fromBytes(err))
  )

  return emailOpHandledEvent
}
