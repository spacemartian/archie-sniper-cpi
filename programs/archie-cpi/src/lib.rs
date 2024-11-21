use anchor_lang::prelude::*;

pub mod instructions;
pub mod constants;

use crate::instructions::*;

declare_id!("81JFbU5xub73UmkJFgUG2KTDLvtx9o7Kq8iK3hJFnEn9");

#[program]
pub mod pump {
    use super::*;

    pub fn raydium_swap_token_in(ctx: Context<SwapTokens>, amount_in: u64, minimum_amount_out: u64, jito_tip_sol: f64) -> Result<()> {
        instructions::swap_exact_in(ctx, amount_in, minimum_amount_out, jito_tip_sol)
    }

    pub fn raydium_swap_token_out(ctx: Context<SwapTokens>, max_amount_in: u64, amount_out: u64, jito_tip_sol: f64) -> Result<()> {
        instructions::swap_exact_out(ctx, max_amount_in, amount_out, jito_tip_sol)
    }

    pub fn pump_fun_buy_token(ctx: Context<BuyPumpToken>, amount_sol: f64, slippage: f64, jito_tip_sol: f64) -> Result<()> {
        instructions::buy_pump_tokens(ctx, amount_sol, slippage, jito_tip_sol)
    }

    pub fn pump_fun_sell_token(ctx: Context<SellPumpToken>, token_amount: u64, slippage: f64, jito_tip_sol: f64) -> Result<()> {
        instructions::sell_pump_tokens(ctx, token_amount, slippage, jito_tip_sol)
    }
}
