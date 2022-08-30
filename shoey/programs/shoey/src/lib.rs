use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::{associated_token, token};
use mpl_token_metadata::instruction::{
    create_master_edition_v3, create_metadata_accounts_v2,
    mint_new_edition_from_master_edition_via_token,
};
use mpl_token_metadata::state::{DataV2, UseMethod, Uses, EDITION_MARKER_BIT_SIZE};

declare_id!("EQTKRBAiJp6sRN9BbDcdb1ppwAZnQeBJ2h8uH9XuUwKg");

#[program]
pub mod shoey {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // create vote mint + metaplex accounts

        // create fungible metadata account
        let data = DataV2 {
            name: "Shoey Vote".to_string(),
            symbol: "VOTE".to_string(),
            uri: "https://foo.com/bar.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let create_metadata_accounts = [
            ctx.accounts.vote_metadata.to_account_info(),
            ctx.accounts.vote_mint.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let metadata_ix = create_metadata_accounts_v2(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.vote_metadata.key(),
            ctx.accounts.vote_mint.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.admin.key(),
            ctx.accounts.manager.key(),
            data.name,
            data.symbol,
            data.uri,
            data.creators,
            data.seller_fee_basis_points,
            false,
            false,
            data.collection,
            data.uses,
        );

        invoke_signed(
            &metadata_ix,
            &create_metadata_accounts,
            &[&[
                ctx.accounts.vote_mint.key().as_ref(),
                &[*ctx.bumps.get("manager").unwrap()],
            ]],
        )?;

        // mint master edition to manager controlled token account
        let shoey_mint_to_accounts = token::MintTo {
            mint: ctx.accounts.shoey_mint.to_account_info(),
            to: ctx
                .accounts
                .shoey_master_edition_vault
                .to_account_info(),
            authority: ctx.accounts.manager.to_account_info(),
        };

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                shoey_mint_to_accounts,
                &[&[
                    ctx.accounts.vote_mint.key().as_ref(),
                    &[*ctx.bumps.get("manager").unwrap()],
                ]],
            ),
            1,
        )?;

        // create master edition metadata
        let data = DataV2 {
            name: "Shoey Owner".to_string(),
            symbol: "OWNER".to_string(),
            uri: "https://foo.com/baz.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let create_metadata_accounts = [
            ctx.accounts.shoey_metadata.to_account_info(),
            ctx.accounts.shoey_mint.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let metadata_ix = create_metadata_accounts_v2(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.shoey_metadata.key(),
            ctx.accounts.shoey_mint.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.admin.key(),
            ctx.accounts.manager.key(),
            data.name,
            data.symbol,
            data.uri,
            data.creators,
            data.seller_fee_basis_points,
            false,
            false,
            data.collection,
            data.uses,
        );

        invoke_signed(
            &metadata_ix,
            &create_metadata_accounts,
            &[&[
                ctx.accounts.vote_mint.key().as_ref(),
                &[*ctx.bumps.get("manager").unwrap()],
            ]],
        )?;

        let create_shoey_master_edition_accounts = [
            ctx.accounts.shoey_master_edition.to_account_info(),
            ctx.accounts.shoey_metadata.to_account_info(),
            ctx.accounts.shoey_mint.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ];

        // create master edition account
        // None == unlimited amount of copies can be created
        let shoey_master_edition_ix = create_master_edition_v3(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.shoey_master_edition.key(),
            ctx.accounts.shoey_mint.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.shoey_metadata.key(),
            ctx.accounts.admin.key(),
            None,
        );

        invoke_signed(
            &shoey_master_edition_ix,
            &create_shoey_master_edition_accounts,
            &[&[
                ctx.accounts.vote_mint.key().as_ref(),
                &[*ctx.bumps.get("manager").unwrap()],
            ]],
        )?;

        Ok(())
    }

    pub fn submit(ctx: Context<Submit>, shoey_name: String, edition_number: u64) -> Result<()> {
        // create storage account
        // create shoey account
        // upload video
        // mint shoey nft

        // mint edition to user token account
        let shoey_mint_to_accounts = token::MintTo {
            mint: ctx.accounts.shoey_edition_mint.to_account_info(),
            to: ctx.accounts.user_shoey_edition_ata.to_account_info(),
            authority: ctx.accounts.manager.to_account_info(),
        };

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                shoey_mint_to_accounts,
                &[&[
                    ctx.accounts.vote_mint.key().as_ref(),
                    &[*ctx.bumps.get("manager").unwrap()],
                ]],
            ),
            1,
        )?;

        // create edition account
        let new_shoey_edition_accounts = [
            ctx.accounts.shoey_edition_metadata.to_account_info(),
            ctx.accounts.shoey_edition.to_account_info(),
            ctx.accounts.shoey_master_edition.to_account_info(),
            ctx.accounts.shoey_edition_mint.to_account_info(),
            ctx.accounts.shoey_edition_marker.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts
                .shoey_master_edition_vault
                .to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts
                .shoey_master_edition_metadata
                .to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let new_shoey_edition_ix = mint_new_edition_from_master_edition_via_token(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.shoey_edition_metadata.key(),
            ctx.accounts.shoey_edition.key(),
            ctx.accounts.shoey_master_edition.key(),
            ctx.accounts.shoey_edition_mint.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.user.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.shoey_master_edition_vault.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.shoey_master_edition_metadata.key(),
            ctx.accounts.shoey_master_edition_mint.key(),
            edition_number,
        );

        invoke_signed(
            &new_shoey_edition_ix,
            &new_shoey_edition_accounts,
            &[&[
                ctx.accounts.vote_mint.key().as_ref(),
                &[*ctx.bumps.get("manager").unwrap()],
            ]],
        )?;

        // mint free votes
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, shoey_name: String) -> Result<()> {
        // accept votes
        // if user has no votes, deposit dust for vote token and pay
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, mint::decimals = 0, mint::authority = manager)]
    pub vote_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: initialized by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), vote_mint.key().as_ref()], bump, seeds::program = mpl_token_metadata::ID)]
    pub vote_metadata: UncheckedAccount<'info>,

    #[account(init, payer = admin, mint::decimals = 0, mint::authority = manager)]
    pub shoey_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: initialized by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_mint.key().as_ref()], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_metadata: UncheckedAccount<'info>,

    /// CHECK: initialized by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_mint.key().as_ref(), b"edition"], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_master_edition: UncheckedAccount<'info>,

    #[account(init, payer = admin, associated_token::mint = shoey_mint, associated_token::authority = manager)]
    pub shoey_master_edition_vault: Account<'info, token::TokenAccount>,

    #[account(init, payer = admin, space = Manager::SPACE, seeds = [vote_mint.key().as_ref()], bump)]
    pub manager: Account<'info, Manager>,

    pub payment_mint: Account<'info, token::Mint>,

    #[account(init, payer = admin, associated_token::mint = payment_mint, associated_token::authority = manager)]
    pub payment_vault: Account<'info, token::TokenAccount>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, token::Token>,

    pub metadata_program: Program<'info, TokenMetadata>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(shoey_name: String, edition_number: u64)]
pub struct Submit<'info> {
    #[account(mut, mint::decimals = 0, mint::authority = manager)]
    pub vote_mint: Box<Account<'info, token::Mint>>,

    #[account(mint::decimals = 0, mint::authority = shoey_master_edition)]
    pub shoey_master_edition_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: checked by metaplex metadata program
    #[account(seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_master_edition_mint.key().as_ref()], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_master_edition_metadata: UncheckedAccount<'info>,

    /// CHECK: checked by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_master_edition_mint.key().as_ref(), b"edition"], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_master_edition: UncheckedAccount<'info>,

    #[account(associated_token::mint = shoey_master_edition_mint, associated_token::authority = manager)]
    pub shoey_master_edition_vault: Account<'info, token::TokenAccount>,

    #[account(init, payer = user, mint::decimals = 0, mint::authority = manager)]
    pub shoey_edition_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: checked by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_edition_mint.key().as_ref()], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_edition_metadata: UncheckedAccount<'info>,

    /// CHECK: checked by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_edition_mint.key().as_ref(), b"edition"], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_edition: UncheckedAccount<'info>,

    /// CHECK: checked by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_master_edition_mint.key().as_ref(), b"edition", (edition_number / EDITION_MARKER_BIT_SIZE).to_string().as_bytes()], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_edition_marker: UncheckedAccount<'info>,

    #[account(mut, seeds = [vote_mint.key().as_ref()], bump)]
    pub manager: Account<'info, Manager>,

    pub payment_mint: Box<Account<'info, token::Mint>>,

    #[account(mut, associated_token::mint = payment_mint, associated_token::authority = manager)]
    pub payment_vault: Box<Account<'info, token::TokenAccount>>,

    #[account(init, payer = user, space = Shoey::SPACE, seeds = [manager.key().as_ref(), shoey_name.as_bytes()], bump)]
    pub shoey: Account<'info, Shoey>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init_if_needed, payer = user, associated_token::mint = vote_mint, associated_token::authority = user)]
    pub user_vote_ata: Box<Account<'info, token::TokenAccount>>,

    #[account(init, payer = user, associated_token::mint = shoey_edition_mint, associated_token::authority = user)]
    pub user_shoey_edition_ata: Box<Account<'info, token::TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, token::Token>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

    pub metadata_program: Program<'info, TokenMetadata>,
}

#[derive(Accounts)]
#[instruction(shoey_name: String)]
pub struct Vote<'info> {
    #[account(mut, mint::decimals = 0, mint::authority = manager)]
    pub vote_mint: Box<Account<'info, token::Mint>>,

    #[account(mut, seeds = [vote_mint.key().as_ref()], bump)]
    pub manager: Account<'info, Manager>,

    pub payment_mint: Account<'info, token::Mint>,

    #[account(mut, token::mint = payment_mint, token::authority = manager)]
    pub payment_vault: Account<'info, token::TokenAccount>,

    #[account(mut, seeds = [manager.key().as_ref(), shoey_name.as_bytes()], bump)]
    pub shoey: Account<'info, Shoey>,

    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(init_if_needed, payer = voter, associated_token::mint = vote_mint, associated_token::authority = voter)]
    pub voter_vote_ata: Account<'info, token::TokenAccount>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, token::Token>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

#[account]
pub struct Manager {
    pub admin: Pubkey,
    pub payment_mint: Pubkey,
    pub payment_vault: Pubkey,
    pub vote_mint: Pubkey,
    pub shoey_mint: Pubkey,
}

impl Manager {
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 32 + 32;
}

#[account]
pub struct Shoey {
    pub name: String,
    pub video_name: String,
    pub votes: u64,
    pub payment_vault: Pubkey,
    pub shoey_mint: Pubkey,
    pub manager: Pubkey,
    pub storage_account: Pubkey,
}

impl Shoey {
    pub const SPACE: usize = 8 + 100 + 50 + 8 + 32 + 32 + 32 + 32;
}

#[derive(Clone)]
pub struct TokenMetadata;

impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        mpl_token_metadata::ID
    }
}
