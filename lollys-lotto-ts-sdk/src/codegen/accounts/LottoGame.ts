import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface LottoGameFields {
  /** The bump seed of this round/LottoGame instance. */
  bump: number
  /** The bump seed of this round/LottoGame vault. */
  lottoGameVaultBump: number
  /** Version of LottoGame state, */
  version: types.LottoGameVersionKind
  /** The state of this round/LottoGame instance. */
  state: types.LottoGameStateKind
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
  /** The winning ticket of this round/LottoGame instance. */
  jackpotWinningTicket: PublicKey
  /**
   * The maximum numbers in a ticket for this round/LottoGame instance (e.g. 0-9).
   * Ticket has only 6 numbers. But the last two bytes are reserved for padding.
   */
  maxNumbersInTicket: Array<number>
  padding1: Array<number>
  /**
   * The jackpot winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set.
   * 8th byte is an indication if jackpot_winning_amount is disbursed or not (0 = not disbursed, 1 = disbursed).
   */
  jackpotWinningNumbers: types.LottoGameWinningNumbersFields
  /**
   * The tier 1 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  tier1WinningNumbers: Array<types.LottoGameWinningNumbersFields>
  /**
   * The tier 2 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  tier2WinningNumbers: Array<types.LottoGameWinningNumbersFields>
  /**
   * The tier 3 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  tier3WinningNumbers: Array<types.LottoGameWinningNumbersFields>
}

export interface LottoGameJSON {
  /** The bump seed of this round/LottoGame instance. */
  bump: number
  /** The bump seed of this round/LottoGame vault. */
  lottoGameVaultBump: number
  /** Version of LottoGame state, */
  version: types.LottoGameVersionJSON
  /** The state of this round/LottoGame instance. */
  state: types.LottoGameStateJSON
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
  /** The winning ticket of this round/LottoGame instance. */
  jackpotWinningTicket: string
  /**
   * The maximum numbers in a ticket for this round/LottoGame instance (e.g. 0-9).
   * Ticket has only 6 numbers. But the last two bytes are reserved for padding.
   */
  maxNumbersInTicket: Array<number>
  padding1: Array<number>
  /**
   * The jackpot winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set.
   * 8th byte is an indication if jackpot_winning_amount is disbursed or not (0 = not disbursed, 1 = disbursed).
   */
  jackpotWinningNumbers: types.LottoGameWinningNumbersJSON
  /**
   * The tier 1 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  tier1WinningNumbers: Array<types.LottoGameWinningNumbersJSON>
  /**
   * The tier 2 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  tier2WinningNumbers: Array<types.LottoGameWinningNumbersJSON>
  /**
   * The tier 3 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  tier3WinningNumbers: Array<types.LottoGameWinningNumbersJSON>
}

export class LottoGame {
  /** The bump seed of this round/LottoGame instance. */
  readonly bump: number
  /** The bump seed of this round/LottoGame vault. */
  readonly lottoGameVaultBump: number
  /** Version of LottoGame state, */
  readonly version: types.LottoGameVersionKind
  /** The state of this round/LottoGame instance. */
  readonly state: types.LottoGameStateKind
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
  /** The winning ticket of this round/LottoGame instance. */
  readonly jackpotWinningTicket: PublicKey
  /**
   * The maximum numbers in a ticket for this round/LottoGame instance (e.g. 0-9).
   * Ticket has only 6 numbers. But the last two bytes are reserved for padding.
   */
  readonly maxNumbersInTicket: Array<number>
  readonly padding1: Array<number>
  /**
   * The jackpot winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set.
   * 8th byte is an indication if jackpot_winning_amount is disbursed or not (0 = not disbursed, 1 = disbursed).
   */
  readonly jackpotWinningNumbers: types.LottoGameWinningNumbers
  /**
   * The tier 1 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  readonly tier1WinningNumbers: Array<types.LottoGameWinningNumbers>
  /**
   * The tier 2 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  readonly tier2WinningNumbers: Array<types.LottoGameWinningNumbers>
  /**
   * The tier 3 winning numbers of this round/LottoGame instance.
   * 7th byte is indication if the winning numbers are set. 0 = not set, 1 = set. 8th byte is padding.
   */
  readonly tier3WinningNumbers: Array<types.LottoGameWinningNumbers>

  static readonly discriminator = Buffer.from([
    226, 110, 158, 219, 41, 231, 30, 168,
  ])

  static readonly layout = borsh.struct([
    borsh.u8("bump"),
    borsh.u8("lottoGameVaultBump"),
    types.LottoGameVersion.layout("version"),
    types.LottoGameState.layout("state"),
    borsh.publicKey("authority"),
    borsh.u64("round"),
    borsh.i64("startDate"),
    borsh.i64("endDate"),
    borsh.u64("ticketPrice"),
    borsh.u64("ticketsSold"),
    borsh.publicKey("lottoGameMint"),
    borsh.publicKey("lottoGameVault"),
    borsh.publicKey("jackpotWinningTicket"),
    borsh.array(borsh.u8(), 6, "maxNumbersInTicket"),
    borsh.array(borsh.u8(), 2, "padding1"),
    types.LottoGameWinningNumbers.layout("jackpotWinningNumbers"),
    borsh.array(
      types.LottoGameWinningNumbers.layout(),
      10,
      "tier1WinningNumbers"
    ),
    borsh.array(
      types.LottoGameWinningNumbers.layout(),
      100,
      "tier2WinningNumbers"
    ),
    borsh.array(
      types.LottoGameWinningNumbers.layout(),
      1000,
      "tier3WinningNumbers"
    ),
  ])

  constructor(fields: LottoGameFields) {
    this.bump = fields.bump
    this.lottoGameVaultBump = fields.lottoGameVaultBump
    this.version = fields.version
    this.state = fields.state
    this.authority = fields.authority
    this.round = fields.round
    this.startDate = fields.startDate
    this.endDate = fields.endDate
    this.ticketPrice = fields.ticketPrice
    this.ticketsSold = fields.ticketsSold
    this.lottoGameMint = fields.lottoGameMint
    this.lottoGameVault = fields.lottoGameVault
    this.jackpotWinningTicket = fields.jackpotWinningTicket
    this.maxNumbersInTicket = fields.maxNumbersInTicket
    this.padding1 = fields.padding1
    this.jackpotWinningNumbers = new types.LottoGameWinningNumbers({
      ...fields.jackpotWinningNumbers,
    })
    this.tier1WinningNumbers = fields.tier1WinningNumbers.map(
      (item) => new types.LottoGameWinningNumbers({ ...item })
    )
    this.tier2WinningNumbers = fields.tier2WinningNumbers.map(
      (item) => new types.LottoGameWinningNumbers({ ...item })
    )
    this.tier3WinningNumbers = fields.tier3WinningNumbers.map(
      (item) => new types.LottoGameWinningNumbers({ ...item })
    )
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
    if (!data.slice(0, 8).equals(LottoGame.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = LottoGame.layout.decode(data.slice(8))

    return new LottoGame({
      bump: dec.bump,
      lottoGameVaultBump: dec.lottoGameVaultBump,
      version: types.LottoGameVersion.fromDecoded(dec.version),
      state: types.LottoGameState.fromDecoded(dec.state),
      authority: dec.authority,
      round: dec.round,
      startDate: dec.startDate,
      endDate: dec.endDate,
      ticketPrice: dec.ticketPrice,
      ticketsSold: dec.ticketsSold,
      lottoGameMint: dec.lottoGameMint,
      lottoGameVault: dec.lottoGameVault,
      jackpotWinningTicket: dec.jackpotWinningTicket,
      maxNumbersInTicket: dec.maxNumbersInTicket,
      padding1: dec.padding1,
      jackpotWinningNumbers: types.LottoGameWinningNumbers.fromDecoded(
        dec.jackpotWinningNumbers
      ),
      tier1WinningNumbers: dec.tier1WinningNumbers.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.LottoGameWinningNumbers.fromDecoded(item)
      ),
      tier2WinningNumbers: dec.tier2WinningNumbers.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.LottoGameWinningNumbers.fromDecoded(item)
      ),
      tier3WinningNumbers: dec.tier3WinningNumbers.map(
        (
          item: any /* eslint-disable-line @typescript-eslint/no-explicit-any */
        ) => types.LottoGameWinningNumbers.fromDecoded(item)
      ),
    })
  }

  toJSON(): LottoGameJSON {
    return {
      bump: this.bump,
      lottoGameVaultBump: this.lottoGameVaultBump,
      version: this.version.toJSON(),
      state: this.state.toJSON(),
      authority: this.authority.toString(),
      round: this.round.toString(),
      startDate: this.startDate.toString(),
      endDate: this.endDate.toString(),
      ticketPrice: this.ticketPrice.toString(),
      ticketsSold: this.ticketsSold.toString(),
      lottoGameMint: this.lottoGameMint.toString(),
      lottoGameVault: this.lottoGameVault.toString(),
      jackpotWinningTicket: this.jackpotWinningTicket.toString(),
      maxNumbersInTicket: this.maxNumbersInTicket,
      padding1: this.padding1,
      jackpotWinningNumbers: this.jackpotWinningNumbers.toJSON(),
      tier1WinningNumbers: this.tier1WinningNumbers.map((item) =>
        item.toJSON()
      ),
      tier2WinningNumbers: this.tier2WinningNumbers.map((item) =>
        item.toJSON()
      ),
      tier3WinningNumbers: this.tier3WinningNumbers.map((item) =>
        item.toJSON()
      ),
    }
  }

  static fromJSON(obj: LottoGameJSON): LottoGame {
    return new LottoGame({
      bump: obj.bump,
      lottoGameVaultBump: obj.lottoGameVaultBump,
      version: types.LottoGameVersion.fromJSON(obj.version),
      state: types.LottoGameState.fromJSON(obj.state),
      authority: new PublicKey(obj.authority),
      round: new BN(obj.round),
      startDate: new BN(obj.startDate),
      endDate: new BN(obj.endDate),
      ticketPrice: new BN(obj.ticketPrice),
      ticketsSold: new BN(obj.ticketsSold),
      lottoGameMint: new PublicKey(obj.lottoGameMint),
      lottoGameVault: new PublicKey(obj.lottoGameVault),
      jackpotWinningTicket: new PublicKey(obj.jackpotWinningTicket),
      maxNumbersInTicket: obj.maxNumbersInTicket,
      padding1: obj.padding1,
      jackpotWinningNumbers: types.LottoGameWinningNumbers.fromJSON(
        obj.jackpotWinningNumbers
      ),
      tier1WinningNumbers: obj.tier1WinningNumbers.map((item) =>
        types.LottoGameWinningNumbers.fromJSON(item)
      ),
      tier2WinningNumbers: obj.tier2WinningNumbers.map((item) =>
        types.LottoGameWinningNumbers.fromJSON(item)
      ),
      tier3WinningNumbers: obj.tier3WinningNumbers.map((item) =>
        types.LottoGameWinningNumbers.fromJSON(item)
      ),
    })
  }
}
