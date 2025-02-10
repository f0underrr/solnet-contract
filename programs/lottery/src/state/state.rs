use anchor_lang::prelude::*;

#[account]
pub struct State {
    pub last_tree_id: u32,
    default_ref_rate: u64
}

impl State {
    pub const ACCOUNT_SIZE: usize = std::mem::size_of::<State>();

    pub fn init(&mut self, default_ref_rate: u64) -> Result<()> {
        self.last_tree_id = 1;
        self.default_ref_rate = default_ref_rate;

        Ok(())
    }

    pub fn set_default_ref_rate(&mut self, default_ref_rate: u64) -> Result<()> {
        self.default_ref_rate = default_ref_rate;

        Ok(())
    }

    pub fn increment_last_tree_id(&mut self) -> Result<()> {
        self.last_tree_id += 1;

        Ok(())
    }

    pub fn get_default_ref_rate(&self) -> u64 {
        self.default_ref_rate
    }
}