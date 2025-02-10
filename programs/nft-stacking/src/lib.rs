use anchor_lang::prelude::*;

declare_id!("7XX5oFNDXNfECEz1AdaqApZKx24rRciaThz8vKxSu3YP");

pub mod constant;
pub mod error;
pub mod instructions;
pub mod state;

use crate::{error::*, instructions::*, state::*};

#[program]
pub mod nft_stacking {
    use super::*;

    pub fn initialize(
        ctx: Context<InitaizeConfig>,
        min_freez_period: u32,
        max_stack: u8,
        reward_per_stack: u8,
    ) -> Result<()> {
        ctx.accounts
            .init_config(min_freez_period, max_stack, reward_per_stack, ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitUser>) -> Result<()> {
        ctx.accounts.save_user_data(ctx.bumps)?;
        Ok(())
    }
}

// - NFT staking is a win-win for both projects and users, Projects benefit from
//    increased engagement, loyalty, and ecosystem growth, while users earn passive income and
//    additional utility from their NFTs.
