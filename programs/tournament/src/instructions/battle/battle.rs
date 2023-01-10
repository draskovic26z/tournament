use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};
use mpl_token_metadata::state::{Collection, Creator};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Battle<'info> {
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

pub fn prepare_for_battle(ctx: Context<Battle>) -> Result<()> {
    Ok(())
}
