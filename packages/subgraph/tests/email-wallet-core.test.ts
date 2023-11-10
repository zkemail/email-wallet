import {
  assert,
  describe,
  test,
  clearStore,
  beforeAll,
  afterAll
} from "matchstick-as/assembly/index"
import { BigInt, Bytes, Address } from "@graphprotocol/graph-ts"
import { EmailOpHandled } from "../generated/schema"
import { EmailOpHandled as EmailOpHandledEvent } from "../generated/EmailWalletCore/EmailWalletCore"
import { handleEmailOpHandled } from "../src/email-wallet-core"
import { createEmailOpHandledEvent } from "./email-wallet-core-utils"

// Tests structure (matchstick-as >=0.5.0)
// https://thegraph.com/docs/en/developer/matchstick/#tests-structure-0-5-0

describe("Describe entity assertions", () => {
  beforeAll(() => {
    let success = "boolean Not implemented"
    let registeredUnclaimId = BigInt.fromI32(234)
    let emailNullifier = Bytes.fromI32(1234567890)
    let emailAddrPointer = Bytes.fromI32(1234567890)
    let recipientEmailAddrCommit = Bytes.fromI32(1234567890)
    let recipientETHAddr = Address.fromString(
      "0x0000000000000000000000000000000000000001"
    )
    let err = Bytes.fromI32(1234567890)
    let newEmailOpHandledEvent = createEmailOpHandledEvent(
      success,
      registeredUnclaimId,
      emailNullifier,
      emailAddrPointer,
      recipientEmailAddrCommit,
      recipientETHAddr,
      err
    )
    handleEmailOpHandled(newEmailOpHandledEvent)
  })

  afterAll(() => {
    clearStore()
  })

  // For more test scenarios, see:
  // https://thegraph.com/docs/en/developer/matchstick/#write-a-unit-test

  test("EmailOpHandled created and stored", () => {
    assert.entityCount("EmailOpHandled", 1)

    // 0xa16081f360e3847006db660bae1c6d1b2e17ec2a is the default address used in newMockEvent() function
    assert.fieldEquals(
      "EmailOpHandled",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "success",
      "boolean Not implemented"
    )
    assert.fieldEquals(
      "EmailOpHandled",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "registeredUnclaimId",
      "234"
    )
    assert.fieldEquals(
      "EmailOpHandled",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "emailNullifier",
      "1234567890"
    )
    assert.fieldEquals(
      "EmailOpHandled",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "emailAddrPointer",
      "1234567890"
    )
    assert.fieldEquals(
      "EmailOpHandled",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "recipientEmailAddrCommit",
      "1234567890"
    )
    assert.fieldEquals(
      "EmailOpHandled",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "recipientETHAddr",
      "0x0000000000000000000000000000000000000001"
    )
    assert.fieldEquals(
      "EmailOpHandled",
      "0xa16081f360e3847006db660bae1c6d1b2e17ec2a-1",
      "err",
      "1234567890"
    )

    // More assert options:
    // https://thegraph.com/docs/en/developer/matchstick/#asserts
  })
})
