use anchor_lang::prelude::*;
use anchor_spl::token::{Token};
use anchor_spl::associated_token::{self, AssociatedToken};


    
// Pump.fun Program
pub static ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
pub static GLOBAL: Pubkey = pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
pub static EVENT_AUTHORITY: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");
pub static FEE: Pubkey = pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM");

// Constants
pub const BUY_DISCRIMINATOR: u64 = 16927863322537952870;
pub const BONDING_CURVE_DISCRIMINATOR: u64 = 6966180631402821399;
pub const TOKEN_DECIMALS: u8 = 6;
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

#[derive(Debug)]
pub struct BondingCurveState {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
}

impl BondingCurveState {
    pub fn try_deserialize(data: &[u8]) -> Result<Self> {
        if data.len() < 8 || u64::from_le_bytes(data[..8].try_into().unwrap()) != BONDING_CURVE_DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData.into());
        }

        let data = &data[8..];
        Ok(Self {
            virtual_token_reserves: u64::from_le_bytes(data[..8].try_into().unwrap()),
            virtual_sol_reserves: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            real_token_reserves: u64::from_le_bytes(data[16..24].try_into().unwrap()),
            real_sol_reserves: u64::from_le_bytes(data[24..32].try_into().unwrap()),
            token_total_supply: u64::from_le_bytes(data[32..40].try_into().unwrap()),
            complete: data[40] != 0,
        })
    }

    pub fn calculate_price(&self) -> Result<f64> {
        if self.virtual_token_reserves == 0 || self.virtual_sol_reserves == 0 {
            return Err(ProgramError::InvalidAccountData.into());
        }

        Ok((self.virtual_sol_reserves as f64 / LAMPORTS_PER_SOL as f64) / 
            (self.virtual_token_reserves as f64 / 10f64.powi(TOKEN_DECIMALS as i32)))
    }
}


#[derive(Accounts)]
pub struct BuyPumpToken<'info> {
    /// Pump global state account
    /// CHECK: This is a global state account
    #[account(constraint = pump_global.key() == GLOBAL)]
    pub pump_global: AccountInfo<'info>,

    /// Pump fee account
    /// CHECK: This is a known static account
    #[account(
        mut,
        constraint = pump_fee.key() == FEE
    )]
    pub pump_fee: AccountInfo<'info>,

    /// Token mint account
    /// CHECK: This is a Token Program owned Mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// Bonding curve state account
    /// CHECK: This account is deserialized and validated inside the instruction logic.
    #[account(mut)]
    pub bonding_curve: AccountInfo<'info>,

    /// Associated bonding curve token account
    /// CHECK: This account is validated through program-specific logic to ensure correctness.
    #[account(mut)]
    pub associated_bonding_curve: AccountInfo<'info>,

    /// User's token account
    /// CHECK: This is a Token Program owned TokenAccount
    #[account(
        mut,
        constraint = token_account.owner == &anchor_spl::token::ID
    )]
    pub token_account: AccountInfo<'info>,

    /// User's wallet
    #[account(mut)]
    pub payer: Signer<'info>,

    /// System Program
    pub system_program: Program<'info, System>,

    /// Token Program
    pub token_program: Program<'info, Token>,

    /// Associated Token Program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Rent Sysvar
    pub rent: Sysvar<'info, Rent>,

    /// Pump event authority
    /// CHECK: This is a known static account
    #[account(constraint = pump_event_authority.key() == EVENT_AUTHORITY)]
    pub pump_event_authority: AccountInfo<'info>,

    /// Pump Program
    pub pump_program: Program<'info, PumpProgram>,
}

// Program wrapper for type safety
#[derive(Clone)]
pub struct PumpProgram;

impl anchor_lang::Id for PumpProgram {
    fn id() -> Pubkey {
        ID
    }
}

pub fn buy_pump_tokens(
    ctx: Context<BuyPumpToken>,
    amount_sol: f64,
    slippage: f64,
) -> Result<()> {
    // Create ATA if it doesn't exist
    // Check if token account needs to be created by checking its data length
    if ctx.accounts.token_account.try_data_len()? == 0 {
        let cpi_accounts = associated_token::Create {
            payer: ctx.accounts.payer.to_account_info(),
            associated_token: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
    
        let cpi_context = CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(), 
            cpi_accounts
        );
    
        associated_token::create(cpi_context)?;
    }
    
    

    // Calculate amounts
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;
    let curve_state = BondingCurveState::try_deserialize(
        &ctx.accounts.bonding_curve.try_borrow_data()?
    )?;
    let token_price = curve_state.calculate_price()?;
    let token_amount = ((amount_sol / token_price) * 10f64.powi(TOKEN_DECIMALS as i32)) as u64;
    let max_amount_lamports = (amount_lamports as f64 * (1.0 + slippage)) as u64;

    // Create the buy instruction
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new_readonly(ctx.accounts.pump_global.key(), false),
            AccountMeta::new(ctx.accounts.pump_fee.key(), false),
            AccountMeta::new_readonly(ctx.accounts.mint.key(), false),
            AccountMeta::new(ctx.accounts.bonding_curve.key(), false),
            AccountMeta::new(ctx.accounts.associated_bonding_curve.key(), false),
            AccountMeta::new(ctx.accounts.token_account.key(), false),
            AccountMeta::new(ctx.accounts.payer.key(), true),
            AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
            AccountMeta::new_readonly(ctx.accounts.pump_event_authority.key(), false),
            AccountMeta::new_readonly(ID, false),
        ],
        data: {
            let mut data = Vec::with_capacity(24);
            data.extend_from_slice(&BUY_DISCRIMINATOR.to_le_bytes());
            data.extend_from_slice(&token_amount.to_le_bytes());
            data.extend_from_slice(&max_amount_lamports.to_le_bytes());
            data
        },
    };

    // Execute CPI
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.pump_global.to_account_info(),
            ctx.accounts.pump_fee.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.bonding_curve.to_account_info(),
            ctx.accounts.associated_bonding_curve.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.pump_event_authority.to_account_info(),
            ctx.accounts.pump_program.to_account_info(),
        ],
    )?;

    Ok(())
}