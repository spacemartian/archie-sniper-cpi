use anchor_lang::prelude::*;
use anchor_spl::token::{Token};
use anchor_lang::system_program;
use raydium_amm_cpi::{
    self,
    {SwapBaseIn, SwapBaseOut},
    ID as RAYDIUM_AMM_PROGRAM_ID
};

#[derive(Accounts)]
pub struct SendTip<'info> {
    #[account(mut)]
    pub user_wallet: Signer<'info>,
    /// CHECK: Safe because this is the Jito fee account
    #[account(mut)]
    pub jito_tip_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts, Clone)]
pub struct SwapTokens<'info> {
    /// CHECK: Safe. amm Account
    #[account(mut)]
    pub amm: UncheckedAccount<'info>,
    /// CHECK: Safe. Amm authority Account
    #[account(
        seeds = [b"amm authority"],
        bump,
    )]
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Safe. amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_coin_vault Amm Account to swap FROM or To
    #[account(mut)]
    pub amm_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. amm_pc_vault Amm Account to swap FROM or To
    #[account(mut)]
    pub amm_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook program id
    pub market_program: UncheckedAccount<'info>,
    /// CHECK: Safe. OpenBook market Account. OpenBook program is the owner
    #[account(mut)]
    pub market: UncheckedAccount<'info>,
    /// CHECK: Safe. bids Account
    #[account(mut)]
    pub market_bids: UncheckedAccount<'info>,
    /// CHECK: Safe. asks Account
    #[account(mut)]
    pub market_asks: UncheckedAccount<'info>,
    /// CHECK: Safe. event_q Account
    #[account(mut)]
    pub market_event_queue: UncheckedAccount<'info>,
    /// CHECK: Safe. coin_vault Account
    #[account(mut)]
    pub market_coin_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. pc_vault Account
    #[account(mut)]
    pub market_pc_vault: UncheckedAccount<'info>,
    /// CHECK: Safe. vault_signer Account
    #[account(mut)]
    pub market_vault_signer: UncheckedAccount<'info>,
    /// CHECK: Safe. user source token Account. user Account to swap from
    #[account(mut)]
    pub user_source_token: UncheckedAccount<'info>,
    /// CHECK: Safe. user destination token Account. user Account to swap to
    #[account(mut)]
    pub user_destination_token: UncheckedAccount<'info>,
    /// CHECK: Safe. user owner Account
    #[account(mut)]
    pub user_owner: Signer<'info>,
    /// CHECK: Safe. The AMM program
    pub amm_program: Program<'info, crate::RaydiumSwap>,
    /// CHECK: Safe. The spl token program
    pub token_program: Program<'info, Token>,
    /// CHECK: Safe. The system program
    pub system_program: Program<'info, System>,
    /// CHECK: Safe because this is the Jito fee account
    #[account(mut)]
    pub jito_tip_account: UncheckedAccount<'info>,
}

pub fn swap_exact_in(
    ctx: Context<SwapTokens>,
    amount_in: u64,
    minimum_amount_out: u64,
    tip_amount: u64,
) -> Result<()> {
    if tip_amount > 0 {
        let cpi_accounts = system_program::Transfer {
            from: ctx.accounts.user_owner.to_account_info(),
            to: ctx.accounts.jito_tip_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
        );

        system_program::transfer(cpi_ctx, tip_amount)?;
    }

    // Raydium Instruction to Swap (Buy) Tokens
    let cpi_accounts = SwapBaseIn {
        amm: ctx.accounts.amm.clone(),
        amm_authority: ctx.accounts.amm_authority.clone(),
        amm_open_orders: ctx.accounts.amm_open_orders.clone(),
        amm_coin_vault: ctx.accounts.amm_coin_vault.clone(),
        amm_pc_vault: ctx.accounts.amm_pc_vault.clone(),
        market_program: ctx.accounts.market_program.clone(),
        market: ctx.accounts.market.clone(),
        market_bids: ctx.accounts.market_bids.clone(),
        market_asks: ctx.accounts.market_asks.clone(),
        market_event_queue: ctx.accounts.market_event_queue.clone(),
        market_coin_vault: ctx.accounts.market_coin_vault.clone(),
        market_pc_vault: ctx.accounts.market_pc_vault.clone(),
        market_vault_signer: ctx.accounts.market_vault_signer.clone(),
        user_source_owner: ctx.accounts.user_owner.clone(),
        user_token_source: ctx.accounts.user_source_token.clone(),
        user_token_destination: ctx.accounts.user_destination_token.clone(),
        token_program: ctx.accounts.token_program.clone(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.amm_program.to_account_info(),
        cpi_accounts,
    );

    raydium_amm_cpi::instructions::swap_base_in(cpi_ctx, amount_in, minimum_amount_out)
}

pub fn swap_exact_out(
    ctx: Context<SwapTokens>,
    max_amount_in: u64,
    amount_out: u64,
    tip_amount: u64,
) -> Result<()> {
    if tip_amount > 0 {
        let cpi_accounts = system_program::Transfer {
            from: ctx.accounts.user_owner.to_account_info(),
            to: ctx.accounts.jito_tip_account.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
        );

        system_program::transfer(cpi_ctx, tip_amount)?;
    }

    // Raydium Instruction to Swap (Sell) Tokens
    let cpi_accounts = SwapBaseOut {
        amm: ctx.accounts.amm.clone(),
        amm_authority: ctx.accounts.amm_authority.clone(),
        amm_open_orders: ctx.accounts.amm_open_orders.clone(),
        amm_coin_vault: ctx.accounts.amm_coin_vault.clone(),
        amm_pc_vault: ctx.accounts.amm_pc_vault.clone(),
        market_program: ctx.accounts.market_program.clone(),
        market: ctx.accounts.market.clone(),
        market_bids: ctx.accounts.market_bids.clone(),
        market_asks: ctx.accounts.market_asks.clone(),
        market_event_queue: ctx.accounts.market_event_queue.clone(),
        market_coin_vault: ctx.accounts.market_coin_vault.clone(),
        market_pc_vault: ctx.accounts.market_pc_vault.clone(),
        market_vault_signer: ctx.accounts.market_vault_signer.clone(),
        user_source_owner: ctx.accounts.user_owner.clone(),
        user_token_source: ctx.accounts.user_source_token.clone(),
        user_token_destination: ctx.accounts.user_destination_token.clone(),
        token_program: ctx.accounts.token_program.clone(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.amm_program.to_account_info(),
        cpi_accounts,
    );

    raydium_amm_cpi::instructions::swap_base_out(cpi_ctx, max_amount_in, amount_out)
}




// This struct represents the Raydium AMM program
#[derive(Clone)]
pub struct RaydiumSwap;

impl anchor_lang::Id for RaydiumSwap {
    fn id() -> Pubkey {
        RAYDIUM_AMM_PROGRAM_ID
    }
}