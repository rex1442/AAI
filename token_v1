use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, MintTo};

declare_id!("YourProgramID");

#[program]
pub mod weekly_and_annual_decrease_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_supply: u64) -> ProgramResult {
        let mint = &mut ctx.accounts.mint;
        let token_account = &mut ctx.accounts.token_account;
        let clock = Clock::get()?;

        // Initialize the mint and token account
        mint.mint_authority = COption::Some(ctx.accounts.authority.key());
        token::mint_to(ctx.accounts.into_mint_to_context(), initial_supply)?;

        // Save the initial timestamp and initial rate
        let state = &mut ctx.accounts.state;
        state.last_mint_timestamp = clock.unix_timestamp;
        state.total_supply = initial_supply;
        state.increase_rate = 0.000192; // Weekly increase rate (0.0192%)
        state.start_year = clock.unix_timestamp / (365 * 24 * 60 * 60); // Start year

        Ok(())
    }

    pub fn weekly_increase(ctx: Context<WeeklyIncrease>) -> ProgramResult {
        let mint = &mut ctx.accounts.mint;
        let clock = Clock::get()?;
        let state = &mut ctx.accounts.state;

        let one_week: i64 = 7 * 24 * 60 * 60; // One week in seconds
        let time_since_last_mint = clock.unix_timestamp - state.last_mint_timestamp;
        let current_year = clock.unix_timestamp / (365 * 24 * 60 * 60); // Current year

        if time_since_last_mint >= one_week {
            // Calculate the new tokens to mint
            let new_tokens = (state.total_supply as f64 * state.increase_rate) as u64;
            token::mint_to(ctx.accounts.into_mint_to_context(), new_tokens)?;

            // Update the total supply and the last mint timestamp
            state.total_supply += new_tokens;
            state.last_mint_timestamp = clock.unix_timestamp;
        }

        // If a year has passed, decrease the rate by 10%
        if current_year > state.start_year {
            state.start_year = current_year;
            state.increase_rate *= 0.9; // Decrease the increase rate by 10%
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = authority, space = 8 + 8 + 8 + 8)]
    pub state: Account<'info, State>,
    #[account(init, payer = authority)]
    pub token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WeeklyIncrease<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}

#[account]
pub struct State {
    pub last_mint_timestamp: i64,
    pub total_supply: u64,
    pub increase_rate: f64,
    pub start_year: i64,
}

impl<'info> Initialize<'info> {
    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.token_account.to_account_info().clone(),
            authority: self.authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}

impl<'info> WeeklyIncrease<'info> {
    fn into_mint_to_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.state.to_account_info().clone(),
            authority: self.authority.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }
}
