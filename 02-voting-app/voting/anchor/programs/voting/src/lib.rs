#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(_ctx: Context<InitializePoll>, _poll_id: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
      init,
      payer = signer,
      space = 8 + Pool::INIT_SPACE,
      seeds = [poll_id.to_le_bytes().as_ref()],
      bump,
    )]
    pub poll: Account<'info, Pool>, 
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)] //calculate the space needed for the account
pub struct Pool {
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub cantidate_amount: u64,
}