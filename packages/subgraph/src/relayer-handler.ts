import {
  RelayerConfigUpdated as RelayerConfigUpdatedEvent,
  RelayerRegistered as RelayerRegisteredEvent,
} from "../generated/RelayerHandler/RelayerHandler";
import { Relayer } from "../generated/schema";

export function handleRelayerRegistered(event: RelayerRegisteredEvent): void {
  let entity = new Relayer(event.params.addr); // Using address as the ID
  entity.address = event.params.addr;
  entity.randHash = event.params.randHash;
  entity.emailAddress = event.params.emailAddr;
  entity.hostname = event.params.hostname;

  entity.save();
}

export function handleRelayerConfigUpdated(event: RelayerConfigUpdatedEvent): void {
  let entity = new Relayer(event.params.addr);
  entity.hostname = event.params.hostname;

  entity.save();
}
