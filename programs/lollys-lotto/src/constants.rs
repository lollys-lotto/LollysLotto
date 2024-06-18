use anchor_lang::prelude::Pubkey;
use solana_program::pubkey;

/// USDC Token mint
pub const USDC_MINT_MAINNET: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const USDC_MINT_DEVNET: Pubkey = pubkey!("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");

#[cfg(not(feature = "devnet"))]
pub const LOLLY_MINT: Pubkey = pubkey!("1o1PXCnsydo5EMihymayXCMweSyd6UAf5SrYkExNzHc");
#[cfg(feature = "devnet")]
pub const LOLLY_MINT: Pubkey = pubkey!("1o1PXCnsydo5EMihymayXCMweSyd6UAf5SrYkExNzHc");

pub const WRAPPED_SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
