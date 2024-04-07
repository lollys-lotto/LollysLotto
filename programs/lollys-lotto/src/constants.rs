use anchor_lang::prelude::Pubkey;
use solana_program::pubkey;

/// USDC Token mint
pub const USDC_MINT_MAINNET: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const USDC_MINT_DEVNET: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

#[cfg(not(feature = "devnet"))]
pub const LOLLY_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
#[cfg(feature = "devnet")]
pub const LOLLY_MINT: Pubkey = pubkey!("USDC4bFm2MkhE24BJy8VTT3r2Pr14V6E7fX6fm4w2q9");

pub const WRAPPED_SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

/// Seven days = 60sec * 60min * 24hrs * 7d
pub const SEVEN_DAYS: i64 = 60 * 60 * 24 * 7;
