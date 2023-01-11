use crate::{state::*, util::check_data};
use anchor_lang::prelude::*;

use anchor_spl::token::{Mint, Token, TokenAccount};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Battle<'info> {
    #[account(mut)]
    pub player1: Signer<'info>,

    #[account()]
    pub player2: SystemAccount<'info>,

    #[account(mut)]
    pub p1_token_acc: Account<'info, TokenAccount>,

    #[account()]
    pub p2_token_acc: Account<'info, TokenAccount>,

    #[account()]
    pub p1_mint: Account<'info, Mint>,

    #[account()]
    pub p2_mint: Account<'info, Mint>,

    #[account()]
    pub p1_mpx_metadata: AccountInfo<'info>,

    #[account(mut)]
    pub warrior_metadata: Box<Account<'info, WarriorMetadata>>,

    #[account(
        init_if_needed,
        payer= player1,
        space = 8 + size_of::<Vault>(),
        seeds = [b"tournament", player1.key().as_ref(), player2.key().as_ref(), b"vault"],
        bump
    )]
    pub escrow: Box<Account<'info, Vault>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn battle(ctx: Context<Battle>) -> Result<()> {
    check_data(&ctx).expect("Failed to validate data");

    //Escrow checks
    match ctx.accounts.escrow.is_ready {
        false => {
            let vault = &mut ctx.accounts.escrow;
            vault.p1_token_account = ctx.accounts.p1_token_acc.key();
            vault.p2_token_account = ctx.accounts.p2_token_acc.key();
            vault.is_ready = true;
            vault.player1 = *ctx.accounts.player1.key;
            vault.player2 = *ctx.accounts.player2.key;
            vault.round = ctx.accounts.warrior_metadata.num_of_victories + 1;
        }
        true => {}
    }

    //...

    Ok(())
}
