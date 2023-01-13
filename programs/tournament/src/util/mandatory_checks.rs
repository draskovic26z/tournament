use anchor_lang::{
    prelude::{Context, Pubkey},
    require, Key, Result,
};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

use crate::{error::TRMTError, instructions::Battle};

pub fn check_data(ctx: &Context<Battle>) -> Result<()> {
    //Check if metaplex metadata is ok
    require!(
        *ctx.accounts.p1_mpx_metadata.owner == mpl_token_metadata::ID,
        TRMTError::WrongMetaplexMetadata
    );

    //Check if metadata has good collection
    let collection_mint = "".parse::<Pubkey>().unwrap();
    let metadata = Metadata::from_account_info(&ctx.accounts.p1_mpx_metadata)?;

    require!(
        metadata.collection.unwrap().key == collection_mint,
        TRMTError::WrongCollection
    );

    //Check TA1

    //Check owner of TA1
    // require!(
    //     ctx.accounts.p1_token_acc.owner == *ctx.accounts.player1.key,
    //     TRMTError::WrongOwnerOfTA
    // );

    //Check if TA1 is for the right mint
    require!(
        ctx.accounts.p1_token_acc.mint == ctx.accounts.p1_mint.key(),
        TRMTError::WrongTokenAccountMint
    );

    //Check if the signer has the NFT
    require!(ctx.accounts.p1_token_acc.amount == 1, TRMTError::NoNftInTA);

    //Check TA2

    //Check owner of TA2
    // require!(
    //     ctx.accounts.p2_token_acc.owner == *ctx.accounts.player2.key,
    //     TRMTError::WrongOwnerOfTA
    // );

    //Check if TA2 is for the right mint
    require!(
        ctx.accounts.p2_token_acc.mint == ctx.accounts.p2_mint.key(),
        TRMTError::WrongTokenAccountMint
    );

    //Warrior metadata checks

    require!(
        ctx.accounts.warrior_metadata.mint == ctx.accounts.p1_mint.key(),
        TRMTError::WarriorMetadataWrongMint
    );

    Ok(())
}
