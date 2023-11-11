import { ExtensionPublished as ExtensionPublishedEvent } from "../generated/ExtensionHandler/ExtensionHandler";
import { Extension } from "../generated/schema";

export function handleExtensionPublished(event: ExtensionPublishedEvent): void {
  let extension = new Extension(event.params.name);
  extension.name = event.params.name.toString();
  extension.extensionAddr = event.params.extensionAddr;
  extension.subjectTemplates = event.params.subjectTemplates;
  extension.maxExecutionGas = event.params.maxExecutionGas;

  extension.createdAt = event.block.number;
  extension.createdBy = event.transaction.from;

  extension.save();
}
