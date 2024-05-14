import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface CloseLottoTicketEventFields {
  round: BN
  numbers: types.LottoTicketNumbersFields
  lottoGame: PublicKey
  lottoTicket: PublicKey
  user: PublicKey
}

export interface CloseLottoTicketEventJSON {
  round: string
  numbers: types.LottoTicketNumbersJSON
  lottoGame: string
  lottoTicket: string
  user: string
}

export class CloseLottoTicketEvent {
  readonly round: BN
  readonly numbers: types.LottoTicketNumbers
  readonly lottoGame: PublicKey
  readonly lottoTicket: PublicKey
  readonly user: PublicKey

  constructor(fields: CloseLottoTicketEventFields) {
    this.round = fields.round
    this.numbers = new types.LottoTicketNumbers({ ...fields.numbers })
    this.lottoGame = fields.lottoGame
    this.lottoTicket = fields.lottoTicket
    this.user = fields.user
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.u64("round"),
        types.LottoTicketNumbers.layout("numbers"),
        borsh.publicKey("lottoGame"),
        borsh.publicKey("lottoTicket"),
        borsh.publicKey("user"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new CloseLottoTicketEvent({
      round: obj.round,
      numbers: types.LottoTicketNumbers.fromDecoded(obj.numbers),
      lottoGame: obj.lottoGame,
      lottoTicket: obj.lottoTicket,
      user: obj.user,
    })
  }

  static toEncodable(fields: CloseLottoTicketEventFields) {
    return {
      round: fields.round,
      numbers: types.LottoTicketNumbers.toEncodable(fields.numbers),
      lottoGame: fields.lottoGame,
      lottoTicket: fields.lottoTicket,
      user: fields.user,
    }
  }

  toJSON(): CloseLottoTicketEventJSON {
    return {
      round: this.round.toString(),
      numbers: this.numbers.toJSON(),
      lottoGame: this.lottoGame.toString(),
      lottoTicket: this.lottoTicket.toString(),
      user: this.user.toString(),
    }
  }

  static fromJSON(obj: CloseLottoTicketEventJSON): CloseLottoTicketEvent {
    return new CloseLottoTicketEvent({
      round: new BN(obj.round),
      numbers: types.LottoTicketNumbers.fromJSON(obj.numbers),
      lottoGame: new PublicKey(obj.lottoGame),
      lottoTicket: new PublicKey(obj.lottoTicket),
      user: new PublicKey(obj.user),
    })
  }

  toEncodable() {
    return CloseLottoTicketEvent.toEncodable(this)
  }
}
