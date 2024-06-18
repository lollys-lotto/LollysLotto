import { PublicKey } from "@solana/web3.js";

export type Network = "MAINNET" | "DEVNET";

const USDC_MINT_DEVNET = new PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");
const USDC_MINT_MAINNET = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const LOLLY_MINT = new PublicKey("1o1PXCnsydo5EMihymayXCMweSyd6UAf5SrYkExNzHc");

const EVENT_EMITTER_IDENT = "event-emitter";
const LOLLY_BURN_STATE_IDENT = "lolly-burn-state";
const LOLLYS_LOTTO_IDENT = "lollys-lotto";
const LOTTO_GAME_IDENT = "lotto-game";
const LOTTO_GAME_VAULT_IDENT = "lotto-game-vault";
const LOTTO_TICKET_IDENT = "lotto-ticket";
const USER_METADATA_IDENT = "user-metadata";