use anchor_lang::prelude::*;
use crate::state::state::State;

pub fn init_state(ctx: Context<InitState>, default_rate: u64) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.init(default_rate)
}


#[derive(Accounts)]
#[instruction(default_rate: u64)]
pub struct InitState<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + State::ACCOUNT_SIZE,
        seeds = [],
        bump,
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
