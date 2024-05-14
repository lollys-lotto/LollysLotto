use anchor_lang::prelude::Pubkey;
use solana_program::pubkey;

/// USDC Token mint
pub const USDC_MINT_MAINNET: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const USDC_MINT_DEVNET: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

#[cfg(not(feature = "devnet"))]
pub const LOLLY_MINT: Pubkey = pubkey!("1o1ohFR7M25XktNXAsbnDbvserNoFkrFLdA9916EGWw");
#[cfg(feature = "devnet")]
pub const LOLLY_MINT: Pubkey = pubkey!("1o1ohFR7M25XktNXAsbnDbvserNoFkrFLdA9916EGWw");

pub const WRAPPED_SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
