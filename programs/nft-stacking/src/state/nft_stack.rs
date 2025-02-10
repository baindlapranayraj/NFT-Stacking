use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct NFTStack {
    pub nft_mint: Pubkey,
    pub owner: Pubkey,
    pub stack_time_at: i64, // why this is in i64
    pub nft_stack_bump: u8,
}
