import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface LottoGameFields {
  /** The authority of this LottoGame instance. */
  authority: PublicKey
  /** The round number of this LottoGame instance. */
  round: BN
  /** The start date of this LottoGame instance. */
  startDate: BN
  /** The end date of this LottoGame instance. */
  endDate: BN
  /** The price of a ticket (in USDC) for this round/LottoGame instance. */
  ticketPrice: BN
  /** The total number of tickets sold for this round/LottoGame instance. */
  ticketsSold: BN
  /** The mint used for ticket sales (in USDC). */
  lottoGameMint: PublicKey
  /** The vault where the USDC ticket sales are stored. */
  lottoGameVault: PublicKey
  /** The winning numbers of this round/LottoGame instance. */
  winningNumbers: Array<number>
  /** The bump seed of this round/LottoGame instance. */
  bump: number
  /** The bump seed of this round/LottoGame vault. */
  lottoGameVaultBump: number
  /** The winning ticket of this round/LottoGame instance. */
  winningTicket: PublicKey
  /** The state of this round/LottoGame instance. */
  state: types.LottoGameStateKind
}

export interface LottoGameJSON {
  /** The authority of this LottoGame instance. */
  authority: string
  /** The round number of this LottoGame instance. */
  round: string
  /** The start date of this LottoGame instance. */
  startDate: string
  /** The end date of this LottoGame instance. */
  endDate: string
  /** The price of a ticket (in USDC) for this round/LottoGame instance. */
  ticketPrice: string
  /** The total number of tickets sold for this round/LottoGame instance. */
  ticketsSold: string
  /** The mint used for ticket sales (in USDC). */
  lottoGameMint: string
  /** The vault where the USDC ticket sales are stored. */
  lottoGameVault: string
  /** The winning numbers of this round/LottoGame instance. */
  winningNumbers: Array<number>
  /** The bump seed of this round/LottoGame instance. */
  bump: number
  /** The bump seed of this round/LottoGame vault. */
  lottoGameVaultBump: number
  /** The winning ticket of this round/LottoGame instance. */
  winningTicket: string
  /** The state of this round/LottoGame instance. */
  state: types.LottoGameStateJSON
}

export class LottoGame {
  /** The authority of this LottoGame instance. */
  readonly authority: PublicKey
  /** The round number of this LottoGame instance. */
  readonly round: BN
  /** The start date of this LottoGame instance. */
  readonly startDate: BN
  /** The end date of this LottoGame instance. */
  readonly endDate: BN
  /** The price of a ticket (in USDC) for this round/LottoGame instance. */
  readonly ticketPrice: BN
  /** The total number of tickets sold for this round/LottoGame instance. */
  readonly ticketsSold: BN
  /** The mint used for ticket sales (in USDC). */
  readonly lottoGameMint: PublicKey
  /** The vault where the USDC ticket sales are stored. */
  readonly lottoGameVault: PublicKey
  /** The winning numbers of this round/LottoGame instance. */
  readonly winningNumbers: Array<number>
  /** The bump seed of this round/LottoGame instance. */
  readonly bump: number
  /** The bump seed of this round/LottoGame vault. */
  readonly lottoGameVaultBump: number
  /** The winning ticket of this round/LottoGame instance. */
  readonly winningTicket: PublicKey
  /** The state of this round/LottoGame instance. */
  readonly state: types.LottoGameStateKind

  static readonly discriminator = Buffer.from([
    226, 110, 158, 219, 41, 231, 30, 168,
  ])

  static readonly layout = borsh.struct([
    borsh.publicKey("authority"),
    borsh.u64("round"),
    borsh.i64("startDate"),
    borsh.i64("endDate"),
    borsh.u64("ticketPrice"),
    borsh.u64("ticketsSold"),
    borsh.publicKey("lottoGameMint"),
    borsh.publicKey("lottoGameVault"),
    borsh.array(borsh.u8(), 6, "winningNumbers"),
    borsh.u8("bump"),
    borsh.u8("lottoGameVaultBump"),
    borsh.publicKey("winningTicket"),
    types.LottoGameState.layout("state"),
  ])

  constructor(fields: LottoGameFields) {
    this.authority = fields.authority
    this.round = fields.round
    this.startDate = fields.startDate
    this.endDate = fields.endDate
    this.ticketPrice = fields.ticketPrice
    this.ticketsSold = fields.ticketsSold
    this.lottoGameMint = fields.lottoGameMint
    this.lottoGameVault = fields.lottoGameVault
    this.winningNumbers = fields.winningNumbers
    this.bump = fields.bump
    this.lottoGameVaultBump = fields.lottoGameVaultBump
    this.winningTicket = fields.winningTicket
    this.state = fields.state
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<LottoGame | null> {
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
  ): Promise<Array<LottoGame | null>> {
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

  static decode(data: Buffer): LottoGame {
    if (!data.subarray(0, 8).equals(LottoGame.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = LottoGame.layout.decode(data.subarray(8))

    return new LottoGame({
      authority: dec.authority,
      round: dec.round,
      startDate: dec.startDate,
      endDate: dec.endDate,
      ticketPrice: dec.ticketPrice,
      ticketsSold: dec.ticketsSold,
      lottoGameMint: dec.lottoGameMint,
      lottoGameVault: dec.lottoGameVault,
      winningNumbers: dec.winningNumbers,
      bump: dec.bump,
      lottoGameVaultBump: dec.lottoGameVaultBump,
      winningTicket: dec.winningTicket,
      state: types.LottoGameState.fromDecoded(dec.state),
    })
  }

  toJSON(): LottoGameJSON {
    return {
      authority: this.authority.toString(),
      round: this.round.toString(),
      startDate: this.startDate.toString(),
      endDate: this.endDate.toString(),
      ticketPrice: this.ticketPrice.toString(),
      ticketsSold: this.ticketsSold.toString(),
      lottoGameMint: this.lottoGameMint.toString(),
      lottoGameVault: this.lottoGameVault.toString(),
      winningNumbers: this.winningNumbers,
      bump: this.bump,
      lottoGameVaultBump: this.lottoGameVaultBump,
      winningTicket: this.winningTicket.toString(),
      state: this.state.toJSON(),
    }
  }

  static fromJSON(obj: LottoGameJSON): LottoGame {
    return new LottoGame({
      authority: new PublicKey(obj.authority),
      round: new BN(obj.round),
      startDate: new BN(obj.startDate),
      endDate: new BN(obj.endDate),
      ticketPrice: new BN(obj.ticketPrice),
      ticketsSold: new BN(obj.ticketsSold),
      lottoGameMint: new PublicKey(obj.lottoGameMint),
      lottoGameVault: new PublicKey(obj.lottoGameVault),
      winningNumbers: obj.winningNumbers,
      bump: obj.bump,
      lottoGameVaultBump: obj.lottoGameVaultBump,
      winningTicket: new PublicKey(obj.winningTicket),
      state: types.LottoGameState.fromJSON(obj.state),
    })
  }
}
