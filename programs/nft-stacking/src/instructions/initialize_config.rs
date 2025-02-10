use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{
    constant::{CONFIG, REWARD_MINT},
    state::Config,
};

// (We creating stacking platform)Accounts
// - admin
// - reward_mint_account
// - config_pda
// -
//

#[derive(Accounts)]
pub struct InitaizeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Config::INIT_SPACE + 8,
        seeds = [CONFIG],
        bump
    )]
    pub config_pda: Account<'info, Config>,

    #[account(
        init,
        payer = admin,
        mint::authority = config_pda,
        mint::decimals = 6,
        seeds = [REWARD_MINT,config_pda.key().as_ref()],
        bump
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitaizeConfig<'info> {
    pub fn init_config(
        &mut self,
        min_freez_period: u32,
        max_stack: u8,
        reward_per_stack: u8,
        bump: InitaizeConfigBumps,
    ) -> Result<()> {
        self.config_pda.set_inner(Config {
            config_bump: bump.config_pda,
            reward_bump: bump.reward_mint,
            min_freez_period,
            max_stack,
            reward_per_stack,
        });
        Ok(())
    }
}
