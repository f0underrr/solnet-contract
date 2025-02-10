use anchor_lang::prelude::*;

#[account]
pub struct Tree {
    pub id: u32,
    price: u64,
    pub last_cell_id: u32,
    is_active: bool,
    default_rate: u64,
}

impl Tree {
    pub const ACCOUNT_SIZE: usize = std::mem::size_of::<Tree>();

    pub fn init(&mut self, id: u32, price: u64,  default_rate: u64) -> Result<()> {
        self.id = id;
        self.price = price;
        self.default_rate = default_rate;
        self.is_active = false;
        self.last_cell_id = 2;
        Ok(())
    }

    pub fn set_is_active(&mut self, is_active: bool) -> Result<()> {
        self.is_active = is_active;

        Ok(())
    }

    pub fn increment_last_cell_id(& mut self) -> Result<()> {
        self.last_cell_id += 1;

        Ok(())
    }

    pub fn get_price(&self) -> u64 {
        self.price
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn get_default_rate(&self) -> u64 {
        self.default_rate
    }
}