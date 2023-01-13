use crate::{
    error::TRMTError,
    state::*,
    util::{check_data, commence_battle, update_arena},
};
use anchor_lang::{prelude::*, solana_program::info};

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
    pub p1_mpx_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub warrior_metadata: Account<'info, WarriorMetadata>,

    #[account(mut)]
    pub opponent_metadata: Account<'info, WarriorMetadata>,

    #[account(
        init_if_needed,
        payer = player1,
        space = 8 + size_of::<Arena>(),
        seeds = [b"tournament", player1.key().as_ref(), player2.key().as_ref(), b"arena"],
        bump
    )]
    pub escrow: Box<Account<'info, Arena>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn battle(ctx: Context<Battle>) -> Result<()> {
    check_data(&ctx).expect("Failed to validate data");

    let arena = &mut ctx.accounts.escrow;
    //Escrow checks
    match arena.is_ready {
        false => {
            arena.p1_token_account = ctx.accounts.p1_token_acc.key();
            arena.p2_token_account = ctx.accounts.p2_token_acc.key();
            arena.is_ready = true;
            arena.player1 = *ctx.accounts.player1.key;
            arena.player2 = *ctx.accounts.player2.key;
            arena.round = ctx.accounts.warrior_metadata.num_of_victories + 1;
            arena.warrior_metadata1 = ctx.accounts.warrior_metadata.key();
            // Write warrior metadata to the arena state account
            // arena.warrior_metadata1 = ctx.accounts.warrior_metadata.;

            //approve nft ta
        }
        true => {
            require!(
                arena.p2_token_account.key() == ctx.accounts.p1_token_acc.key(),
                TRMTError::WrongCollection //todo
            );
            require!(
                arena.player1.key() == ctx.accounts.player2.key(),
                TRMTError::WrongCollection //todo
            );

            arena.warrior_metadata2 = ctx.accounts.warrior_metadata.key();

            //approve nft ta

            //Get warrior metadata
            let war_meta_1 = &ctx.accounts.warrior_metadata;
            let war_meta_2 = &ctx.accounts.opponent_metadata;

            //Make it into a struct for easier fighting and readability
            // let mut fighter1: Fighter = Fighter {
            //     attack: war_meta_1.attack,
            //     defense: (),
            //     attack_speed: (),
            //     armor: (),
            // };

            // let mut fighter2: Fighter = Fighter {
            //     attack: war_meta_2.attack,
            //     defense: (),
            //     attack_speed: (),
            //     armor: (),
            // };

            let mut counter: u8 = 1;
            loop {
                if counter == 10_u8 {
                    break;
                };
                //..

                //todo

                //Increment
                counter += 1;
            }
        }
    }

    Ok(())
}
