use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use anchor_spl::token::{Token, TokenAccount};

use crate::{
    constants::{LOLLY_MINT, USDC_MINT_DEVNET},
    errors::LollysLottoError,
    pda_identifier::PDAIdentifier,
    state::{
        lolly_burn_state::LollyBurnState, EventEmitter, LollysLottoProgramEventData,
        SwapUsdcLollyEvent,
    },
};

mod jupiter {
    use anchor_lang::declare_id;
    declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
}

#[derive(Clone)]
pub struct Jupiter;
impl anchor_lang::Id for Jupiter {
    fn id() -> Pubkey {
        jupiter::id()
    }
}

#[derive(Accounts)]
pub struct SwapUsdcLolly<'info> {
    /// CHECK: Authority of the LollyBurnState instance
    pub authority: Signer<'info>,
    // lolly_burn_state account is a PDA signer all the swap, burn CPIs. It is the PDA which will receive USDC fees to its USDC ATA
    #[account(
        mut,
        has_one = authority,
        seeds=[
            LollyBurnState::IDENT,
            authority.key().as_ref()
        ],
        bump = lolly_burn_state.bump)]
    pub lolly_burn_state: Box<Account<'info, LollyBurnState>>,
    /// token_in_mint to be swapped using jupiter
    /// Mint address of the USDC token
    // #[account(address = USDC_MINT_DEVNET)]
    // pub usdc_mint: Account<'info, Mint>,

    /// associated_token_account of token_in_mint
    /// USDC token account which is used to swap USDC to LOLLY using jupiter owned by LollyBurnState PDA
    #[account(
        mut,
        associated_token::mint = USDC_MINT_DEVNET,
        associated_token::authority = lolly_burn_state,
    )]
    pub lolly_burn_state_usdc_vault: Box<Account<'info, TokenAccount>>,

    /// token_out_mint to be swapped using jupiter
    /// Mint address of the LOLLY token
    // #[account(address = LOLLY_MINT @LollysLottoError::OnlySwapToLOLLYAllowed)]
    // pub lolly_mint: Account<'info, Mint>,

    /// associated_token_account of token_out_mint
    /// LOLLY token account to store LOLLY swapped from USDC of lolly_burn_state_usdc_vault using jupiter owned by LollyBurnState PDA
    #[account(
        mut,
        associated_token::mint = LOLLY_MINT,
        associated_token::authority = lolly_burn_state,
    )]
    pub lolly_burn_state_lolly_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub event_emitter: Box<Account<'info, EventEmitter>>,

    pub jupiter_program: Program<'info, Jupiter>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn swap_usdc_lolly<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, SwapUsdcLolly<'info>>,
    data: Vec<u8>,
) -> Result<()> {
    msg!("Swap on Jupiter");

    // 3rd account in remaining_accounts is the source token account/USDC account
    let jupiter_source_token_account: Account<TokenAccount> =
        Account::try_from_unchecked(&ctx.remaining_accounts[3])?;

    if jupiter_source_token_account.mint != USDC_MINT_DEVNET {
        return err!(LollysLottoError::OnlySwapFromUSDCAllowed);
    }
    if (jupiter_source_token_account.mint != ctx.accounts.lolly_burn_state_usdc_vault.mint)
        && (jupiter_source_token_account.key() != ctx.accounts.lolly_burn_state_usdc_vault.key())
    {
        return err!(LollysLottoError::JupiterIxSourceTokenAccountMismatch);
    }

    // 6th account in remaining_accounts is the destination token account/LOLLY account
    let jupiter_destination_token_account: Account<TokenAccount> =
        Account::try_from_unchecked(&ctx.remaining_accounts[6])?;

    if jupiter_destination_token_account.mint != LOLLY_MINT {
        return err!(LollysLottoError::OnlySwapToLOLLYAllowed);
    }

    if (jupiter_destination_token_account.mint != ctx.accounts.lolly_burn_state_lolly_vault.mint)
        && (jupiter_destination_token_account.key()
            != ctx.accounts.lolly_burn_state_lolly_vault.key())
    {
        return err!(LollysLottoError::JupiterIxDestinationTokenAccountMismatch);
    }

    if (jupiter_source_token_account.owner != ctx.accounts.lolly_burn_state.key())
        && (jupiter_destination_token_account.owner != ctx.accounts.lolly_burn_state.key())
    {
        return err!(LollysLottoError::TokenAccountAuthorityMismatch);
    }

    let accounts: Vec<AccountMeta> = ctx
        .remaining_accounts
        .iter()
        .map(|acc| AccountMeta {
            pubkey: *acc.key,
            is_signer: if *acc.key == ctx.accounts.lolly_burn_state.key() {
                true
            } else {
                acc.is_signer
            },
            is_writable: acc.is_writable,
        })
        .collect();

    let accounts_infos: Vec<AccountInfo> = ctx
        .remaining_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    // lolly_burn_state PDA is signing this jupiter swap transaction
    let _ = invoke_signed(
        &Instruction {
            program_id: *ctx.accounts.jupiter_program.key,
            accounts,
            data,
        },
        &accounts_infos,
        &[&[
            LollyBurnState::IDENT,
            ctx.accounts.lolly_burn_state.authority.as_ref(),
            &[ctx.accounts.lolly_burn_state.bump],
        ]],
    );

    let block_time = Clock::get()?.unix_timestamp;
    ctx.accounts.event_emitter.emit_new_event(
        Some(block_time),
        LollysLottoProgramEventData::SwapUsdcLolly(SwapUsdcLollyEvent {
            authority: *ctx.accounts.authority.key,
            lolly_burn_state: *ctx.accounts.lolly_burn_state.to_account_info().key,
        }),
    )?;

    Ok(())
}
