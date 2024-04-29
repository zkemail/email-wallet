import {
  RelayerConfigUpdated as RelayerConfigUpdatedEvent,
  RelayerRegistered as RelayerRegisteredEvent,
} from "../generated/RelayerHandler/RelayerHandler";
import { Relayer } from "../generated/schema";

export function handleRelayerRegistered(event: RelayerRegisteredEvent): void {
  let relayer = new Relayer(event.params.addr); // Using address as the ID
  relayer.address = event.params.addr;
  // relayer.randHash = event.params.randHash;
  relayer.emailAddress = event.params.emailAddr;
  relayer.hostname = event.params.hostname;
  relayer.createdAt = event.block.timestamp.toHexString();
  relayer.updatedAt = event.block.timestamp.toHexString();

  relayer.save();
}

export function handleRelayerConfigUpdated(event: RelayerConfigUpdatedEvent): void {
  let relayer = new Relayer(event.params.addr);
  relayer.hostname = event.params.hostname;
  relayer.updatedAt = event.block.timestamp.toHexString();

  relayer.save();
}

