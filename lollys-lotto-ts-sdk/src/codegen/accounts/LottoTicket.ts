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
  numbers: types.LottoTicketNumbersFields
  padding1: Array<number>
  /** The price of this ticket in USDC. */
  ticketPrice: BN
  /** The date this ticket was bought. */
  buyDate: BN
  /** The date this ticket was checked for winning numbers. */
  checkDate: BN
  /** A flag to indicate if this ticket has been checked for winning numbers. */
  isChecked: number
  /** A flag to indicate number of times the numbers have been duplicated. */
  isDuplicated: number
  /** A flag to indicate if this ticket is the winning ticket of the round. */
  isWinner: number
  padding2: Array<number>
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
  numbers: types.LottoTicketNumbersJSON
  padding1: Array<number>
  /** The price of this ticket in USDC. */
  ticketPrice: string
  /** The date this ticket was bought. */
  buyDate: string
  /** The date this ticket was checked for winning numbers. */
  checkDate: string
  /** A flag to indicate if this ticket has been checked for winning numbers. */
  isChecked: number
  /** A flag to indicate number of times the numbers have been duplicated. */
  isDuplicated: number
  /** A flag to indicate if this ticket is the winning ticket of the round. */
  isWinner: number
  padding2: Array<number>
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
  readonly numbers: types.LottoTicketNumbers
  readonly padding1: Array<number>
  /** The price of this ticket in USDC. */
  readonly ticketPrice: BN
  /** The date this ticket was bought. */
  readonly buyDate: BN
  /** The date this ticket was checked for winning numbers. */
  readonly checkDate: BN
  /** A flag to indicate if this ticket has been checked for winning numbers. */
  readonly isChecked: number
  /** A flag to indicate number of times the numbers have been duplicated. */
  readonly isDuplicated: number
  /** A flag to indicate if this ticket is the winning ticket of the round. */
  readonly isWinner: number
  readonly padding2: Array<number>
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
    types.LottoTicketNumbers.layout("numbers"),
    borsh.array(borsh.u8(), 2, "padding1"),
    borsh.u64("ticketPrice"),
    borsh.i64("buyDate"),
    borsh.i64("checkDate"),
    borsh.u8("isChecked"),
    borsh.u32("isDuplicated"),
    borsh.u8("isWinner"),
    borsh.array(borsh.u8(), 2, "padding2"),
    borsh.u64("prize"),
  ])

  constructor(fields: LottoTicketFields) {
    this.user = fields.user
    this.ticketNumber = fields.ticketNumber
    this.lottoGame = fields.lottoGame
    this.round = fields.round
    this.numbers = new types.LottoTicketNumbers({ ...fields.numbers })
    this.padding1 = fields.padding1
    this.ticketPrice = fields.ticketPrice
    this.buyDate = fields.buyDate
    this.checkDate = fields.checkDate
    this.isChecked = fields.isChecked
    this.isDuplicated = fields.isDuplicated
    this.isWinner = fields.isWinner
    this.padding2 = fields.padding2
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
      numbers: types.LottoTicketNumbers.fromDecoded(dec.numbers),
      padding1: dec.padding1,
      ticketPrice: dec.ticketPrice,
      buyDate: dec.buyDate,
      checkDate: dec.checkDate,
      isChecked: dec.isChecked,
      isDuplicated: dec.isDuplicated,
      isWinner: dec.isWinner,
      padding2: dec.padding2,
      prize: dec.prize,
    })
  }

  toJSON(): LottoTicketJSON {
    return {
      user: this.user.toString(),
      ticketNumber: this.ticketNumber.toString(),
      lottoGame: this.lottoGame.toString(),
      round: this.round.toString(),
      numbers: this.numbers.toJSON(),
      padding1: this.padding1,
      ticketPrice: this.ticketPrice.toString(),
      buyDate: this.buyDate.toString(),
      checkDate: this.checkDate.toString(),
      isChecked: this.isChecked,
      isDuplicated: this.isDuplicated,
      isWinner: this.isWinner,
      padding2: this.padding2,
      prize: this.prize.toString(),
    }
  }

  static fromJSON(obj: LottoTicketJSON): LottoTicket {
    return new LottoTicket({
      user: new PublicKey(obj.user),
      ticketNumber: new BN(obj.ticketNumber),
      lottoGame: new PublicKey(obj.lottoGame),
      round: new BN(obj.round),
      numbers: types.LottoTicketNumbers.fromJSON(obj.numbers),
      padding1: obj.padding1,
      ticketPrice: new BN(obj.ticketPrice),
      buyDate: new BN(obj.buyDate),
      checkDate: new BN(obj.checkDate),
      isChecked: obj.isChecked,
      isDuplicated: obj.isDuplicated,
      isWinner: obj.isWinner,
      padding2: obj.padding2,
      prize: new BN(obj.prize),
    })
  }
}
