import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CrankTransferWinningAmountToUserRewardsVaultEventFields {
  round: BN
  winningNumbers: types.LottoTicketNumbersFields
  numberOfTicketsWithDuplicateNumbers: number
  lottoGame: PublicKey
  user: PublicKey
  lottoTicket: PublicKey
  winningAmount: BN
}

export interface CrankTransferWinningAmountToUserRewardsVaultEventJSON {
  round: string
  winningNumbers: types.LottoTicketNumbersJSON
  numberOfTicketsWithDuplicateNumbers: number
  lottoGame: string
  user: string
  lottoTicket: string
  winningAmount: string
}

/** Event emitted when admin crank transfers winning amount to user rewards vault. */
export class CrankTransferWinningAmountToUserRewardsVaultEvent {
  readonly round: BN
  readonly winningNumbers: types.LottoTicketNumbers
  readonly numberOfTicketsWithDuplicateNumbers: number
  readonly lottoGame: PublicKey
  readonly user: PublicKey
  readonly lottoTicket: PublicKey
  readonly winningAmount: BN

  constructor(fields: CrankTransferWinningAmountToUserRewardsVaultEventFields) {
    this.round = fields.round
    this.winningNumbers = new types.LottoTicketNumbers({
      ...fields.winningNumbers,
    })
    this.numberOfTicketsWithDuplicateNumbers =
      fields.numberOfTicketsWithDuplicateNumbers
    this.lottoGame = fields.lottoGame
    this.user = fields.user
    this.lottoTicket = fields.lottoTicket
    this.winningAmount = fields.winningAmount
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("round"),
        types.LottoTicketNumbers.layout("winningNumbers"),
        borsh.u32("numberOfTicketsWithDuplicateNumbers"),
        borsh.publicKey("lottoGame"),
        borsh.publicKey("user"),
        borsh.publicKey("lottoTicket"),
        borsh.u64("winningAmount"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CrankTransferWinningAmountToUserRewardsVaultEvent({
      round: obj.round,
      winningNumbers: types.LottoTicketNumbers.fromDecoded(obj.winningNumbers),
      numberOfTicketsWithDuplicateNumbers:
        obj.numberOfTicketsWithDuplicateNumbers,
      lottoGame: obj.lottoGame,
      user: obj.user,
      lottoTicket: obj.lottoTicket,
      winningAmount: obj.winningAmount,
    })
  }

  static toEncodable(
    fields: CrankTransferWinningAmountToUserRewardsVaultEventFields
  ) {
    return {
      round: fields.round,
      winningNumbers: types.LottoTicketNumbers.toEncodable(
        fields.winningNumbers
      ),
      numberOfTicketsWithDuplicateNumbers:
        fields.numberOfTicketsWithDuplicateNumbers,
      lottoGame: fields.lottoGame,
      user: fields.user,
      lottoTicket: fields.lottoTicket,
      winningAmount: fields.winningAmount,
    }
  }

  toJSON(): CrankTransferWinningAmountToUserRewardsVaultEventJSON {
    return {
      round: this.round.toString(),
      winningNumbers: this.winningNumbers.toJSON(),
      numberOfTicketsWithDuplicateNumbers:
        this.numberOfTicketsWithDuplicateNumbers,
      lottoGame: this.lottoGame.toString(),
      user: this.user.toString(),
      lottoTicket: this.lottoTicket.toString(),
      winningAmount: this.winningAmount.toString(),
    }
  }

  static fromJSON(
    obj: CrankTransferWinningAmountToUserRewardsVaultEventJSON
  ): CrankTransferWinningAmountToUserRewardsVaultEvent {
    return new CrankTransferWinningAmountToUserRewardsVaultEvent({
      round: new BN(obj.round),
      winningNumbers: types.LottoTicketNumbers.fromJSON(obj.winningNumbers),
      numberOfTicketsWithDuplicateNumbers:
        obj.numberOfTicketsWithDuplicateNumbers,
      lottoGame: new PublicKey(obj.lottoGame),
      user: new PublicKey(obj.user),
      lottoTicket: new PublicKey(obj.lottoTicket),
      winningAmount: new BN(obj.winningAmount),
    })
  }

  toEncodable() {
    return CrankTransferWinningAmountToUserRewardsVaultEvent.toEncodable(this)
  }
}
