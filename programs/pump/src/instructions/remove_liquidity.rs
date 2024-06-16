use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{LiquidityPool, LiquidityPoolAccount, LiquidityProvider};
use raydium_contract_instructions::amm_instruction;

pub fn remove_liquidity(
    ctx: Context<RemoveLiquidity>,
    nonce: u8,
    init_pc_amount: u64,
) -> Result<()> {
    
    // If you want to Interact with CPI, then plz contact to me.

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(
        mut,
        seeds = [LiquidityPool::POOL_SEED_PREFIX.as_bytes(), coin_mint.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,

    /// CHECK
    #[account(
        mut,
        seeds = [b"global"],
        bump,
    )]
    pub global_account: AccountInfo<'info>,
    pub coin_mint: Account<'info, Mint>,
    pub pool_account: Account<'info, TokenAccount>,
    pub user_account: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
    // CHECK: Safe - CPI accounts
    
}

