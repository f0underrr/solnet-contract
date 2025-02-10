use anchor_lang::prelude::*;

#[account]
pub struct TreeUser {
    tree_id: u32,
    last_cell: u32
}

impl TreeUser {
    pub const ACCOUNT_SIZE: usize = std::mem::size_of::<TreeUser>();

    pub fn init(&mut self, tree_id: u32) -> Result<()> {
        self.tree_id = tree_id;
        self.last_cell = 0;

        Ok(())
    }

    pub fn set_last_cell(&mut self, last_cell: u32) -> Result<()> {
        self.last_cell = u32::max(self.last_cell, last_cell);

        Ok(())
    }

    pub fn get_tree_id(&self) -> u32 {
        self.tree_id
    }

    pub fn get_last_cell(&self) -> u32 {
        self.last_cell
    }
}