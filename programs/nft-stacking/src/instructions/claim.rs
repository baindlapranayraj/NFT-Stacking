use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use crate::{
    constant::{CONFIG, REWARD_MINT, USER},
    state::{Config, User},
};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [CONFIG],
        bump = config_account.config_bump
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [USER,config_account.key().to_bytes().as_ref(),user.key().as_ref()],
        bump = user_account.user_bump,
    )]
    pub user_account: Account<'info, User>,

    #[account(
        mut,
        mint::authority = config_account,
        seeds = [REWARD_MINT,config_account.key().as_ref()],
        bump = config_account.reward_bump
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub user_reward_ata: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Claim<'info> {
    pub fn claim_reward(&mut self) -> Result<()> {
        let program = self.token_program.to_account_info();
        let accounts = MintTo {
            authority: self.config_account.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            mint: self.reward_mint.to_account_info(),
        };

        let seeds = &[CONFIG, &[self.config_account.config_bump]];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);
        let amount =
            (self.user_account.reward_point as u64) * 10_u64.pow(self.reward_mint.decimals as u32);
        mint_to(ctx, amount)?;

        Ok(())
    }
}
