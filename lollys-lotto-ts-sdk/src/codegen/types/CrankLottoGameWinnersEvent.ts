import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CrankLottoGameWinnersEventFields {
  round: BN
  winningNumbers: types.LottoTicketNumbersFields
  winningNumbersIndex: Array<BN>
  winningUser: PublicKey
  lottoTicket: PublicKey
  lottoGame: PublicKey
}

export interface CrankLottoGameWinnersEventJSON {
  round: string
  winningNumbers: types.LottoTicketNumbersJSON
  winningNumbersIndex: Array<string>
  winningUser: string
  lottoTicket: string
  lottoGame: string
}

/** Event emitted when a admin cranks a lotto game winner. */
export class CrankLottoGameWinnersEvent {
  readonly round: BN
  readonly winningNumbers: types.LottoTicketNumbers
  readonly winningNumbersIndex: Array<BN>
  readonly winningUser: PublicKey
  readonly lottoTicket: PublicKey
  readonly lottoGame: PublicKey

  constructor(fields: CrankLottoGameWinnersEventFields) {
    this.round = fields.round
    this.winningNumbers = new types.LottoTicketNumbers({
      ...fields.winningNumbers,
    })
    this.winningNumbersIndex = fields.winningNumbersIndex
    this.winningUser = fields.winningUser
    this.lottoTicket = fields.lottoTicket
    this.lottoGame = fields.lottoGame
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("round"),
        types.LottoTicketNumbers.layout("winningNumbers"),
        borsh.array(borsh.i64(), 4, "winningNumbersIndex"),
        borsh.publicKey("winningUser"),
        borsh.publicKey("lottoTicket"),
        borsh.publicKey("lottoGame"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CrankLottoGameWinnersEvent({
      round: obj.round,
      winningNumbers: types.LottoTicketNumbers.fromDecoded(obj.winningNumbers),
      winningNumbersIndex: obj.winningNumbersIndex,
      winningUser: obj.winningUser,
      lottoTicket: obj.lottoTicket,
      lottoGame: obj.lottoGame,
    })
  }

  static toEncodable(fields: CrankLottoGameWinnersEventFields) {
    return {
      round: fields.round,
      winningNumbers: types.LottoTicketNumbers.toEncodable(
        fields.winningNumbers
      ),
      winningNumbersIndex: fields.winningNumbersIndex,
      winningUser: fields.winningUser,
      lottoTicket: fields.lottoTicket,
      lottoGame: fields.lottoGame,
    }
  }

  toJSON(): CrankLottoGameWinnersEventJSON {
    return {
      round: this.round.toString(),
      winningNumbers: this.winningNumbers.toJSON(),
      winningNumbersIndex: this.winningNumbersIndex.map((item) =>
        item.toString()
      ),
      winningUser: this.winningUser.toString(),
      lottoTicket: this.lottoTicket.toString(),
      lottoGame: this.lottoGame.toString(),
    }
  }

  static fromJSON(
    obj: CrankLottoGameWinnersEventJSON
  ): CrankLottoGameWinnersEvent {
    return new CrankLottoGameWinnersEvent({
      round: new BN(obj.round),
      winningNumbers: types.LottoTicketNumbers.fromJSON(obj.winningNumbers),
      winningNumbersIndex: obj.winningNumbersIndex.map((item) => new BN(item)),
      winningUser: new PublicKey(obj.winningUser),
      lottoTicket: new PublicKey(obj.lottoTicket),
      lottoGame: new PublicKey(obj.lottoGame),
    })
  }

  toEncodable() {
    return CrankLottoGameWinnersEvent.toEncodable(this)
  }
}
