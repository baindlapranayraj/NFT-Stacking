use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token_interface::{mint_to, revoke, Mint, Revoke, TokenAccount, TokenInterface},
};

use crate::{
    constant::{CONFIG, REWARD_MINT, USER, USER_NFT},
    state::{Config, NFTStack, User},
};

// ++++++++ Accounts ++++++++
// - user_acc
// - user_nft_acc
// - mint
// - config_pda
// - user_pda
// - stack_pda
// - reward_mint_acc
// - edition_acc

#[derive(Accounts)]
pub struct Unstack<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub user_nft_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [CONFIG],
        bump = config_account.config_bump
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [USER,config_account.key().to_bytes().as_ref(),user.key().as_ref()],
        bump = user_account.user_bump
    )]
    pub user_account: Account<'info, User>,

    #[account(
        mut,
        seeds = [USER_NFT,user_account.key().to_bytes().as_ref(),user_nft_ata.key().to_bytes().as_ref()],
        bump = stack_account.nft_stack_bump,
        has_one = nft_mint.key(),
        close = user
    )]
    pub stack_account: Account<'info, NFTStack>,

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
    pub reward_ata: InterfaceAccount<'info, TokenAccount>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadta",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key == collection_mint.key(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadta",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstack<'info> {
    pub fn unstack(&mut self) -> Result<()> {
        let program = &self.metadata_program.to_account_info();

        let accounts = ThawDelegatedAccountCpiAccounts {
            delegate: &self.stack_account.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.nft_mint.to_account_info(),
            token_account: &self.user_nft_ata.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        let mint_seed = self.nft_mint.key().to_bytes();
        let user_seeds = self.user_account.key().to_bytes();

        let seeds = &[
            USER_NFT,
            user_seeds.as_ref(),
            mint_seed.as_ref(),
            &[self.stack_account.nft_stack_bump],
        ];

        let signers_seeds = &[&seeds.clone()[..]];

        // Lifting the freez from account
        ThawDelegatedAccountCpi::new(program, accounts).invoke_signed(signers_seeds)?;

        let accounts = Revoke {
            source: self.user_nft_ata.to_account_info(),
            authority: self.user_account.to_account_info(),
        };
        let program = self.token_program.to_account_info();

        let signers_seeds = &[&seeds.clone()[..]];

        // Lifting the Ownership
        let ctx = CpiContext::new_with_signer(program, accounts, signers_seeds);
        revoke(ctx)?;

        self.reward_user()?;
        Ok(())
    }

    fn reward_user(&mut self) -> Result<()> {
        // let amount;
        let clock = Clock::get().unwrap();
        let current_time = clock.unix_timestamp;
        let total_time_stacked = current_time - self.stack_account.stack_time_at;

        // let ctx = CpiContext::new_with_signer(program, accounts, sigener_seeds);

        // mint_to(ctx, amount);
        // Todo:- Complete the reward and Testing

        Ok(())
    }
}

// reward user
// unfreez the NFT
// update pda
