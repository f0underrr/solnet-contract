use anchor_lang::prelude::*;
use crate::state::tree::Tree;
use crate::state::user::User;
use crate::state::tree_user::TreeUser;


use crate::globals::{SUFFIX_TAG, USER_TAG, TREE_USER_TAG};


pub fn init_tree_user(ctx: Context<InitTreeUser>) -> Result<()> {
    let tree_user = &mut ctx.accounts.tree_user;
    let tree = &mut ctx.accounts.tree;

    tree_user.init(tree.id)
}


#[derive(Accounts)]
pub struct InitTreeUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + TreeUser::ACCOUNT_SIZE,
        seeds = [TREE_USER_TAG, SUFFIX_TAG, tree.id.to_le_bytes().as_ref(), SUFFIX_TAG, user.key().as_ref()],
        bump,
    )]
    pub tree_user: Account<'info, TreeUser>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + User::ACCOUNT_SIZE,
        seeds = [USER_TAG, SUFFIX_TAG, user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub tree: Account<'info, Tree>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
