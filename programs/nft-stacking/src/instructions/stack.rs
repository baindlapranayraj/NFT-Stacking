use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token_interface::{approve, Approve, Mint, TokenAccount, TokenInterface},
};

use crate::{
    constant::{CONFIG, USER, USER_NFT},
    error::StackError,
    state::{Config, NFTStack, User},
};

// ++++ Accounts +++++
// - user <>
// - user_pda <>
// - config_pda <>
// - user_nft_account <>
// - nft_state_pda <>
// - user_nft_mint <>
//
// If u r stacking NFT then u should provide credentails of your NFT(So that the nft is not fake)

#[derive(Accounts)]
pub struct Stack<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub user_nft_mint: InterfaceAccount<'info, Mint>,
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadta",
            metadata_program.key().as_ref(),
            user_nft_mint.key().as_ref()
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
            user_nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub edition: Account<'info, MasterEditionAccount>, // This for to make sure it is NFT and not an spl-token

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
        associated_token::mint = user_nft_mint,
        associated_token::authority = user
    )]
    pub user_nft: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        space = NFTStack::INIT_SPACE,
        seeds = [USER_NFT,user_account.key().to_bytes().as_ref(),user_nft_mint.key().to_bytes().as_ref()],
        bump
    )]
    pub stack_account: Account<'info, NFTStack>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stack<'info> {
    pub fn stack(&mut self, bump: StackBumps) -> Result<()> {
        require!(
            self.user_account.no_of_stacked_nft < self.config_account.max_stack,
            StackError::MaxStackReach
        );

        let cpi_program = self.token_program.to_account_info();

        // Giving the ownership aproval to this program/stack_account pda
        let accounts = Approve {
            to: self.user_nft.to_account_info(),
            delegate: self.stack_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi = CpiContext::new(cpi_program, accounts);
        approve(cpi, 1)?;

        // - We dont have to transfer the NFT to a vault account, we can just freez it to a certain period so the
        //   user dont need to loose his ownership and keep the NFT Locked.

        let freeze_cpi = &self.metadata_program.to_account_info();
        let freeze_account = FreezeDelegatedAccountCpiAccounts {
            delegate: &self.stack_account.to_account_info(),
            token_account: &self.user_nft.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.user_nft_mint.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };

        let mint_seed = self.user_nft_mint.key().to_bytes();
        let user_seeds = self.user_account.key().to_bytes();

        let seeds = &[
            USER_NFT,
            user_seeds.as_ref(),
            mint_seed.as_ref(),
            &[self.stack_account.nft_stack_bump],
        ];

        let signers_seeds = &[&seeds[..]];

        // Freezing the NFT,without removing user ownership
        FreezeDelegatedAccountCpi::new(freeze_cpi, freeze_account).invoke_signed(signers_seeds)?;

        // Saving/updating the PDA data
        let clock = Clock::get().unwrap().unix_timestamp;
        self.stack_account.set_inner(NFTStack {
            nft_mint: self.user_nft_mint.key(),
            owner: self.user.key(),
            stack_time_at: clock,
            nft_stack_bump: bump.stack_account,
        });
        self.user_account.no_of_stacked_nft = self.user_account.no_of_stacked_nft + 1;

        Ok(())
    }
}

// +++++++++++++ Learnings +++++++++++++++++
// - The Approve instruction in the Anchor program is used to grant a delegate (e.g., your program or a specific account)
//    the authority to manage or transfer a user's NFT.
// - This is a secure and standard way to handle delegation in Solana programs,ensuring that the user retains ownership
//   while allowing the program to perform necessary actions like stacking the NFT.
//
