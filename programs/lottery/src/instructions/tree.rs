use anchor_lang::prelude::*;
use crate::state::tree::Tree;
use crate::state::state::State;
use crate::globals::{TREE_TAG, SUFFIX_TAG, CELL_TAG};
use crate::state::cell::Cell;

pub fn init_tree(ctx: Context<InitTree>, price: u64) -> Result<()> {
    let tree = &mut ctx.accounts.tree;
    let state = &mut ctx.accounts.state;
    let cell = &mut ctx.accounts.cell;
    let user = &mut ctx.accounts.user;

    let last_tree_id = state.last_tree_id;
    cell.init(1, user.key(), 0, last_tree_id).unwrap();
    state.increment_last_tree_id().unwrap();
    tree.init(last_tree_id, price, state.get_default_ref_rate())
}

pub fn open_tree(ctx: Context<OpenTree>) -> Result<()> {
    let tree = &mut ctx.accounts.tree;
    tree.set_is_active(true)
}

#[derive(Accounts)]
pub struct InitTree<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Tree::ACCOUNT_SIZE,
        seeds = [TREE_TAG, SUFFIX_TAG, state.last_tree_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub tree: Account<'info, Tree>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Cell::ACCOUNT_SIZE,
        seeds = [CELL_TAG, SUFFIX_TAG, 1i32.to_le_bytes().as_ref(), SUFFIX_TAG, state.last_tree_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub cell: Account<'info, Cell>,

    #[account(mut)]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenTree<'info> {
    #[account(mut)]
    pub tree: Account<'info, Tree>,
    #[account(mut)]
    pub payer: Signer<'info>,
}