import {
    AccountCreated as AccountCreatedEvent,
} from "../generated/AccountHandler/AccountHandler";
import { AccountCreation } from "../generated/schema";


export function handleAccountCreated(event: AccountCreatedEvent): void {
    let entity = new AccountCreation(event.params.walletSalt); // Using address as the ID
    entity.emailAddrPointer = event.params.emailAddrPointer;
    entity.accountKeyCommit = event.params.accountKeyCommit;
    entity.walletSalt = event.params.walletSalt;
    entity.psiPoint = event.params.psiPoint;

    entity.save();
}

