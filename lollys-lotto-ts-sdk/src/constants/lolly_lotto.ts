import { PublicKey } from "@solana/web3.js";

export type Network = "MAINNET" | "DEVNET";

const USDC_MINT_DEVNET = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
const USDC_MINT_MAINNET = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

const EVENT_EMITTER_IDENT = "event-emitter";
const LOLLY_BURN_STATE_IDENT = "lolly_burn_state";
const LOLLYS_LOTTO_IDENT = "lollys_lotto";
const LOTTO_GAME_IDENT = "lotto_game";
const LOTTO_GAME_VAULT_IDENT = "lotto_game_vault";
const LOTTO_TICKET_IDENT = "lotto_ticket";
const USER_METADATA_IDENT = "user_metadata";