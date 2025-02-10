use anchor_lang::prelude::*;
use crate::state::state::State;
use crate::state::user::User;
use crate::globals::{USER_TAG, SUFFIX_TAG};

pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
    let user_account =&mut ctx.accounts.user_account;
    let state = &mut ctx.accounts.state;

    user_account.init(state.get_default_ref_rate(), ctx.program_id)
}

pub fn change_ref_rate(ctx: Context<ChangeRefRate>, new_ref_rate: u64) -> Result<()> {
    let user_account =&mut ctx.accounts.user_account;
    user_account.set_ref_rate(new_ref_rate)
}

pub fn withdraw_ref_rewards(ctx: Context<WithdrawRefRewards>) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    let user = &mut ctx.accounts.user;

    let amount = user_account.get_ref_amount();
    user_account.reset_ref_amount().unwrap();
    user_account.sub_lamports(amount).unwrap();
    user.add_lamports(amount).unwrap();

    Ok(())
}


#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + User::ACCOUNT_SIZE,
        seeds = [USER_TAG, SUFFIX_TAG, user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(referral_address: Pubkey)]
pub struct ChangeRefRate<'info> {
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + User::ACCOUNT_SIZE,
        seeds = [USER_TAG, SUFFIX_TAG, referral_address.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawRefRewards<'info> {
    #[account(mut)]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub user: Signer<'info>,
}