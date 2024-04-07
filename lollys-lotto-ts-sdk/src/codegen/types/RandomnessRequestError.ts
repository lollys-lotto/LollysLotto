import { PublicKey } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as types from "../types" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh"

export interface InvalidAuthorityJSON {
  kind: "InvalidAuthority"
}

export class InvalidAuthority {
  static readonly discriminator = 0
  static readonly kind = "InvalidAuthority"
  readonly discriminator = 0
  readonly kind = "InvalidAuthority"

  toJSON(): InvalidAuthorityJSON {
    return {
      kind: "InvalidAuthority",
    }
  }

  toEncodable() {
    return {
      InvalidAuthority: {},
    }
  }
}

export interface InvalidEscrowJSON {
  kind: "InvalidEscrow"
}

export class InvalidEscrow {
  static readonly discriminator = 1
  static readonly kind = "InvalidEscrow"
  readonly discriminator = 1
  readonly kind = "InvalidEscrow"

  toJSON(): InvalidEscrowJSON {
    return {
      kind: "InvalidEscrow",
    }
  }

  toEncodable() {
    return {
      InvalidEscrow: {},
    }
  }
}

export interface ArrayOverflowJSON {
  kind: "ArrayOverflow"
}

export class ArrayOverflow {
  static readonly discriminator = 2
  static readonly kind = "ArrayOverflow"
  readonly discriminator = 2
  readonly kind = "ArrayOverflow"

  toJSON(): ArrayOverflowJSON {
    return {
      kind: "ArrayOverflow",
    }
  }

  toEncodable() {
    return {
      ArrayOverflow: {},
    }
  }
}

export interface StaleDataJSON {
  kind: "StaleData"
}

export class StaleData {
  static readonly discriminator = 3
  static readonly kind = "StaleData"
  readonly discriminator = 3
  readonly kind = "StaleData"

  toJSON(): StaleDataJSON {
    return {
      kind: "StaleData",
    }
  }

  toEncodable() {
    return {
      StaleData: {},
    }
  }
}

export interface InvalidTrustedSignerJSON {
  kind: "InvalidTrustedSigner"
}

export class InvalidTrustedSigner {
  static readonly discriminator = 4
  static readonly kind = "InvalidTrustedSigner"
  readonly discriminator = 4
  readonly kind = "InvalidTrustedSigner"

  toJSON(): InvalidTrustedSignerJSON {
    return {
      kind: "InvalidTrustedSigner",
    }
  }

  toEncodable() {
    return {
      InvalidTrustedSigner: {},
    }
  }
}

export interface InvalidMrEnclaveJSON {
  kind: "InvalidMrEnclave"
}

export class InvalidMrEnclave {
  static readonly discriminator = 5
  static readonly kind = "InvalidMrEnclave"
  readonly discriminator = 5
  readonly kind = "InvalidMrEnclave"

  toJSON(): InvalidMrEnclaveJSON {
    return {
      kind: "InvalidMrEnclave",
    }
  }

  toEncodable() {
    return {
      InvalidMrEnclave: {},
    }
  }
}

export interface InvalidSymbolJSON {
  kind: "InvalidSymbol"
}

export class InvalidSymbol {
  static readonly discriminator = 6
  static readonly kind = "InvalidSymbol"
  readonly discriminator = 6
  readonly kind = "InvalidSymbol"

  toJSON(): InvalidSymbolJSON {
    return {
      kind: "InvalidSymbol",
    }
  }

  toEncodable() {
    return {
      InvalidSymbol: {},
    }
  }
}

export interface IncorrectSwitchboardFunctionJSON {
  kind: "IncorrectSwitchboardFunction"
}

export class IncorrectSwitchboardFunction {
  static readonly discriminator = 7
  static readonly kind = "IncorrectSwitchboardFunction"
  readonly discriminator = 7
  readonly kind = "IncorrectSwitchboardFunction"

  toJSON(): IncorrectSwitchboardFunctionJSON {
    return {
      kind: "IncorrectSwitchboardFunction",
    }
  }

  toEncodable() {
    return {
      IncorrectSwitchboardFunction: {},
    }
  }
}

export interface InvalidSwitchboardFunctionJSON {
  kind: "InvalidSwitchboardFunction"
}

export class InvalidSwitchboardFunction {
  static readonly discriminator = 8
  static readonly kind = "InvalidSwitchboardFunction"
  readonly discriminator = 8
  readonly kind = "InvalidSwitchboardFunction"

  toJSON(): InvalidSwitchboardFunctionJSON {
    return {
      kind: "InvalidSwitchboardFunction",
    }
  }

  toEncodable() {
    return {
      InvalidSwitchboardFunction: {},
    }
  }
}

export interface FunctionValidationFailedJSON {
  kind: "FunctionValidationFailed"
}

export class FunctionValidationFailed {
  static readonly discriminator = 9
  static readonly kind = "FunctionValidationFailed"
  readonly discriminator = 9
  readonly kind = "FunctionValidationFailed"

  toJSON(): FunctionValidationFailedJSON {
    return {
      kind: "FunctionValidationFailed",
    }
  }

  toEncodable() {
    return {
      FunctionValidationFailed: {},
    }
  }
}

export interface SwitchboardRequestNotSuccessfulJSON {
  kind: "SwitchboardRequestNotSuccessful"
}

export class SwitchboardRequestNotSuccessful {
  static readonly discriminator = 10
  static readonly kind = "SwitchboardRequestNotSuccessful"
  readonly discriminator = 10
  readonly kind = "SwitchboardRequestNotSuccessful"

  toJSON(): SwitchboardRequestNotSuccessfulJSON {
    return {
      kind: "SwitchboardRequestNotSuccessful",
    }
  }

  toEncodable() {
    return {
      SwitchboardRequestNotSuccessful: {},
    }
  }
}

export interface RoundInactiveJSON {
  kind: "RoundInactive"
}

export class RoundInactive {
  static readonly discriminator = 11
  static readonly kind = "RoundInactive"
  readonly discriminator = 11
  readonly kind = "RoundInactive"

  toJSON(): RoundInactiveJSON {
    return {
      kind: "RoundInactive",
    }
  }

  toEncodable() {
    return {
      RoundInactive: {},
    }
  }
}

export interface HouseInsufficientFundsJSON {
  kind: "HouseInsufficientFunds"
}

export class HouseInsufficientFunds {
  static readonly discriminator = 12
  static readonly kind = "HouseInsufficientFunds"
  readonly discriminator = 12
  readonly kind = "HouseInsufficientFunds"

  toJSON(): HouseInsufficientFundsJSON {
    return {
      kind: "HouseInsufficientFunds",
    }
  }

  toEncodable() {
    return {
      HouseInsufficientFunds: {},
    }
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function fromDecoded(obj: any): types.RandomnessRequestErrorKind {
  if (typeof obj !== "object") {
    throw new Error("Invalid enum object")
  }

  if ("InvalidAuthority" in obj) {
    return new InvalidAuthority()
  }
  if ("InvalidEscrow" in obj) {
    return new InvalidEscrow()
  }
  if ("ArrayOverflow" in obj) {
    return new ArrayOverflow()
  }
  if ("StaleData" in obj) {
    return new StaleData()
  }
  if ("InvalidTrustedSigner" in obj) {
    return new InvalidTrustedSigner()
  }
  if ("InvalidMrEnclave" in obj) {
    return new InvalidMrEnclave()
  }
  if ("InvalidSymbol" in obj) {
    return new InvalidSymbol()
  }
  if ("IncorrectSwitchboardFunction" in obj) {
    return new IncorrectSwitchboardFunction()
  }
  if ("InvalidSwitchboardFunction" in obj) {
    return new InvalidSwitchboardFunction()
  }
  if ("FunctionValidationFailed" in obj) {
    return new FunctionValidationFailed()
  }
  if ("SwitchboardRequestNotSuccessful" in obj) {
    return new SwitchboardRequestNotSuccessful()
  }
  if ("RoundInactive" in obj) {
    return new RoundInactive()
  }
  if ("HouseInsufficientFunds" in obj) {
    return new HouseInsufficientFunds()
  }

  throw new Error("Invalid enum object")
}

export function fromJSON(
  obj: types.RandomnessRequestErrorJSON
): types.RandomnessRequestErrorKind {
  switch (obj.kind) {
    case "InvalidAuthority": {
      return new InvalidAuthority()
    }
    case "InvalidEscrow": {
      return new InvalidEscrow()
    }
    case "ArrayOverflow": {
      return new ArrayOverflow()
    }
    case "StaleData": {
      return new StaleData()
    }
    case "InvalidTrustedSigner": {
      return new InvalidTrustedSigner()
    }
    case "InvalidMrEnclave": {
      return new InvalidMrEnclave()
    }
    case "InvalidSymbol": {
      return new InvalidSymbol()
    }
    case "IncorrectSwitchboardFunction": {
      return new IncorrectSwitchboardFunction()
    }
    case "InvalidSwitchboardFunction": {
      return new InvalidSwitchboardFunction()
    }
    case "FunctionValidationFailed": {
      return new FunctionValidationFailed()
    }
    case "SwitchboardRequestNotSuccessful": {
      return new SwitchboardRequestNotSuccessful()
    }
    case "RoundInactive": {
      return new RoundInactive()
    }
    case "HouseInsufficientFunds": {
      return new HouseInsufficientFunds()
    }
  }
}

export function layout(property?: string) {
  const ret = borsh.rustEnum([
    borsh.struct([], "InvalidAuthority"),
    borsh.struct([], "InvalidEscrow"),
    borsh.struct([], "ArrayOverflow"),
    borsh.struct([], "StaleData"),
    borsh.struct([], "InvalidTrustedSigner"),
    borsh.struct([], "InvalidMrEnclave"),
    borsh.struct([], "InvalidSymbol"),
    borsh.struct([], "IncorrectSwitchboardFunction"),
    borsh.struct([], "InvalidSwitchboardFunction"),
    borsh.struct([], "FunctionValidationFailed"),
    borsh.struct([], "SwitchboardRequestNotSuccessful"),
    borsh.struct([], "RoundInactive"),
    borsh.struct([], "HouseInsufficientFunds"),
  ])
  if (property !== undefined) {
    return ret.replicate(property)
  }
  return ret
}
