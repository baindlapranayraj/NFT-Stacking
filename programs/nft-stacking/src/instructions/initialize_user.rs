use anchor_lang::prelude::*;

use crate::{
    constant::{CONFIG, USER},
    state::{Config, User},
};

// Accounts
// - user
// - user_pda
// - config_pda

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG],
        bump = config_pda.config_bump
    )]
    pub config_pda: Account<'info, Config>,

    #[account(
        init,
        payer = user,
        space = User::INIT_SPACE + 8,
        seeds = [USER,config_pda.key().to_bytes().as_ref(),user.key().to_bytes().as_ref()],
        bump
    )]
    pub user_pda: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitUser<'info> {
    pub fn save_user_data(&mut self, bump: InitUserBumps) -> Result<()> {
        self.user_pda.set_inner(User {
            user_bump: bump.user_pda,
            reward_point: 0,
            no_of_stacked_nft: 0,
        });

        Ok(())
    }
}
