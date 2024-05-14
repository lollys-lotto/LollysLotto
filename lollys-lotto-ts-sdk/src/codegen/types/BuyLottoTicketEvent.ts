import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface BuyLottoTicketEventFields {
  user: PublicKey
  userMetadata: PublicKey
  userTicketCount: BN
  lottoTicket: PublicKey
  lottoGame: PublicKey
  ticketsSold: BN
  round: BN
  ticketNumber: BN
  numbers: types.LottoTicketNumbersFields
  ticketPrice: BN
  buyDate: BN
}

export interface BuyLottoTicketEventJSON {
  user: string
  userMetadata: string
  userTicketCount: string
  lottoTicket: string
  lottoGame: string
  ticketsSold: string
  round: string
  ticketNumber: string
  numbers: types.LottoTicketNumbersJSON
  ticketPrice: string
  buyDate: string
}

/** Event emitted when a user buys a lotto ticket. */
export class BuyLottoTicketEvent {
  readonly user: PublicKey
  readonly userMetadata: PublicKey
  readonly userTicketCount: BN
  readonly lottoTicket: PublicKey
  readonly lottoGame: PublicKey
  readonly ticketsSold: BN
  readonly round: BN
  readonly ticketNumber: BN
  readonly numbers: types.LottoTicketNumbers
  readonly ticketPrice: BN
  readonly buyDate: BN

  constructor(fields: BuyLottoTicketEventFields) {
    this.user = fields.user
    this.userMetadata = fields.userMetadata
    this.userTicketCount = fields.userTicketCount
    this.lottoTicket = fields.lottoTicket
    this.lottoGame = fields.lottoGame
    this.ticketsSold = fields.ticketsSold
    this.round = fields.round
    this.ticketNumber = fields.ticketNumber
    this.numbers = new types.LottoTicketNumbers({ ...fields.numbers })
    this.ticketPrice = fields.ticketPrice
    this.buyDate = fields.buyDate
  }

  static layout(property?: string) {
    return borsh.struct(
      [
        borsh.publicKey("user"),
        borsh.publicKey("userMetadata"),
        borsh.u64("userTicketCount"),
        borsh.publicKey("lottoTicket"),
        borsh.publicKey("lottoGame"),
        borsh.u64("ticketsSold"),
        borsh.u64("round"),
        borsh.u64("ticketNumber"),
        types.LottoTicketNumbers.layout("numbers"),
        borsh.u64("ticketPrice"),
        borsh.i64("buyDate"),
      ],
      property
    )
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  static fromDecoded(obj: any) {
    return new BuyLottoTicketEvent({
      user: obj.user,
      userMetadata: obj.userMetadata,
      userTicketCount: obj.userTicketCount,
      lottoTicket: obj.lottoTicket,
      lottoGame: obj.lottoGame,
      ticketsSold: obj.ticketsSold,
      round: obj.round,
      ticketNumber: obj.ticketNumber,
      numbers: types.LottoTicketNumbers.fromDecoded(obj.numbers),
      ticketPrice: obj.ticketPrice,
      buyDate: obj.buyDate,
    })
  }

  static toEncodable(fields: BuyLottoTicketEventFields) {
    return {
      user: fields.user,
      userMetadata: fields.userMetadata,
      userTicketCount: fields.userTicketCount,
      lottoTicket: fields.lottoTicket,
      lottoGame: fields.lottoGame,
      ticketsSold: fields.ticketsSold,
      round: fields.round,
      ticketNumber: fields.ticketNumber,
      numbers: types.LottoTicketNumbers.toEncodable(fields.numbers),
      ticketPrice: fields.ticketPrice,
      buyDate: fields.buyDate,
    }
  }

  toJSON(): BuyLottoTicketEventJSON {
    return {
      user: this.user.toString(),
      userMetadata: this.userMetadata.toString(),
      userTicketCount: this.userTicketCount.toString(),
      lottoTicket: this.lottoTicket.toString(),
      lottoGame: this.lottoGame.toString(),
      ticketsSold: this.ticketsSold.toString(),
      round: this.round.toString(),
      ticketNumber: this.ticketNumber.toString(),
      numbers: this.numbers.toJSON(),
      ticketPrice: this.ticketPrice.toString(),
      buyDate: this.buyDate.toString(),
    }
  }

  static fromJSON(obj: BuyLottoTicketEventJSON): BuyLottoTicketEvent {
    return new BuyLottoTicketEvent({
      user: new PublicKey(obj.user),
      userMetadata: new PublicKey(obj.userMetadata),
      userTicketCount: new BN(obj.userTicketCount),
      lottoTicket: new PublicKey(obj.lottoTicket),
      lottoGame: new PublicKey(obj.lottoGame),
      ticketsSold: new BN(obj.ticketsSold),
      round: new BN(obj.round),
      ticketNumber: new BN(obj.ticketNumber),
      numbers: types.LottoTicketNumbers.fromJSON(obj.numbers),
      ticketPrice: new BN(obj.ticketPrice),
      buyDate: new BN(obj.buyDate),
    })
  }

  toEncodable() {
    return BuyLottoTicketEvent.toEncodable(this)
  }
}
