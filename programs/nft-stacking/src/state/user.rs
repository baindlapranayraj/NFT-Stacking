use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user_bump: u8,
    pub reward_point: u32,
    pub no_of_stacked_nft: u8,
}
