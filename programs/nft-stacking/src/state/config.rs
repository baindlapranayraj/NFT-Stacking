use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub config_bump: u8,
    pub reward_bump: u8,       //1)
    pub min_freez_period: u32, // It is in type of unix_timestamp type
    pub max_stack: u8,         // Limited no.of stackes per user at a given time
    pub reward_per_stack: u8,  // ==> Depends on time or freez_period
}

// ++++++++++++++++++ Key learnings ++++++++++++++++++
// 1) Well storing the bump(u8) of reward_mint account is more efficent then storing the Pubkey([u8;32])
// 2) This config state is like brain of this entire program, where u can write your custoum stacking rules.
