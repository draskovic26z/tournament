use crate::{
    error::TRMTError,
    state::*,
    util::{check_data, commence_battle},
};
use anchor_lang::prelude::*;

use anchor_spl::token::*;
use mpl_token_metadata::instruction::{freeze_delegated_account, thaw_delegated_account};
use solana_program::program::invoke;
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

    ///CHECK
    #[account()]
    pub p1_mpx_metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub warrior_metadata: Account<'info, WarriorMetadata>,

    #[account(mut)]
    pub opponent_metadata: Account<'info, WarriorMetadata>,

    #[account(init_if_needed, payer = player1, space = 8 + size_of::<Arena>(), seeds = [b"tournament", player1.key().as_ref(), player2.key().as_ref(), b"arena"], bump)]
    pub arena: Box<Account<'info, Arena>>,

    #[account(init, seeds=[b"escrow", arena.key().as_ref(), player1.key().as_ref()], bump, payer = player1, token::mint = p1_mint, token::authority = escrow)]
    pub escrow: Box<Account<'info, TokenAccount>>,

    ///CHECK
    pub p1_edition: UncheckedAccount<'info>,

    ///CHECK
    pub p2_edition: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,

    ///CHECK
    pub metaplex_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn battle(ctx: Context<Battle>) -> Result<()> {
    //Data checks
    check_data(&ctx).expect("Failed to validate data");

    let arena = &mut ctx.accounts.arena;

    anchor_spl::token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Approve {
                to: ctx.accounts.p1_token_acc.to_account_info(),
                delegate: arena.to_account_info(),
                authority: ctx.accounts.player1.to_account_info(),
            },
        ),
        1,
    )?;

    let freeze_ix = freeze_delegated_account(
        ctx.accounts.metaplex_program.key(),
        arena.key(),
        ctx.accounts.p1_token_acc.key(),
        ctx.accounts.p1_edition.key(),
        ctx.accounts.p1_mint.key(),
    );

    let account_infos = vec![
        arena.to_account_info(),
        ctx.accounts.p1_token_acc.to_account_info(),
        ctx.accounts.p1_edition.to_account_info(),
        ctx.accounts.p1_mint.to_account_info(),
    ];

    invoke(&freeze_ix, &account_infos)?;

    if arena.is_ready {
        arena.p1_escrow = ctx.accounts.p1_token_acc.key();
        arena.p2_escrow = ctx.accounts.p2_token_acc.key();
        arena.is_ready = true;
        arena.player1 = *ctx.accounts.player1.key;
        arena.player2 = *ctx.accounts.player2.key;
        arena.round = ctx.accounts.warrior_metadata.num_of_victories + 1;
        arena.warrior_metadata1 = ctx.accounts.warrior_metadata.key();
        arena.warrior_metadata2 = ctx.accounts.opponent_metadata.key();
        arena.p1_escrow = ctx.accounts.escrow.key();
    } else {
        require!(
            arena.p2_escrow.key() == ctx.accounts.p1_token_acc.key(),
            TRMTError::WrongCollection //todo
        );
        require!(
            arena.player1.key() == ctx.accounts.player2.key(),
            TRMTError::WrongCollection //todo
        );

        arena.warrior_metadata2 = ctx.accounts.warrior_metadata.key();
        arena.p2_escrow = ctx.accounts.escrow.key();

        //todo send nft token

        let war_meta_1 = &ctx.accounts.opponent_metadata;
        let war_meta_2 = &ctx.accounts.warrior_metadata;

        let mut p1 = Warrior::new(
            war_meta_1.attack,
            war_meta_1.defense,
            war_meta_1.armor,
            war_meta_1.armor_pen,
            war_meta_1.background,
            war_meta_1.helmet,
            war_meta_1.body,
            war_meta_1.hand,
            war_meta_1.weapon,
        );

        let mut p2 = Warrior::new(
            war_meta_2.attack,
            war_meta_2.defense,
            war_meta_2.armor,
            war_meta_2.armor_pen,
            war_meta_2.background,
            war_meta_2.helmet,
            war_meta_2.body,
            war_meta_2.hand,
            war_meta_2.weapon,
        );

        p1.trait_buffs(&p2);
        p2.trait_buffs(&p1);

        commence_battle(&mut p1, &mut p2);

        if p1.won {
            ctx.accounts.opponent_metadata.num_of_victories += 1;
        } else {
            ctx.accounts.warrior_metadata.num_of_victories += 1;
        }

        let thaw1_ix = thaw_delegated_account(
            ctx.accounts.metaplex_program.key(),
            arena.key(),
            ctx.accounts.p1_token_acc.key(),
            ctx.accounts.p1_edition.key(),
            ctx.accounts.p1_mint.key(),
        );

        let account_infos1 = vec![
            arena.to_account_info(),
            ctx.accounts.p1_token_acc.to_account_info(),
            ctx.accounts.p1_edition.to_account_info(),
            ctx.accounts.p1_mint.to_account_info(),
        ];

        invoke(&thaw1_ix, &account_infos1)?;

        let thaw2_ix = thaw_delegated_account(
            ctx.accounts.metaplex_program.key(),
            arena.key(),
            ctx.accounts.p2_token_acc.key(),
            ctx.accounts.p2_edition.key(),
            ctx.accounts.p2_mint.key(),
        );

        let account_infos2 = vec![
            arena.to_account_info(),
            ctx.accounts.p2_token_acc.to_account_info(),
            ctx.accounts.p2_edition.to_account_info(),
            ctx.accounts.p2_mint.to_account_info(),
        ];

        invoke(&thaw2_ix, &account_infos2)?;
    }
    Ok(())
}
