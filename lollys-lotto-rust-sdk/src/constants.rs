use anchor_client::anchor_lang::solana_program::pubkey::Pubkey;
use solana_sdk::pubkey;

pub const USDC_MAINNET_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const USDC_DEVNET_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

pub const EVENT_EMITTER_IDENT: &[u8] = b"event-emitter";
pub const LOLLY_BURN_STATE_IDENT: &[u8] = b"lolly-burn-state";
pub const LOLLYS_LOTTO_IDENT: &[u8] = b"lollys-lotto";
pub const LOTTO_GAME_IDENT: &[u8] = b"lotto-game";
pub const LOTTO_GAME_VAULT_IDENT: &[u8] = b"lotto-game-vault";
pub const LOTTO_TICKET_IDENT: &[u8] = b"lotto-ticket";
pub const USER_METADATA_IDENT: &[u8] = b"user-metadata";
