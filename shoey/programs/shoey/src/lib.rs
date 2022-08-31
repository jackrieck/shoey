use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::{associated_token, token};
use mpl_token_metadata::instruction::{
    create_master_edition_v3, create_metadata_accounts_v2,
    mint_new_edition_from_master_edition_via_token,
};
use mpl_token_metadata::state::{DataV2, EDITION_MARKER_BIT_SIZE};

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
            mint: ctx.accounts.shoey_master_edition_mint.to_account_info(),
            to: ctx.accounts.shoey_master_edition_vault.to_account_info(),
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
            ctx.accounts.shoey_master_edition_metadata.to_account_info(),
            ctx.accounts.shoey_master_edition_mint.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let metadata_ix = create_metadata_accounts_v2(
            ctx.accounts.metadata_program.key(),
            ctx.accounts.shoey_master_edition_metadata.key(),
            ctx.accounts.shoey_master_edition_mint.key(),
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
            ctx.accounts.shoey_master_edition_metadata.to_account_info(),
            ctx.accounts.shoey_master_edition_mint.to_account_info(),
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
            ctx.accounts.shoey_master_edition_mint.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.manager.key(),
            ctx.accounts.shoey_master_edition_metadata.key(),
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

        let manager = &mut ctx.accounts.manager;
        manager.vote_mint = ctx.accounts.vote_mint.key();
        manager.vote_metadata = ctx.accounts.vote_metadata.key();
        manager.shoey_master_edition_mint = ctx.accounts.shoey_master_edition_mint.key();
        manager.shoey_master_edition_metadata = ctx.accounts.shoey_master_edition_metadata.key();
        manager.shoey_master_edition = ctx.accounts.shoey_master_edition.key();
        manager.shoey_master_edition_vault = ctx.accounts.shoey_master_edition_vault.key();
        manager.payment_mint = ctx.accounts.payment_mint.key();
        manager.payment_vault = ctx.accounts.payment_vault.key();
        manager.admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn submit(ctx: Context<Submit>, shoey_name: String, edition_number: u64) -> Result<()> {
        // transfer upload payment to vault
        let transfer_upload_payment_accounts = token::Transfer {
            from: ctx.accounts.user_payment_ata.to_account_info(),
            to: ctx.accounts.payment_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_upload_payment_accounts,
            ),
            ui_amount_to_amount(1.0, ctx.accounts.payment_mint.decimals),
        )?;

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
            ctx.accounts.shoey_master_edition_vault.to_account_info(),
            ctx.accounts.manager.to_account_info(),
            ctx.accounts.shoey_master_edition_metadata.to_account_info(),
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

        if shoey_name.len() > 100 {
            return Err(error!(ErrorCode::ShoeyNameTooLong));
        }

        // set shoey account params
        let shoey = &mut ctx.accounts.shoey;
        shoey.name = shoey_name;
        shoey.manager = ctx.accounts.manager.key();
        shoey.edition_mint = ctx.accounts.shoey_edition_mint.key();
        shoey.payment_vault = ctx.accounts.shoey_payment_vault.key();
        shoey.total_votes = 0;

        // mint free votes for submitting
        let mint_votes_accounts = token::MintTo {
            mint: ctx.accounts.vote_mint.to_account_info(),
            to: ctx.accounts.user_vote_ata.to_account_info(),
            authority: ctx.accounts.manager.to_account_info(),
        };

        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                mint_votes_accounts,
                &[&[
                    ctx.accounts.vote_mint.key().as_ref(),
                    &[*ctx.bumps.get("manager").unwrap()],
                ]],
            ),
            10,
        )
    }

    pub fn vote(ctx: Context<Vote>, _shoey_name: String) -> Result<()> {
        // if votes are available, use a vote instead of payment mint
        if ctx.accounts.voter_vote_ata.amount > 0 {
            // burn a vote token
            let burn_votes_accounts = token::Burn {
                mint: ctx.accounts.vote_mint.to_account_info(),
                from: ctx.accounts.voter_vote_ata.to_account_info(),
                authority: ctx.accounts.voter.to_account_info(),
            };

            token::burn(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    burn_votes_accounts,
                ),
                1,
            )?;

            // transfer payment to the shoey vault
            let transfer_accounts = token::Transfer {
                from: ctx.accounts.payment_vault.to_account_info(),
                to: ctx.accounts.shoey_payment_vault.to_account_info(),
                authority: ctx.accounts.manager.to_account_info(),
            };

            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    transfer_accounts,
                    &[&[
                        ctx.accounts.vote_mint.key().as_ref(),
                        &[*ctx.bumps.get("manager").unwrap()],
                    ]],
                ),
                ui_amount_to_amount(1.0, ctx.accounts.payment_mint.decimals),
            )?;
        } else {
            // transfer payment to the shoey vault
            let transfer_accounts = token::Transfer {
                from: ctx.accounts.voter_payment_ata.to_account_info(),
                to: ctx.accounts.shoey_payment_vault.to_account_info(),
                authority: ctx.accounts.voter.to_account_info(),
            };

            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    transfer_accounts,
                ),
                ui_amount_to_amount(1.0, ctx.accounts.payment_mint.decimals),
            )?;
        }

        // increment total votes by 1
        let shoey = &mut ctx.accounts.shoey;
        shoey.total_votes += 1;

        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, shoey_name: String) -> Result<()> {
        // transfer everything from the shoey vault to the owner
        let transfer_payment_accounts = token::Transfer {
            from: ctx.accounts.shoey_payment_vault.to_account_info(),
            to: ctx.accounts.shoey_owner_payment_ata.to_account_info(),
            authority: ctx.accounts.shoey.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_payment_accounts,
                &[&[
                    ctx.accounts.manager.key().as_ref(),
                    shoey_name.as_bytes(),
                    &[*ctx.bumps.get("shoey").unwrap()],
                ]],
            ),
            ctx.accounts.shoey_payment_vault.amount,
        )
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
    pub shoey_master_edition_mint: Box<Account<'info, token::Mint>>,

    /// CHECK: initialized by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_master_edition_mint.key().as_ref()], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_master_edition_metadata: UncheckedAccount<'info>,

    /// CHECK: initialized by metaplex metadata program
    #[account(mut, seeds = [b"metadata", mpl_token_metadata::ID.as_ref(), shoey_master_edition_mint.key().as_ref(), b"edition"], bump, seeds::program = mpl_token_metadata::ID)]
    pub shoey_master_edition: UncheckedAccount<'info>,

    #[account(init, payer = admin, associated_token::mint = shoey_master_edition_mint, associated_token::authority = manager)]
    pub shoey_master_edition_vault: Box<Account<'info, token::TokenAccount>>,

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
    pub shoey_master_edition_vault: Box<Account<'info, token::TokenAccount>>,

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

    #[account(init, payer = user, associated_token::mint = payment_mint, associated_token::authority = shoey)]
    pub shoey_payment_vault: Box<Account<'info, token::TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, associated_token::mint = payment_mint, associated_token::authority = user)]
    pub user_payment_ata: Box<Account<'info, token::TokenAccount>>,

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

    pub payment_mint: Box<Account<'info, token::Mint>>,

    #[account(mut, token::mint = payment_mint, token::authority = manager)]
    pub payment_vault: Box<Account<'info, token::TokenAccount>>,

    #[account(mut, seeds = [manager.key().as_ref(), shoey_name.as_bytes()], bump)]
    pub shoey: Account<'info, Shoey>,

    #[account(mut, associated_token::mint = payment_mint, associated_token::authority = shoey)]
    pub shoey_payment_vault: Box<Account<'info, token::TokenAccount>>,

    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(init_if_needed, payer = voter, associated_token::mint = payment_mint, associated_token::authority = voter)]
    pub voter_payment_ata: Box<Account<'info, token::TokenAccount>>,

    #[account(init_if_needed, payer = voter, associated_token::mint = vote_mint, associated_token::authority = voter)]
    pub voter_vote_ata: Box<Account<'info, token::TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, token::Token>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(shoey_name: String)]
pub struct Claim<'info> {
    #[account(mut, mint::decimals = 0, mint::authority = manager)]
    pub vote_mint: Box<Account<'info, token::Mint>>,

    pub payment_mint: Box<Account<'info, token::Mint>>,

    #[account(mut, associated_token::mint = payment_mint, associated_token::authority = manager)]
    pub payment_vault: Box<Account<'info, token::TokenAccount>>,

    #[account(mut, has_one = vote_mint, has_one = payment_mint, has_one = payment_vault, seeds = [vote_mint.key().as_ref()], bump)]
    pub manager: Account<'info, Manager>,

    #[account(mut)]
    pub shoey_owner: Signer<'info>,

    #[account(init_if_needed, payer = shoey_owner, associated_token::mint = payment_mint, associated_token::authority = shoey_owner)]
    pub shoey_owner_payment_ata: Box<Account<'info, token::TokenAccount>>,

    #[account(constraint = shoey_owner_edition_mint_ata.amount == 1, associated_token::mint = shoey_edition_mint, associated_token::authority = shoey_owner)]
    pub shoey_owner_edition_mint_ata: Account<'info, token::TokenAccount>,

    #[account(mut, has_one = manager, seeds = [manager.key().as_ref(), shoey_name.as_bytes()], bump)]
    pub shoey: Account<'info, Shoey>,

    #[account(mut, associated_token::mint = payment_mint, associated_token::authority = shoey)]
    pub shoey_payment_vault: Box<Account<'info, token::TokenAccount>>,

    pub shoey_edition_mint: Box<Account<'info, token::Mint>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Program<'info, token::Token>,

    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

#[account]
pub struct Manager {
    pub vote_mint: Pubkey,
    pub vote_metadata: Pubkey,
    pub shoey_master_edition_mint: Pubkey,
    pub shoey_master_edition_metadata: Pubkey,
    pub shoey_master_edition: Pubkey,
    pub shoey_master_edition_vault: Pubkey,
    pub payment_mint: Pubkey,
    pub payment_vault: Pubkey,
    pub admin: Pubkey,
}

impl Manager {
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 32 + 32 + 32 + 32 + 32 + 32;
}

#[account]
pub struct Shoey {
    pub name: String,
    pub manager: Pubkey,
    pub edition_mint: Pubkey,
    pub payment_vault: Pubkey,
    pub total_votes: u64,
}

impl Shoey {
    pub const SPACE: usize = 8 + 100 + 32 + 32 + 32 + 8;
}

#[derive(Clone)]
pub struct TokenMetadata;

impl anchor_lang::Id for TokenMetadata {
    fn id() -> Pubkey {
        mpl_token_metadata::ID
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Shoey Name Too Long")]
    ShoeyNameTooLong,
}

/// Convert the UI representation of a token amount (using the decimals field defined in its mint)
/// to the raw amount
pub fn ui_amount_to_amount(ui_amount: f64, decimals: u8) -> u64 {
    (ui_amount * 10_usize.pow(decimals as u32) as f64) as u64
}

/// Convert a raw amount to its UI representation (using the decimals field defined in its mint)
pub fn amount_to_ui_amount(amount: u64, decimals: u8) -> f64 {
    amount as f64 / 10_usize.pow(decimals as u32) as f64
}
