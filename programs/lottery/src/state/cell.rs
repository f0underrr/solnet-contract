use anchor_lang::prelude::*;

#[account]
pub struct Cell {
    pub number: u32,
    pub owner: Pubkey,
    amount: u64,
    pub tree: u32,
}

impl Cell {
    pub const ACCOUNT_SIZE: usize = std::mem::size_of::<Cell>();

    pub fn init(&mut self, number: u32, owner: Pubkey, amount: u64, tree: u32) -> Result<()> {
        self.number = number;
        self.owner = owner;
        self.amount = amount;
        self.tree = tree;

        Ok(())
    }

    pub fn set_amount(&mut self, amount: u64) -> Result<()> {
        self.amount = amount;

        Ok(())
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }
}