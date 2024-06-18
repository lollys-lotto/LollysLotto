import * as anchor from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import { 
    ProcessWinningNumbersAccounts, processWinningNumbers 
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";

export function getProcessWinningNumbersIx(
    lollysLottoProgram: anchor.Program,
    authority: PublicKey,
    randomnessAccountData: PublicKey,
    lottoGamePda: PublicKey,
    eventEmitterPda: PublicKey,
    connection: Connection,
): TransactionInstruction{
    const processWinningNumbersAccounts: ProcessWinningNumbersAccounts = {
        authority,
        lottoGame: lottoGamePda,
        randomnessAccountData,
        eventEmitter: eventEmitterPda,
    };

    const processWinningNumbersIx = processWinningNumbers(processWinningNumbersAccounts, lollysLottoProgram.programId);
    return processWinningNumbersIx;
}