use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use mpl_token_metadata::instruction::verify_sized_collection_item;

#[derive(Accounts)]
pub struct VerifyNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    ///CHECK
    pub metaplex_program: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub collection_mint: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub collection_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub metadata: UncheckedAccount<'info>,
    ///CHECK
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    ///CHECK
    pub collection_authority_record: UncheckedAccount<'info>,
    #[account(mut)]
    ///CHECK
    pub collection_edition: UncheckedAccount<'info>,
    ///CHECK
    pub system_program: AccountInfo<'info>,
}

pub fn verify_nft(ctx: Context<VerifyNFT>) -> Result<()> {
    let verify_ix = verify_sized_collection_item(
        ctx.accounts.metaplex_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.user.key(),
        ctx.accounts.user.key(),
        ctx.accounts.collection_mint.key(),
        ctx.accounts.collection_metadata.key(),
        ctx.accounts.collection_edition.key(),
        Some(ctx.accounts.collection_authority_record.key()),
    );

    let verify_account_infos = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.collection_edition.to_account_info(),
        ctx.accounts.collection_authority_record.to_account_info(),
    ];

    invoke(&verify_ix, &verify_account_infos)?;

    Ok(())
}
