import type { LollysLotto } from "./target/types/lollys_lotto";

import * as anchor from "@coral-xyz/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { RandomnessService } from "@switchboard-xyz/solana-randomness-service";
import assert from "assert";
import { PublicKey, Connection, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import { loadKeypair } from "@switchboard-xyz/solana.js";

