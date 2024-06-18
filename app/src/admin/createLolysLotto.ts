import * as anchor from "@coral-xyz/anchor";
import {
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import { getLollysLottoPda } from "../utils";

export async function createLollysLottoIfNotExist(
    lollysLottoProgram: anchor.Program,
    authority: Keypair,
    eventEmitterPda: PublicKey,
  ): Promise<PublicKey> {
    const lollysLottoPda = getLollysLottoPda(authority.publicKey, lollysLottoProgram.programId);
  
    try {
      const lollysLottoAccount = await lollysLottoProgram.account.lollysLotto.fetch(lollysLottoPda);
      console.log(`Lolly Lotto Account: ${JSON.stringify(lollysLottoAccount)}`);
      console.log(`lollysLottoAccount.lottoGameCount: ${lollysLottoAccount.lottoGameCount}`);
    } catch {
      console.log("Lolly Lotto Account not found, creating...");
  
      const createLollysLottoSig = await lollysLottoProgram.methods
        .createLollysLotto()
        .accounts({
          authority: authority.publicKey,
          lollysLotto: lollysLottoPda,
          eventEmitter: eventEmitterPda,
        })
        .signers([authority])
        .rpc();
      console.log(`[TX] createLollysLotto: ${createLollysLottoSig}`);
    }
  
    return lollysLottoPda;
  }
  