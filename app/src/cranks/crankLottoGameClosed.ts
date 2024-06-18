import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import { LottoTicketNumbersFields } from "../../../lollys-lotto-ts-sdk/src/codegen/types";
import { getLottoTicketPda, sendVersionedTx } from "../utils";
import { 
    CrankLottoGameClosedArgs, 
    CrankLottoGameClosedAccounts, 
    crankLottoGameClosed 
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";

export async function crankLottoGameClosedTx(
    lollysLottoProgram: anchor.Program,
    authority: Keypair,
    round: anchor.BN,
    lottoGamePda: PublicKey,
    eventEmitterPda: PublicKey,
    connection: Connection,
): Promise<PublicKey> {
    let sig;
    try {
        const crankLottoGameClosedArgs: CrankLottoGameClosedArgs = {
            round,
        };
        const crankLottoGameClosedAccounts: CrankLottoGameClosedAccounts = {
            authority: authority.publicKey,
            lottoGame: lottoGamePda,
            eventEmitter: eventEmitterPda,
        };
        const crankLottoGameClosedIx = crankLottoGameClosed(crankLottoGameClosedArgs, crankLottoGameClosedAccounts, lollysLottoProgram.programId);
        sig = await sendVersionedTx(connection, [crankLottoGameClosedIx], authority.publicKey, [authority]);
        console.log(`[TX] crankLottoGameClosed: ${sig}`);
    } catch {
        console.log(`Error cranking Lotto Game Closed for round ${round.toNumber()}`);
    }
    return sig;
}
