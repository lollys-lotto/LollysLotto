import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { 
  createEventEmitter, 
  CreateEventEmitterAccounts, 
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";
import { 
  getEventEmitterPda, 
  sendVersionedTx 
} from "../utils";

export async function createEventEmitterIfNotExist(
    lollysLottoProgram: anchor.Program,
    authority: Keypair,
    connection: Connection
  ) : Promise<PublicKey> {
    
    const eventEmitterPda = getEventEmitterPda(lollysLottoProgram.programId);
  
    try {
      const eventEmitterAccount = await lollysLottoProgram.account.eventEmitter.fetch(eventEmitterPda);
      console.log(`Event Emitter Account: ${JSON.stringify(eventEmitterAccount)}`);
    } catch {
      console.log("Event Emitter Account not found, creating...");
  
      const createEventEmitterAccounts: CreateEventEmitterAccounts = {
        funder: authority.publicKey,
        eventEmitter: eventEmitterPda,
        systemProgram: SystemProgram.programId,
      };
      const createEventEmitterIx = createEventEmitter(createEventEmitterAccounts, lollysLottoProgram.programId);
  
      const sig = await sendVersionedTx(connection, [createEventEmitterIx], authority.publicKey, [authority]);
      console.log(`[TX] createEventEmitter: ${sig}`);
    }
  
    return eventEmitterPda;
  }
  