use std::str::FromStr;
use anchor_lang::prelude::*;
use crate::globals::{EMPTY_PUBKEY, SUFFIX_TAG, USER_TAG};

#[account]
pub struct User {
    last_bought_tree_index: u32,
    ref_rate: u64,
    ref_amount: u64,
    referral: Pubkey
}

impl User {
    pub const ACCOUNT_SIZE: usize = std::mem::size_of::<User>();

    pub fn init(&mut self, ref_rate: u64, program_id: &Pubkey) -> Result<()> {

        self.last_bought_tree_index = 0;
        self.ref_amount = 0;
        self.ref_rate = ref_rate;
        self.referral = get_user_pda(Pubkey::from_str(EMPTY_PUBKEY).unwrap(), program_id);

        Ok(())
    }

    pub fn set_last_bought_tree_index(&mut self, index: u32) -> Result<()> {
        self.last_bought_tree_index = u32::max(self.last_bought_tree_index, index);

        Ok(())
    }

    pub fn set_ref_rate(&mut self, ref_rate: u64) -> Result<()> {
        self.ref_rate = ref_rate;

        Ok(())
    }

    pub fn set_referral(&mut self, referral: Pubkey) -> Result<()> {
        self.referral = referral;

        Ok(())
    }

    pub fn add_ref_amount(&mut self, amount: u64) -> Result<()> {
        self.ref_amount += amount;

        Ok(())
    }

    pub fn reset_ref_amount(&mut self) -> Result<()> {
        self.ref_amount = 0;

        Ok(())
    }

    pub fn get_last_bought_tree_index(&self) -> u32 {
        self.last_bought_tree_index
    }

    pub fn get_ref_rate(&self) -> u64 {
        self.ref_rate
    }

    pub fn get_ref_amount(&self) -> u64 {
        self.ref_amount
    }

    pub fn get_referral(&self) -> Pubkey {
        self.referral
    }
}

fn get_user_pda(user_key: Pubkey, program_id: &Pubkey) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(
        &[USER_TAG, SUFFIX_TAG, user_key.as_ref()],
        program_id,
    );

    return pda;
}

