import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface LottoTicketFields {
  /** The user who bought this ticket. */
  user: PublicKey
  /** The ticket number of this ticket for the current lotto_game. */
  ticketNumber: BN
  /** The LottoGame instance this ticket is associated with. */
  lottoGame: PublicKey
  /** The round number of the LottoGame instance this ticket is associated with. */
  round: BN
  /** The numbers the user has chosen for this ticket. */
  numbers: Array<number>
  /** A flag to indicate if this ticket is the winning ticket of the round. */
  isWinner: number
  /** The amount the user has been paid for this ticket if this is the winning ticket. */
  prize: BN
}

export interface LottoTicketJSON {
  /** The user who bought this ticket. */
  user: string
  /** The ticket number of this ticket for the current lotto_game. */
  ticketNumber: string
  /** The LottoGame instance this ticket is associated with. */
  lottoGame: string
  /** The round number of the LottoGame instance this ticket is associated with. */
  round: string
  /** The numbers the user has chosen for this ticket. */
  numbers: Array<number>
  /** A flag to indicate if this ticket is the winning ticket of the round. */
  isWinner: number
  /** The amount the user has been paid for this ticket if this is the winning ticket. */
  prize: string
}

export class LottoTicket {
  /** The user who bought this ticket. */
  readonly user: PublicKey
  /** The ticket number of this ticket for the current lotto_game. */
  readonly ticketNumber: BN
  /** The LottoGame instance this ticket is associated with. */
  readonly lottoGame: PublicKey
  /** The round number of the LottoGame instance this ticket is associated with. */
  readonly round: BN
  /** The numbers the user has chosen for this ticket. */
  readonly numbers: Array<number>
  /** A flag to indicate if this ticket is the winning ticket of the round. */
  readonly isWinner: number
  /** The amount the user has been paid for this ticket if this is the winning ticket. */
  readonly prize: BN

  static readonly discriminator = Buffer.from([
    101, 56, 111, 178, 180, 153, 120, 203,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("user"),
    borsh.u64("ticketNumber"),
    borsh.publicKey("lottoGame"),
    borsh.u64("round"),
    borsh.array(borsh.u8(), 6, "numbers"),
    borsh.u16("isWinner"),
    borsh.u64("prize"),
  ])

  constructor(fields: LottoTicketFields) {
    this.user = fields.user
    this.ticketNumber = fields.ticketNumber
    this.lottoGame = fields.lottoGame
    this.round = fields.round
    this.numbers = fields.numbers
    this.isWinner = fields.isWinner
    this.prize = fields.prize
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<LottoTicket | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<LottoTicket | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): LottoTicket {
    if (!data.subarray(0, 8).equals(LottoTicket.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = LottoTicket.layout.decode(data.subarray(8))

    return new LottoTicket({
      user: dec.user,
      ticketNumber: dec.ticketNumber,
      lottoGame: dec.lottoGame,
      round: dec.round,
      numbers: dec.numbers,
      isWinner: dec.isWinner,
      prize: dec.prize,
    })
  }

  toJSON(): LottoTicketJSON {
    return {
      user: this.user.toString(),
      ticketNumber: this.ticketNumber.toString(),
      lottoGame: this.lottoGame.toString(),
      round: this.round.toString(),
      numbers: this.numbers,
      isWinner: this.isWinner,
      prize: this.prize.toString(),
    }
  }

  static fromJSON(obj: LottoTicketJSON): LottoTicket {
    return new LottoTicket({
      user: new PublicKey(obj.user),
      ticketNumber: new BN(obj.ticketNumber),
      lottoGame: new PublicKey(obj.lottoGame),
      round: new BN(obj.round),
      numbers: obj.numbers,
      isWinner: obj.isWinner,
      prize: new BN(obj.prize),
    })
  }
}
