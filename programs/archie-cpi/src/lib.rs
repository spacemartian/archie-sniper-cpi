use anchor_lang::prelude::*;

pub mod instructions;

use crate::instructions::*;

declare_id!("81JFbU5xub73UmkJFgUG2KTDLvtx9o7Kq8iK3hJFnEn9");

#[program]
pub mod pump {
    use super::*;

    pub fn raydium_swap_token_in(ctx: Context<SwapTokens>, amount_in: u64, minimum_amount_out: u64, tip_amount: u64) -> Result<()> {
        instructions::swap_exact_in(ctx, amount_in, minimum_amount_out, tip_amount)
    }

    pub fn raydium_swap_token_out(ctx: Context<SwapTokens>, max_amount_in: u64, amount_out: u64, tip_amount: u64) -> Result<()> {
        instructions::swap_exact_out(ctx, max_amount_in, amount_out, tip_amount)
    }
}
