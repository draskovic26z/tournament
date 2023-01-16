use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};
use mpl_token_metadata::state::{Collection, Creator};
use std::mem::size_of;

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    ///CHECK
    pub metaplex_program: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub metadata_account: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    ///CHECK
    pub collection_mint: UncheckedAccount<'info>,

    #[account(mut)]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer=user,
        space= 8 + size_of::<WarriorMetadata>(),
        seeds= [
            b"metadata",
            program_id.as_ref(),
            mint.key().as_ref(),
            b"warrior"
        ],
        bump
        )]
    pub warrior_metadata: Box<Account<'info, WarriorMetadata>>,
    #[account(mut)]
    ///CHECK
    pub edition: UncheckedAccount<'info>,
    ///CHECK
    pub system_program: AccountInfo<'info>,
    ///CHECK
    pub token_program: AccountInfo<'info>,
}

pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
    //needs minting also on chain

    let create_metadata_ix = create_metadata_accounts_v3(
        ctx.accounts.metaplex_program.key(),
        ctx.accounts.metadata_account.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.user.key(), //mint auth
        ctx.accounts.user.key(), // payer
        ctx.accounts.user.key(), // update auth
        String::from("Warrior #1"),
        String::from("TRMT"),
        String::from("Uri"),
        Some(vec![Creator {
            address: ctx.accounts.user.key(),
            share: 100,
            verified: true,
        }]), //creators
        0_u16, //seller fee
        true,  //update authority is signer
        false, //is mutable
        Some(Collection {
            key: ctx.accounts.collection_mint.key(),
            verified: false,
        }), //collection
        None,  //uses
        None,  //collection details
    );

    let account_infos = vec![
        ctx.accounts.metadata_account.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];

    invoke(&create_metadata_ix, &account_infos)?;

    let master_edition_ix = create_master_edition_v3(
        ctx.accounts.metaplex_program.key(),
        ctx.accounts.edition.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        ctx.accounts.metadata_account.key(),
        ctx.accounts.user.key(),
        Some(0),
    );

    let master_edition_account_infos = vec![
        ctx.accounts.edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.metadata_account.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];

    invoke(&master_edition_ix, &master_edition_account_infos)?;

    let warrior_metadata: &mut Account<WarriorMetadata> = &mut ctx.accounts.warrior_metadata;
    warrior_metadata.mint = ctx.accounts.mint.key();
    warrior_metadata.attack = 0;
    warrior_metadata.defense = 0;
    warrior_metadata.armor = 0;
    warrior_metadata.armor_pen = 0;

    Ok(())
}
