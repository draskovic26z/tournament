pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod util;

use instructions::*;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod tournament {
    use super::*;

    pub fn battle(ctx: Context<Battle>) -> Result<()> {
        instructions::battle(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
