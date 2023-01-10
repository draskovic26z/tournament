pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tournament {
    use super::*;

    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
        instructions::mint_nft(ctx)
    }

    pub fn verify_collection(ctx: Context<VerifyCollection>) -> Result<()> {
        instructions::verify_collection(ctx)
    }

    pub fn verify_nft(ctx: Context<VerifyNFT>) -> Result<()> {
        instructions::verify_nft(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
