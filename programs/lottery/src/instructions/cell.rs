
use anchor_lang::{
    prelude::*,
    solana_program::{ program::invoke, system_instruction::transfer },
};

use std::str::FromStr;
use crate::errors;
use crate::events::{BoughtCell, ClaimedCell};
use crate::state::cell::Cell;
use crate::globals::{CELL_TAG, DEFAULT_TREE_RATE, EMPTY_PUBKEY, SUFFIX_TAG, TREASURY, USER_TAG, TREE_USER_TAG};
use crate::state::tree::Tree;
use crate::state::user::User;
use crate::state::tree_user::TreeUser;


pub fn init_cell(ctx: Context<InitCell>, referral_address: Pubkey) -> Result<()> {
    let account_infos = &mut ctx.accounts.to_account_infos();
    let cell = &mut ctx.accounts.cell;
    let tree = &mut ctx.accounts.tree;
    let user_account = &mut ctx.accounts.user_account;
    let referral_user_account = &mut ctx.accounts.referral_user_account;
    let user = &mut ctx.accounts.user;
    let tree_user = &mut ctx.accounts.tree_user;
    let treasury = &mut ctx.accounts.treasury;

    let last_cell_id = tree.last_cell_id;

    if Ok(treasury.key()) != Pubkey::from_str(TREASURY) {
        return err!(errors::ErrorCode::InvalidTreasury);
    }

    if user_account.get_last_bought_tree_index() < tree.id - 1 {
        return err!(errors::ErrorCode::SkippedTree);
    }

    if last_cell_id < (tree_user.get_last_cell() * 2) + 1 {
        return err!(errors::ErrorCode::PreviousCellActive);
    }

    if !tree.is_active() {
        return err!(errors::ErrorCode::TreeNotActive);
    }

    let (ref_address, ref_reward_percent) = get_ref_reward_percent(referral_user_account, tree, user_account, ctx.program_id);

    let tree_reward: u64 = (tree.get_price() * DEFAULT_TREE_RATE) / 10u64.pow(9);
    let ref_reward: u64 = (tree.get_price() * ref_reward_percent) / 10u64.pow(9);
    let protocol_reward: u64 = tree.get_price() - tree_reward - ref_reward;

    if tree_reward > 0 {
        let user_pay_instruction = &transfer(&user.key(), &cell.key(), tree_reward);
        invoke(user_pay_instruction, account_infos).unwrap();

    }

    if ref_reward > 0 {
        let ref_pay_instruction = &transfer(&user.key(), &ref_address, ref_reward);
        invoke(ref_pay_instruction, account_infos).unwrap();

        referral_user_account.add_ref_amount(ref_reward).unwrap();
    }

    if protocol_reward > 0 {
        let treasury_address = Pubkey::from_str(TREASURY).unwrap();
        let protocol_pay_instruction = &transfer(&user.key(), &treasury_address, protocol_reward);
        invoke(protocol_pay_instruction, account_infos).unwrap();
    }

    user_account.set_last_bought_tree_index(tree.id).unwrap();
    tree_user.set_last_cell(last_cell_id).unwrap();

    tree.increment_last_cell_id().unwrap();

    emit!(BoughtCell {
        cell_number: last_cell_id,
        tree_id: tree.id,
        owner: user.key(),
        amount: tree.get_price(),
        stored_reward_amount: tree_reward,
        referral: user_account.get_referral()
    });

    return cell.init(last_cell_id, user.key(), tree_reward, tree.id)
}

pub fn claim_cell(ctx: Context<ClaimCell>) -> Result<()> {
    let cell = &mut ctx.accounts.cell;
    let user = &mut ctx.accounts.user;
    let parent_cell = &mut ctx.accounts.parent_cell;

    let amount = parent_cell.get_amount();

    parent_cell.sub_lamports(amount).unwrap();
    user.add_lamports(amount).unwrap();

    parent_cell.set_amount(0).unwrap();

    emit!(ClaimedCell {
        cell_number: cell.number,
        parent_cell_number: parent_cell.number,
        tree_id: cell.tree,
        owner: user.key(),
        claimed_amount: amount
    });

    Ok(())
}

fn get_ref_reward_percent(referral: &mut Account<User>, tree: &mut Account<Tree>, user_account: &mut Account<User>, program_id: &Pubkey) -> (Pubkey, u64) {
    let empty_ref_pda = get_user_pda(Pubkey::from_str(EMPTY_PUBKEY).unwrap(), program_id);

    if referral.key() == user_account.get_referral() && referral.key() != empty_ref_pda {
        return (user_account.get_referral(), u64::max(referral.get_ref_rate(), tree.get_default_rate()));
    }

    if user_account.get_referral() == empty_ref_pda && referral.key() != empty_ref_pda {
        user_account.set_referral(referral.key()).unwrap();
        return (referral.key(), u64::max(referral.get_ref_rate(), tree.get_default_rate()));
    }

    return (empty_ref_pda, 0);
}


fn get_user_pda(user_key: Pubkey, program_id: &Pubkey) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(
        &[USER_TAG, SUFFIX_TAG, user_key.as_ref()],
        program_id,
    );

    return pda;
}

#[derive(Accounts)]
#[instruction(referral_address: Pubkey)]
pub struct InitCell<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Cell::ACCOUNT_SIZE,
        seeds = [CELL_TAG, SUFFIX_TAG, tree.last_cell_id.to_le_bytes().as_ref(), SUFFIX_TAG, tree.id.to_le_bytes().as_ref()],
        bump,
    )]
    pub cell: Account<'info, Cell>,

    #[account(mut)]
    pub tree: Account<'info, Tree>,
    #[account(mut)]
    pub user_account: Account<'info, User>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + User::ACCOUNT_SIZE,
        seeds = [USER_TAG, SUFFIX_TAG, referral_address.key().as_ref()],
        bump,
    )]
    pub referral_user_account: Account<'info, User>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + TreeUser::ACCOUNT_SIZE,
        seeds = [TREE_USER_TAG, SUFFIX_TAG, tree.id.to_le_bytes().as_ref(), SUFFIX_TAG, user.key().as_ref()],
        bump,
    )]
    pub tree_user: Account<'info, TreeUser>,

    #[account(mut)]
    /// CHECK: assertion in function
    pub treasury: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct ClaimCell<'info> {
    #[account(mut,
        constraint = cell.owner == user.key(),
        constraint = cell.tree == parent_cell.tree
    )]
    pub cell: Account<'info, Cell>,

    #[account(mut,
        constraint = parent_cell.number / 2u32 == cell.number,
    )]
    pub parent_cell: Account<'info, Cell>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}