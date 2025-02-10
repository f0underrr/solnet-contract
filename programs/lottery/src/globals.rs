use anchor_lang::prelude::*;

pub const CELL_TAG: &[u8] = b"CELL";
pub const USER_TAG: &[u8] = b"USER";
pub const TREE_TAG: &[u8] = b"TREE";
pub const SUFFIX_TAG: &[u8] = b"+";
pub const TREE_USER_TAG : &[u8] = b"TREE_USER";

pub const   EMPTY_PUBKEY: &str = "11111111111111111111111111111111";

pub const DEFAULT_TREE_RATE: u64 = 70 * 10u64.pow(7); // 75%, when 100% = 10**9
pub const TREASURY: &str = "AvzTueiWX9GNCShtMgsNPDHSzbVxrnNhTVfKWKt7HBqN";

pub const OWNERS: &[&str] = &["AvzTueiWX9GNCShtMgsNPDHSzbVxrnNhTVfKWKt7HBqN"]; // TODO: change

pub fn is_owner(address: Pubkey) -> bool {
    return OWNERS.contains(&address.to_string().as_str());
}
