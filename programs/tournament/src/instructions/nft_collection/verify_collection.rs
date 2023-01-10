use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::{mint_to, MintTo, TokenAccount};
use mpl_token_metadata::instruction::{approve_collection_authority, create_metadata_accounts_v3};
use mpl_token_metadata::state::Creator;
use mpl_token_metadata::{instruction::create_master_edition_v3, state::CollectionDetails::V1};

#[derive(Accounts)]
pub struct VerifyCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    ///CHECK
    pub metaplex_program: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub collection_mint: UncheckedAccount<'info>,
    #[account(
        init,
        seeds=[user.key().as_ref(),collection_mint.key().as_ref()],
        bump,
        token::mint=collection_mint,
        token::authority=user,payer=user
    )]
    pub collection_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,
    ///CHECK
    pub system_program: AccountInfo<'info>,
    ///CHECK
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    ///CHECK
    pub collection_authority_record: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub edition: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn verify_collection(ctx: Context<VerifyCollection>) -> Result<()> {
    //create the collection metadata, collection field is set to NONE, and collection details SHOULD be set
    let create_collection_metadata_ix = create_metadata_accounts_v3(
        ctx.accounts.metaplex_program.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        String::from("The Tournament"),
        String::from("TRMT"),
        String::from("Collection URI"),
        Some(vec![Creator {
            address: ctx.accounts.user.key(),
            share: 100,
            verified: true,
        }]),
        0_u16,
        true,
        false,
        None,
        None,
        Some(V1 { size: (0) }),
    );

    let account_infos = vec![
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];

    invoke(&create_collection_metadata_ix, &account_infos)?;

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                authority: ctx.accounts.user.to_account_info(),
                mint: ctx.accounts.collection_mint.to_account_info(),
                to: ctx.accounts.collection_token_account.to_account_info(),
            },
        ),
        1,
    )?;

    let master_edition_ix = create_master_edition_v3(
        ctx.accounts.metaplex_program.key(),
        ctx.accounts.edition.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.user.key(),
        Some(0),
    );

    let master_account_infos = vec![
        ctx.accounts.edition.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];

    invoke(&master_edition_ix, &master_account_infos)?;

    let approve_collection_authority_ix = approve_collection_authority(
        ctx.accounts.metaplex_program.key(),
        ctx.accounts.collection_authority_record.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.collection_mint.key(),
    );

    let approve_accounts = vec![
        ctx.accounts.metaplex_program.to_account_info(),
        ctx.accounts.collection_authority_record.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
    ];

    invoke(&approve_collection_authority_ix, &approve_accounts)?;

    Ok(())
}
