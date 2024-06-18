import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Connection,
  PublicKey,
  Keypair,
} from "@solana/web3.js";
import { LottoTicketNumbersFields } from "../../../lollys-lotto-ts-sdk/src/codegen/types";
import { 
    getLottoGameVaultPda, 
    getLottoGameVaultSignerPda, 
    getLottoTicketPda, 
    getUserMetadataPda, 
    getUserRewardsVaultPda, 
    sendVersionedTx 
} from "../utils";
import { 
    CrankTransferWinningAmountToUserRewardsVaultAccounts, 
    crankTransferWinningAmountToUserRewardsVault,
    CrankTransferWinningAmountToUserRewardsVaultArgs 
} from "../../../lollys-lotto-ts-sdk/src/codegen/instructions";

export async function crankTransferWinningAmountToUserRewardsVaultTx(
    lollysLottoProgram: anchor.Program,
    round: anchor.BN,
    winningNumbers: LottoTicketNumbersFields,
    numberOfTicketsWithDuplicateNumbers: number,
    authority: Keypair,
    lottoGamePda: PublicKey,
    user: PublicKey,
    userMetadataPda: PublicKey,
    lottoNumbers: LottoTicketNumbersFields,
    eventEmitterPda: PublicKey,
    connection: Connection,
): Promise<string> {
    const lottoGameVaultSigner = getLottoGameVaultSignerPda(lottoGamePda, lollysLottoProgram.programId);
    const lottoGameVaultPda = await getLottoGameVaultPda(lottoGameVaultSigner);
    const lottoTicketPda = getLottoTicketPda(lottoGamePda, userMetadataPda, lottoNumbers, lollysLottoProgram.programId);
    const userRewardsVault = await getUserRewardsVaultPda(user, lollysLottoProgram.programId);
    let sig: string;
    try {
        const crankTransferWinningAmountToUserRewardsVaultArgs: CrankTransferWinningAmountToUserRewardsVaultArgs = {
            round,
            winningNumbers,
            numberOfTicketsWithDuplicateNumbers,
        };
        const crankTransferWinningAmountToUserRewardsVaultAccounts: CrankTransferWinningAmountToUserRewardsVaultAccounts = {
            authority: authority.publicKey,
            lottoGame: lottoGamePda,
            lottoGameVaultSigner,
            lottoGameVault: lottoGameVaultPda,
            user,
            userMetadata: userMetadataPda,
            userRewardsVault: userRewardsVault,
            lottoTicket: lottoTicketPda,
            eventEmitter: eventEmitterPda,
            tokenProgram: TOKEN_PROGRAM_ID,
        };
        const crankTransferWinningAmountToUserRewardsVaultIx = crankTransferWinningAmountToUserRewardsVault(crankTransferWinningAmountToUserRewardsVaultArgs, crankTransferWinningAmountToUserRewardsVaultAccounts, lollysLottoProgram.programId);
        sig = await sendVersionedTx(connection, [crankTransferWinningAmountToUserRewardsVaultIx], authority.publicKey, [authority]);
        console.log(`[TX] crankLottoGameWinner: ${sig}`);
    } catch {
        console.log(`Error cranking Lotto Game to transfer winning amount to user rewards vault for round ${round.toNumber()}`);
    }
    return sig;
}
