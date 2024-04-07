use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;

use crate::{constants::USDC_MINT_DEVNET, pda_identifier::PDAIdentifier};

#[account]
#[derive(Debug, Copy)]
#[repr(C)]
pub struct LottoGame {
    /// The authority of this LottoGame instance.
    pub authority: Pubkey,
    /// The round number of this LottoGame instance.
    pub round: u64,
    /// The start date of this LottoGame instance.
    pub start_date: i64,
    /// The end date of this LottoGame instance.
    pub end_date: i64,
    /// The price of a ticket (in USDC) for this round/LottoGame instance.
    pub ticket_price: u64,
    /// The total number of tickets sold for this round/LottoGame instance.
    pub tickets_sold: u64,
    /// The mint used for ticket sales (in USDC).
    pub lotto_game_mint: Pubkey,
    /// The vault where the USDC ticket sales are stored.
    pub lotto_game_vault: Pubkey,
    /// The winning numbers of this round/LottoGame instance.
    pub winning_numbers: [u8; 6],
    /// The bump seed of this round/LottoGame instance.
    pub bump: u8,
    /// The bump seed of this round/LottoGame vault.
    pub lotto_game_vault_bump: u8,
    /// The winning ticket of this round/LottoGame instance.
    pub winning_ticket: Pubkey,
    /// The state of this round/LottoGame instance.
    pub state: LottoGameState,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, AnchorDeserialize, AnchorSerialize)]
pub enum LottoGameState {
    Open,
    Closed,
    Finished,
}

impl PDAIdentifier for LottoGame {
    const IDENT: &'static [u8] = b"lotto-game";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl LottoGame {
    pub fn signer_address(authority: Pubkey, round: u64) -> Pubkey {
        Self::get_address(&[authority.as_ref(), &round.to_le_bytes()])
    }
    pub fn address(authority: Pubkey, round: u64) -> Pubkey {
        Self::get_address(&[authority.as_ref(), &round.to_le_bytes()])
    }

    pub fn address_with_bump(authority: Pubkey, round: u64) -> (Pubkey, u8) {
        Self::get_address_with_bump(&[authority.as_ref(), &round.to_le_bytes()])
    }
}

/// Does not need to be initialized, only to act as a signer.
/// This signer owns the USDC token account where ticket sales in USDC is stored.
#[account]
#[derive(Debug)]
pub struct LottoGameVault {}

impl PDAIdentifier for LottoGameVault {
    const IDENT: &'static [u8] = b"lotto-game-vault";

    fn program_id() -> &'static Pubkey {
        &crate::ID
    }
}

impl LottoGameVault {
    /// This PDA signer's USDC associated token account.
    pub fn vault_address(lotto_game: Pubkey) -> Pubkey {
        get_associated_token_address(&Self::signer_address(lotto_game), &USDC_MINT_DEVNET)
    }

    pub fn signer_address(lotto_game: Pubkey) -> Pubkey {
        Self::get_address(&[lotto_game.as_ref()])
    }
}
