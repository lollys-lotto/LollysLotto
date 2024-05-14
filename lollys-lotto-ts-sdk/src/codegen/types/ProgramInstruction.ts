import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface BurnLollyJSON {
  kind: "BurnLolly"
}

export class BurnLolly {
  static readonly discriminator = 0
  static readonly kind = "BurnLolly"
  readonly discriminator = 0
  readonly kind = "BurnLolly"

  toJSON(): BurnLollyJSON {
    return {
      kind: "BurnLolly",
    }
  }

  toEncodable() {
    return {
      BurnLolly: {},
    }
  }
}

export interface CloseEventEmitterJSON {
  kind: "CloseEventEmitter"
}

export class CloseEventEmitter {
  static readonly discriminator = 1
  static readonly kind = "CloseEventEmitter"
  readonly discriminator = 1
  readonly kind = "CloseEventEmitter"

  toJSON(): CloseEventEmitterJSON {
    return {
      kind: "CloseEventEmitter",
    }
  }

  toEncodable() {
    return {
      CloseEventEmitter: {},
    }
  }
}

export interface CloseLollyBurnStateJSON {
  kind: "CloseLollyBurnState"
}

export class CloseLollyBurnState {
  static readonly discriminator = 2
  static readonly kind = "CloseLollyBurnState"
  readonly discriminator = 2
  readonly kind = "CloseLollyBurnState"

  toJSON(): CloseLollyBurnStateJSON {
    return {
      kind: "CloseLollyBurnState",
    }
  }

  toEncodable() {
    return {
      CloseLollyBurnState: {},
    }
  }
}

export interface CloseLollysLottoJSON {
  kind: "CloseLollysLotto"
}

export class CloseLollysLotto {
  static readonly discriminator = 3
  static readonly kind = "CloseLollysLotto"
  readonly discriminator = 3
  readonly kind = "CloseLollysLotto"

  toJSON(): CloseLollysLottoJSON {
    return {
      kind: "CloseLollysLotto",
    }
  }

  toEncodable() {
    return {
      CloseLollysLotto: {},
    }
  }
}

export interface CloseLottoGameJSON {
  kind: "CloseLottoGame"
}

export class CloseLottoGame {
  static readonly discriminator = 4
  static readonly kind = "CloseLottoGame"
  readonly discriminator = 4
  readonly kind = "CloseLottoGame"

  toJSON(): CloseLottoGameJSON {
    return {
      kind: "CloseLottoGame",
    }
  }

  toEncodable() {
    return {
      CloseLottoGame: {},
    }
  }
}

export interface CreateLollyBurnStateJSON {
  kind: "CreateLollyBurnState"
}

export class CreateLollyBurnState {
  static readonly discriminator = 5
  static readonly kind = "CreateLollyBurnState"
  readonly discriminator = 5
  readonly kind = "CreateLollyBurnState"

  toJSON(): CreateLollyBurnStateJSON {
    return {
      kind: "CreateLollyBurnState",
    }
  }

  toEncodable() {
    return {
      CreateLollyBurnState: {},
    }
  }
}

export interface CreateLollysLottoJSON {
  kind: "CreateLollysLotto"
}

export class CreateLollysLotto {
  static readonly discriminator = 6
  static readonly kind = "CreateLollysLotto"
  readonly discriminator = 6
  readonly kind = "CreateLollysLotto"

  toJSON(): CreateLollysLottoJSON {
    return {
      kind: "CreateLollysLotto",
    }
  }

  toEncodable() {
    return {
      CreateLollysLotto: {},
    }
  }
}

export interface StartLottoGameJSON {
  kind: "StartLottoGame"
}

export class StartLottoGame {
  static readonly discriminator = 7
  static readonly kind = "StartLottoGame"
  readonly discriminator = 7
  readonly kind = "StartLottoGame"

  toJSON(): StartLottoGameJSON {
    return {
      kind: "StartLottoGame",
    }
  }

  toEncodable() {
    return {
      StartLottoGame: {},
    }
  }
}

export interface SwapUsdcLollyJSON {
  kind: "SwapUsdcLolly"
}

export class SwapUsdcLolly {
  static readonly discriminator = 8
  static readonly kind = "SwapUsdcLolly"
  readonly discriminator = 8
  readonly kind = "SwapUsdcLolly"

  toJSON(): SwapUsdcLollyJSON {
    return {
      kind: "SwapUsdcLolly",
    }
  }

  toEncodable() {
    return {
      SwapUsdcLolly: {},
    }
  }
}

export interface ProcessWinningNumbersJSON {
  kind: "ProcessWinningNumbers"
}

export class ProcessWinningNumbers {
  static readonly discriminator = 9
  static readonly kind = "ProcessWinningNumbers"
  readonly discriminator = 9
  readonly kind = "ProcessWinningNumbers"

  toJSON(): ProcessWinningNumbersJSON {
    return {
      kind: "ProcessWinningNumbers",
    }
  }

  toEncodable() {
    return {
      ProcessWinningNumbers: {},
    }
  }
}

export interface RequestWinningNumbersJSON {
  kind: "RequestWinningNumbers"
}

export class RequestWinningNumbers {
  static readonly discriminator = 10
  static readonly kind = "RequestWinningNumbers"
  readonly discriminator = 10
  readonly kind = "RequestWinningNumbers"

  toJSON(): RequestWinningNumbersJSON {
    return {
      kind: "RequestWinningNumbers",
    }
  }

  toEncodable() {
    return {
      RequestWinningNumbers: {},
    }
  }
}

export interface TestEmitWinningNumbersJSON {
  kind: "TestEmitWinningNumbers"
}

export class TestEmitWinningNumbers {
  static readonly discriminator = 11
  static readonly kind = "TestEmitWinningNumbers"
  readonly discriminator = 11
  readonly kind = "TestEmitWinningNumbers"

  toJSON(): TestEmitWinningNumbersJSON {
    return {
      kind: "TestEmitWinningNumbers",
    }
  }

  toEncodable() {
    return {
      TestEmitWinningNumbers: {},
    }
  }
}

export interface BuyLottoTicketJSON {
  kind: "BuyLottoTicket"
}

export class BuyLottoTicket {
  static readonly discriminator = 12
  static readonly kind = "BuyLottoTicket"
  readonly discriminator = 12
  readonly kind = "BuyLottoTicket"

  toJSON(): BuyLottoTicketJSON {
    return {
      kind: "BuyLottoTicket",
    }
  }

  toEncodable() {
    return {
      BuyLottoTicket: {},
    }
  }
}

export interface ClaimUserRewardsJSON {
  kind: "ClaimUserRewards"
}

export class ClaimUserRewards {
  static readonly discriminator = 13
  static readonly kind = "ClaimUserRewards"
  readonly discriminator = 13
  readonly kind = "ClaimUserRewards"

  toJSON(): ClaimUserRewardsJSON {
    return {
      kind: "ClaimUserRewards",
    }
  }

  toEncodable() {
    return {
      ClaimUserRewards: {},
    }
  }
}

export interface CloseLottoTicketJSON {
  kind: "CloseLottoTicket"
}

export class CloseLottoTicket {
  static readonly discriminator = 14
  static readonly kind = "CloseLottoTicket"
  readonly discriminator = 14
  readonly kind = "CloseLottoTicket"

  toJSON(): CloseLottoTicketJSON {
    return {
      kind: "CloseLottoTicket",
    }
  }

  toEncodable() {
    return {
      CloseLottoTicket: {},
    }
  }
}

export interface CloseUserMetadataJSON {
  kind: "CloseUserMetadata"
}

export class CloseUserMetadata {
  static readonly discriminator = 15
  static readonly kind = "CloseUserMetadata"
  readonly discriminator = 15
  readonly kind = "CloseUserMetadata"

  toJSON(): CloseUserMetadataJSON {
    return {
      kind: "CloseUserMetadata",
    }
  }

  toEncodable() {
    return {
      CloseUserMetadata: {},
    }
  }
}

export interface CreateUserMetadataJSON {
  kind: "CreateUserMetadata"
}

export class CreateUserMetadata {
  static readonly discriminator = 16
  static readonly kind = "CreateUserMetadata"
  readonly discriminator = 16
  readonly kind = "CreateUserMetadata"

  toJSON(): CreateUserMetadataJSON {
    return {
      kind: "CreateUserMetadata",
    }
  }

  toEncodable() {
    return {
      CreateUserMetadata: {},
    }
  }
}

export interface CrankLottoGameClosedJSON {
  kind: "CrankLottoGameClosed"
}

export class CrankLottoGameClosed {
  static readonly discriminator = 17
  static readonly kind = "CrankLottoGameClosed"
  readonly discriminator = 17
  readonly kind = "CrankLottoGameClosed"

  toJSON(): CrankLottoGameClosedJSON {
    return {
      kind: "CrankLottoGameClosed",
    }
  }

  toEncodable() {
    return {
      CrankLottoGameClosed: {},
    }
  }
}

export interface CrankLottoGameWinnersJSON {
  kind: "CrankLottoGameWinners"
}

export class CrankLottoGameWinners {
  static readonly discriminator = 18
  static readonly kind = "CrankLottoGameWinners"
  readonly discriminator = 18
  readonly kind = "CrankLottoGameWinners"

  toJSON(): CrankLottoGameWinnersJSON {
    return {
      kind: "CrankLottoGameWinners",
    }
  }

  toEncodable() {
    return {
      CrankLottoGameWinners: {},
    }
  }
}

export interface CrankTransferWinningAmountToUserRewardsVaultJSON {
  kind: "CrankTransferWinningAmountToUserRewardsVault"
}

export class CrankTransferWinningAmountToUserRewardsVault {
  static readonly discriminator = 19
  static readonly kind = "CrankTransferWinningAmountToUserRewardsVault"
  readonly discriminator = 19
  readonly kind = "CrankTransferWinningAmountToUserRewardsVault"

  toJSON(): CrankTransferWinningAmountToUserRewardsVaultJSON {
    return {
      kind: "CrankTransferWinningAmountToUserRewardsVault",
    }
  }

  toEncodable() {
    return {
      CrankTransferWinningAmountToUserRewardsVault: {},
    }
  }
}

export interface CrankTransferToBuyAndBurnVaultJSON {
  kind: "CrankTransferToBuyAndBurnVault"
}

export class CrankTransferToBuyAndBurnVault {
  static readonly discriminator = 20
  static readonly kind = "CrankTransferToBuyAndBurnVault"
  readonly discriminator = 20
  readonly kind = "CrankTransferToBuyAndBurnVault"

  toJSON(): CrankTransferToBuyAndBurnVaultJSON {
    return {
      kind: "CrankTransferToBuyAndBurnVault",
    }
  }

  toEncodable() {
    return {
      CrankTransferToBuyAndBurnVault: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.ProgramInstructionKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("BurnLolly" in obj) {
    return new BurnLolly()
  }
  if ("CloseEventEmitter" in obj) {
    return new CloseEventEmitter()
  }
  if ("CloseLollyBurnState" in obj) {
    return new CloseLollyBurnState()
  }
  if ("CloseLollysLotto" in obj) {
    return new CloseLollysLotto()
  }
  if ("CloseLottoGame" in obj) {
    return new CloseLottoGame()
  }
  if ("CreateLollyBurnState" in obj) {
    return new CreateLollyBurnState()
  }
  if ("CreateLollysLotto" in obj) {
    return new CreateLollysLotto()
  }
  if ("StartLottoGame" in obj) {
    return new StartLottoGame()
  }
  if ("SwapUsdcLolly" in obj) {
    return new SwapUsdcLolly()
  }
  if ("ProcessWinningNumbers" in obj) {
    return new ProcessWinningNumbers()
  }
  if ("RequestWinningNumbers" in obj) {
    return new RequestWinningNumbers()
  }
  if ("TestEmitWinningNumbers" in obj) {
    return new TestEmitWinningNumbers()
  }
  if ("BuyLottoTicket" in obj) {
    return new BuyLottoTicket()
  }
  if ("ClaimUserRewards" in obj) {
    return new ClaimUserRewards()
  }
  if ("CloseLottoTicket" in obj) {
    return new CloseLottoTicket()
  }
  if ("CloseUserMetadata" in obj) {
    return new CloseUserMetadata()
  }
  if ("CreateUserMetadata" in obj) {
    return new CreateUserMetadata()
  }
  if ("CrankLottoGameClosed" in obj) {
    return new CrankLottoGameClosed()
  }
  if ("CrankLottoGameWinners" in obj) {
    return new CrankLottoGameWinners()
  }
  if ("CrankTransferWinningAmountToUserRewardsVault" in obj) {
    return new CrankTransferWinningAmountToUserRewardsVault()
  }
  if ("CrankTransferToBuyAndBurnVault" in obj) {
    return new CrankTransferToBuyAndBurnVault()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.ProgramInstructionJSON
): types.ProgramInstructionKind {
  switch (obj.kind) {
    case "BurnLolly": {
      return new BurnLolly()
    }
    case "CloseEventEmitter": {
      return new CloseEventEmitter()
    }
    case "CloseLollyBurnState": {
      return new CloseLollyBurnState()
    }
    case "CloseLollysLotto": {
      return new CloseLollysLotto()
    }
    case "CloseLottoGame": {
      return new CloseLottoGame()
    }
    case "CreateLollyBurnState": {
      return new CreateLollyBurnState()
    }
    case "CreateLollysLotto": {
      return new CreateLollysLotto()
    }
    case "StartLottoGame": {
      return new StartLottoGame()
    }
    case "SwapUsdcLolly": {
      return new SwapUsdcLolly()
    }
    case "ProcessWinningNumbers": {
      return new ProcessWinningNumbers()
    }
    case "RequestWinningNumbers": {
      return new RequestWinningNumbers()
    }
    case "TestEmitWinningNumbers": {
      return new TestEmitWinningNumbers()
    }
    case "BuyLottoTicket": {
      return new BuyLottoTicket()
    }
    case "ClaimUserRewards": {
      return new ClaimUserRewards()
    }
    case "CloseLottoTicket": {
      return new CloseLottoTicket()
    }
    case "CloseUserMetadata": {
      return new CloseUserMetadata()
    }
    case "CreateUserMetadata": {
      return new CreateUserMetadata()
    }
    case "CrankLottoGameClosed": {
      return new CrankLottoGameClosed()
    }
    case "CrankLottoGameWinners": {
      return new CrankLottoGameWinners()
    }
    case "CrankTransferWinningAmountToUserRewardsVault": {
      return new CrankTransferWinningAmountToUserRewardsVault()
    }
    case "CrankTransferToBuyAndBurnVault": {
      return new CrankTransferToBuyAndBurnVault()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "BurnLolly"),
    borsh.struct([], "CloseEventEmitter"),
    borsh.struct([], "CloseLollyBurnState"),
    borsh.struct([], "CloseLollysLotto"),
    borsh.struct([], "CloseLottoGame"),
    borsh.struct([], "CreateLollyBurnState"),
    borsh.struct([], "CreateLollysLotto"),
    borsh.struct([], "StartLottoGame"),
    borsh.struct([], "SwapUsdcLolly"),
    borsh.struct([], "ProcessWinningNumbers"),
    borsh.struct([], "RequestWinningNumbers"),
    borsh.struct([], "TestEmitWinningNumbers"),
    borsh.struct([], "BuyLottoTicket"),
    borsh.struct([], "ClaimUserRewards"),
    borsh.struct([], "CloseLottoTicket"),
    borsh.struct([], "CloseUserMetadata"),
    borsh.struct([], "CreateUserMetadata"),
    borsh.struct([], "CrankLottoGameClosed"),
    borsh.struct([], "CrankLottoGameWinners"),
    borsh.struct([], "CrankTransferWinningAmountToUserRewardsVault"),
    borsh.struct([], "CrankTransferToBuyAndBurnVault"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
