use anchor_lang::prelude::*;

#[event]
pub struct BoughtCell {
    pub cell_number: u32,
    pub tree_id: u32,
    pub owner: Pubkey,
    pub amount: u64,
    pub stored_reward_amount: u64,
    pub referral: Pubkey,
}

#[event]
pub struct ClaimedCell {
    pub cell_number: u32,
    pub parent_cell_number: u32,
    pub tree_id: u32,
    pub owner: Pubkey,
    pub claimed_amount: u64,
}
