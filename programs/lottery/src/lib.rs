use instructions::*;
use anchor_lang::prelude::*;

pub mod globals;
pub mod errors;

pub mod state;
pub mod instructions;
mod events;

declare_id!("BAHGeGi5ZNZxULbTXyB8aNmHVNE7muCsFBjYRWisszB4");

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "SOLNET",
    project_url: "https://solnet.digital",
    contacts: "mailto:security@solnet.digital",
    policy: "https://solnet.digital/privacy-policy",
    preferred_languages: "en"
}

#[program]
pub mod lottery {
    use super::*;


    /// STATE
    pub fn initialize(ctx: Context<InitState>, default_rate: u64) -> Result<()> {
        if !globals::OWNERS.contains(&ctx.accounts.payer.key().to_string().as_str()) {
            return err!(errors::ErrorCode::NotOwner);
        }

        instructions::state::init_state(ctx, default_rate)
    }

    /// TREE
    pub fn init_tree(ctx: Context<InitTree>, price: u64) -> Result<()> {
        if !globals::OWNERS.contains(&ctx.accounts.user.key().to_string().as_str()) {
            return err!(errors::ErrorCode::NotOwner);
        }

        instructions::tree::init_tree(ctx, price)
    }

    pub fn open_tree(ctx: Context<OpenTree>) -> Result<()> {
        if !globals::OWNERS.contains(&ctx.accounts.payer.key().to_string().as_str()) {
            return err!(errors::ErrorCode::NotOwner);
        }

        instructions::tree::open_tree(ctx)
    }

    /// USER
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        instructions::user::init_user(ctx)
    }


    pub fn change_ref_rate(ctx: Context<ChangeRefRate>, referral_address:Pubkey, new_ref_rate: u64) -> Result<()> {
        if !globals::OWNERS.contains(&ctx.accounts.user.key().to_string().as_str()) {
            return err!(errors::ErrorCode::NotOwner);
        }

        instructions::user::change_ref_rate(ctx, new_ref_rate)
    }

    pub fn withdraw_ref_rewards(ctx: Context<WithdrawRefRewards>) -> Result<()> {
        instructions::user::withdraw_ref_rewards(ctx)
    }

    /// TREE_USER
    pub fn init_tree_user(ctx: Context<InitTreeUser>) -> Result<()> {
        instructions::tree_user::init_tree_user(ctx)
    }

    /// CELL
    pub fn init_cell(ctx: Context<InitCell>, referral_address: Pubkey) -> Result<()> {
        instructions::cell::init_cell(ctx, referral_address)
    }

    pub fn claim_cell(ctx: Context<ClaimCell>) -> Result<()> {
        instructions::cell::claim_cell(ctx)
    }

}

