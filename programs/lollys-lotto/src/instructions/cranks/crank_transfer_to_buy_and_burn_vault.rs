pub use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    constants::USDC_MINT_DEVNET,
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{LottoGame, LottoGameVault},
};

#[derive(Accounts)]
#[instruction(round: u64, winning_numbers: [u8; 6])]
pub struct CrankTransferToBuyAndBurnVault<'info> {
    /// CHECK: Authority of the LottoGame instance
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority,
        has_one = lotto_game_vault,
        constraint = lotto_game.load()?.round == round @LollysLottoError::InvalidRound,
        seeds = [
            LottoGame::IDENT,
            authority.key().as_ref(),
            lotto_game.load()?.round.to_le_bytes().as_ref(),
        ],
        bump = lotto_game.load()?.bump,
    )]
    pub lotto_game: AccountLoader<'info, LottoGame>,

    /// CHECK: Just a PDA signer
    #[account(
        seeds = [
            LottoGameVault::IDENT,
            lotto_game.key().as_ref(),
        ],
        bump,
    )]
    pub lotto_game_vault_signer: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = USDC_MINT_DEVNET,
        associated_token::authority = lotto_game_vault_signer,
    )]
    pub lotto_game_vault: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

pub fn crank_transfer_to_buy_and_burn_vault(
    ctx: Context<CrankTransferToBuyAndBurnVault>,
    round: u64,
) -> Result<()> {
    // if winning_numbers is not the same as the lotto_game.winning_numbers, then return

    Ok(())
}
